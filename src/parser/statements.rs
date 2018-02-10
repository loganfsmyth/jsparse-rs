use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::OptResult;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_statement(&mut self) -> OptResult<()> {
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

    pub fn parse_block_statement(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

        Ok(Some(()))
    }

    pub fn parse_variable_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("var"));

        Ok(Some(()))
    }

    pub fn parse_empty_statement(&mut self) -> OptResult<()> {
        Ok(None)
    }

    pub fn parse_expression_statement(&mut self) -> OptResult<()> {
        try_fn!(self.with(Flag::In).parse_expression());

        try_token!(self.semicolon());

        Ok(Some(()))
    }

    pub fn parse_if_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("if"));

        Ok(Some(()))
    }

    pub fn parse_breakable_statement(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_iteration_statement(),
            self.parse_switch_statement(),
        )
    }

    pub fn parse_iteration_statement(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_do_while_statement(),
            self.parse_while_statement(),
            self.parse_for_statement(),
        )
    }

    pub fn parse_do_while_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("do"));

        Ok(Some(()))
    }

    pub fn parse_while_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("while"));

        Ok(Some(()))
    }

    pub fn parse_for_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("for"));

        Ok(Some(()))
    }

    pub fn parse_switch_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("switch"));

        Ok(Some(()))
    }

    pub fn parse_continue_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("continue"));

        Ok(Some(()))
    }
    pub fn parse_break_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("break"));

        Ok(Some(()))
    }

    pub fn parse_return_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("return"));

        Ok(Some(()))
    }

    pub fn parse_with_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("with"));

        Ok(Some(()))
    }

    pub fn parse_labelled_statement(&mut self) -> OptResult<()> {
        Ok(None)
    }

    pub fn parse_throw_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("throw"));

        Ok(Some(()))
    }

    pub fn parse_try_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("try"));

        Ok(Some(()))
    }

    pub fn parse_debugger_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("debugger"));

        Ok(Some(()))
    }
}
