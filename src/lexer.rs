// Represents a single token produced by our lexer
#[derive(Clone, Debug, PartialEq)]
enum Token {
    // <!--
    CommentOpen,
    // -->
    CommentClose,
    // `
    Tick,
    // The \n character
    Newline,
    // Everything else is just raw text.
    Raw(String),
}
