
use crate::prelude::*;
use std::{
  fmt::{
    self,
    Debug,
    Formatter,
  }
};


pub static FLOAT_SUFFIXES: &[&[u8]]=&[
  FloatKind::F16.as_str().as_bytes(),
  FloatKind::F32.as_str().as_bytes(),
  FloatKind::F64.as_str().as_bytes(),
  FloatKind::F128.as_str().as_bytes(),
];



#[derive(Clone)]
pub struct Float {
  pub repr: f64,
  pub span: Span,
  pub kind: Option<FloatKind>,
  _marker: ProcMacroAutoTraits,
}

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum FloatKind {
  F16,
  F32,
  #[default]
  F64,
  F128,
}


impl Float {
  #[inline]
  pub fn parse_token(buf: &[u8],span: Span,kind: Option<FloatKind>)-> Token {
    let repr=match lexical_core::parse::<f64>(buf) {
      Ok(repr)=> repr,
      Err(err)=> return Illegal::new(buf,span,Some(err.into())).into_token(),
    };

    Self {
      span,
      kind,
      repr,
      _marker: MARKER
    }.into_token()
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

impl TokenExt for Float {
  #[inline]
  fn into_token(self)-> Token {
    Token::Float(self)
  }
}

impl Debug for Float {
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
  Float:
  f32 => FloatKind::F32,
  f64 => FloatKind::F64,
}






