extern crate regex_macro;

use std::char;
use std::iter::Peekable;

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

#[derive(Debug)]
enum State {
    Start,
    NumberWhole,
    NumberDecimal,
    Ident,
    String,
    StringEscape,
}

struct Tokenizer<'a> {
    input: Peekable<std::str::Chars<'a>>,
    state: State,
    curent: String,
    position: usize,
    start_pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input.chars().peekable(),
            state: State::Start,
            curent: String::new(),
            position: 0,
            start_pos: 0,
        }
    }
    fn consume_char(&mut self) -> char {
        self.position += 1;
        self.input.next().expect("A char was expected.")
    }
    fn next_token(&mut self) -> Token {
        while let Some(c) = self.input.peek() {
            match self.state {
                State::Start => match c {
                    '0'..='9' => {
                        self.state = State::NumberWhole;
                        let c = self.consume_char();
                        self.curent.push(c);
                        // self.curent.push(self.consume_char());
                    }
                    ws if ws.is_whitespace() => {
                        self.consume_char();
                        self.start_pos = self.position;
                    }
                    op if ['+', '-', '%', '/', '=', '*'].contains(op) => {
                        let t = Token::Operator(op.to_string(), self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    '(' => {
                        let t = Token::LParen(self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    ')' => {
                        let t = Token::RParen(self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    '{' => {
                        let t = Token::StartBlock(self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    '}' => {
                        let t = Token::EndBlock(self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    c if c.is_alphabetic() || c == &'_' => {
                        let c = self.consume_char();
                        self.curent.push(c);
                        self.state = State::Ident
                    }
                    ';' => {
                        let t = Token::Delim(self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        return t;
                    }
                    '"' => {
                        self.consume_char();
                        self.state = State::String;
                    }
                    c => todo!(
                        "Tokens starting with '{}' are not implemented.",
                        c
                    ),
                },
                State::NumberWhole => match c {
                    '0'..='9' => {
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                    '.' => {
                        self.state = State::NumberDecimal;
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                    _ => {
                        self.state = State::Start;
                        let t = Token::Numb(
                            self.curent.parse().unwrap_or_else(|_| {
                                panic!(
                                    "'{}' is not a valid number.",
                                    self.curent
                                )
                            }),
                            self.start_pos,
                        );
                        self.start_pos = self.position;
                        self.curent.clear();
                        return t;
                    }
                },
                State::NumberDecimal => match c {
                    '0'..='9' => self.curent.push(self.input.next().unwrap()),
                    _ => {
                        self.state = State::Start;
                        let t = Token::Numb(
                            self.curent.parse().unwrap(),
                            self.start_pos,
                        );
                        self.start_pos = self.position;
                        self.curent.clear();
                        return t;
                    }
                },
                State::Ident => match c {
                    c if c.is_alphabetic()
                        || c.is_ascii_digit()
                        || c == &'_' =>
                    {
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                    _ => {
                        let t =
                            Token::Ident(self.curent.clone(), self.start_pos);
                        self.curent.clear();
                        self.state = State::Start;
                        self.start_pos = self.position;
                        return t;
                    }
                },
                State::String => match c {
                    '\\' => {
                        self.state = State::StringEscape;
                        self.consume_char();
                    }
                    '"' => {
                        self.state = State::Start;
                        let t =
                            Token::String(self.curent.clone(), self.start_pos);
                        self.consume_char();
                        self.start_pos = self.position;
                        self.curent.clear();
                        return t;
                    }
                    _ => {
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                },
                State::StringEscape => match c {
                    '\\' => {
                        self.state = State::String;
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                    '"' => {
                        self.state = State::String;
                        let c = self.consume_char();
                        self.curent.push(c);
                    }
                    'n' => {
                        self.state = State::String;
                        self.consume_char();
                        self.curent.push('\n');
                    }
                    't' => {
                        self.state = State::String;
                        self.consume_char();
                        self.curent.push('\t');
                    }
                    'v' => {
                        self.state = State::String;
                        self.consume_char();
                        self.curent.push('\x0B');
                    }
                    'r' => {
                        self.state = State::String;
                        self.consume_char();
                        self.curent.push('\r');
                    }
                    _ => todo!("Unknown escape pattern."),
                },
            }
        }
        Token::EOF(self.position)
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut tokenizer = Tokenizer::new(input);
    let mut eof = false;
    loop {
        let t = tokenizer.next_token();
        if matches!(t, Token::EOF(_)) {
            if eof {
                return out;
            }
            eof = true;
        }
        out.push(t);
    }
}
