use crate::parser::ASTTree;

use std::{collections::HashMap, mem, fmt};

#[derive(Clone, Debug)]
pub struct Context{
    pub variables: HashMap<String, Object>
}

#[derive(Clone, Debug)]
pub enum Object{
    Float(f64),
    String(String),
    Variable(String, Box<Context>),
    Function(String, Vec<ASTTree>, Vec<ASTTree>)
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Float(x) => write!(f, "{}", x),
            Object::Variable(x, context) => write!(f, "{} = {}", x, context.variables.get(x.as_str()).unwrap()),
            Object::String(x) => write!(f, "{}", x),
            Object::Function(name, _, _) => write!(f, "Function {}", name)
        }
    }
}

impl Object {
    fn add(&self, other: &Self) -> Object{
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {return Object::Float(s+x)}
                _ => todo!("Operation ADD not implemented for {:?} and {:?}", self, other)
            }
        }
        todo!("Operation ADD not implemented for {:?} and {:?}", self, other)
    }

    fn neg(&self) -> Object{
        match self {
            Object::Float(x) => {return Object::Float(-x)}
            _ => todo!("Operation NEG not implemented for {:?}", self)
        }
    }

    fn mult(&self, other: &Self) -> Object{
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {return Object::Float(s*x)}
                _ => todo!("Operation MULT not implemented for {:?} and {:?}", self, other)
            }
        }
        todo!("Operation MULT not implemented for {:?} and {:?}", self, other)
    }

    fn div(&self, other: &Self) -> Object{
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {return Object::Float(s/x)}
                _ => todo!("Operation DIV not implemented for {:?} and {:?}", self, other)
            }
        }
        todo!("Operation DIV not implemented for {:?} and {:?}", self, other)
    }

    fn modu(&self, other: &Self) -> Object{
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {return Object::Float(s%x)}
                _ => todo!("Operation MODU not implemented for {:?} and {:?}", self, other)
            }
        }
        todo!("Operation MODU not implemented for {:?} and {:?}", self, other)
    }

    fn pow(&self, other: &Self) -> Object{
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {return Object::Float(s.powf(*x))}
                _ => todo!("Operation POW not implemented for {:?} and {:?}", self, other)
            }
        }
        todo!("Operation POW not implemented for {:?} and {:?}", self, other)
    }
}

trait Run {
    fn execute(&self, context: &mut Context) -> Object;
}

impl Run for ASTTree{
    fn execute(&self, context: &mut Context) -> Object{
        match self {
            ASTTree::Number(num) => {return Object::Float(*num);}
            ASTTree::BinaryOp(op, lhs, rhs) => {
                match op.as_str(){
                    "+" => {return lhs.execute(context).add(&rhs.execute(context))}
                    "-" => {return lhs.execute(context).add(&rhs.execute(context).neg())}
                    "*" => {return lhs.execute(context).mult(&rhs.execute(context))}
                    "/" => {return lhs.execute(context).div(&rhs.execute(context))}
                    "%" => {return lhs.execute(context).modu(&rhs.execute(context))}
                    "**" => {return lhs.execute(context).pow(&rhs.execute(context))}
                    "=" => {
                        let name;
                        // println!("This");
                        let mut cntx = context.clone();
                        context.variables.insert(
                            match &**lhs{ ASTTree::Variable(s) => {name = s; s.clone()}, _ => panic!() },
                            rhs.execute(&mut cntx)
                        );
                        // dbg!(name, &context.variables);
                        // println!("And This");
                        return Object::Variable(name.clone(), Box::new(context.clone()))
                    }
                    _ => todo!()
                }
            }
            ASTTree::UnaryOp(op, exp) => {
                match op.as_str() {
                    "+" => {return exp.execute(context);}
                    "-" => {return exp.execute(context).neg();}
                    default => todo!("No implementation for operator `{}`.", default)
                }
            }
            ASTTree::Variable(name) => {
                // dbg!(name, VARIABLES.lock().unwrap());
                return context.variables.get(name.as_str()).unwrap().clone()}
            ASTTree::String(string) => {
                return Object::String(string.clone())
            }
            ASTTree::Function(name, variables, code) => {
                return Object::Function(name.clone(), variables.clone(), code.clone());
            }
            default => todo!("No implementation for {:?}", default)
        }
    }
}

pub fn run(ast: Vec<ASTTree>, context: &mut Context){
    for (l, a) in ast.iter().enumerate(){
        // println!("Something");
        //let mut cntx = context.clone();
        println!("{}: {}", l, a.execute(context));
        dbg!(&context);
    }
}
