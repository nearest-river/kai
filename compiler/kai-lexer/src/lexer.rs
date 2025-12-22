
mod token_hint;
pub(crate) mod tracker;

use token_hint::TokenHint;

use crate::{
  token::*,
  prelude::*,
  token_stream::parse,
};

use tracker::{
  LexTracker,
  NumberLexTracker,
  CommentLexTracker,
  StringCharLexTracker,
};




pub struct Lexer<'b> {
  buf: &'b [u8],
  cursor: usize,
  line: usize,
  bol: usize, // begining of line.
}

impl<'b> Lexer<'b> {
  pub fn new(buf: &'b [u8])-> Self {
    Self {
      buf,
      cursor: 0,
      line: 0,
      bol: 0,
    }
  }

  #[inline(always)]
  const fn update(&mut self,count: usize) {
    self.cursor+=count;
  }

  #[inline(always)]
  pub const fn cursor(&self)-> usize {
    self.cursor
  }

  #[inline(always)]
  pub const fn col(&self)-> usize {
    self.cursor-self.bol
  }

  #[inline(always)]
  pub const fn line(&self)-> usize {
    self.line
  }

  #[inline(always)]
  pub const fn bol(&self)-> usize {
    self.bol
  }

  #[inline(always)]
  pub const fn is_eof(&self)-> bool {
    self.cursor>=self.buf.len()
  }

  #[inline(always)]
  pub const fn eof_in(&self,count: usize)-> bool {
    self.cursor+count-1>=self.buf.len()
  }

  #[inline]
  pub fn parse(self)-> Result<TokenStream,LexErr> {
    parse::parse(self)
  }
}



impl Iterator for Lexer<'_> {
  type Item=Token;
  fn next(&mut self)-> Option<Token> {
    self.skip_whitespaces();
    if self.is_eof() {
      return None;
    }

    let (end_idx,hint)=self.next_sep();
    let token=&self.buf[self.cursor..end_idx];

    let span=Span {
      lo: self.cursor as u32,
      hi: end_idx as u32,
    };
    self.update(token.len());

    let token=match hint {
      TokenHint::Other=> Self::try_parse_token(token,span),
      TokenHint::Float(kind)=> Float::parse_token(token,span,kind),
      TokenHint::Int(kind)=> Int::parse_token(token,span,kind),
      TokenHint::Str(kind)=> Str::parse_token(token,span,kind),
      TokenHint::Char(kind)=> Char::parse_token(token,span,kind),
      TokenHint::Comment(kind)=> Comment::new(token,span,kind).into_token(),
      TokenHint::Illegal(reason)=> Illegal::new(token,span,reason).into_token(),
    };

    Some(token)
  }
}

impl Lexer<'_> {
  #[inline]
  fn next_sep(&mut self)-> (usize,TokenHint) {
    let ch0=self.buf[self.cursor];
    assert!(!ch0.is_ascii_whitespace());

    // Check single char Token
    if Self::is_special1(ch0) {
      return (self.cursor+1,TokenHint::Other);
    }

    if !Self::is_special2plus(ch0) {
      let mut i=self.cursor;
      let mut tracker=Option::<LexTracker>::None;
      while i<self.buf.len() {
        let ch=self.buf[i];

        // handling number literals.
        if let None=&tracker && let Some(num_tracker)=NumberLexTracker::number_starts(&self.buf[i..]) {
          let prefix_len=num_tracker.prefix_len();
          tracker=Some(LexTracker::Num(num_tracker));
          i+=prefix_len.unwrap_or(1);
          continue;
        } else if let Some(LexTracker::Num(num_tracker))=&mut tracker &&
                  let Some(hint)=num_tracker.number_ends(&self.buf[i..]) {
          let suffix_len=hint.suffix_size_hint()
          .unwrap_or_default();
          return (i+suffix_len,hint);
        }

        // skip escape sequence.
        if ch==b'\\' {
          i+=2;
        }

        // handling string literals
        if let None=&tracker && let Some(str_ch_tracker)=StringCharLexTracker::sec_starts(&self.buf[i..]) {
          let prefix_len=str_ch_tracker.prefix_len();
          tracker=Some(LexTracker::StrChar(str_ch_tracker));
          i+=prefix_len;
          continue;
        } else if let Some(LexTracker::StrChar(str_ch_tracker))=&mut tracker &&
                  let Some(hint)=str_ch_tracker.sec_ends(&self.buf[i..]) {
          return (i+str_ch_tracker.suffix_len(),hint);
        }

        if let None=&tracker {
          tracker=Some(LexTracker::Other);
        }

        // handling skips
        match &tracker {
          Some(LexTracker::Other)|None=> (),
          Some(_)=> {
            i+=1;
            continue;
          }
        }

        // breaks
        if ch.is_ascii_whitespace() {
          break;
        }

        if Self::is_special2plus(ch) && !ch.is_ascii_alphanumeric() {
          break;
        }

        if Self::is_special1(ch) {
          break;
        }

        i+=1;
      }

      return (i,TokenHint::Other);
    }

    if let Some(mut com_tracker)=CommentLexTracker::seq_starts(&self.buf[self.cursor..]) {
      let mut i=self.cursor+com_tracker.prefix_len();
      while i<self.buf.len() {
        if let Some(hint)=com_tracker.seq_ends(&self.buf[i..]) {
          let suffix_len=hint.suffix_size_hint()
          .unwrap_or_default();
          return (i+suffix_len,hint);
        }

        i+=1;
      }

      let reason=Reason::ParseCommentErr("missing */");
      return (i,TokenHint::Illegal(Some(reason)));
    }


    let pats=match Self::seperator_pats(ch0) {
      Some(pats)=> pats,
      None=> unreachable!(),
    };

    // pattern map is defiened in ascending order of length.
    for &pat in pats.iter().rev() {
      let start=self.cursor;
      let end=self.cursor+pat.len();
      if !self.buf[start..end].starts_with(pat) {
        continue;
      }

      return (self.cursor+pat.len(),TokenHint::Other);
    }

    (self.buf.len(),TokenHint::Other)
  }
}


