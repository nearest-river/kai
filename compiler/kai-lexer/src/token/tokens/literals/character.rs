
use crate::prelude::*;


#[derive(Clone)]
pub struct Char {
  pub repr: char,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum CharKind {
  Char,
  BChar,
}


impl_literal_tokens! {
  Char
}

impl Char {
  #[inline]
  // TODO(nate): impl escape seq
  pub fn parse_token(_buf: &[u8],span: Span,_kind: CharKind)-> Token {
    Self {
      span,
      repr: '\0',
      _marker: MARKER
    }.into_token()
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













impl CharKind {
  pub(crate) const CHAR_PREFIX: &[u8]=b"\'";
  pub(crate) const BCHAR_PREFIX: &[u8]=b"b\'";

  pub(crate) const SUFFIXB: u8=b'\'';
  pub(crate) const SUFFIX: &[u8]=b"\'";

  #[inline(always)]
  pub const fn prefix(&self)-> &[u8] {
    match self {
      Self::Char=> Self::CHAR_PREFIX,
      Self::BChar=> Self::BCHAR_PREFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Char|Self::BChar=> Self::SUFFIX,
    }
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> usize {
    self.prefix().len()
  }

  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }
}



