use core::panic;

extern crate lazy_static;


use lazy_static::lazy_static;

use crate::tokenizer::{Token};
use std::collections::HashMap;

lazy_static!{
    static ref BINOP_PRECEDENSE: HashMap<&'static str, i32> = HashMap::from(
        [
            (" ", -10),
            ("+", 10),
            ("-", 10),
            ("*", 20),
            ("%", 20),
            ("/", 20),
            ("**", 30)
        ]
    );
}

#[derive(Debug, Clone)]
pub enum ASTTree{
    Number(f64),
    Variable(String),
    UnaryOp(String, Box<ASTTree>),
    BinaryOp(String, Box<ASTTree>, Box<ASTTree>),
    Call(String, Vec<ASTTree>)
}

fn parse_number(tokens: &mut Vec<Token>) -> ASTTree{
    ASTTree::Number(if let Token::Numb(n, _) = tokens.pop().unwrap(){n} else {panic!("WTF")})
}

fn parse_bin_op_rhs(tokens: &mut Vec<Token>, lhs_: ASTTree, min_token_precedense: i32) -> ASTTree{
    let mut lhs = lhs_.clone();
    loop{
        let tok_precedense;
        {
            let cur_tok = tokens.last().unwrap();
            tok_precedense = BINOP_PRECEDENSE.get(
                if let Token::Operator(op, _) = cur_tok { op.as_str() }
                else {return lhs}
            ).unwrap_or(&-1);
        }
        if *tok_precedense < min_token_precedense{
            return lhs;
        }
        let operator = tokens.pop().unwrap();
        let mut rhs = parse_primary(tokens);
        let next_tok_precedense = BINOP_PRECEDENSE.get(
            if let Token::Operator(op, _) = tokens.last().unwrap() { op.as_str() }
            else {" "}
        ).unwrap_or(&-1);
        if tok_precedense < next_tok_precedense{
            rhs = parse_bin_op_rhs(tokens, rhs, tok_precedense + 1);
        }
        lhs = ASTTree::BinaryOp(
            if let Token::Operator(op, _) = operator { op }
            else {panic!("Imposible")},
            Box::new(lhs),
            Box::new(rhs)
        );
    }
}

fn parse_paren(tokens: &mut Vec<Token>) -> ASTTree{
    tokens.pop();
    let lhs = parse_expression(tokens);
    if tokens.pop().unwrap() != Token::RParen(0){
        panic!("Expected ')'");
    }

    return lhs;
}

fn parse_unary(tokens: &mut Vec<Token>) -> ASTTree{
    let op = if let Token::Operator(op, _) = tokens.pop().unwrap() { op } else { panic!("Imposible") };
    return ASTTree::UnaryOp(op, Box::new(parse_primary(tokens)));
}

fn parse_primary(tokens: &mut Vec<Token>) -> ASTTree{
    let lhs;
    match tokens.last().unwrap(){
        Token::Numb(_,_) => {lhs = parse_number(tokens);}
        Token::LParen(_) => {lhs = parse_paren(tokens);}
        Token::Operator(_,_) => {lhs = parse_unary(tokens);}
        default => {panic!("Parser error: Unexpected token. {:?}.\nRest of tokens: {:?}", default, tokens)}
    }
    return lhs;
}

fn parse_expression(tokens: &mut Vec<Token>) -> ASTTree{
    let lhs = parse_primary(tokens);

    return parse_bin_op_rhs(tokens, lhs, 0)
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<ASTTree>{
    let mut ast: Vec<ASTTree> = vec![];
    tokens.reverse();
    loop {
        match tokens.last().unwrap() {
            Token::EOF(_) => {return ast}
            Token::Delim(_) => {tokens.pop(); continue;}
            _ => {ast.push(parse_expression(tokens));}
        }
    }
}