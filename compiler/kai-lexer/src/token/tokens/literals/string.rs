
use crate::prelude::*;


#[derive(Clone)]
pub struct Str {
  pub repr: Box<str>,
  pub span: Span,
  pub kind: StrKind,
  _marker: ProcMacroAutoTraits,
}


#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum StrKind {
  Str,
  BStr,
  RStr,
}


impl_literal_tokens! {
  Str
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

impl<S: AsRef<str>> PartialEq<S> for Str {
  fn eq(&self,other: &S) -> bool {
    other.as_ref()==&*self.repr
  }
}

impl StrKind {
  pub(crate) const STR_PREFIX: &[u8]=b"\"";
  pub(crate) const BSTR_PREFIX: &[u8]=b"b\"";
  pub(crate) const CSTR_PREFIX: &[u8]=b"c\"";
  pub(crate) const RSTR_PREFIX: &[u8]=b"r#\"";

  pub(crate) const SUFFIXB: u8=b'"';
  pub(crate) const SUFFIX: &[u8]=b"\"";
  pub(crate) const RSUFFIX: &[u8]=b"\"#";

  #[inline(always)]
  pub const fn prefix(&self)-> &[u8] {
    match self {
      Self::Str=> Self::STR_PREFIX,
      Self::BStr=> Self::BSTR_PREFIX,
      Self::RStr=> Self::RSTR_PREFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Str|Self::BStr=> Self::SUFFIX,
      Self::RStr=> Self::RSUFFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> usize {
    self.prefix().len()
  }

  #[inline(always)]
  pub const fn from_prefix(pat: &[u8])-> Option<Self> {
    match pat {
      Self::STR_PREFIX=> Some(Self::Str),
      Self::BSTR_PREFIX=> Some(Self::BStr),
      Self::RSTR_PREFIX=> Some(Self::RStr),
      _=> None,
    }
  }
}




