
use crate::token::tokens::{
  comment::CommentKind,
  literals::{
    string::StrKind,
    character::CharKind,
    numbers::{
      IntKind,
      FloatKind,
    },
  }
};


#[derive(Debug)]
pub enum TokenHint {
  Float(Option<FloatKind>),
  Int(Option<IntKind>),
  Str(StrKind),
  Char(CharKind),
  Comment(CommentKind),
  Illegal(Option<&'static str>),
  Other,
}

impl TokenHint {
  pub const INFERRED_FLOAT: TokenHint=TokenHint::Float(None);
  pub const INFERRED_INT: TokenHint=TokenHint::Int(None);

  #[inline]
  pub const fn suffix_size_hint(&self)-> Option<usize> {
    match self {
      Self::Float(Some(kind))=> Some(kind.suffix_len()),
      Self::Int(Some(kind))=> Some(kind.suffix_len()),
      Self::Comment(kind)=> Some(kind.suffix_len()),
      _=> return None,
    }
  }
}









