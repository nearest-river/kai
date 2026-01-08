
use crate::{
  lexer::TokenHint,
  token::literals::numbers::*,
};


#[repr(u8)]
#[derive(Debug,PartialEq,Eq)]
pub(crate) enum NumberLexTracker {
  Dec(DecProps),
  Hex,
  Oct,
  Bin,
}

#[repr(transparent)]
#[derive(Debug,PartialEq,Eq,Default)]
pub(crate) struct DecProps {
  flags: u8,
}


impl NumberLexTracker {
  pub fn number_starts(buf: &[u8])-> Option<Self> {
    let tracker=match buf {
      buf if buf.starts_with(HEX_PREFIX)=> Self::Hex,
      buf if buf.starts_with(BIN_PREFIX)=> Self::Bin,
      buf if buf.starts_with(OCT_PREFIX)=> Self::Oct,
      buf if buf[0].is_ascii_digit()=> Self::Dec(DecProps::new()),
      _=> return None
    };

    Some(tracker)
  }

  #[inline]
  pub const fn prefix_len(&self)-> Option<usize> {
    let prefix_len=match self {
      Self::Hex=> HEX_PREFIX.len(),
      Self::Bin=> BIN_PREFIX.len(),
      Self::Oct=> OCT_PREFIX.len(),
      _=> return None
    };

    Some(prefix_len)
  }

  #[inline]
  pub fn number_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    match (&mut *self,buf[0]) {
      (Self::Hex,b'0'..=b'9'|b'a'..=b'f'|b'A'..=b'F'|b'_')=> return None,
      (Self::Oct,b'0'..=b'7'|b'_')=> return None,
      (Self::Bin,b'0'|b'1'|b'_')=> return None,
      (Self::Dec(_),b'0'..=b'9'|b'_')=> return None,
      (Self::Dec(props),b'.')=> return props.toggle_dot().then_some(TokenHint::INFERRED_FLOAT),
      (Self::Dec(props),b'e'|b'E')=> return props.toggle_exp().then_some(TokenHint::INFERRED_FLOAT),
      (Self::Dec(props),b'-')=> return props.toggle_neg().then_some(TokenHint::INFERRED_FLOAT),
      _=> (),
    };

    for &suffix in INT_SUFFIXES {
      if starts_with_ignore_ascii_case(buf,suffix) && let Some(kind)=IntKind::from_suffix(suffix) {
        return Some(TokenHint::Int(Some(kind)));
      }
    }

    match self {
      Self::Dec(_)=> (),
      _=> return Some(TokenHint::INFERRED_INT)
    }

    for &suffix in FLOAT_SUFFIXES {
      if starts_with_ignore_ascii_case(buf,suffix) && let Some(kind)=FloatKind::from_suffix(suffix) {
        return Some(TokenHint::Float(Some(kind)));
      }
    }

    match self {
      Self::Dec(props) if props.none()=> Some(TokenHint::INFERRED_INT),
      Self::Dec(_)=> Some(TokenHint::INFERRED_FLOAT),
      _=> unreachable!(),
    }
  }
}

impl DecProps {
  const DOT: u8=1;
  const EXP: u8=2;
  const NEG: u8=3;


  #[inline]
  const fn new()-> Self {
    Self { flags: 0 }
  }

  const fn dot(&self)-> bool {
    self.flags & (1 << Self::DOT) != 0
  }

  const fn toggle_dot(&mut self)-> bool {
    let hit=self.dot();
    self.flags^=1 << Self::DOT;
    hit
  }

  const fn exp(&self)-> bool {
    self.flags & (1 << Self::EXP) != 0
  }

  const fn toggle_exp(&mut self)-> bool {
    let hit=self.exp();
    self.flags^=1 << Self::EXP;
    hit
  }

  const fn neg(&self)-> bool {
    self.flags & (1 << Self::NEG) != 0
  }

  const fn toggle_neg(&mut self)-> bool {
    let hit=self.neg();
    self.flags^=1 << Self::NEG;
    hit
  }

  const fn none(&self)-> bool {
    self.flags==0
  }
}




#[inline]
const fn starts_with_ignore_ascii_case(heystack: &[u8],needle: &[u8])-> bool {
  let n=needle.len();
  if heystack.len()<n {
    return false;
  }

  let mut i=0usize;
  while i<n {
    if !heystack[i].eq_ignore_ascii_case(&needle[i]) {
      return false;
    }

    i+=1;
  }

  true
}












