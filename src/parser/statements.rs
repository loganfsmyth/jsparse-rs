use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_statement(&mut self) -> InnerResult<()> {
        self.expect_expression();

        try_sequence!(
            self.parse_block_statement(),
            self.parse_variable_statement(),
            self.parse_empty_statement(),
            self.parse_expression_statement(),
            self.parse_if_statement(),
            self.parse_breakable_statement(),
            self.parse_continue_statement(),
            self.parse_break_statement(),
            self.parse_return_statement(),
            self.parse_with_statement(),
            self.parse_labelled_statement(),
            self.parse_throw_statement(),
            self.parse_try_statement(),
            self.parse_debugger_statement(),
        )
    }

    pub fn parse_block_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_variable_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("var")?;
    }

    pub fn parse_empty_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_expression_statement(&mut self) -> InnerResult<()> {
        self.with_in(true).parse_expression()?;

        self.semicolon();

        Ok(())
    }

    pub fn parse_if_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("if")?;
    }

    pub fn parse_breakable_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_continue_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("continue")?;
    }
    pub fn parse_break_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("break")?;
    }

    pub fn parse_return_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("return")?;
    }

    pub fn parse_with_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("with")?;
    }

    pub fn parse_labelled_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_throw_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("throw")?;
    }

    pub fn parse_try_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("try")?;
    }

    pub fn parse_debugger_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
        self.try_identifier("debugger")?;
    }
}
