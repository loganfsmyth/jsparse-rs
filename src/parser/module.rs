use tokenizer::Tokenizer;
use parser::Parser;
use parser::utils::OptResult;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_import_declaration(&mut self) -> OptResult<()>  {
        try_token!(self.keyword("import"));

        Ok(Some(()))
    }

    pub fn parse_export_declaration(&mut self) -> OptResult<()>  {
        try_token!(self.keyword("export"));

        Ok(Some(()))
    }
}
