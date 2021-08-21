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
}

impl<'a> Lexer<'a> {
    /// Create a new lexer with some source code.
    fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekmore(),
            pos: 0,
            raw_acc: String::new(),
        }
    }

    fn finalize(self) -> Option<Token> {
        if self.raw_acc.is_empty() {
            None
        } else {
            Some(Token::Raw(self.raw_acc))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        loop {
            let next = match self.src.next() {
                None => return None,
                Some(c) => c,
            };

            match next {
                '\n' => return Some(Newline),
                '`' => return Some(Tick),
                '<' => {
                    if self.src.peek_nth(0).map_or(false, |x| *x == '!')
                        && self.src.peek_nth(1).map_or(false, |x| *x == '-')
                        && self.src.peek_nth(2).map_or(false, |x| *x == '-')
                    {
                        self.src.next();
                        self.src.next();
                        self.src.next();

                        return Some(CommentOpen);
                    }
                }
                '-' => {
                    if self.src.peek_nth(0).map_or(false, |x| *x == '-')
                        && self.src.peek_nth(1).map_or(false, |x| *x == '>')
                    {
                        self.src.next();
                        self.src.next();

                        return Some(CommentClose);
                    }
                }
                _ => {}
            }
            self.raw_acc.push(next)
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
