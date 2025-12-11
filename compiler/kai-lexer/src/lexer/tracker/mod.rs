
mod rident;
mod number;
mod comment;
mod string_char;

#[allow(unused_imports)]
pub(crate) use rident::*;
pub(crate) use number::*;
pub(crate) use comment::*;
pub(crate) use string_char::*;





pub enum LexTracker {
  Num(NumberLexTracker),
  StrChar(StringCharLexTracker),
  Other,
}












