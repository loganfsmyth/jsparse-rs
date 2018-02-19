use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{OptResult, TokenResult};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_import_declaration(&mut self) -> OptResult<()>  {
        try_value!(self.keyword("import"));

        if let TokenResult::Some(_) = self.string() {

        } else {
            let try_names = if let TokenResult::Some(_) = self.binding_identifier() {
                opt_value!(self.punc(tokens::PunctuatorToken::Comma)).is_some()
            } else {
                true
            };

            if try_names {
                if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Star) {
                    eat_value!(self.keyword("as"));
                    eat_value!(self.binding_identifier());
                } else {
                    eat_value!(self.punc(tokens::PunctuatorToken::CurlyOpen));

                    while let TokenResult::Some(_) = self.parse_import_specifier()? {
                        if let TokenResult::None = self.punc(tokens::PunctuatorToken::Comma) {
                            break;
                        }
                    }

                    eat_value!(self.punc(tokens::PunctuatorToken::CurlyClose));
                }
            }

            eat_value!(self.keyword("from"));
            eat_value!(self.string());
        }

        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    fn parse_import_specifier(&mut self) -> OptResult<()> {
        try_value!(self.identifier());

        if let TokenResult::Some(_) = self.keyword("as") {
            // TODO: Validate BindingIdentifier
            eat_value!(self.identifier());
        } else {
            // TODO: Validate first is BindingIdentifier
        }

        Ok(TokenResult::Some(()))
    }

    pub fn parse_export_declaration(&mut self) -> OptResult<()>  {
        try_value!(self.keyword("export"));

        if let TokenResult::Some(_) = self.keyword("default") {
            let mut parser = self.with(Flag::Default);

            if let TokenResult::Some(_) = parser.parse_function_declaration()? {

            } else if let TokenResult::Some(_) = parser.parse_class_declaration()? {

            } else {
                eat_value!(parser.with(Flag::In).parse_expression()?);
                eat_value!(parser.semicolon());
            }
        } else {
            if let TokenResult::Some(_) = self.parse_variable_statement()? {

            } else if let TokenResult::Some(_) = self.parse_let_declaration()? {

            } else if let TokenResult::Some(_) = self.parse_const_declaration()? {

            } else if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Star) {
                eat_value!(self.keyword("from"));
                eat_value!(self.string());
            } else {
                eat_value!(self.punc(tokens::PunctuatorToken::CurlyOpen));

                while let TokenResult::Some(_) = self.parse_export_specifier()? {
                    if let TokenResult::None = self.punc(tokens::PunctuatorToken::Comma) {
                        break;
                    }
                }

                eat_value!(self.punc(tokens::PunctuatorToken::CurlyClose));

                if let TokenResult::Some(_) = self.keyword("from") {
                    eat_value!(self.string());
                } else {
                    // TODO: Validate BindingIdentifier for local specifiers
                }
            }

            eat_value!(self.semicolon());
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_export_specifier(&mut self) -> OptResult<()> {
        try_value!(self.identifier());

        if let TokenResult::Some(_) = self.keyword("as") {
            eat_value!(self.identifier());
        }

        Ok(TokenResult::Some(()))
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
