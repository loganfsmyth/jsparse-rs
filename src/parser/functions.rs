use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{OptResult};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_function_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("function"));

        Ok(Some(()))
    }
    pub fn parse_function_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("function"));

        Ok(Some(()))
    }
}
