
use crate::prelude::*;

pub static HEX_PREFIX: &[u8]=b"0x";
pub static OCT_PREFIX: &[u8]=b"0o";
pub static BIN_PREFIX: &[u8]=b"0b";

pub static INT_SUFFIXES: &[&[u8]]=&[
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



#[derive(Clone)]
pub struct Int {
  pub repr: Box<str>,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
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



impl_literal_tokens! {
  Int
}

impl Int {
  #[inline]
  pub fn parse_token(buf: &[u8],span: Span,_kind: Option<IntKind>)-> Token {
    let repr=str::from_utf8(buf)
    .expect("ain't it supposed to be utf-8")
    .into();
    Self {
      span,
      repr,
      _marker: MARKER
    }.into_token()
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


