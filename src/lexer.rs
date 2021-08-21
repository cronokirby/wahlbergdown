use std::mem;
use std::str::Chars;

use peekmore::{PeekMore, PeekMoreIterator};

/// Represents a single token produced by our lexer
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// <!--
    CommentOpen,
    /// -->
    CommentClose,
    /// `
    Tick,
    /// The \n character
    Newline,
    /// Everything else is just raw text.
    Raw(String),
}

/// A Lexer uses our source code to emit tokens.
#[derive(Debug)]
struct Lexer<'a> {
    /// The source code for our program.
    src: PeekMoreIterator<Chars<'a>>,
    /// The current position in our source code.
    pos: usize,
    /// Used to a accumulate a raw string token
    raw_acc: String,
    /// This may contain a buffered output token
    produced: Option<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer with some source code.
    fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekmore(),
            pos: 0,
            raw_acc: String::new(),
            produced: None,
        }
    }

    fn take_raw(&mut self) -> Option<Token> {
        if self.raw_acc.is_empty() {
            None
        } else {
            Some(Token::Raw(mem::take(&mut self.raw_acc)))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if let Some(tok) = mem::take(&mut self.produced) {
            return Some(tok);
        }

        loop {
            let next = match self.src.next() {
                None => return self.take_raw(),
                Some(c) => c,
            };

            let produced = match next {
                '\n' => Some(Newline),
                '`' => Some(Tick),
                '<' => {
                    if self.src.peek_nth(0).map_or(false, |x| *x == '!')
                        && self.src.peek_nth(1).map_or(false, |x| *x == '-')
                        && self.src.peek_nth(2).map_or(false, |x| *x == '-')
                    {
                        self.src.next();
                        self.src.next();
                        self.src.next();

                        return Some(CommentOpen);
                    } else {
                        None
                    }
                }
                '-' => {
                    if self.src.peek_nth(0).map_or(false, |x| *x == '-')
                        && self.src.peek_nth(1).map_or(false, |x| *x == '>')
                    {
                        self.src.next();
                        self.src.next();

                        Some(CommentClose)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(tok) = produced {
                if let Some(raw) = self.take_raw() {
                    self.produced = Some(tok);
                    return Some(raw);
                } else {
                    return Some(tok);
                }
            }
            self.raw_acc.push(next);
        }
    }
}

/// Run a lexer on some character input.
///
///
/// This will return an iterator living as long as the string data, and yielding tokens.
pub fn lex<'a>(src: &'a str) -> impl Iterator<Item = Token> + 'a {
    Lexer::new(src)
}
