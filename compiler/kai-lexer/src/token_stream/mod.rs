
mod leaf;
mod error;
mod group;

pub(crate) mod parse;
pub(crate) mod rcvec;
pub(crate) mod location;
pub(crate) mod file_info;
pub(crate) mod source_map;


pub use leaf::Leaf;
pub use error::LexErr;
pub use group::{
  Group,
  Delimiter,
};

use crate::{
  Lexer,
  prelude::*,
};

use std::{
  ptr,
  str::FromStr,
  mem::ManuallyDrop,
  fmt::{
    self,
    Debug,
    Formatter,
  },
};




#[derive(Clone)]
pub struct TokenStream {
  inner: RcVec<TokenTree>,
}

#[derive(Clone)]
pub enum TokenTree {
  Group(Group),
  Leaf(Leaf),
}


pub(crate) struct TokenStreamBuilder {
  inner: RcVecBuilder<TokenTree>,
}

pub(crate) type TokenTreeIter=RcVecIntoIter<TokenTree>;

impl TokenStream {
  pub(crate) fn new()-> Self {
    TokenStream {
      inner: RcVecBuilder::new().build()
    }
  }

  pub fn parse(buf: &[u8])-> Result<Self,LexErr> {
    parse::parse(Lexer::new(buf))
  }

  pub fn is_empty(&self)-> bool {
    self.inner.is_empty()
  }

  fn take_inner(self)-> RcVecBuilder<TokenTree> {
    let nodrop=ManuallyDrop::new(self);
    unsafe {
      ptr::read(&nodrop.inner)
    }.make_owned()
  }
}

impl Debug for TokenStream {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    f.write_str("TokenStream ")?;
    f.debug_list()
    .entries(self.clone())
    .finish()
  }
}

impl FromIterator<TokenTree> for TokenStream {
  fn from_iter<T: IntoIterator<Item=TokenTree>>(tokens: T)-> Self {
    let mut stream = TokenStream::new();
    stream.extend(tokens);
    stream
  }
}


impl FromIterator<TokenStream> for TokenStream {
  fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
    let mut builder=RcVecBuilder::new();

    for stream in streams {
      builder.extend(stream.take_inner());
    }

    TokenStream {
      inner: builder.build()
    }
  }
}

impl Extend<TokenTree> for TokenStream {
  fn extend<I: IntoIterator<Item=TokenTree>>(&mut self,tokens: I) {
    let mut vec=self.inner.make_mut();
    for token in tokens {
      vec.push(token);
    }
  }
}

impl Extend<TokenStream> for TokenStream {
  fn extend<I: IntoIterator<Item=TokenStream>>(&mut self,streams: I) {
    self.inner
    .make_mut()
    .extend(streams.into_iter().flatten());
  }
}

impl IntoIterator for TokenStream {
  type Item=TokenTree;
  type IntoIter=TokenTreeIter;

  fn into_iter(self)-> Self::IntoIter {
    self.take_inner().into_iter()
  }
}

impl Drop for TokenStream {
  fn drop(&mut self) {
    let mut stack=Vec::new();
    let mut current=match self.inner.get_mut() {
      Some(inner)=> inner.take().into_iter(),
      None=> return,
    };

    loop {
      while let Some(token)=current.next() {
        let mut group=match token {
          TokenTree::Group(group)=> group,
          _=> continue,
        };

        if let Some(inner)=group.stream.inner.get_mut() {
          stack.push(current);
          current=inner.take().into_iter();
        }
      }

      match stack.pop() {
        Some(next)=> current=next,
        None=> return,
      }
    }
  }
}

impl FromStr for TokenStream {
  type Err=LexErr;

  fn from_str(mut s: &str)-> Result<Self,Self::Err> {
    // Strip a byte order mark if present
    const BYTE_ORDER_MARK: &str="\u{feff}";
    if s.starts_with(BYTE_ORDER_MARK) {
      s=&s[BYTE_ORDER_MARK.len()..]
    }

    TokenStream::parse(s.as_bytes())
  }
}


impl TokenStreamBuilder {
  pub(crate) fn new()-> Self {
    Self {
      inner: RcVecBuilder::new()
    }
  }

  #[allow(dead_code)]
  pub(crate) fn with_capacity(cap: usize)-> Self {
    Self {
      inner: RcVecBuilder::with_capacity(cap)
    }
  }

  pub(crate) fn push_token_from_parser(&mut self,tt: TokenTree) {
    self.inner.push(tt);
  }

  pub(crate) fn build(self)-> TokenStream {
    TokenStream {
      inner: self.inner.build(),
    }
  }
}


impl Debug for TokenTree {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      return match self {
        Self::Leaf(leaf)=> Debug::fmt(leaf,f),
        Self::Group(group)=> Debug::fmt(group,f)
      };
    }

    match self {
      Self::Leaf(leaf)=> {
        f.debug_tuple(stringify!(Leaf))
        .field(leaf)
        .finish()
      },
      Self::Group(group)=> {
        f.debug_tuple(stringify!(Group))
        .field(group)
        .finish()
      },
    }
  }
}




