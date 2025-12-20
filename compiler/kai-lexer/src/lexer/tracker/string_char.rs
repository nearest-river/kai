
use crate::{
  prelude::*,
  lexer::TokenHint,
  token::literals::{
    string::StrKind,
    character::CharKind,
  },
};


pub(crate) struct StringCharLexTracker {
  kind: TrackerKind,
  escape_suffix: bool,
}

#[repr(u8)]
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum TrackerKind {
  Str,
  BStr,
  CStr,
  RStr,
  Char,
  BChar,
}





impl StringCharLexTracker {
  #[inline]
  pub fn sec_starts(buf: &[u8])-> Option<Self> {
    let kind=match buf {
      buf if buf.starts_with(StrKind::Str.prefix())=> TrackerKind::Str,
      buf if buf.starts_with(StrKind::BStr.prefix())=> TrackerKind::BStr,
      buf if buf.starts_with(StrKind::CStr.prefix())=> TrackerKind::CStr,
      buf if buf.starts_with(StrKind::RStr.prefix())=> TrackerKind::RStr,
      buf if buf.starts_with(CharKind::Char.prefix())=> TrackerKind::Char,
      buf if buf.starts_with(CharKind::BChar.prefix())=> TrackerKind::BChar,
      _=> return None,
    };

    Some(Self {
      kind,
      escape_suffix: false,
    })
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    match self.kind {
      TrackerKind::Str=> StrKind::Str.prefix_len(),
      TrackerKind::BStr=> StrKind::BStr.prefix_len(),
      TrackerKind::CStr=> StrKind::CStr.prefix_len(),
      TrackerKind::RStr=> StrKind::RStr.prefix_len(),
      TrackerKind::Char=> CharKind::Char.prefix_len(),
      TrackerKind::BChar=> CharKind::BChar.prefix_len(),
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    match self.kind {
      TrackerKind::Str=> StrKind::Str.suffix_len(),
      TrackerKind::BStr=> StrKind::BStr.suffix_len(),
      TrackerKind::CStr=> StrKind::CStr.suffix_len(),
      TrackerKind::RStr=> StrKind::RStr.suffix_len(),
      TrackerKind::Char=> CharKind::Char.suffix_len(),
      TrackerKind::BChar=> CharKind::BChar.suffix_len(),
    }
  }

  #[inline]
  pub fn sec_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    let hint=self.hint();

    if self.escape_suffix {
      self.escape_suffix=false;
      return None;
    }

    let cond=match (self.kind,buf[0],buf.get(1).copied()) {
      (TrackerKind::RStr,_,_) if buf.starts_with(StrKind::RSUFFIX)=> true,
      (TrackerKind::Char|TrackerKind::BChar,b'\n'|b'\r'|CharKind::SUFFIXB,_)=> true,
      (TrackerKind::Str|TrackerKind::BStr|TrackerKind::CStr,b'\n'|b'\r'|StrKind::SUFFIXB,_)=> true,
      (TrackerKind::Str|TrackerKind::BStr|TrackerKind::CStr,b'\\',Some(StrKind::SUFFIXB))=> {
        self.escape_suffix=true;
        false
      },
      (TrackerKind::Char|TrackerKind::BChar,b'\\',Some(CharKind::SUFFIXB))=> {
        self.escape_suffix=true;
        false
      },
      _=> false
    };

    cond.then_some(hint)
  }

  const fn hint(&self)-> TokenHint {
    match self.kind {
      TrackerKind::Str=> TokenHint::Str(StrKind::Str),
      TrackerKind::BStr=> TokenHint::Str(StrKind::BStr),
      TrackerKind::CStr=> TokenHint::Str(StrKind::CStr),
      TrackerKind::RStr=> TokenHint::Str(StrKind::RStr),
      TrackerKind::Char=> TokenHint::Char(CharKind::Char),
      TrackerKind::BChar=> TokenHint::Char(CharKind::BChar),
    }
  }
}












