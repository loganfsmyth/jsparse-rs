use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_function_declaration(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    pub fn parse_function_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
}
