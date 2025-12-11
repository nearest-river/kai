
use crate::prelude::*;


#[derive(Clone)]
pub struct Bool {
  pub repr: bool,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

impl_literal_tokens! {
  Bool
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







