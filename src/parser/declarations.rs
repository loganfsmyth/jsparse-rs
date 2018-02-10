use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::OptResult;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_declaration(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_function_declaration(),
            self.parse_class_declaration(),
            self.parse_let_declaration(),
            self.parse_const_declaration(),
        )
    }

    pub fn parse_let_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("let"));

        Ok(Some(()))
    }

    pub fn parse_const_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("const"));

        Ok(Some(()))
    }
}
