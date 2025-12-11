
use crate::prelude::*;

use std::fmt::{
  self,
  Debug,
  Formatter,
};



#[derive(Clone)]
pub struct Group {
  pub(crate) delimiter: Delimiter,
  pub(crate) stream: TokenStream,
  pub(crate) span: Span,
}

/// Describes how a sequence of token trees is delimited.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delimiter {
  Paren,
  /// `{ ... }`
  Brace,
  /// `[ ... ]`
  Bracket,
  /// `∅ ... ∅`
  ///
  /// An invisible delimiter, that may, for example, appear around tokens
  /// coming from a "macro variable" `$var`. It is important to preserve
  /// operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
  /// Invisible delimiters may not survive roundtrip of a token stream through
  /// a string.
  ///
  /// <div class="warning">
  ///
  /// Note: rustc currently can ignore the grouping of tokens delimited by `None` in the output
  /// of a proc_macro. Only `None`-delimited groups created by a macro_rules macro in the input
  /// of a proc_macro macro are preserved, and only in very specific circumstances.
  /// Any `None`-delimited groups (re)created by a proc_macro will therefore not preserve
  /// operator priorities as indicated above. The other `Delimiter` variants should be used
  /// instead in this context. This is a rustc bug. For details, see
  /// [rust-lang/rust#67062](https://github.com/rust-lang/rust/issues/67062).
  ///
  /// </div>
  None,
}


impl Group {
  pub(crate) fn new(delimiter: Delimiter,stream: TokenStream)-> Self {
    Self {
      stream,
      delimiter,
      span: Span::call_site(),
    }
  }

  pub fn delimiter(&self)-> Delimiter {
    self.delimiter
  }

  pub fn stream(&self)-> TokenStream {
    self.stream.clone()
  }

  pub fn span(&self)-> Span {
    self.span
  }

  pub fn span_open(&self)-> Span {
    self.span.first_byte()
  }

  pub fn span_close(&self)-> Span {
    self.span.last_byte()
  }

  pub fn set_span(&mut self,span: Span) {
    self.span=span;
  }
}


impl Debug for Group {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if !f.alternate() {
      let mut dbg=f.debug_struct(stringify!(Group));

      dbg.field("delimiter",&self.delimiter);
      dbg.field("stream",&self.stream);
      dbg.field("span",&self.span);

      return dbg.finish();
    }

    // We attempt to match libproc_macro's formatting.
    // Empty parens: ()
    // Nonempty parens: (...)
    // Empty brackets: []
    // Nonempty brackets: [...]
    // Empty braces: { }
    // Nonempty braces: { ... }
    let (open,close)=match self.delimiter {
      Delimiter::Paren=> ("(",")"),
      Delimiter::Brace=> ("{","}"),
      Delimiter::Bracket=> ("[","]"),
      Delimiter::None=> ("",""),
    };

    f.write_str(open)?;

    Debug::fmt(&self.stream,f)?;
    if self.delimiter==Delimiter::Brace && !self.stream.inner.is_empty() {
      f.write_str(" ")?;
    }

    f.write_str(close)
  }
}












