use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_declaration(&mut self) -> InnerResult<()> {
        try_sequence!(
            self.parse_function_declaration(),
            self.parse_class_declaration(),
            self.parse_let_declaration(),
            self.parse_const_declaration(),
        )
    }

    pub fn parse_let_declaration(&mut self) -> InnerResult<()> {
        self.keyword("let")?;

        Ok(())
    }

    pub fn parse_const_declaration(&mut self) -> InnerResult<()> {
        self.keyword("const")?;

        Ok(())
    }
}
