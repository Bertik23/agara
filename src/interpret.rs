use crate::parser::ASTTree;

trait Run {
    fn execute(&self) -> f64;
}

impl Run for ASTTree{
    fn execute(&self) -> f64{
        match self {
            ASTTree::Number(num) => {return *num;}
            ASTTree::BinaryOp(op, lhs, rhs) => {
                match op.as_str(){
                    "+" => {return lhs.execute() + rhs.execute()}
                    "-" => {return lhs.execute() - rhs.execute()}
                    "*" => {return lhs.execute() * rhs.execute()}
                    "/" => {return lhs.execute() / rhs.execute()}
                    "%" => {return lhs.execute() % rhs.execute()}
                    "**" => {return lhs.execute().powf(rhs.execute())}
                    _ => todo!()
                }
            }
            ASTTree::UnaryOp(op, exp) => {
                match op.as_str() {
                    "+" => {return exp.execute();}
                    "-" => {return -exp.execute();}
                    default => todo!("No implementation for operator `{}`.", default)
                }
            }
            default => todo!("No implementation for {:?}", default)
        }
    }
}

pub fn run(ast: Vec<ASTTree>){
    for (l, a) in ast.into_iter().enumerate(){
        println!("{}: {}", l, a.execute());
    }
}