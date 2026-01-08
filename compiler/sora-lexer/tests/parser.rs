
use std::fs;
use kai_lexer::token_stream::TokenStream;


static PATH: &str="assets/example.hgo";


#[test]
pub fn xd() {
  let code=fs::read(PATH).unwrap();

  let stream=TokenStream::parse(&code).unwrap();

  println!("{stream:#?}");
}

