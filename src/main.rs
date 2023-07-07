pub mod tokenizer;
pub mod parser;
pub mod interpret;

use std::{env, fs, io::{self, Write}, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut context = interpret::Context{variables: HashMap::from([(String::from("pi"), interpret::Object::Float(3.14))])};
    // dbg!(&args);
    if args.len() > 1{
        let source = fs::read_to_string(&args[1]).expect("Coudn't open file.");
        let mut tokens = tokenizer::tokenize(source.as_str());
        // dbg!(&tokens);
        let ast = parser::parse(&mut tokens);
        interpret::run(ast, &mut context);
    } else {
        loop{
            let mut input: String = String::new();
            print!(">>> ");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut input);
            let mut tokens = tokenizer::tokenize(&input.as_str());
            dbg!(&tokens);
            // dbg!(&context);
            tokens.reverse();
            let ast = parser::parse(&mut tokens);
            // dbg!(&context);
            dbg!(&ast);
            interpret::run(ast, &mut context);
            // dbg!(&context);
        }
    }
}
