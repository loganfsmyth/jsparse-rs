mod root;

use tokenizer::{IntoTokenizer, Tokenizer};
use tokenizer::tokens;

struct Ast {
    field: u32,
}

fn parse_root<T, P>(t: T) -> P
where
    T: IntoTokenizer,
    P: FromTokenizer
{
    FromTokenizer::from_tokenizer(t)
}

pub trait FromTokenizer {
    fn from_tokenizer<T: IntoTokenizer>(t: T) -> Self;
}

impl FromTokenizer for Ast {
    fn from_tokenizer<T: IntoTokenizer>(t: T) -> Ast {
        let mut p = Parser {
            tok: t.into_tokenizer(),
        };

        p.parse_root();
    }
}

struct Parser<T> {
    tok: T,

}

impl<T> Parser<T> {

}

