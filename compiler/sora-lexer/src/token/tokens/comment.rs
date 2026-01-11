
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Display,
  Formatter,
};


#[derive(Clone)]
pub struct Comment {
  pub repr: Box<str>,
  pub span: Span,
  pub kind: CommentKind,
  _marker: ProcMacroAutoTraits,
}

#[derive(Clone,Debug,PartialEq,Eq)]
/// Same conventions as rust.
pub enum CommentKind {
  Line,
  DocLine,
  Block,
  DocBlock,
}

impl Comment {
  /// PANICS: This function panics if `buf` doesn't satifsy any of the comment prefixes and/or suffixes.
  pub fn new(repr: &[u8],span: Span,kind: CommentKind)-> Self {
    let repr=str::from_utf8(repr)
    .expect("utf-8 errors shouldn't happen")
    .into();

    Self {
      span,
      kind,
      repr,
      _marker: MARKER,
    }
  }
}

impl Debug for Comment {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      return Display::fmt(self,f);
    }

    let mut dbg=f.debug_struct(stringify!(Comment));

    dbg.field("repr",&self.repr);
    dbg.field("kind",&self.kind);
    dbg.field("span",&self.span);
    dbg.finish()
  }
}

impl Display for Comment {
  fn fmt(&self, f: &mut Formatter<'_>)-> fmt::Result {
    write!(f,r#"{}::{:#?}("{}")"#,stringify!(Comment),self.kind,&*self.repr)
  }
}

impl Eq for Comment {}
impl PartialEq for Comment {
  fn eq(&self,other: &Self)-> bool {
    self.repr==other.repr && self.kind==other.kind
  }
}

impl<S: AsRef<str>> PartialEq<S> for Comment {
  fn eq(&self,other: &S)-> bool {
    other.as_ref()==&*self.repr
  }
}

impl Hash for Comment {
  fn hash<H: Hasher>(&self,state: &mut H) {
    self.repr.hash(state);
  }
}

impl TokenExt for Comment {
  fn into_token(self)-> Token {
    Token::Comment(self)
  }

  #[inline]
  fn span(&self)-> Span {
    self.span
  }

  #[inline]
  fn set_span(&mut self,span: Span) {
    self.span=span
  }
}

impl CommentKind {
  #[inline]
  pub fn parse(repr: &[u8])-> Option<Self> {
    let kind=match repr {
      buf if buf.starts_with(b"/**")=> Self::DocBlock,
      buf if buf.starts_with(b"/*")=> Self::Block,
      buf if buf.starts_with(b"///")=> Self::DocLine,
      buf if buf.starts_with(b"//")=> Self::Line,
      _=> return None
    };

    Some(kind)
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    match self {
      Self::Line=> "//".len(),
      Self::DocLine=> "///".len(),
      Self::Block=> "/*".len(),
      Self::DocBlock=> "/**".len(),
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    match self {
      Self::Block|Self::DocBlock=> "*/".len(),
      Self::Line|Self::DocLine=> "".len(),
    }
  }
}




