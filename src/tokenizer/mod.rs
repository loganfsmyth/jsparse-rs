mod slice;
pub mod tokens;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Hint {
    expression: bool,
    template: bool,
    strict: bool,
}
impl Hint {
    pub fn expression(mut self, expression: bool) -> Hint {
        self.expression = expression;
        self
    }
    pub fn template(mut self, template: bool) -> Hint {
        self.template = template;
        self
    }
    pub fn strict(mut self, strict: bool) -> Hint {
        self.strict = strict;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    // A offset in the byte stream.
    pub offset: usize,

    // A 1-indexed line number, treating \r\n as a single line.
    pub line: usize,

    // A 0-indexed column number in code points.
    pub column: usize,
}

pub struct TokenRange {
    pub start: Position,
    pub end: Position,
}

use std::collections::HashMap;

pub trait Tokenizer<'code>: Clone + ::std::fmt::Debug {

    fn stats(&self) -> &HashMap<&'static str, ( u64, u64 )>;

    fn next_token(&mut self, &Hint) -> (tokens::Token<'code>, TokenRange);
}

pub trait IntoTokenizer<'code> {
    type Item: Tokenizer<'code>;

    fn into_tokenizer(self) -> Self::Item;
}
