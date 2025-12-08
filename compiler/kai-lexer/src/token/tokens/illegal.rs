
use crate::prelude::*;

#[derive(Clone)]
pub struct Illegal {
  pub repr: Box<str>,
  pub span: Span,
  pub reason: Option<&'static str>,
  _marker: ProcMacroAutoTraits,
}

impl_repr_tokens! {
  Illegal
}

impl Illegal {
  pub fn new(repr: &[u8],span: Span,reason: Option<&'static str>)-> Self {
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




