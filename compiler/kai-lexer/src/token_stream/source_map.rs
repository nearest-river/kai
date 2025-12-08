
use crate::prelude::*;
use std::cmp::Ordering;
use super::file_info::FileInfo;


pub(crate) struct SourceMap {
  files: Vec<FileInfo>,
}

impl SourceMap {
  pub(crate) fn new(files: Vec<FileInfo>)-> Self {
    Self {
      files,
    }
  }

  pub(crate) fn next_start_pos(&self)-> u32 {
    // Add 1 so there's always space between files.
    //
    // We'll always have at least 1 file, as we initialize our files list
    // with a dummy file.
    self.files.last().unwrap().span.hi + 1
  }

  fn add_file(&mut self,src: &str)-> Span {
    let (len,lines)=lines_offsets(src);
    let lo=self.next_start_pos();
    let span=Span {
      lo,
      hi: lo+(len as u32),
    };

    self.files.push(FileInfo {
      source_text: src.into(),
      span,
      lines,
      // Populated lazily by source_text().
      char_index_to_byte_offset: BTreeMap::new(),
    });

    span
  }

  pub(crate) fn find(&self,span: Span)-> usize {
    let idx=self.files.binary_search_by(|file| {
      if file.span.hi < span.lo {
        Ordering::Less
      } else if file.span.lo > span.hi {
        Ordering::Greater
      } else {
        assert!(file.span_within(span));
        Ordering::Equal
      }
    });

    match idx {
      Ok(i)=> i,
      Err(_)=> unreachable!("Invalid span with no related FileInfo!"),
    }
  }

  pub(crate) fn filepath(&self, span: Span)-> String {
    let i=self.find(span);
    if i==0 {
      "<unspecified>".to_owned()
    } else {
      format!("<parsed string {}>", i)
    }
  }

  pub(crate) fn fileinfo(&self,span: Span)-> &FileInfo {
    let i=self.find(span);
    &self.files[i]
  }

  pub(crate) fn fileinfo_mut(&mut self,span: Span)-> &mut FileInfo {
    let i=self.find(span);
    &mut self.files[i]
  }
}


fn lines_offsets(s: &str)-> (usize,Vec<usize>) {
  let mut lines=vec![0];
  let mut total=0;

  for ch in s.chars() {
    total+=1;
    if ch=='\n' {
      lines.push(total);
    }
  }

  (total,lines)
}

