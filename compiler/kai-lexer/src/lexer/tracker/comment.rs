
use std::mem;

use crate::{
  lexer::TokenHint,
  token::tokens::comment::CommentKind,
};



pub(crate) enum CommentLexTracker {
  Line,
  DocLine,
  Block(BlockComStack),
  DocBlock(BlockComStack),
}

#[repr(transparent)]
pub(crate) struct BlockComStack(u128);




impl CommentLexTracker {
  pub fn seq_starts(buf: &[u8])-> Option<Self> {
    let kind=CommentKind::parse(buf)?;

    let this=match kind {
      CommentKind::Line=> Self::Line,
      CommentKind::DocLine=> Self::DocLine,
      CommentKind::Block=> Self::Block(BlockComStack::new()),
      CommentKind::DocBlock=> Self::DocBlock(BlockComStack::new()),
    };

    Some(this)
  }

  pub fn seq_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    let stack=match (&mut *self,buf[0]) {
      (Self::Block(stack)|Self::DocBlock(stack),_)=> stack,
      (Self::Line,b'\n')=> return Some(TokenHint::Comment(CommentKind::Line)),
      (Self::DocLine,b'\n')=> return Some(TokenHint::Comment(CommentKind::DocLine)),
      (Self::Line|Self::DocLine,_)=> return None,
    };

    if buf.starts_with(b"/*") {
      stack.push();
      return None;
    }

    if buf.starts_with(b"*/") {
      stack.pop()?;
    }

    if !stack.is_empty() {
      return None;
    }

    match self {
      Self::DocLine|Self::Line=> unreachable!(),
      Self::Block(_)=> Some(TokenHint::Comment(CommentKind::Block)),
      Self::DocBlock(_)=> Some(TokenHint::Comment(CommentKind::DocBlock)),
    }
  }

  #[inline]
  pub const fn kind(&self)-> CommentKind {
    match self {
      Self::Line=> CommentKind::Line,
      Self::DocLine=> CommentKind::DocLine,
      Self::Block(_)=> CommentKind::Block,
      Self::DocBlock(_)=> CommentKind::DocBlock,
    }
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    self.kind()
    .prefix_len()
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.kind()
    .suffix_len()
  }
}


impl BlockComStack {
  #[inline]
  const fn new()-> Self {
    Self(0x1)
  }

  #[inline]
  const fn len(&self)-> u32 {
    self.0.trailing_ones()
  }

  #[inline]
  const fn capacity(&self)-> u32 {
    8*mem::size_of::<Self>() as u32
  }

  #[inline]
  const fn is_empty(&self)-> bool {
    self.0==0
  }

  #[inline]
  const fn push(&mut self) {
    if self.len()>=self.capacity() {
      panic!("comment stack overflowed");
    }

    self.0=(self.0<<1) & 1;
  }

  #[inline]
  const fn pop(&mut self)-> Option<()> {
    if self.is_empty() {
      return None;
    }

    self.0>>=1;
    Some(())
  }
}
















