
use crate::{
  prelude::*,
  lexer::TokenHint,
};


#[repr(u8)]
#[derive(Debug,PartialEq,Eq)]
pub(crate) enum StringCharLexTracker {
  Str,
  BStr,
  RStr,
  Char,
  BChar,
}

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum StrKind {
  Str,
  BStr,
  RStr,
}

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum CharKind {
  Char,
  BChar,
}


impl StringCharLexTracker {
  pub(crate) const DOUBLEQ_PREFIX: &[u8]=b"\"";
  pub(crate) const DOUBLEQB_PREFIX: &[u8]=b"b\"";
  pub(crate) const DOUBLEQC_PREFIX: &[u8]=b"c\"";
  pub(crate) const DOUBLEQR_PREFIX: &[u8]=b"r#\"";
  pub(crate) const SINGLEQ_PREFIX: &[u8]=b"\'";
  pub(crate) const SINGLEQB_PREFIX: &[u8]=b"b\'";

  pub(crate) const DOUBLEQ_SUFFIX: u8=b'\"';
  pub(crate) const DOUBLEQB_SUFFIX: u8=b'\"';
  pub(crate) const DOUBLEQR_SUFFIX: &[u8]=b"\"#";
  pub(crate) const SINGLEQ_SUFFIX: u8=b'\'';
  pub(crate) const SINGLEQB_SUFFIX: u8=b'\'';



  #[inline]
  pub fn sec_starts(buf: &[u8])-> Option<Self> {
    let kind=match buf {
      buf if buf.starts_with(Self::DOUBLEQ_PREFIX)=> Self::Str,
      buf if buf.starts_with(Self::DOUBLEQB_PREFIX)=> Self::BStr,
      buf if buf.starts_with(Self::DOUBLEQR_PREFIX)=> Self::RStr,
      buf if buf.starts_with(Self::SINGLEQ_PREFIX)=> Self::Char,
      buf if buf.starts_with(Self::SINGLEQB_PREFIX)=> Self::BChar,
      _=> return None,
    };

    Some(kind)
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    match self {
      Self::Str=> Self::DOUBLEQ_PREFIX.len(),
      Self::BStr=> Self::DOUBLEQB_PREFIX.len(),
      Self::RStr=> Self::DOUBLEQR_PREFIX.len(),
      Self::Char=> Self::SINGLEQ_PREFIX.len(),
      Self::BChar=> Self::SINGLEQB_PREFIX.len(),
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    match self {
      Self::Str=> 1,
      Self::BStr=> 1,
      Self::RStr=> Self::DOUBLEQR_SUFFIX.len(),
      Self::Char=> 1,
      Self::BChar=> 1,
    }
  }

  #[inline]
  pub fn sec_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    let hint=self.hint();
    let cond=match (self,buf[0]) {
      (Self::Str,b'\n'|b'\r'|Self::DOUBLEQ_SUFFIX)=> true,
      (Self::BStr,b'\n'|b'\r'|Self::DOUBLEQB_SUFFIX)=> true,
      (Self::RStr,_) if buf.starts_with(b"\"#")=> true,
      (Self::Char,b'\n'|b'\r'|Self::SINGLEQ_SUFFIX)=> true,
      (Self::BChar,b'\n'|b'\r'|Self::SINGLEQB_SUFFIX)=> true,
      _=> false
    };

    cond.then_some(hint)
  }

  const fn hint(&self)-> TokenHint {
    match self {
      Self::Str=> TokenHint::Str(StrKind::Str),
      Self::BStr=> TokenHint::Str(StrKind::BStr),
      Self::RStr=> TokenHint::Str(StrKind::RStr),
      Self::Char=> TokenHint::Char(CharKind::Char),
      Self::BChar=> TokenHint::Char(CharKind::BChar),
    }
  }
}



impl StrKind {
  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    match self {
      Self::Str=> 1,
      Self::BStr=> 1,
      Self::RStr=> StringCharLexTracker::DOUBLEQR_SUFFIX.len(),
    }
  }
}

impl CharKind {
  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    match self {
      Self::Char=> 1,
      Self::BChar=> 1,
    }
  }
}







