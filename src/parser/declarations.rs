use tokenizer::{Tokenizer, tokens};
use parser::Parser;
use parser::utils::OptResult;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_declaration(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_function_declaration(),
            self.parse_class_declaration(),
            self.parse_let_declaration(),
            self.parse_const_declaration(),
        )
    }

    pub fn parse_let_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("let"));

        eat_fn!(self.parse_lexical_declarator(false)?);

        while let Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_fn!(self.parse_lexical_declarator(false)?);
        }
        eat_token!(self.semicolon());

        Ok(Some(()))
    }

    pub fn parse_const_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("const"));

        eat_fn!(self.parse_lexical_declarator(true)?);

        while let Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_fn!(self.parse_lexical_declarator(true)?);
        }
        eat_token!(self.semicolon());

        Ok(Some(()))
    }

    pub fn parse_lexical_declarator(&mut self, initializer_required: bool) -> OptResult<()> {
        if let Some(_) = self.parse_binding_pattern()? {
            eat_fn!(self.parse_initializer()?);
        } else {
            eat_token!(self.binding_identifier());

            if initializer_required {
                eat_fn!(self.parse_initializer()?);
            } else {
                self.parse_initializer();
            }
        }
        Ok(Some(()))
    }
}
