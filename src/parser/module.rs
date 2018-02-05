use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_import_declaration(&mut self) -> InnerResult<()>  {
        return Err(InnerError::NotFound);
        self.try_identifier("import")?;

        Ok(())
    }

    pub fn parse_export_declaration(&mut self) -> InnerResult<()>  {
        return Err(InnerError::NotFound);
        self.try_identifier("export")?;

        Ok(())
    }
}
