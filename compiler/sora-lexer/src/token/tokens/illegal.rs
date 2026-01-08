
use std::str::Utf8Error;

use crate::{
  prelude::*,
  error::LiteralParseErr,
};


#[derive(Clone)]
pub struct Illegal {
  pub repr: Box<str>,
  pub span: Span,
  pub reason: Option<Reason>,
  _marker: ProcMacroAutoTraits,
}

#[derive(Clone,Debug)]
pub enum Reason {
  ParseCommentErr(&'static str),
  ParseLiteralErr(LiteralParseErr),
  Utf8Error(Utf8Error),
  Other(Box<str>),
}


impl_repr_tokens! {
  Illegal
}

impl Illegal {
  pub fn new(repr: &[u8],span: Span,reason: Option<Reason>)-> Self {
    let repr=str::from_utf8(repr)
    .expect("ain't it supposed to be utf-8, eh?")
    .into();

    Self {
      repr,
      span,
      reason,
      _marker: MARKER,
    }
  }
}


impl From<LiteralParseErr> for Reason {
  #[inline]
  fn from(err: LiteralParseErr)-> Self {
    Self::ParseLiteralErr(err)
  }
}



