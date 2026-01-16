
use crate::{
  prelude::*,
  lexer::Lexer,
  token_stream::{
    *,
    group::*,
  },
};

#[allow(unused)]
#[derive(PartialEq,Eq)]
enum DeliOpen {
  Paren(LParen),
  Brace(LBrace),
  Bracket(LBracket),
  // reserved for future.
  None,
}

#[allow(unused)]
#[derive(PartialEq,Eq)]
enum DeliClose {
  Paren(RParen),
  Brace(RBrace),
  Bracket(RBracket),
  // reserved for future.
  None,
}



type StackFrame=(usize,DeliOpen,TokenStreamBuilder);
pub(crate) fn parse(mut lexer: Lexer<'_>)-> Result<TokenStream,LexErr> {
  let mut stream=TokenStreamBuilder::new();
  let mut stack=Vec::<StackFrame>::new();

  while let Some(token)=lexer.next() {
    if let Some(deli)=get_opening_deli(&token) {
      let frame=(lexer.cursor(),deli,TokenStreamBuilder::new());
      stack.push(frame);
      continue;
    }

    if let Some(deli_close)=get_closing_deli(&token) {
      let (_,deli_open,tree)=match stack.pop() {
        Some((off,deli_open,_)) if deli_open!=deli_close=> return Err(lex_error(off)),
        None=> return Err(lex_error(lexer.cursor())),
        Some(frame)=> frame,
      };

      // SAFETY: 
      let deli=unsafe { deli_new(deli_open,deli_close) };
      let tt=TokenTree::Group(Group::new(deli,tree.build()));
      match stack.last_mut() {
        Some((_,_,last1))=> last1.push_token_from_parser(tt),
        None=> stream.push_token_from_parser(tt),
      }
      continue;
    }

    let leaf=Leaf::try_from(token).expect("this should be unreachable");
    match stack.last_mut() {
      None=> stream.push_token_from_parser(TokenTree::Leaf(leaf)),
      Some((_,_,last))=> {
        last.push_token_from_parser(TokenTree::Leaf(leaf));
      }
    }
  }


  Ok(stream.build())
}





impl PartialEq<DeliClose> for DeliOpen {
  fn eq(&self,other: &DeliClose)-> bool {
    type Other=DeliClose;
    match (self,other) {
      (Self::Paren(_),Other::Paren(_))=> true,
      (Self::Brace(_),Other::Brace(_))=> true,
      (Self::Bracket(_),Other::Bracket(_))=> true,
      _=> false,
    }
  }
}


#[inline]
unsafe fn deli_new(open: DeliOpen,close: DeliClose)-> Delimiter {
  match (open,close) {
    (DeliOpen::Paren(open),DeliClose::Paren(close))=> Delimiter::Paren(ParenPair::new(open,close)),
    (DeliOpen::Brace(open),DeliClose::Brace(close))=> Delimiter::Brace(BracePair::new(open,close)),
    (DeliOpen::Bracket(open),DeliClose::Bracket(close))=> Delimiter::Bracket(BracketPair::new(open,close)),
    _=> unreachable!(),
  }
}


#[inline]
const fn get_closing_deli(token: &Token)-> Option<DeliClose> {
  match &token {
    Token::RParen(token)=> Some(DeliClose::Paren(*token)),
    Token::RBrace(token)=> Some(DeliClose::Brace(*token)),
    Token::RBracket(token)=> Some(DeliClose::Bracket(*token)),
    _=> None
  }
}

#[inline]
const fn get_opening_deli(token: &Token)-> Option<DeliOpen> {
  match &token {
    Token::LParen(token)=> Some(DeliOpen::Paren(*token)),
    Token::LBrace(token)=> Some(DeliOpen::Brace(*token)),
    Token::LBracket(token)=> Some(DeliOpen::Bracket(*token)),
    _=> None
  }
}

fn lex_error(off: usize)-> LexErr {
  LexErr {
    span: Span {
      lo: off as u32,
      hi: off as u32,
    },
  }
}










