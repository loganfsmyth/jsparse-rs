use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
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
            self.parse_statement(),
            self.parse_debugger_statement(),
        )
    }

    pub fn parse_block_statement(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::CurlyOpen)?;

        Ok(())
    }

    pub fn parse_variable_statement(&mut self) -> InnerResult<()> {
        self.keyword("var")?;

        Ok(())
    }

    pub fn parse_empty_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_expression_statement(&mut self) -> InnerResult<()> {
        self.with(Flag::In).parse_expression()?;

        self.semicolon()?;

        Ok(())
    }

    pub fn parse_if_statement(&mut self) -> InnerResult<()> {
        self.keyword("if")?;

        Ok(())
    }

    pub fn parse_breakable_statement(&mut self) -> InnerResult<()> {
        try_sequence!(
            self.parse_iteration_statement(),
            self.parse_switch_statement(),
        )
    }

    pub fn parse_iteration_statement(&mut self) -> InnerResult<()> {
        try_sequence!(
            self.parse_do_while_statement(),
            self.parse_while_statement(),
            self.parse_for_statement(),
        )
    }

    pub fn parse_do_while_statement(&mut self) -> InnerResult<()> {
        self.keyword("do")?;

        Ok(())
    }

    pub fn parse_while_statement(&mut self) -> InnerResult<()> {
        self.keyword("while")?;

        Ok(())
    }

    pub fn parse_for_statement(&mut self) -> InnerResult<()> {
        self.keyword("for")?;

        Ok(())
    }

    pub fn parse_switch_statement(&mut self) -> InnerResult<()> {
        self.keyword("switch")?;

        Ok(())
    }

    pub fn parse_continue_statement(&mut self) -> InnerResult<()> {
        self.keyword("continue")?;

        Ok(())
    }
    pub fn parse_break_statement(&mut self) -> InnerResult<()> {
        self.keyword("break")?;

        Ok(())
    }

    pub fn parse_return_statement(&mut self) -> InnerResult<()> {
        self.keyword("return")?;

        Ok(())
    }

    pub fn parse_with_statement(&mut self) -> InnerResult<()> {
        self.keyword("with")?;

        Ok(())
    }

    pub fn parse_labelled_statement(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_throw_statement(&mut self) -> InnerResult<()> {
        self.keyword("throw")?;

        Ok(())
    }

    pub fn parse_try_statement(&mut self) -> InnerResult<()> {
        self.keyword("try")?;

        Ok(())
    }

    pub fn parse_debugger_statement(&mut self) -> InnerResult<()> {
        self.keyword("debugger")?;

        Ok(())
    }
}
