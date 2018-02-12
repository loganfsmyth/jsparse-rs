use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{OptResult, Result, ParseError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_script(&mut self) -> Result<()> {
        let mut body = vec![];
        loop {
            match self.parse_script_item()? {
                Some(item) => body.push(item),
                None => { break; }
            }
        }

        eat_token!(self.eof());
        Ok(())
    }
    pub fn parse_module(&mut self) -> Result<()> {
        let mut body = vec![];
        loop {
            match self.parse_module_item()? {
                Some(item) => body.push(item),
                None => { break; }
            }
        }

        eat_token!(self.eof());
        Ok(())
    }

    fn parse_script_item(&mut self) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_statement()?,
            self.parse_declaration()?,
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
