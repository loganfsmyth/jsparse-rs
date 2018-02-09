use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> InnerResult<()> {
        self.try_keyword("class")?;

        Ok(())
    }
    pub fn parse_class_expression(&mut self) -> InnerResult<()> {
        self.try_keyword("class")?;

        Ok(())
    }
}
