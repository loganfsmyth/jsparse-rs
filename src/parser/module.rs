use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::OptResult;
use parser::utils;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_import_declaration(&mut self) -> OptResult<()>  {
        try_token!(self.keyword("import"));

        if let Ok(s) = self.string() {

        } else {
            let try_names = if let Ok(def) = self.binding_identifier() {
                self.punc(tokens::PunctuatorToken::Comma).is_ok()
            } else {
                true
            };

            if try_names {
                if let Ok(_) = self.punc(tokens::PunctuatorToken::Star) {
                    eat_token!(self.keyword("as"));
                    eat_token!(self.binding_identifier());
                } else {
                    eat_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

                    while let Ok(_) = self.parse_import_specifier()? {
                        if let Err(utils::NotFound) = self.punc(tokens::PunctuatorToken::Comma) {
                            break;
                        }
                    }

                    eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));
                }
            }

            eat_token!(self.keyword("from"));
            eat_token!(self.string());
        }

        eat_token!(self.semicolon());

        Ok(Ok(()))
    }

    fn parse_import_specifier(&mut self) -> OptResult<()> {
        try_token!(self.identifier());

        if let Ok(_) = self.keyword("as") {
            // TODO: Validate BindingIdentifier
            eat_token!(self.identifier());
        } else {
            // TODO: Validate first is BindingIdentifier
        }

        Ok(Ok(()))
    }

    pub fn parse_export_declaration(&mut self) -> OptResult<()>  {
        try_token!(self.keyword("export"));

        if let Ok(_) = self.keyword("default") {
            let mut parser = self.with(Flag::Default);

            if let Ok(_) = parser.parse_function_declaration()? {

            } else if let Ok(_) = parser.parse_class_declaration()? {

            } else {
                eat_fn!(parser.with(Flag::In).parse_expression()?);
                eat_token!(parser.semicolon());
            }
        } else {
            if let Ok(_) = self.parse_variable_statement()? {

            } else if let Ok(_) = self.parse_let_declaration()? {

            } else if let Ok(_) = self.parse_const_declaration()? {

            } else if let Ok(_) = self.punc(tokens::PunctuatorToken::Star) {
                eat_token!(self.keyword("from"));
                eat_token!(self.string());
            } else {
                eat_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

                while let Ok(_) = self.parse_export_specifier()? {
                    if let Err(utils::NotFound) = self.punc(tokens::PunctuatorToken::Comma) {
                        break;
                    }
                }

                eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

                if let Ok(_) = self.keyword("from") {
                    eat_token!(self.string());
                } else {
                    // TODO: Validate BindingIdentifier for local specifiers
                }
            }

            eat_token!(self.semicolon());
        }

        Ok(Ok(()))
    }

    fn parse_export_specifier(&mut self) -> OptResult<()> {
        try_token!(self.identifier());

        if let Ok(_) = self.keyword("as") {
            eat_token!(self.identifier());
        }

        Ok(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;
    use tokenizer::IntoTokenizer;

    #[test]
    fn it_parses_imports() {
        let mut p = Parser {
            tok: "
                import \"foo\";
                import foo from \"foo\";
                import * as ns from \"foo\";
                import { named, named as other } from \"foo\";
                import { named, named as other, } from \"foo\";
                import foo, * as ns from \"foo\";
                import foo, { named, named as other } from \"foo\";
                import foo, { named, named as other, } from \"foo\";
            ".into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],
            lookahead: None,
            token: None,
        };

        p.parse_module().unwrap();
    }

    #[test]
    fn it_parses_exports() {
        let mut p = Parser {
            tok: "
                export * from \"foo\";
                export { foo, foo as other } from \"foo\";
                export { foo, foo as other, } from \"foo\";
                export { foo, foo as other };
                export { foo, foo as other, };
                export default this;
            ".into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],
            lookahead: None,
            token: None,
        };

        p.parse_module().unwrap();
    }
}
