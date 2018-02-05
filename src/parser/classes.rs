use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);

        self.try_identifier("class")?;

        Ok(())
    }
    pub fn parse_class_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("class")?;

        Ok(())
    }
}
