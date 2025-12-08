
mod numbers;

pub use numbers::*;
use crate::{
  prelude::*,
  lexer::tracker::{
    StrKind,
    CharKind,
  },
};


#[derive(Clone)]
pub struct Str {
  pub repr: Box<str>,
  pub span: Span,
  pub kind: StrKind,
  _marker: ProcMacroAutoTraits,
}

#[derive(Clone)]
pub struct Char {
  pub repr: char,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

#[derive(Clone)]
pub struct Bool {
  pub repr: bool,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

impl_literal_tokens! {
  Str
  Char
  Bool
}


impl Str {
  #[inline]
  pub fn parse_token(buf: &[u8],span: Span,kind: StrKind)-> Token {
    let repr=str::from_utf8(buf)
    .expect("ain't it supposed to be utf-8")
    .into();
    Self {
      span,
      repr,
      kind,
      _marker: MARKER
    }.into_token()
  }
}

impl Char {
  #[inline]
  pub fn parse_token(_buf: &[u8],span: Span,_kind: CharKind)-> Token {
    Self {
      span,
      repr: '\0',
      _marker: MARKER
    }.into_token()
  }
}

impl Bool {
  #[inline]
  pub fn new(repr: bool,span: Span)-> Self {
    Self {
      repr,
      span,
      _marker: MARKER,
    }
  }
}





impl PartialEq<bool> for Bool {
  fn eq(&self,&other: &bool) -> bool {
    self.repr==other
  }
}

impl<S: AsRef<str>> PartialEq<S> for Str {
  fn eq(&self,other: &S) -> bool {
    other.as_ref()==&*self.repr
  }
}

impl PartialEq<char> for Char {
  fn eq(&self,&other: &char)-> bool {
    self.repr==other
  }
}

impl PartialEq<u8> for Char {
  fn eq(&self,&other: &u8)-> bool {
    self.repr==other as char
  }
}













