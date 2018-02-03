extern crate ucd;

// #[macro_use]
// pub mod ast;

mod tokenizer;

pub use tokenizer::IntoTokenizer;
pub use tokenizer::Tokenizer;

// pub mod parser;

#[derive(Debug)]
pub struct Parser<T> {
    tok: T
}

impl<T: Tokenizer> Parser<T>
{
    pub fn new<U>(code: U) -> Parser<T>
    where
        U: tokenizer::IntoTokenizer<Item = T>
    {
        Parser {
            tok: code.into_tokenizer()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Parser::new("a_var");
    }
}
