use crate::parser::AST;

use std::{collections::HashMap, fmt, mem};

#[derive(Clone, Debug)]
pub struct Context {
    pub variables: HashMap<String, Object>,
}

#[derive(Clone, Debug)]
pub enum Object {
    Float(f64),
    String(String),
    Variable(String, Box<Context>),
    Function(String, Vec<AST>, Vec<AST>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Float(x) => write!(f, "{}", x),
            Object::Variable(x, context) => write!(
                f,
                "{} = {}",
                x,
                context.variables.get(x.as_str()).unwrap()
            ),
            Object::String(x) => write!(f, "{}", x),
            Object::Function(name, _, _) => write!(f, "Function {}", name),
        }
    }
}

impl Object {
    fn add(&self, other: &Self) -> Object {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {
                    return Object::Float(s + x)
                }
                _ => todo!(
                    "Operation ADD not implemented for {:?} and {:?}",
                    self,
                    other
                ),
            }
        }
        todo!(
            "Operation ADD not implemented for {:?} and {:?}",
            self,
            other
        )
    }

    fn neg(&self) -> Object {
        match self {
            Object::Float(x) => Object::Float(-x),
            _ => todo!("Operation NEG not implemented for {:?}", self),
        }
    }

    fn mult(&self, other: &Self) -> Object {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {
                    return Object::Float(s * x)
                }
                _ => todo!(
                    "Operation MULT not implemented for {:?} and {:?}",
                    self,
                    other
                ),
            }
        }
        todo!(
            "Operation MULT not implemented for {:?} and {:?}",
            self,
            other
        )
    }

    fn div(&self, other: &Self) -> Object {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {
                    return Object::Float(s / x)
                }
                _ => todo!(
                    "Operation DIV not implemented for {:?} and {:?}",
                    self,
                    other
                ),
            }
        }
        todo!(
            "Operation DIV not implemented for {:?} and {:?}",
            self,
            other
        )
    }

    fn modu(&self, other: &Self) -> Object {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {
                    return Object::Float(s % x)
                }
                _ => todo!(
                    "Operation MODU not implemented for {:?} and {:?}",
                    self,
                    other
                ),
            }
        }
        todo!(
            "Operation MODU not implemented for {:?} and {:?}",
            self,
            other
        )
    }

    fn pow(&self, other: &Self) -> Object {
        if mem::discriminant(self) == mem::discriminant(other) {
            match (self, other) {
                (Object::Float(s), Object::Float(x)) => {
                    return Object::Float(s.powf(*x))
                }
                _ => todo!(
                    "Operation POW not implemented for {:?} and {:?}",
                    self,
                    other
                ),
            }
        }
        todo!(
            "Operation POW not implemented for {:?} and {:?}",
            self,
            other
        )
    }
}

trait Run {
    fn execute(&self, context: &mut Context) -> Object;
}

impl Run for AST {
    fn execute(&self, context: &mut Context) -> Object {
        match self {
            AST::Number(num) => Object::Float(*num),
            AST::BinaryOp(op, lhs, rhs) => {
                match op.as_str() {
                    "+" => lhs.execute(context).add(&rhs.execute(context)),
                    "-" => {
                        lhs.execute(context).add(&rhs.execute(context).neg())
                    }
                    "*" => lhs.execute(context).mult(&rhs.execute(context)),
                    "/" => lhs.execute(context).div(&rhs.execute(context)),
                    "%" => lhs.execute(context).modu(&rhs.execute(context)),
                    "**" => lhs.execute(context).pow(&rhs.execute(context)),
                    "=" => {
                        let name;
                        // println!("This");
                        let mut cntx = context.clone();
                        context.variables.insert(
                            match &**lhs {
                                AST::Variable(s) => {
                                    name = s;
                                    s.clone()
                                }
                                _ => panic!(),
                            },
                            rhs.execute(&mut cntx),
                        );
                        // dbg!(name, &context.variables);
                        // println!("And This");
                        Object::Variable(
                            name.clone(),
                            Box::new(context.clone()),
                        )
                    }
                    _ => todo!(),
                }
            }
            AST::UnaryOp(op, exp) => match op.as_str() {
                "+" => exp.execute(context),
                "-" => exp.execute(context).neg(),
                default => {
                    todo!("No implementation for operator `{}`.", default)
                }
            },
            AST::Variable(name) => {
                // dbg!(name, VARIABLES.lock().unwrap());
                return context.variables.get(name.as_str()).unwrap().clone();
            }
            AST::String(string) => Object::String(string.clone()),
            AST::Function(name, variables, code) => {
                Object::Function(name.clone(), variables.clone(), code.clone())
            }
            default => todo!("No implementation for {:?}", default),
        }
    }
}

pub fn run(ast: Vec<AST>, context: &mut Context) {
    for (l, a) in ast.iter().enumerate() {
        // println!("Something");
        //let mut cntx = context.clone();
        println!("{}: {}", l, a.execute(context));
        dbg!(&context);
    }
}
