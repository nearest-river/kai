
use crate::prelude::*;


pub static FLOAT_SUFFIXES: &[&[u8]]=&[
  FloatKind::F16.as_str().as_bytes(),
  FloatKind::F32.as_str().as_bytes(),
  FloatKind::F64.as_str().as_bytes(),
  FloatKind::F128.as_str().as_bytes(),
];



#[derive(Clone)]
pub struct Float {
  pub repr: Box<str>,
  pub span: Span,
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
  pub fn parse_token(buf: &[u8],span: Span,_kind: Option<FloatKind>)-> Token {
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

impl_repr_tokens! {
  Float
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



/*
impl Debug for Float {
  fn fmt(&self, f: &mut Formatter<'_>)-> fmt::Result {
    f.write_str(stringify!(Float))
  }
}*/



/*
    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))?;
        if f.alternate() {
          write!(f,"({:#?})",self.repr)
        } else {
          write!(f,"({:?})",self.repr)
        }
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,other: &$name)-> bool {
        self.repr==other.repr
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,state: &mut H) {
        self.repr.hash(state);
      }
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
*/














