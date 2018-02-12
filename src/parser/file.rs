use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{OptResult, Result, ParseError, TokenResult};
use parser::utils;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_script(&mut self) -> Result<()> {
        while let TokenResult::Some(_) = self.parse_script_item()? {
        }

        eat_value!(self.eof());
        Ok(())
    }
    pub fn parse_module(&mut self) -> Result<()> {
        while let TokenResult::Some(_) = self.parse_module_item()? {
        }

        eat_value!(self.eof());
        Ok(())
    }

    fn parse_script_item(&mut self) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_declaration()?,
            self.parse_statement()?,
        ))
    }

    fn parse_module_item(&mut self) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_script_item()?,
            self.parse_import_declaration()?,
            self.parse_export_declaration()?,
        ))
    }
}
