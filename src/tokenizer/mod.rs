mod new;
pub mod tokens;

#[derive(Debug, Default)]
pub struct Hint {
  expression: bool,
  template: bool,
  strict: bool
}
impl Hint {
    fn expression(mut self) -> Hint {
        self.expression = true;
        self
    }
    fn template(mut self) -> Hint {
        self.template = true;
        self
    }
    fn strict(mut self) -> Hint {
        self.strict = true;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    // The number of characters in this token.
    pub len: usize,

    // The number of _new_ lines created by this token.
    // \r\n counts as one line terminator.
    pub lines: usize,

    // The number of characters in the last line that this
    // token covers.
    pub width: usize,
}

pub trait Tokenizer: Clone {
    fn next_token(&mut self, &Hint) -> tokens::Token;
}

pub trait IntoTokenizer {
    type Item: Tokenizer;

    fn into_tokenizer(self) -> Self::Item;
}


// #[derive(Clone)]
// pub struct IteratorTokenizer<T> {
//     code: T,
// }

// impl<T> Tokenizer for IteratorTokenizer<T>
// where
//     T: Iterator<Item = char> + Clone
// {
//     fn next_token(&self, _hint: &Hint) -> tokens::Token {
//         unimplemented!("tokenize iterator")
//     }
// }


