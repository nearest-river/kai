
use crate::{
  prelude::*,
  lexer::TokenHint,
  token::literals::{
    string::StrKind,
    character::CharKind,
  },
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





impl StringCharLexTracker {
  #[inline]
  pub fn sec_starts(buf: &[u8])-> Option<Self> {
    let kind=match buf {
      buf if buf.starts_with(StrKind::Str.prefix())=> Self::Str,
      buf if buf.starts_with(StrKind::BStr.prefix())=> Self::BStr,
      buf if buf.starts_with(StrKind::RStr.prefix())=> Self::RStr,
      buf if buf.starts_with(CharKind::Char.prefix())=> Self::Char,
      buf if buf.starts_with(CharKind::BChar.prefix())=> Self::BChar,
      _=> return None,
    };

    Some(kind)
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    match self {
      Self::Str=> StrKind::Str.prefix_len(),
      Self::BStr=> StrKind::BStr.prefix_len(),
      Self::RStr=> StrKind::RStr.prefix_len(),
      Self::Char=> CharKind::Char.prefix_len(),
      Self::BChar=> CharKind::BChar.prefix_len(),
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    match self {
      Self::Str=> StrKind::Str.suffix_len(),
      Self::BStr=> StrKind::BStr.suffix_len(),
      Self::RStr=> StrKind::RStr.suffix_len(),
      Self::Char=> 1,
      Self::BChar=> 1,
    }
  }

  #[inline]
  pub fn sec_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    let hint=self.hint();
    let cond=match (self,buf[0]) {
      (Self::Str,b'\n'|b'\r'|StrKind::SUFFIXB)=> true,
      (Self::BStr,b'\n'|b'\r'|StrKind::SUFFIXB)=> true,
      (Self::RStr,_) if buf.starts_with(StrKind::RSUFFIX)=> true,
      (Self::Char,b'\n'|b'\r'|CharKind::SUFFIXB)=> true,
      (Self::BChar,b'\n'|b'\r'|CharKind::SUFFIXB)=> true,
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












