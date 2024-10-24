use core::panic;

extern crate lazy_static;

use lazy_static::lazy_static;

use crate::tokenizer::Token;
use std::collections::HashMap;

lazy_static! {
    static ref BINOP_PRECEDENSE: HashMap<&'static str, i32> = HashMap::from([
        (" ", -10),
        ("+", 10),
        ("-", 10),
        ("*", 20),
        ("%", 20),
        ("/", 20),
        ("**", 30),
        ("=", 1)
    ]);
}

#[derive(Debug, Clone)]
pub enum AST {
    Number(f64),
    Variable(String),
    UnaryOp(String, Box<AST>),
    BinaryOp(String, Box<AST>, Box<AST>),
    Call(String, Vec<AST>),
    Function(String, Vec<AST>, Vec<AST>),
    String(String),
}

fn parse_number(tokens: &mut Vec<Token>) -> AST {
    AST::Number(if let Token::Numb(n, _) = tokens.pop().unwrap() {
        n
    } else {
        panic!("WTF")
    })
}

fn parse_bin_op_rhs(
    tokens: &mut Vec<Token>,
    lhs_: AST,
    min_token_precedense: i32,
) -> AST {
    let mut lhs = lhs_.clone();
    loop {
        let tok_precedense;
        {
            let cur_tok = tokens.last().unwrap();
            tok_precedense = BINOP_PRECEDENSE
                .get(if let Token::Operator(op, _) = cur_tok {
                    op.as_str()
                } else {
                    return lhs;
                })
                .unwrap_or(&-1);
        }
        if *tok_precedense < min_token_precedense {
            return lhs;
        }
        let operator = tokens.pop().unwrap();
        let mut rhs = parse_primary(tokens);
        let next_tok_precedense = BINOP_PRECEDENSE
            .get(if let Token::Operator(op, _) = tokens.last().unwrap() {
                op.as_str()
            } else {
                " "
            })
            .unwrap_or(&-1);
        if tok_precedense < next_tok_precedense {
            rhs = parse_bin_op_rhs(tokens, rhs, tok_precedense + 1);
        }
        lhs = AST::BinaryOp(
            if let Token::Operator(op, _) = operator {
                op
            } else {
                panic!("Imposible")
            },
            Box::new(lhs),
            Box::new(rhs),
        );
    }
}

fn parse_paren(tokens: &mut Vec<Token>) -> AST {
    tokens.pop();
    let lhs = parse_expression(tokens);
    if !matches!(tokens.pop().unwrap(), Token::RParen(_)) {
        panic!("Expected ')'");
    }

    lhs
}

fn parse_unary(tokens: &mut Vec<Token>) -> AST {
    let op = if let Token::Operator(op, _) = tokens.pop().unwrap() {
        op
    } else {
        panic!("Imposible")
    };
    AST::UnaryOp(op, Box::new(parse_primary(tokens)))
}

fn parse_init(tokens: &mut Vec<Token>) -> AST {
    let lhs = match tokens.pop().unwrap() {
        Token::Ident(id, _) => AST::Variable(id),
        _ => panic!("Expected identifier."),
    };

    parse_bin_op_rhs(tokens, lhs, 0)
}

fn parse_function_def(tokens: &mut Vec<Token>) -> AST {
    let mut params: Vec<AST> = vec![];
    let Token::Ident(name, _) = tokens.pop().unwrap() else {
        panic!("Expected function name")
    };
    if let Token::LParen(_) = tokens.pop().unwrap() {
    } else {
        panic!("Expected `(`")
    }
    while let Token::Ident(name, _) = tokens.pop().unwrap() {
        params.push(AST::Variable(name))
    }

    if let Token::StartBlock(_) = tokens.pop().unwrap() {
    } else {
        panic!("Expected code block")
    }
    dbg!(&tokens);
    let next = parse(tokens);
    dbg!(&next);
    AST::Function(name, params, next)
}

fn parse_call(tokens: &mut Vec<Token>) -> Vec<AST> {
    let mut params = vec![];
    tokens.pop();
    while matches!(tokens.last().unwrap(), Token::RParen(_)) {
        params.push(parse_primary(tokens));
    }
    tokens.pop();
    params
}

fn parse_ident(tokens: &mut Vec<Token>) -> AST {
    match tokens.pop().unwrap() {
        Token::Ident(id, _) => {
            match id.as_str() {
                "let" => parse_init(tokens),
                "fun" => parse_function_def(tokens),
                _ => {
                    if let Token::LParen(_) = tokens.last().unwrap() {
                        return AST::Call(id, parse_call(tokens));
                    }
                    AST::Variable(id)
                } // _ => panic!("Unknown indentifier `{}`", id)
            }
        }
        _ => panic!("Can't happen."),
    }
}

fn parse_string(tokens: &mut Vec<Token>) -> AST {
    match tokens.pop().unwrap() {
        Token::String(string, _) => AST::String(string),
        _ => panic!("Cant happen."),
    }
}

fn parse_primary(tokens: &mut Vec<Token>) -> AST {
    let lhs;
    match tokens.last().unwrap() {
        Token::Numb(_, _) => {
            lhs = parse_number(tokens);
        }
        Token::LParen(_) => {
            lhs = parse_paren(tokens);
        }
        Token::Operator(_, _) => {
            lhs = parse_unary(tokens);
        }
        Token::Ident(_, _) => {
            lhs = parse_ident(tokens);
        }
        Token::String(_, _) => {
            lhs = parse_string(tokens);
        }
        Token::Unknown => {
            panic!("Unknown token.");
        }
        Token::EOF(_) => {
            lhs = AST::Number(0.0);
            // tokens.pop();
            // lhs = parse_primary(tokens);
        }
        default => {
            panic!(
                "Parser error: Unexpected token. {:?}.\nRest of tokens: {:?}",
                default, tokens
            )
        }
    }
    lhs
}

fn parse_expression(tokens: &mut Vec<Token>) -> AST {
    let lhs = parse_primary(tokens);

    parse_bin_op_rhs(tokens, lhs, 0)
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<AST> {
    let mut ast: Vec<AST> = vec![];
    // tokens.reverse();
    loop {
        match tokens.last().unwrap() {
            Token::EOF(_) => return ast,
            Token::Delim(_) => {
                tokens.pop();
                continue;
            }
            Token::EndBlock(_) => {
                dbg!("Hello", tokens.pop(), &ast);
                return ast;
            }
            _ => {
                ast.push(parse_expression(tokens));
            }
        }
    }
}
