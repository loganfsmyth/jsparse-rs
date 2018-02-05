use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_script(&mut self) {
        let mut body = vec![];
        while let Ok(item) = self.parse_script_item() {
            body.push(item);
        }
    }
    pub fn parse_module(&mut self) {
        let mut body = vec![];
        while let Ok(item) = self.parse_module_item() {
            body.push(item);
        }
    }

    fn parse_script_item(&mut self) -> InnerResult<()> {
        self.expect_expression();

        try_sequence!(
            self.parse_statement(),
            self.parse_declaration(),
        )
    }

    fn parse_module_item(&mut self) -> InnerResult<()> {
        self.expect_expression();

        try_sequence!(
            self.parse_script_item(),
            self.parse_import_declaration(),
            self.parse_export_declaration(),
        )
    }
}
