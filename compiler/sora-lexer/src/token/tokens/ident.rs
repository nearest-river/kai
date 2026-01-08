
use super::TokenExt;
use crate::prelude::*;


#[derive(Clone)]
pub struct Ident {
  pub repr: Box<str>,
  pub span: Span,
  pub raw: bool,
  _marker: ProcMacroAutoTraits,
}

impl Ident {
  pub const RAW_PREFIX: &[u8]=b"r#";

  pub fn parse_token(buf: &[u8],span: Span,raw: bool)-> Token {
    const OFF: usize=Ident::RAW_PREFIX.len();
    let reason=Some(Reason::Other("invalid identifier".into()));

    let repr_buf=if raw {
      if !buf.starts_with(Self::RAW_PREFIX) || buf.len()<=Self::RAW_PREFIX.len() {
        return Illegal::new(buf,span,reason).into_token();
      }

      &buf[OFF..]
    } else {
      buf
    };

    let repr=str::from_utf8(repr_buf)
    .expect("ain't it supposed to be utf-8, eh?")
    .into();

    match raw {
      false if !Self::validate_ident(repr)=> {
        Illegal::new(buf,span,reason)
        .into_token()
      },
      true if !Self::validate_ident_raw(repr)=> {
        let reason=Reason::Other("invalid raw identifier".into());
        Illegal::new(buf,span,Some(reason))
        .into_token()
      },
      _=> Self::new_unchecked(repr,span,raw).into_token(),
    }
  }

  #[inline]
  pub(crate) fn new_unchecked(repr: &str,span: Span,raw: bool)-> Self {
    Self {
      raw,
      span,
      repr: repr.into(),
      _marker: MARKER,
    }
  }

  #[inline]
  #[must_use]
  pub(crate) fn is_start(ch: char)-> bool {
    ch=='_' || unicode_ident::is_xid_start(ch)
  }

  #[inline]
  #[must_use]
  pub(crate) fn is_continue(ch: char)-> bool {
    unicode_ident::is_xid_continue(ch)
  }

  #[must_use]
  #[track_caller]
  fn validate_ident(s: &str)-> bool {
    debug_assert!(!s.is_empty());
    if s.bytes().all(|digit| b'0'<=digit && digit<=b'9') {
      return false;
    }

    fn ident_ok(s: &str)-> bool {
      let mut chars=s.chars();

      let ch0=chars.next().unwrap();
      if !Ident::is_start(ch0) {
        return false;
      }

      for ch in chars {
        if !Ident::is_continue(ch) {
          return false;
        }
      }

      true
    }

    ident_ok(s)
  }

  #[track_caller]
  fn validate_ident_raw(s: &str)-> bool {
    match s {
      "_"|"super"|"self"|"Self"|"crate"=> false,
      s=> Self::validate_ident(s),
    }
  }
}


impl_repr_tokens! {
  Ident
}







