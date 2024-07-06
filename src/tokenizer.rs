extern crate regex_macro;
use regex_macro::regex;
use std::mem;

pub use self::Token::{Delim, EndBlock, Ident, Numb, Operator, StartBlock, EOF};

#[derive(Debug)]
pub enum Token {
    Ident(String, usize),
    Numb(f64, usize),
    Operator(String, usize),
    Delim(usize),
    LParen(usize),
    RParen(usize),
    EOF(usize),
    String(String, usize),
    StartBlock(usize),
    EndBlock(usize),
    Unknown,
}

impl PartialEq for Token {
    fn ne(&self, other: &Self) -> bool {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Ident(name, _), Ident(oname, _)) => name == oname,
                (Operator(name, _), Operator(oname, _)) => name == oname,
                (Numb(name, _), Numb(oname, _)) => name == oname,
                _ => false,
            }
        } else {
            false
        }
    }
    fn eq(&self, other: &Self) -> bool {
        !self.ne(other)
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let r = regex!(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<num>\d+\.?\d*)|",
        r#""(?P<str>.*)"|"#,
        r"(?P<delim>;)|",
        r"(?P<lpar>\()|",
        r"(?P<rpar>\))|",
        r"(?P<sbl>\{)|",
        r"(?P<ebl>\})|",
        r"(?P<op>[+=%\-*<>!/]+)",
        r"(?P<else>.*)"
    ));
    let mut out: Vec<Token> = vec![];
    for cap in r.captures_iter(input) {
        let t: Token = if cap.name("ident").is_some() {
            Token::Ident(
                cap.name("ident").unwrap().as_str().to_string(),
                cap.name("ident").unwrap().start(),
            )
        } else if cap.name("num").is_some() {
            match cap.name("num").unwrap().as_str().parse() {
                Ok(number) => Token::Numb(number, cap.name("num").unwrap().start()),
                Err(_) => panic!("Lexer failed to parse number"),
            }
        } else if cap.name("delim").is_some() {
            Token::Delim(cap.name("delim").unwrap().start())
        } else if cap.name("op").is_some() {
            Token::Operator(
                cap.name("op").unwrap().as_str().to_string(),
                cap.name("op").unwrap().start(),
            )
        } else if cap.name("lpar").is_some() {
            Token::LParen(cap.name("lpar").unwrap().start())
        } else if cap.name("rpar").is_some() {
            Token::RParen(cap.name("rpar").unwrap().start())
        } else if cap.name("sbl").is_some() {
            Token::StartBlock(cap.name("sbl").unwrap().start())
        } else if cap.name("ebl").is_some() {
            Token::EndBlock(cap.name("ebl").unwrap().start())
        } else if cap.name("str").is_some() {
            Token::String(
                cap.name("str").unwrap().as_str().to_string(),
                cap.name("str").unwrap().start(),
            )
        } else if cap.name("else").is_some() {
            Token::Unknown
        } else {
            panic!("Wtf")
        };
        out.push(t);
    }
    out.push(EOF(input.len()));
    out
}
