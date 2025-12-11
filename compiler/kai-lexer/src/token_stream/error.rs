
use thiserror::Error;
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Display,
  Formatter,
};



#[derive(Error,Debug)]
pub struct LexErr {
  pub(crate) span: Span,
}

impl LexErr {
  #[inline]
  pub const fn span(&self)-> Span {
    self.span
  }

  pub(crate) fn call_site()-> Self {
    Self {
      span: Span::call_site(),
    }
  }
}

impl Display for LexErr {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    Display::fmt(&self.span,f)
  }
}






