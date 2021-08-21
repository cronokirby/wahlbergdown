use std::iter::Peekable;

use crate::interpreter::lexer::{Lexer, Token};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Nil,
    Int(i64),
    Ident(Ident),
    Call(Ident, Vec<Expr>),
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

    fn peek(&mut self) -> ParseResult<Option<&Token>> {
        match self.tokens.peek() {
            None => Ok(None),
            Some(Err(e)) => Err(e.clone()),
            Some(Ok(t)) => Ok(Some(t)),
        }
    }

    fn next(&mut self) -> ParseResult<Option<Token>> {
        match self.tokens.next() {
            None => Ok(None),
            Some(t) => Ok(Some(t?)),
        }
    }

    fn expect<T, F>(&mut self, matcher: F) -> ParseResult<T>
    where
        F: Fn(Token) -> Option<T>,
    {
        match self.next()? {
            None => Err(format!("unexpected EOF")),
            Some(tok) => match matcher(tok.clone()) {
                None => Err(format!("unexpected token {:?}", tok)),
                Some(t) => Ok(t),
            },
        }
    }

    fn expect_end(&mut self) -> ParseResult<()> {
        match self.next()? {
            None => Ok(()),
            Some(tok) => Err(format!("unexpected token {:?}", tok)),
        }
    }

    fn ident(&mut self) -> ParseResult<Ident> {
        self.expect(|x| match x {
            Token::Identifier(i) => Some(Ident(i)),
            _ => None,
        })
    }

    fn call(&mut self) -> ParseResult<Expr> {
        self.expect(|x| match x {
            Token::OpenParens => Some(()),
            _ => None,
        })?;
        let ident = self.ident()?;
        let mut exprs = Vec::new();
        loop {
            match self.peek()? {
                None => return Err(format!("unexpected EOF")),
                Some(Token::CloseParens) => {
                    self.next()?;
                    return Ok(Expr::Call(ident, exprs));
                }
                Some(_) => exprs.push(self.expr()?),
            }
        }
    }

    fn expr(&mut self) -> ParseResult<Expr> {
        match self.peek()? {
            None => Err(format!("unexpected EOF")),
            Some(Token::Identifier(_)) => match self.next()? {
                Some(Token::Identifier(i)) => Ok(Expr::Ident(Ident(i))),
                _ => unreachable!(),
            },
            Some(Token::Int(i)) => {
                let expr = Expr::Int(*i);
                self.next()?;
                Ok(expr)
            }
            Some(Token::Nil) => {
                self.next()?;
                Ok(Expr::Nil)
            }
            Some(Token::OpenParens) => self.call(),
            Some(tok) => Err(format!("unexpected token {:?}", tok)),
        }
    }

    fn definition(&mut self) -> ParseResult<Definition> {
        let ident = self.ident()?;
        self.expect(|x| match x {
            Token::Is => Some(()),
            _ => None,
        })?;
        let expr = self.expr()?;
        Ok(Definition { ident, expr })
    }

    pub fn top_level_expr(&mut self) -> ParseResult<Expr> {
        let expr = self.expr()?;
        self.expect_end()?;
        Ok(expr)
    }

    pub fn top_level_definition(&mut self) -> ParseResult<Definition> {
        let def = self.definition()?;
        self.expect_end()?;
        Ok(def)
    }
}
