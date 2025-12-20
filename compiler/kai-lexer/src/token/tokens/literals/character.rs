
use crate::{
  token::*,
  prelude::*,
};

use super::numbers::{
  Int,
  IntKind,
};

use std::fmt::{
  self,
  Debug,
  Formatter,
};


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




impl Char {
  #[inline]
  // TODO(nate): impl escape seq
  pub fn parse_token(buf: &[u8],span: Span,kind: CharKind)-> Token {
    let repr=match buf {
      buf if buf.len()<3=> {
        let reason=Some(Reason::Other("not valid char literal".into()));
        return Illegal::new(buf,span,reason).into_token();
      },
      buf=> {
        let unquoted=&buf[kind.prefix_len()..buf.len()-kind.suffix_len()];
        str::from_utf8(unquoted)
        .ok()
        .and_then(unescape::unescape)
      },
    };

    let ch=match repr {
      Some(repr)=> repr.chars().nth(0),
      None=> return Illegal::new(buf,span,Some(Reason::Other("invalid escape sequense".into()))).into_token(),
    };

    let repr=match ch {
      Some(ch)=> ch,
      None=> return Illegal::new(buf,span,Some(Reason::Other("empty char".into()))).into_token(),
    };


    match kind {
      CharKind::Char=> Char {
        span,
        repr,
        _marker: MARKER,
      }.into_token(),
      CharKind::BChar=> Int::new(repr as u8 as u128,span,Some(IntKind::U8)).into_token()
    }
  }
}

impl TokenExt for Char {
  #[inline]
  fn into_token(self)-> Token {
    Token::Char(self)
  }
}

impl Eq for Char {}
impl PartialEq for Char {
  fn eq(&self,other: &Char)-> bool {
    self.repr==other.repr
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

impl Debug for Char {
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

impl Hash for Char {
  fn hash<H: Hasher>(&self,state: &mut H) {
    self.repr.hash(state);
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




