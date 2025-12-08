
use crate::{
  prelude::*,
  lexer::tracker::FloatKind,
};


#[derive(Clone)]
pub struct Float {
  pub repr: Box<str>,
  pub span: Span,
  _marker: ProcMacroAutoTraits,
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














