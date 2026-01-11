
#[macro_use]
mod macros;
pub mod tokens;

use crate::Span;
pub(crate) use tokens::*;






pub trait TokenExt {
  fn into_token(self)-> Token;
  fn span(&self)-> Span;
  fn set_span(&mut self,span: Span);

  #[inline(always)]
  fn span_open(&self)-> Span {
    self.span().first_byte()
  }

  #[inline(always)]
  fn span_close(&self)-> Span {
    self.span().last_byte()
  }
}

macro_rules! token_match_all {
  ($tok:expr,$token:ident-> $f:expr)=> {
    match $tok {
      Token::Ident($token)=> $f,
      Token::Comment($token)=> $f,
      Token::Illegal($token)=> $f,
      Token::Str($token)=> $f,
      Token::Char($token)=> $f,
      Token::Bool($token)=> $f,
      Token::Float($token)=> $f,
      Token::Int($token)=> $f,
      Token::Unsafe($token)=> $f,
      Token::Super($token)=> $f,
      Token::Crate($token)=> $f,
      Token::At($token)=> $f,
      Token::And($token)=> $f,
      Token::AndAnd($token)=> $f,
      Token::AndEq($token)=> $f,
      Token::Caret($token)=> $f,
      Token::CaretEq($token)=> $f,
      Token::Colon($token)=> $f,
      Token::Comma($token)=> $f,
      Token::Dollar($token)=> $f,
      Token::Dot($token)=> $f,
      Token::DotDot($token)=> $f,
      Token::DotDotDot($token)=> $f,
      Token::DotDotEq($token)=> $f,
      Token::Equal($token)=> $f,
      Token::EqualEqual($token)=> $f,
      Token::FatArrow($token)=> $f,
      Token::Ge($token)=> $f,
      Token::Gt($token)=> $f,
      Token::Le($token)=> $f,
      Token::Lt($token)=> $f,
      Token::LArrow($token)=> $f,
      Token::Minus($token)=> $f,
      Token::MinusEq($token)=> $f,
      Token::NotEq($token)=> $f,
      Token::Not($token)=> $f,
      Token::Or($token)=> $f,
      Token::OrEq($token)=> $f,
      Token::OrOr($token)=> $f,
      Token::PathSep($token)=> $f,
      Token::Percent($token)=> $f,
      Token::PercentEq($token)=> $f,
      Token::Plus($token)=> $f,
      Token::PlusEq($token)=> $f,
      Token::Pound($token)=> $f,
      Token::Question($token)=> $f,
      Token::RArrow($token)=> $f,
      Token::SemiColon($token)=> $f,
      Token::Shl($token)=> $f,
      Token::Shr($token)=> $f,
      Token::ShlEq($token)=> $f,
      Token::ShrEq($token)=> $f,
      Token::Slash($token)=> $f,
      Token::SlashEq($token)=> $f,
      Token::Star($token)=> $f,
      Token::StarEq($token)=> $f,
      Token::Underscore($token)=> $f,
      Token::LParen($token)=> $f,
      Token::RParen($token)=> $f,
      Token::LBrace($token)=> $f,
      Token::RBrace($token)=> $f,
      Token::LBracket($token)=> $f,
      Token::RBracket($token)=> $f,
      Token::As($token)=> $f,
      Token::In($token)=> $f,
      Token::Fn($token)=> $f,
      Token::Struct($token)=> $f,
      Token::Const($token)=> $f,
      Token::Let($token)=> $f,
      Token::Static($token)=> $f,
      Token::Enum($token)=> $f,
      Token::Impl($token)=> $f,
      Token::Trait($token)=> $f,
      Token::Auto($token)=> $f,
      Token::Async($token)=> $f,
      Token::Type($token)=> $f,
      Token::Extern($token)=> $f,
      Token::Mod($token)=> $f,
      Token::Use($token)=> $f,
      Token::Default($token)=> $f,
      Token::Dyn($token)=> $f,
      Token::Ref($token)=> $f,
      Token::Pub($token)=> $f,
      Token::Break($token)=> $f,
      Token::Continue($token)=> $f,
      Token::Return($token)=> $f,
      Token::Yeet($token)=> $f,
      Token::Await($token)=> $f,
      Token::If($token)=> $f,
      Token::Else($token)=> $f,
      Token::Match($token)=> $f,
      Token::While($token)=> $f,
      Token::For($token)=> $f,
      Token::Loop($token)=> $f,
      Token::Macro($token)=> $f,
      Token::Move($token)=> $f,
      Token::Mut($token)=> $f,
      Token::Raw($token)=> $f,
      Token::SelfType($token)=> $f,
      Token::SelfValue($token)=> $f,
      Token::Typeof($token)=> $f,
      Token::Union($token)=> $f,
      Token::Where($token)=> $f,
      Token::Yield($token)=> $f,
    }
  };
}



