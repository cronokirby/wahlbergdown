mod lexer;
mod parser;

use std::{collections::HashMap, fmt, hash::Hash};

use parser::{Definition, Expr};

use crate::parser::Code;

use self::parser::Ident;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Nil,
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Value::Int(x) => *x != 0,
            Value::Nil => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug)]
struct Function {
    args: Vec<Ident>,
    body: Expr,
}

fn new_parser<'a>(code: &'a Code) -> parser::Parser<'a> {
    parser::Parser::new(lexer::Lexer::new(&code.0))
}

#[derive(Clone, Debug)]
struct Values {
    scopes: Vec<HashMap<Ident, Value>>,
}

impl Values {
    fn new() -> Self {
        Values {
            scopes: vec![HashMap::new()],
        }
    }

    fn exit(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn put(&mut self, ident: Ident, value: Value) {
        self.scopes.last_mut().unwrap().insert(ident, value);
    }

    fn get(&mut self, ident: Ident) -> Value {
        self.scopes
            .last()
            .unwrap()
            .get(&ident)
            .cloned()
            .unwrap_or(Value::Nil)
    }
}

#[derive(Clone, Debug)]
pub struct Interpreter {
    values: Values,
    funcs: HashMap<Ident, Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            values: Values::new(),
            funcs: HashMap::new(),
        }
    }

    fn eval_definition(&mut self, def: Definition) {
        match def {
            Definition::Value(i, e) => {
                let v = self.eval_expr(e);
                self.values.put(i, v);
            }
            Definition::Func(name, args, body) => {
                self.funcs.insert(name, Function { args, body });
            }
        }
    }

    fn accumulate<A, T>(
        &mut self,
        args: Vec<Expr>,
        extract: impl Fn(Value) -> Option<T>,
        init: A,
        combine: impl Fn(A, T) -> A,
        wrap: impl Fn(A) -> Value,
    ) -> Value {
        let mut acc = init;
        for x in args {
            match extract(self.eval_expr(x)) {
                None => return Value::Nil,
                Some(v) => acc = combine(acc, v),
            }
        }
        wrap(acc)
    }

    fn function_call(&mut self, ident: Ident, arg_values: Vec<Value>) -> Value {
        match self.funcs.get(&ident) {
            None => Value::Nil,
            Some(Function { args, body }) => {
                self.values.enter();
                for (i, arg_name) in args.iter().enumerate() {
                    self.values.put(
                        arg_name.clone(),
                        arg_values.get(i).unwrap_or(&Value::Nil).clone(),
                    )
                }
                let the_body = body.clone();
                let ret = self.eval_expr(the_body);
                self.values.exit();
                ret
            }
        }
    }

    fn call(&mut self, ident: Ident, args: Vec<Expr>) -> Value {
        match ident.0.as_str() {
            "+" => self.accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                0,
                |x, y| x + y,
                |x| Value::Int(x),
            ),
            "*" => self.accumulate(
                args,
                |x| match x {
                    Value::Int(i) => Some(i),
                    _ => None,
                },
                1,
                |x, y| x * y,
                |x| Value::Int(x),
            ),
            "-" => self.accumulate(
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
            "/" => self.accumulate(
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
            "if" => {
                let condition = args
                    .get(0)
                    .map_or(Value::Nil, |x| self.eval_expr(x.clone()));
                if condition.truthy() {
                    args.get(1)
                        .map_or(Value::Nil, |x| self.eval_expr(x.clone()))
                } else {
                    args.get(2)
                        .map_or(Value::Nil, |x| self.eval_expr(x.clone()))
                }
            }
            _ => {
                let arg_values = args.into_iter().map(|x| self.eval_expr(x)).collect();
                self.function_call(ident, arg_values)
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Nil => Value::Nil,
            Expr::Int(i) => Value::Int(i),
            Expr::Ident(i) => self.values.get(i),
            Expr::Call(i, args) => self.call(i, args),
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
