
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Formatter,
};


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
  CStr,
  RStr,
}


impl Str {
  #[inline]
  pub fn parse_token(buf: &[u8],span: Span,kind: StrKind)-> Token {
    let repr=match buf {
      b"\"\""=> Some("".into()),
      buf if buf.len()<3=> {
        let reason=Some(Reason::Other("not valid string literal".into()));
        return Illegal::new(buf,span,reason).into_token();
      },
      buf=> {
        let unquoted=&buf[kind.prefix_len()..buf.len()-kind.suffix_len()];
        str::from_utf8(unquoted)
        .ok()
        .and_then(unescape::unescape)
      },
    };

    let repr=match repr {
      Some(repr)=> repr.into_boxed_str(),
      None=> return Illegal::new(buf,span,Some(Reason::Other("invalid escape sequense".into()))).into_token(),
    };

    Self {
      span,
      repr,
      kind,
      _marker: MARKER
    }.into_token()
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
      Self::CStr=> Self::CSTR_PREFIX,
      Self::RStr=> Self::RSTR_PREFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Str|Self::BStr|Self::CStr=> Self::SUFFIX,
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
      Self::CSTR_PREFIX=> Some(Self::CStr),
      Self::RSTR_PREFIX=> Some(Self::RStr),
      _=> None,
    }
  }
}

impl TokenExt for Str {
  #[inline]
  fn into_token(self)-> Token {
    Token::Str(self)
  }
}


impl Debug for Str {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      f.write_str(stringify!(Str))?;
      return write!(f,"({:#?})",self.repr);
    }

    let mut dbg=f.debug_struct(stringify!(Str));

    dbg.field("repr",&self.repr);
    dbg.field("span",&self.span);
    dbg.field("kind",&self.kind);
    dbg.finish()
  }
}

impl Eq for Str {}
impl PartialEq for Str {
  fn eq(&self,other: &Self)-> bool {
    self.kind==other.kind && self.repr==other.repr
  }
}

impl<S: AsRef<str>> PartialEq<S> for Str {
  fn eq(&self,other: &S) -> bool {
    other.as_ref()==&*self.repr
  }
}

impl Hash for Str {
  fn hash<H: Hasher>(&self,state: &mut H) {
    self.repr.hash(state);
  }
}


