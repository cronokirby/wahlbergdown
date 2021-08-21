/// Code represents a snippet of actual code.
#[derive(Debug, PartialEq)]
pub struct Code(String);

/// Chunk represent an individual chunk composing our document.
#[derive(Debug, PartialEq)]
pub enum Chunk {
    /// A commented bit of code, which should be executed, but the result discarded.
    Comment(Code),
    /// A interpolated bit of code, which should be executed and inlined.
    Interpolate(Code),
    /// A raw chunk of document which doesn't need to be executed at all.
    Raw(String),
}

/// A document represents a Wahlbergdown document.
#[derive(Debug, PartialEq)]
pub struct Document {
    /// Chunks contains each of the individual chunks composing our document.
    pub chunks: Vec<Chunk>,
}
