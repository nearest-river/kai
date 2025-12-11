
use crate::lexer::TokenHint;


pub(crate) static HEX_PREFIX: &[u8]=b"0x";
pub(crate) static OCT_PREFIX: &[u8]=b"0o";
pub(crate) static BIN_PREFIX: &[u8]=b"0b";

pub(crate) static FLOAT_SUFFIXES: &[&[u8]]=&[
  FloatKind::F16.as_str().as_bytes(),
  FloatKind::F32.as_str().as_bytes(),
  FloatKind::F64.as_str().as_bytes(),
  FloatKind::F128.as_str().as_bytes(),
];

pub(crate) static INT_SUFFIXES: &[&[u8]]=&[
  IntKind::U8.as_str().as_bytes(),
  IntKind::U16.as_str().as_bytes(),
  IntKind::U32.as_str().as_bytes(),
  IntKind::U64.as_str().as_bytes(),
  IntKind::U128.as_str().as_bytes(),
  IntKind::U256.as_str().as_bytes(),
  IntKind::Usize.as_str().as_bytes(),
  IntKind::I8.as_str().as_bytes(),
  IntKind::I16.as_str().as_bytes(),
  IntKind::I32.as_str().as_bytes(),
  IntKind::I64.as_str().as_bytes(),
  IntKind::I128.as_str().as_bytes(),
  IntKind::I256.as_str().as_bytes(),
  IntKind::Isize.as_str().as_bytes(),
];

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum FloatKind {
  F16,
  F32,
  #[default]
  F64,
  F128,
}


#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum IntKind {
  U8,
  U16,
  U32,
  U64,
  U128,
  U256,
  Usize,
  I8,
  I16,
  #[default]
  I32,
  I64,
  I128,
  I256,
  Isize,
}




#[repr(u8)]
#[derive(Debug,PartialEq,Eq)]
pub enum NumberLexTracker {
  Dec(DecProps),
  Hex,
  Oct,
  Bin,
}

#[repr(transparent)]
#[derive(Debug,PartialEq,Eq,Default)]
pub struct DecProps {
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


impl IntKind {
  #[inline]
  pub const fn as_str(&self)-> &'static str {
    match self {
      Self::U8=> "u8",
      Self::U16=> "u16",
      Self::U32=> "u32",
      Self::U64=> "u64",
      Self::U128=> "u128",
      Self::U256=> "u256",
      Self::Usize=> "usize",
      Self::I8=> "i8",
      Self::I16=> "i16",
      Self::I32=> "i32",
      Self::I64=> "i64",
      Self::I128=> "i128",
      Self::I256=> "i256",
      Self::Isize=> "isize",
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.as_str().len()
  }

  #[inline]
  pub const fn from_suffix(suffix: &[u8])-> Option<Self> {
    let kind=match suffix {
      b"u8"|b"U8"=> Self::U8,
      b"u16"|b"U16"=> Self::U16,
      b"u32"|b"U32"=> Self::U32,
      b"u64"|b"U64"=> Self::U64,
      b"u128"|b"U128"=> Self::U128,
      b"u256"|b"U256"=> Self::U256,
      b"usize"=> Self::Usize,
      b"i8"|b"I8"=> Self::I8,
      b"i16"|b"I16"=> Self::I16,
      b"i32"|b"I32"=> Self::I32,
      b"i64"|b"I64"=> Self::I64,
      b"i128"|b"I128"=> Self::I128,
      b"i256"|b"I256"=> Self::I256,
      b"isize"=> Self::Isize,
      suffix if suffix.eq_ignore_ascii_case(b"usize")=> Self::Usize,
      suffix if suffix.eq_ignore_ascii_case(b"isize")=> Self::Isize,
      _=> return None,
    };

    Some(kind)
  }
}

impl FloatKind {
  #[inline]
  pub const fn as_str(&self)-> &'static str {
    match self {
      Self::F16=> "f16",
      Self::F32=> "f32",
      Self::F64=> "f64",
      Self::F128=> "f128",
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.as_str().len()
  }

  #[inline]
  pub const fn from_suffix(suffix: &[u8])-> Option<Self> {
    let kind=match suffix {
      b"f16"|b"F16"=> Self::F16,
      b"f32"|b"F32"=> Self::F32,
      b"f64"|b"F64"=> Self::F64,
      b"f128"|b"F128"=> Self::F128,
      _=> return None,
    };

    Some(kind)
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












