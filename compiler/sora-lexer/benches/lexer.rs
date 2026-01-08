
use std::fs;
use sora_lexer::Lexer;

use criterion::{
  Criterion,
  criterion_main,
  criterion_group,
};



static PATH: &str="assets/example.hgo";


fn lex_benchmark(c: &mut Criterion) {
  c.bench_function("lexer",|b| b.iter(|| lex));
}

fn lex() {
  let code=fs::read(PATH).unwrap();
  let lexer=Lexer::new(&code);


  for token in lexer {
    print!("{token:?} ")
  }
}


criterion_group!(benches,lex_benchmark);
criterion_main!(benches);




