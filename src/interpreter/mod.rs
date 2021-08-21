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

fn accumulate<A, T>(
    args: Vec<Value>,
    extract: impl Fn(Value) -> Option<T>,
    init: A,
    combine: impl Fn(A, T) -> A,
    wrap: impl Fn(A) -> Value,
) -> Value {
    let mut acc = init;
    for x in args {
        match extract(x) {
            None => return Value::Nil,
            Some(v) => acc = combine(acc, v),
        }
    }
    wrap(acc)
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

    fn call(&mut self, ident: Ident, args: Vec<Value>) -> Value {
        match ident.0.as_str() {
            "+" => accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                0,
                |x, y| x + y,
                |x| Value::Int(x),
            ),
            "*" => accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                1,
                |x, y| x * y,
                |x| Value::Int(x),
            ),
            "-" => accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                None,
                |x, y| match x {
                    None => Some(y),
                    Some(acc) => Some(acc - y),
                },
                |x| Value::Int(x.unwrap_or(0)),
            ),
            "/" => accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                None,
                |x, y| match x {
                    None => Some(y),
                    Some(acc) => Some(acc / y),
                },
                |x| Value::Int(x.unwrap_or(1)),
            ),
            _ => Value::Nil,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Nil => Value::Nil,
            Expr::Int(i) => Value::Int(i),
            Expr::Ident(i) => self.values.get(&i).unwrap_or(&Value::Nil).clone(),
            Expr::Call(i, args) => {
                let arg_values: Vec<Value> = args.into_iter().map(|a| self.eval_expr(a)).collect();
                self.call(i, arg_values)
            }
        }
    }

    pub fn definition(&mut self, code: Code) -> Result<(), String> {
        let def = new_parser(&code).top_level_definition()?;
        Ok(self.eval_definition(def))
    }

    pub fn expr(&mut self, code: Code) -> Result<Value, String> {
        let expr = new_parser(&code).top_level_expr()?;
        Ok(self.eval_expr(expr))
    }
}
