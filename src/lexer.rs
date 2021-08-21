/// Represents a single token produced by our lexer
#[derive(Clone, Debug, PartialEq)]
enum Token {
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
struct Lexer {
    /// The source code for our program.
    src: String,
    /// The current position in our source code.
    pos: usize,
}

impl Lexer {
    /// Create a new lexer with some source code.
    fn new(src: String) -> Self {
        Lexer { src, pos: 0 }
    }
}