#[sora_lexer_macro::impl_token_debug]
#[derive(Clone)]
pub enum Token {
  // wtf?
  Ident(Ident),
  Comment(Comment),
  Illegal(Illegal),

  // literals
  Str(Str),
  Char(Char),
  Bool(Bool),
  Float(Float),
  Int(Int),

  // weird stuff
  // Eof,
  Unsafe(Unsafe),
  Super(Super),
  Crate(Crate),

  // Punctuations
  At(At),
  And(And),
  AndAnd(AndAnd),
  AndEq(AndEq),
  Caret(Caret),
  CaretEq(CaretEq),
  Colon(Colon),
  Comma(Comma),
  Dollar(Dollar),
  Dot(Dot),
  DotDot(DotDot),
  DotDotDot(DotDotDot),
  DotDotEq(DotDotEq),
  Equal(Equal),
  EqualEqual(EqualEqual),
  FatArrow(FatArrow),
  Ge(Ge),
  Gt(Gt),
  Le(Le),
  Lt(Lt),
  LArrow(LArrow),
  Minus(Minus),
  MinusEq(MinusEq),
  NotEq(NotEq),
  Not(Not),
  Or(Or),
  OrEq(OrEq),
  OrOr(OrOr),
  PathSep(PathSep),
  Percent(Percent),
  PercentEq(PercentEq),
  Plus(Plus),
  PlusEq(PlusEq),
  Pound(Pound),
  Question(Question),
  RArrow(RArrow),
  SemiColon(SemiColon),
  Shl(Shl),
  Shr(Shr),
  ShlEq(ShlEq),
  ShrEq(ShrEq),
  Slash(Slash),
  SlashEq(SlashEq),
  Star(Star),
  StarEq(StarEq),

  Underscore(Underscore),

  LParen(LParen),
  RParen(RParen),
  LBrace(LBrace),
  RBrace(RBrace),
  LBracket(LBracket),
  RBracket(RBracket),

  // keywords
  As(As),
  In(In),
  Fn(Fn),
  Struct(Struct),
  Const(Const),
  Let(Let),
  Static(Static),
  Enum(Enum),
  Impl(Impl),
  Trait(Trait),
  Auto(Auto),
  Async(Async),
  Type(Type),
  Extern(Extern),
  Mod(Mod),
  Use(Use),
  Default(Default),
  Dyn(Dyn),
  Ref(Ref),
  Pub(Pub),

  // Control Flow,
  Break(Break),
  Continue(Continue),
  Return(Return),
  Yeet(Yeet),
  Await(Await),

  If(If),
  Else(Else),
  Match(Match),
  While(While),
  For(For),
  Loop(Loop),
  Macro(Macro),
  Move(Move),
  Mut(Mut),
  Raw(Raw),
  SelfType(SelfType),
  SelfValue(SelfValue),
  Typeof(Typeof),
  Union(Union),
  Where(Where),
  Yield(Yield),
}



impl TokenExt for Token {
  fn span(&self)-> Span {
    token_match_all!(self,token-> token.span())
  }

  fn set_span(&mut self,span: Span) {
    token_match_all!(self,token-> token.set_span(span))
  }

  fn into_token(self)-> Token {
    self
  }
}









