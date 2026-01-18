
use crate::{
  prelude::*,
  file_info::FileInfo,
  location::LineColumn,
  source_map::SourceMap,
};

use std::{
  cmp,
  ops::Range,
  cell::RefCell,
  path::PathBuf,
  collections::BTreeMap,
  fmt::{
    self,
    Display,
    Formatter,
  }
};


thread_local! {
  static SOURCE_MAP: RefCell<SourceMap>=RefCell::new(SourceMap::new(
    // Start with a single dummy file which all call_site() and def_site()
    // spans reference.
    vec![FileInfo {
      source_text: Box::default(),
      lines: vec![0],
      span: Span::new(0,0),
      char_index_to_byte_offset: BTreeMap::new(),
    }],
  ));
}




#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub struct Span {
  pub(crate) lo: u32,
  pub(crate) hi: u32,
}

#[allow(dead_code)]
impl Span {
  #[inline]
  pub(crate) const fn new(lo: u32,hi: u32)-> Self {
    Self {
      lo,
      hi,
    }
  }

  pub fn call_site()-> Self {
    Self {
      lo: 0,
      hi: 0,
    }
  }

  pub fn resolved_at(&self,_other: Span)-> Self {
    *self
  }

  pub fn located_at(&self,other: Span)-> Self {
    other
  }

  pub fn byte_range(&self)-> Range<usize> {
    if self.is_call_site() {
      0..0
    } else {
      SOURCE_MAP.with(|sm| {
        sm.borrow_mut()
        .fileinfo_mut(*self)
        .byte_range(*self)
      })
    }
  }

  pub fn start(&self)-> LineColumn {
    SOURCE_MAP.with(|sm| {
      let sm=sm.borrow();
      let fi=sm.fileinfo(*self);
      fi.offset_line_column(self.lo as usize)
    })
  }

  pub fn end(&self)-> LineColumn {
    SOURCE_MAP.with(|sm| {
      let sm=sm.borrow();
      let fi=sm.fileinfo(*self);
      fi.offset_line_column(self.hi as usize)
    })
  }

  pub fn file(&self)-> String {
    SOURCE_MAP.with(|sm| {
      let sm=sm.borrow();
      sm.filepath(*self)
    })
  }

  pub fn local_file(&self)-> Option<PathBuf> {
    None
  }

  pub fn join(&self,other: Span)-> Option<Span> {
    SOURCE_MAP.with(|sm| {
      let sm=sm.borrow();
      // If `other` is not within the same FileInfo as us, return None.
      if !sm.fileinfo(*self).span_within(other) {
        return None;
      }

      Some(Span {
        lo: cmp::min(self.lo,other.lo),
        hi: cmp::max(self.hi,other.hi),
      })
    })
  }

  pub fn source_text(&self)-> Option<Box<str>> {
    if self.is_call_site() {
      None
    } else {
      Some(SOURCE_MAP.with(|sm| {
        sm.borrow_mut()
        .fileinfo_mut(*self)
        .source_text(*self)
        .into()
      }))
    }
  }

  pub fn first_byte(self)-> Self {
    Span {
      lo: self.lo,
      hi: cmp::min(self.lo.saturating_add(1), self.hi),
    }
  }

  pub fn last_byte(self)-> Self {
    Span {
      lo: cmp::max(self.hi.saturating_sub(1), self.lo),
      hi: self.hi,
    }
  }

  fn is_call_site(&self)-> bool {
    self.lo==0 && self.hi==0
  }
}


impl Display for Span {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let mut dbg=f.debug_tuple(stringify!(Span));

    dbg.field(&self.lo);
    dbg.field(&self.hi);
    dbg.finish()
  }
}


