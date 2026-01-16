
use crate::{
  prelude::*,
  token_stream::TokenTree,
};

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
  Paren(ParenPair),
  /// `{ ... }`
  Brace(BracePair),
  /// `[ ... ]`
  Bracket(BracketPair),
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
    let span=Self::calc_span(stream.as_ref())
    .unwrap_or(Span::call_site());

    Self {
      span,
      stream,
      delimiter,
    }
  }

  #[inline(always)]
  fn calc_span(tts: &[TokenTree])-> Option<Span> {
    let start=tts.first()?.span();
    let end=tts.last()?.span();

    start.join(end)
  }

  pub fn delimiter(&self)-> Delimiter {
    self.delimiter
  }

  pub fn outer_span(&self)-> Span {
    self.delimiter.span()
  }

  pub fn stream(&self)-> TokenStream {
    self.stream.clone()
  }

  #[inline]
  pub fn span(&self)-> Span {
    self.span
  }

  #[inline]
  pub fn span_open(&self)-> Span {
    self.span.first_byte()
  }

  #[inline]
  pub fn span_close(&self)-> Span {
    self.span.last_byte()
  }

  #[inline]
  pub fn set_span(&mut self,span: Span) {
    self.span=span;
  }

  pub fn into_stream(self)-> TokenStream {
    self.stream
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
    match self.delimiter {
      Delimiter::Paren(_)=> {
        if self.stream.is_empty() {
          return f.write_str("()");
        }

        let mut dbg=f.debug_tuple("");
        for entry in self.stream.inner.iter() {
          dbg.field(entry);
        }

        dbg.finish()
      },
      Delimiter::Brace(_)=> {
        f.debug_set()
        .entries(self.stream.inner.iter())
        .finish()
      }
      Delimiter::Bracket(_)=> {
        f.debug_list()
        .entries(self.stream.inner.iter())
        .finish()
      }
      Delimiter::None=> Debug::fmt(&self.stream,f)
    }
  }
}

impl Delimiter {
  #[inline]
  pub fn as_chars(&self)-> (char,char) {
    match self {
      Self::Paren(_)=> ('(',')'),
      Self::Brace(_)=> ('{','}'),
      Self::Bracket(_)=> ('[',']'),
      Self::None=> (' ',' '),
    }
  }

  #[inline(always)]
  pub fn span(&self)-> Span {
    match self {
      Self::Paren(paren)=> paren.span(),
      Self::Brace(brace)=> brace.span(),
      Self::Bracket(bracket)=> bracket.span(),
      Self::None=> Span::call_site(),
    }
  }
}



macro_rules! declare_deli_pair {
  ($($vis:vis struct $name:ident($open_ty:ty,$close_ty:ty);)*)=> {
    $(
    #[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
    $vis struct $name {
      pub open: $open_ty,
      pub close: $close_ty,
    }

    impl $name {
      #[inline(always)]
      pub fn new(open: $open_ty,close: $close_ty)-> Self {
        Self {
          open,
          close,
        }
      }

      #[inline(always)]
      pub fn span(&self)-> Span {
        // SAFETY: opening and closing delimiters cant possibly to different files.
        // unless you messed around
        self.open.span()
        .join(self.close.span())
        .unwrap()
      }
    }
    )*
  };
}



declare_deli_pair! {
  pub struct ParenPair(LParen,RParen);
  pub struct BracePair(LBrace,RBrace);
  pub struct BracketPair(LBracket,RBracket);
}









