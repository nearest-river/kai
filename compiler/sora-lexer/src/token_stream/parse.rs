
use crate::{
  prelude::*,
  lexer::Lexer,
  token_stream::*,
};


type StackFrame=(usize,Delimiter,TokenStreamBuilder);
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
      let (_,deli,tree)=match stack.pop() {
        Some((off,deli_open,_)) if deli_open!=deli_close=> return Err(lex_error(off)),
        None=> return Err(lex_error(lexer.cursor())),
        Some(frame)=> frame,
      };

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



#[inline]
const fn get_closing_deli(token: &Token)-> Option<Delimiter> {
  match &token {
    Token::RParen(_)=> Some(Delimiter::Paren),
    Token::RBrace(_)=> Some(Delimiter::Brace),
    Token::RBracket(_)=> Some(Delimiter::Bracket),
    _=> None
  }
}

#[inline]
const fn get_opening_deli(token: &Token)-> Option<Delimiter> {
  match &token {
    Token::LParen(_)=> Some(Delimiter::Paren),
    Token::LBrace(_)=> Some(Delimiter::Brace),
    Token::LBracket(_)=> Some(Delimiter::Bracket),
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










