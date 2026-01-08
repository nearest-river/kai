
use std::ops::Range;
use crate::prelude::*;



pub(crate) struct FileInfo {
  pub(crate) source_text: Box<str>,
  pub(crate) span: Span,
  pub(crate) lines: Vec<usize>,
  pub(crate) char_index_to_byte_offset: BTreeMap<usize,usize>,
}

impl FileInfo {
  pub(crate) fn offset_line_column(&self,offset: usize)-> LineColumn {
    assert!(self.span_within(Span {
      lo: offset as u32,
      hi: offset as u32,
    }));

    let offset=offset-self.span.lo as usize;
    match self.lines.binary_search(&offset) {
      Ok(found)=> LineColumn {
        line: found+1,
        column: 0,
      },
      Err(idx)=> LineColumn {
        line: idx,
        column: offset-self.lines[idx-1],
      },
    }
  }

  pub(crate) fn span_within(&self,span: Span)-> bool {
    span.lo >= self.span.lo && span.hi <= self.span.hi
  }

  pub(crate) fn byte_range(&mut self,span: Span)-> Range<usize> {
    let lo_char = (span.lo - self.span.lo) as usize;

    // Look up offset of the largest already-computed char index that is
    // less than or equal to the current requested one. We resume counting
    // chars from that point.
    let (&last_char_index, &last_byte_offset)=self.char_index_to_byte_offset
    .range(..=lo_char)
    .next_back()
    .unwrap_or((&0,&0));

    let lo_byte=if last_char_index==lo_char {
      last_byte_offset
    } else {
      let buf=self.source_text[last_byte_offset..]
      .char_indices()
      .nth(lo_char-last_char_index);
      let total_byte_offset=match buf {
        Some((additional_offset, _ch))=> last_byte_offset + additional_offset,
        None => self.source_text.len(),
      };

      self.char_index_to_byte_offset.insert(lo_char,total_byte_offset);
      total_byte_offset
    };

    let trunc_lo=&self.source_text[lo_byte..];
    let char_len=(span.hi-span.lo) as usize;
    let hi_byte=match trunc_lo.char_indices().nth(char_len) {
      Some((offset,_ch))=> lo_byte + offset,
      None=> self.source_text.len(),
    };

    lo_byte..hi_byte
  }

  pub(crate) fn source_text(&mut self,span: Span)-> Box<str> {
    let byte_range=self.byte_range(span);
    self.source_text[byte_range].into()
  }
}






