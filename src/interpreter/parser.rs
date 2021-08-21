use std::iter::Peekable;

use crate::interpreter::lexer::{Lexer, Token};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident(String);

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    Nil,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub ident: Ident,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

pub type ParseResult<T> = Result<T, String>;

impl<'a> Parser<'a> {
    pub fn new(tokens: Lexer<'a>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn done(&mut self) -> bool {
        self.peek().is_none()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn expect<T, F>(&mut self, matcher: F) -> ParseResult<T>
    where
        F: Fn(Token) -> Option<T>,
    {
        match self.next() {
            None => Err(format!("unexpected EOF")),
            Some(tok) => match matcher(tok.clone()) {
                None => Err(format!("unexpected token {:?}", tok)),
                Some(t) => Ok(t),
            },
        }
    }

    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        match self.peek() {
            None => Err(format!("unexpected EOF")),
            Some(Token::Identifier(_)) => match self.next() {
                Some(Token::Identifier(i)) => Ok(Expr::Ident(Ident(i))),
                _ => unreachable!(),
            },
            Some(Token::Int(i)) => {
                let expr = Expr::Int(*i);
                self.next();
                Ok(expr)
            }
            Some(Token::Nil) => {
                self.next();
                Ok(Expr::Nil)
            }
            Some(tok) => Err(format!("unexpected token {:?}", tok)),
        }
    }

    pub fn parse_definition(&mut self) -> ParseResult<Definition> {
        let ident = self.expect(|x| match x {
            Token::Identifier(i) => Some(Ident(i)),
            _ => None,
        })?;
        self.expect(|x| match x {
            Token::Is => Some(()),
            _ => None,
        })?;
        let expr = self.parse_expr()?;
        Ok(Definition { ident, expr })
    }
}