impl Lexer<'_> {
  #[inline(always)]
  fn try_parse_token(token: &[u8],span: Span)-> Token {
    match token {
      b"_"           => Token::Underscore(Underscore::new(span)),
      b"("           => Token::LParen(LParen::new(span)),
      b")"           => Token::RParen(RParen::new(span)),
      b"{"           => Token::LBrace(LBrace::new(span)),
      b"}"           => Token::RBrace(RBrace::new(span)),
      b"["           => Token::LBracket(LBracket::new(span)),
      b"]"           => Token::RBracket(RBracket::new(span)),
      b"as"          => Token::As(As::new(span)),
      b"async"       => Token::Async(Async::new(span)),
      b"await"       => Token::Await(Await::new(span)),
      b"break"       => Token::Break(Break::new(span)),
      b"const"       => Token::Const(Const::new(span)),
      b"continue"    => Token::Continue(Continue::new(span)),
      b"this"        => Token::This(This::new(span)),
      b"default"     => Token::Default(Default::new(span)),
      b"dyn"         => Token::Dyn(Dyn::new(span)),
      b"else"        => Token::Else(Else::new(span)),
      b"enum"        => Token::Enum(Enum::new(span)),
      b"extern"      => Token::Extern(Extern::new(span)),
      b"false"       => Token::Bool(Bool::new(false,span)),
      b"fn"          => Token::Fn(Fn::new(span)),
      b"for"         => Token::For(For::new(span)),
      b"if"          => Token::If(If::new(span)),
      b"impl"        => Token::Impl(Impl::new(span)),
      b"in"          => Token::In(In::new(span)),
      b"let"         => Token::Let(Let::new(span)),
      b"loop"        => Token::Loop(Loop::new(span)),
      b"macro"       => Token::Macro(Macro::new(span)),
      b"match"       => Token::Match(Match::new(span)),
      b"mod"         => Token::Mod(Mod::new(span)),
      b"move"        => Token::Move(Move::new(span)),
      b"mut"         => Token::Mut(Mut::new(span)),
      b"pub"         => Token::Pub(Pub::new(span)),
      b"raw"         => Token::Raw(Raw::new(span)),
      b"return"      => Token::Return(Return::new(span)),
      b"Self"        => Token::SelfType(SelfType::new(span)),
      b"self"        => Token::SelfValue(SelfValue::new(span)),
      b"static"      => Token::Static(Static::new(span)),
      b"struct"      => Token::Struct(Struct::new(span)),
      b"super"       => Token::Super(Super::new(span)),
      b"trait"       => Token::Trait(Trait::new(span)),
      b"true"        => Token::Bool(Bool::new(true,span)),
      b"type"        => Token::Type(Type::new(span)),
      b"typeof"      => Token::Typeof(Typeof::new(span)),
      b"union"       => Token::Union(Union::new(span)),
      b"unsafe"      => Token::Unsafe(Unsafe::new(span)),
      b"use"         => Token::Use(Use::new(span)),
      b"where"       => Token::Where(Where::new(span)),
      b"while"       => Token::While(While::new(span)),
      b"yield"       => Token::Yield(Yield::new(span)),
      b"yeet"        => Token::Yeet(Yeet::new(span)),
      b"&"           => Token::And(And::new(span)),
      b"&&"          => Token::AndAnd(AndAnd::new(span)),
      b"&="          => Token::AndEq(AndEq::new(span)),
      b"@"           => Token::At(At::new(span)),
      b"^"           => Token::Caret(Caret::new(span)),
      b"^="          => Token::CaretEq(CaretEq::new(span)),
      b":"           => Token::Colon(Colon::new(span)),
      b","           => Token::Comma(Comma::new(span)),
      b"$"           => Token::Dollar(Dollar::new(span)),
      b"."           => Token::Dot(Dot::new(span)),
      b".."          => Token::DotDot(DotDot::new(span)),
      b"..."         => Token::DotDotDot(DotDotDot::new(span)),
      b"..="         => Token::DotDotEq(DotDotEq::new(span)),
      b"="           => Token::Equal(Equal::new(span)),
      b"=="          => Token::EqualEqual(EqualEqual::new(span)),
      b"=>"          => Token::FatArrow(FatArrow::new(span)),
      b">="          => Token::Ge(Ge::new(span)),
      b">"           => Token::Gt(Gt::new(span)),
      b"<-"          => Token::LArrow(LArrow::new(span)),
      b"<="          => Token::Le(Le::new(span)),
      b"<"           => Token::Lt(Lt::new(span)),
      b"-"           => Token::Minus(Minus::new(span)),
      b"-="          => Token::MinusEq(MinusEq::new(span)),
      b"!="          => Token::NotEq(NotEq::new(span)),
      b"!"           => Token::Not(Not::new(span)),
      b"|"           => Token::Or(Or::new(span)),
      b"|="          => Token::OrEq(OrEq::new(span)),
      b"||"          => Token::OrOr(OrOr::new(span)),
      b"::"          => Token::PathSep(PathSep::new(span)),
      b"%"           => Token::Percent(Percent::new(span)),
      b"%="          => Token::PercentEq(PercentEq::new(span)),
      b"+"           => Token::Plus(Plus::new(span)),
      b"+="          => Token::PlusEq(PlusEq::new(span)),
      b"#"           => Token::Pound(Pound::new(span)),
      b"?"           => Token::Question(Question::new(span)),
      b"->"          => Token::RArrow(RArrow::new(span)),
      b";"           => Token::SemiColon(SemiColon::new(span)),
      b"<<"          => Token::Shl(Shl::new(span)),
      b"<<="         => Token::ShlEq(ShlEq::new(span)),
      b">>"          => Token::Shr(Shr::new(span)),
      b">>="         => Token::ShrEq(ShrEq::new(span)),
      b"/"           => Token::Slash(Slash::new(span)),
      b"/="          => Token::SlashEq(SlashEq::new(span)),
      b"*"           => Token::Star(Star::new(span)),
      b"*="          => Token::StarEq(StarEq::new(span)),
      repr           => Ident::parse_token(repr,span,false) // handles both ident and illegal
    }
  }


  #[inline]
  const fn is_special1(ch: u8)-> bool {
    match ch {
      b';'|b','|b'#'|b'?'=> true,
      b'('|b')'|b'{'|b'}'|b'['|b']'=> true,
      _=> false,
    }
  }

  #[inline]
  const fn is_special2plus(ch: u8)-> bool {
    match ch {
      b'&'|b'^'|b'>'|b'<'|b'|'|b'!'|b'='=> true,
      b':'|b'.'=> true,
      b'-'|b'%'|b'+'|b'/'|b'*'=> true,
      _=> false
    }
  }

  #[inline]
  const fn seperator_pats(ch: u8)-> Option<&'static [&'static [u8]]> {
    let seps: &'static [&'static [u8]]=match ch {
      b'&'=> &[b"&",b"&&",b"&="],
      b'^'=> &[b"^",b"^="],
      b':'=> &[b":",b"::"],
      b'.'=> &[b".",b"..",b"...",b"..="],
      b'='=> &[b"=",b"==",b"=>"],
      b'>'=> &[b">",b">=",b">>",b">>="],
      b'<'=> &[b"<",b"<=",b"<-",b"<<",b"<<="],
      b'-'=> &[b"-",b"-=",b"->"],
      b'!'=> &[b"!",b"!="],
      b'|'=> &[b"|",b"||",b"|="],
      b'%'=> &[b"%",b"%="],
      b'+'=> &[b"+",b"+="],
      b'/'=> &[b"/",b"/="],
      b'*'=> &[b"*",b"*="],
      _=> return None
    };

    Some(seps)
  }

  #[inline]
  const fn skip_whitespaces(&mut self) {
    let mut i=self.cursor;
    while i<self.buf.len() {
      match self.buf[i] {
        b'\n'|b'\r' if i+1<self.buf.len()=> self.bol=i+1,
        b'\n'|b'\r'|b'\t'|b'\x0C'| b' '=> (),
        _=> break,
      }

      i+=1;
    }

    self.update(i-self.cursor);
  }
}








































