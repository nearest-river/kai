
use crate::{
  prelude::*,
  lexer::Lexer,
  token_stream::*,
};


type StackFrame=(usize,Delimiter,TokenStreamBuilder);
pub fn parse(buf: &[u8])-> Result<TokenStream,LexErr> {
  let mut trees=TokenStreamBuilder::new();
  let mut stack=Vec::<StackFrame>::new();

  let mut lexer=Lexer::new(buf);
  while let Some(token)=lexer.next() {
    if let Some(deli_open)=get_opening_deli(&token) {
      todo!("{deli_open:#?}");
    } else if let Some(deli_close)=get_closing_deli(&token) {

    }




  }



  todo!()
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










