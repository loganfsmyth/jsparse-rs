use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_declaration(&mut self) -> InnerResult<()> {
        try_sequence!(
            self.parse_function_declaration(),
            self.parse_class_declaration(),
            self.parse_let_declaration(),
            self.parse_const_declaration(),
        )
    }

    pub fn parse_let_declaration(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("let")?;
    }

    pub fn parse_const_declaration(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("const")?;
    }
}
