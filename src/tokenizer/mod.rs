mod slice;
pub mod tokens;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Hint {
  expression: bool,
  template: bool,
  strict: bool
}
impl Hint {
    fn expression(mut self, expression: bool) -> Hint {
        self.expression = expression;
        self
    }
    fn template(mut self, template: bool) -> Hint {
        self.template = template;
        self
    }
    fn strict(mut self, strict: bool) -> Hint {
        self.strict = strict;
        self
    }
}

pub trait Tokenizer: Clone {
    fn next_token(&mut self, &Hint) -> tokens::Token;
}

pub trait IntoTokenizer {
    type Item: Tokenizer;

    fn into_tokenizer(self) -> Self::Item;
}
