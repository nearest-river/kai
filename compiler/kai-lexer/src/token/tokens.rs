
mod ident;
pub mod illegal;
pub mod comment;
pub mod literals;

pub use ident::Ident;
pub use comment::Comment;
pub use illegal::Illegal;
pub use literals::{
  string::Str,
  boolean::Bool,
  character::Char,
  numbers::{
    Int,
    Float,
  }
};


use crate::prelude::*;






define_punctuation_structs! {
  "_" pub struct Underscore/1 /// wildcard patterns, inferred types, unnamed items in constants, extern crates, use declarations, and destructuring assignment
}

define_delimitirs! {
  "("           pub struct LParen
  ")"           pub struct RParen
  "{"           pub struct LBrace
  "}"           pub struct RBrace
  "["           pub struct LBracket
  "]"           pub struct RBracket
}


define_keywords! {
  "as"          pub struct As
  "async"       pub struct Async
  "auto"        pub struct Auto
  "await"       pub struct Await
  "break"       pub struct Break
  "const"       pub struct Const
  "continue"    pub struct Continue
  "this"        pub struct This
  "default"     pub struct Default
  "dyn"         pub struct Dyn
  "else"        pub struct Else
  "enum"        pub struct Enum
  "extern"      pub struct Extern
  "fn"          pub struct Fn
  "for"         pub struct For
  "if"          pub struct If
  "impl"        pub struct Impl
  "in"          pub struct In
  "let"         pub struct Let
  "loop"        pub struct Loop
  "macro"       pub struct Macro
  "match"       pub struct Match
  "mod"         pub struct Mod
  "move"        pub struct Move
  "mut"         pub struct Mut
  "pub"         pub struct Pub
  "raw"         pub struct Raw
  "ref"         pub struct Ref
  "return"      pub struct Return
  "Self"        pub struct SelfType
  "self"        pub struct SelfValue
  "static"      pub struct Static
  "struct"      pub struct Struct
  "super"       pub struct Super
  "trait"       pub struct Trait
  "type"        pub struct Type
  "typeof"      pub struct Typeof
  "union"       pub struct Union
  "unsafe"      pub struct Unsafe
  "use"         pub struct Use
  "where"       pub struct Where
  "while"       pub struct While
  "yield"       pub struct Yield
  "yeet"        pub struct Yeet
}

define_punctuation! {
  "&"           pub struct And/1        /// bitwise and logical AND, borrow, references, reference patterns
  "&&"          pub struct AndAnd/2     /// lazy AND, borrow, references, reference patterns
  "&="          pub struct AndEq/2      /// bitwise AND assignment
  "@"           pub struct At/1         /// subpattern binding
  "^"           pub struct Caret/1      /// bitwise and logical XOR
  "^="          pub struct CaretEq/2    /// bitwise XOR assignment
  ":"           pub struct Colon/1      /// various separators
  ","           pub struct Comma/1      /// various separators
  "$"           pub struct Dollar/1     /// macros
  "."           pub struct Dot/1        /// field access, tuple index
  ".."          pub struct DotDot/2     /// range, struct expressions, patterns, range patterns
  "..."         pub struct DotDotDot/3  /// variadic functions, range patterns
  "..="         pub struct DotDotEq/3   /// inclusive range, range patterns
  "="           pub struct Equal/1         /// assignment, attributes, various type definitions
  "=="          pub struct EqualEqual/2       /// equal
  "=>"          pub struct FatArrow/2   /// match arms, macros
  ">="          pub struct Ge/2         /// greater than or equal to, generics
  ">"           pub struct Gt/1         /// greater than, generics, paths
  "<-"          pub struct LArrow/2     /// unused
  "<="          pub struct Le/2         /// less than or equal to
  "<"           pub struct Lt/1         /// less than, generics, paths
  "-"           pub struct Minus/1      /// subtraction, negation
  "-="          pub struct MinusEq/2    /// subtraction assignment
  "!="          pub struct NotEq/2         /// not equal
  "!"           pub struct Not/1        /// bitwise and logical NOT, macro calls, inner attributes, never type, negative impls
  "|"           pub struct Or/1         /// bitwise and logical OR, closures, patterns in match, if let, and while let
  "|="          pub struct OrEq/2       /// bitwise OR assignment
  "||"          pub struct OrOr/2       /// lazy OR, closures
  "::"          pub struct PathSep/2    /// path separator
  "%"           pub struct Percent/1    /// remainder
  "%="          pub struct PercentEq/2  /// remainder assignment
  "+"           pub struct Plus/1       /// addition, trait bounds, macro Kleene matcher
  "+="          pub struct PlusEq/2     /// addition assignment
  "#"           pub struct Pound/1      /// attributes
  "?"           pub struct Question/1   /// question mark operator, questionably sized, macro Kleene matcher
  "->"          pub struct RArrow/2     /// function return type, closure return type, function pointer type
  ";"           pub struct SemiColon/1       /// terminator for various items and statements, array types
  "<<"          pub struct Shl/2        /// shift left, nested generics
  "<<="         pub struct ShlEq/3      /// shift left assignment
  ">>"          pub struct Shr/2        /// shift right, nested generics
  ">>="         pub struct ShrEq/3      /// shift right assignment, nested generics
  "/"           pub struct Slash/1      /// division
  "/="          pub struct SlashEq/2    /// division assignment
  "*"           pub struct Star/1       /// multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards
  "*="          pub struct StarEq/2     /// multiplication assignment
}



