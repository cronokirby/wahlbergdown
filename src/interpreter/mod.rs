mod lexer;
mod parser;

use std::{collections::HashMap, fmt};

use parser::{Definition, Expr};

use crate::parser::Code;

use self::parser::Ident;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn new_parser<'a>(code: &'a Code) -> parser::Parser<'a> {
    parser::Parser::new(lexer::Lexer::new(&code.0))
}

#[derive(Clone, Debug)]
pub struct Interpreter {
    values: HashMap<Ident, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn eval_definition(&mut self, def: Definition) {
        let val = self.eval_expr(def.expr);
        self.values.insert(def.ident, val);
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Nil => Value::Nil,
            Expr::Int(i) => Value::Int(i),
            Expr::Ident(i) => self.values.get(&i).unwrap_or(&Value::Nil).clone(),
        }
    }

    pub fn definition(&mut self, code: Code) -> Result<(), String> {
        let def = new_parser(&code).parse_definition()?;
        Ok(self.eval_definition(def))
    }

    pub fn expr(&mut self, code: Code) -> Result<Value, String> {
        let expr = new_parser(&code).parse_expr()?;
        Ok(self.eval_expr(expr))
    }
}
