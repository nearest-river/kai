
mod ident;
mod number;
mod comment;
mod string_char;

pub use ident::*;
pub use number::*;
pub use comment::*;
pub use string_char::*;





pub enum LexTracker {
  Num(NumberLexTracker),
  StrChar(StringCharLexTracker),
  Other,
}












