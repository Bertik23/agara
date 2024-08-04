pub mod interpret;
pub mod parser;
pub mod tokenizer;

use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut context = interpret::Context {
        variables: HashMap::from([(
            String::from("pi"),
            interpret::Object::Float(std::f64::consts::PI),
        )]),
    };
    // dbg!(&args);
    if args.len() > 1 {
        let mut source =
            fs::read_to_string(&args[1]).expect("Coudn't open file.");
        let mut tokens = tokenizer::tokenize(&mut source);
        // dbg!(&tokens);
        let ast = parser::parse(&mut tokens);
        interpret::run(ast, &mut context);
    } else {
        loop {
            let mut input: String = String::new();
            print!(">>> ");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut input);
            let mut tokens = tokenizer::tokenize(&mut input);
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
