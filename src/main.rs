pub mod tokenizer;
pub mod parser;
pub mod interpret;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let source = fs::read_to_string(&args[1]).expect("Coudn't open file.");
    let mut tokens = tokenizer::tokenize(source.as_str());
    // dbg!(&tokens);
    let ast = parser::parse(&mut tokens);
    interpret::run(ast);
}
