
use std::fs;
use kai_lexer::{
  Lexer,
  Token,
};


static PATH: &str="assets/example.hgo";


#[test]
pub fn xd() {
  let code=fs::read(PATH).unwrap();
  let lexer=Lexer::new(&code);


  for token in lexer {
    print!("{token:#?} ");
  }
}






