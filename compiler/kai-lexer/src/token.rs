
#[macro_use]
mod macros;
pub mod tokens;

pub(crate) use tokens::*;






pub trait TokenExt {
  fn into_token(self)-> Token;
}




#[kai_lexer_macro::impl_token_debug]
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
  This(This),

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













