
use super::TokenExt;
use crate::{
  prelude::*,
  token::Illegal,
};


#[derive(Clone)]
pub struct Ident {
  pub repr: Box<str>,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
}

impl Ident {
  pub fn parse_token(buf: &[u8],span: Span)-> Token {
    static REASON: &str="invalid identifier";

    match buf.get(0) {
      Some(b'_')=> (),
      Some(byte) if byte.is_ascii_alphabetic()=> (),
      Some(_)=> return Illegal::new(buf,span,Some(REASON)).into_token(),
      None=> unreachable!(),
    }

    for &byte in buf.iter().skip(1) {
      if !byte.is_ascii_alphanumeric() && !byte==b'_' {
        return Illegal::new(buf,span,Some(REASON)).into_token();
      }
    }

    let repr=str::from_utf8(buf)
    .expect("ain't it supposed to be utf-8, eh?")
    .into();

    Self {
      repr,
      span,
      _marker: MARKER,
    }.into_token()
  }
}


impl_repr_tokens! {
  Ident
}







