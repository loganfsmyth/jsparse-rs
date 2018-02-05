use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_function_declaration(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    pub fn parse_function_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
}
