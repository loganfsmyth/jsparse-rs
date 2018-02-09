use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_import_declaration(&mut self) -> InnerResult<()>  {
        return Err(InnerError::NotFound);
        self.try_keyword("import")?;

        Ok(())
    }

    pub fn parse_export_declaration(&mut self) -> InnerResult<()>  {
        return Err(InnerError::NotFound);
        self.try_keyword("export")?;

        Ok(())
    }
}
