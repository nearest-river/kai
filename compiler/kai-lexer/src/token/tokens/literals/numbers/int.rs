
use crate::{
  prelude::*,
  lexer::tracker::IntKind,
};

#[derive(Clone)]
pub struct Int {
  pub repr: Box<str>,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

impl_literal_tokens! {
  Int
}

impl Int {
  #[inline]
  pub fn parse_token(buf: &[u8],span: Span,_kind: Option<IntKind>)-> Token {
    let repr=str::from_utf8(buf)
    .expect("ain't it supposed to be utf-8")
    .into();
    Self {
      span,
      repr,
      _marker: MARKER
    }.into_token()
  }
}



