
use crate::prelude::*;
use std::{
  fmt::{
    self,
    Debug,
    Formatter,
  }
};

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

#[allow(dead_code)]
// definitely gonne be used later.
mod consts {
  pub(in super) const I8_MAX: u128=i8::MAX.cast_unsigned() as u128;
  pub(in super) const I8_MIN: u128=i8::MIN.cast_unsigned() as u128;
  pub(in super) const I16_MAX: u128=i16::MAX.cast_unsigned() as u128;
  pub(in super) const I16_MIN: u128=i16::MIN.cast_unsigned() as u128;
  pub(in super) const I32_MAX: u128=i32::MAX.cast_unsigned() as u128;
  pub(in super) const I32_MIN: u128=i32::MIN.cast_unsigned() as u128;
  pub(in super) const I64_MAX: u128=i64::MAX.cast_unsigned() as u128;
  pub(in super) const I64_MIN: u128=i64::MIN.cast_unsigned() as u128;
  pub(in super) const I128_MAX: u128=i128::MAX.cast_unsigned() as u128;
  pub(in super) const I128_MIN: u128=i128::MIN.cast_unsigned() as u128;
  pub(in super) const ISIZE_MAX: u128=isize::MAX.cast_unsigned() as u128;
  pub(in super) const ISIZE_MIN: u128=isize::MIN.cast_unsigned() as u128;

  pub(in super) const U8_MAX: u128=u8::MAX as u128;
  pub(in super) const U8_MIN: u128=u8::MIN as u128;
  pub(in super) const U16_MAX: u128=u16::MAX as u128;
  pub(in super) const U16_MIN: u128=u16::MIN as u128;
  pub(in super) const U32_MAX: u128=u32::MAX as u128;
  pub(in super) const U32_MIN: u128=u32::MIN as u128;
  pub(in super) const U64_MAX: u128=u64::MAX as u128;
  pub(in super) const U64_MIN: u128=u64::MIN as u128;
  pub(in super) const U128_MAX: u128=u128::MAX as u128;
  pub(in super) const U128_MIN: u128=u128::MIN as u128;
  pub(in super) const USIZE_MAX: u128=usize::MAX as u128;
  pub(in super) const USIZE_MIN: u128=usize::MIN as u128;
}



#[derive(Clone)]
pub struct Int {
  pub repr: u128,
  pub span: Span,
  pub kind: Option<IntKind>,
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



impl Int {
  #[inline]
  pub fn new(repr: u128,span: Span,kind: Option<IntKind>)-> Self {
    Self {
      span,
      kind,
      repr,
      _marker: MARKER,
    }
  }

  #[inline]
  // FIXME(nate)
  pub fn parse_token(buf: &[u8],span: Span,kind: Option<IntKind>)-> Token {
    let repr=match lexical_core::parse::<u128>(buf) {
      Ok(repr)=> repr,
      Err(err)=> return Illegal::new(buf,span,Some(err.into())).into_token(),
    };

    Self {
      span,
      repr,
      kind,
      _marker: MARKER
    }.into_token()
  }
}

impl TokenExt for Int {
  #[inline]
  fn into_token(self)-> Token {
    Token::Int(self)
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


  #[inline]
  pub const fn signed(&self)-> bool {
    match self {
      Self::I8|Self::I16|Self::I32|Self::I64|Self::I128|Self::I256|Self::Isize=> true,
      Self::U8|Self::U16|Self::U32|Self::U64|Self::U128|Self::U256|Self::Usize=> false,
    }
  }
}


impl Debug for Int {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      return write!(f,"{}",self.repr);
    }

    let mut dbg=f.debug_struct(stringify!(Float));

    dbg.field("repr",&self.repr);
    dbg.field("kind",&self.kind);
    dbg.field("span",&self.span);
    dbg.finish()
  }
}

impl_literal_partial_eqs! {
  Int:
  u8 => IntKind::U8,
  u16 => IntKind::U16,
  u32 => IntKind::U32,
  u64 => IntKind::U64,
  u128 => IntKind::U128,
  usize => IntKind::Usize,
  i8 => IntKind::I8,
  i16 => IntKind::I16,
  i32 => IntKind::I32,
  i64 => IntKind::I64,
  i128 => IntKind::I128,
  isize => IntKind::Isize,
}

impl Hash for Int {
  fn hash<H: Hasher>(&self,state: &mut H) {
    self.repr.hash(state);
    self.kind.hash(state);
  }
}



