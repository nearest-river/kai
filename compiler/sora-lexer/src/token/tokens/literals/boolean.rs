
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Formatter,
};


#[derive(Clone)]
pub struct Bool {
  pub repr: bool,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
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

impl TokenExt for Bool {
  #[inline]
  fn into_token(self)-> Token {
    Token::Bool(self)
  }

  #[inline]
  fn span(&self)-> Span {
    self.span
  }

  #[inline]
  fn set_span(&mut self,span: Span) {
    self.span=span
  }
}

impl Eq for Bool {}
impl PartialEq for Bool {
  fn eq(&self,other: &Bool)-> bool {
    self.repr==other.repr
  }
}

impl PartialEq<bool> for Bool {
  fn eq(&self,&other: &bool) -> bool {
    self.repr==other
  }
}

impl Debug for Bool {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      f.write_str(stringify!(Char))?;
      return write!(f,"({:#?})",self.repr);
    }

    let mut dbg=f.debug_struct(stringify!(Char));

    dbg.field("repr",&self.repr);
    dbg.field("span",&self.span);
    dbg.finish()
  }
}

impl Hash for Bool {
  fn hash<H: Hasher>(&self,state: &mut H) {
    self.repr.hash(state);
  }
}





