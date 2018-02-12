use tokenizer::{Tokenizer, tokens};
use parser::Parser;
use parser::utils::{OptResult, TokenResult};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_declaration(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_function_declaration()?,
            self.parse_class_declaration()?,
            self.parse_let_declaration()?,
            self.parse_const_declaration()?,
        ))
    }

    pub fn parse_let_declaration(&mut self) -> OptResult<()> {
        try_value!(self.keyword("let"));

        eat_value!(self.parse_lexical_declarator(false)?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_value!(self.parse_lexical_declarator(false)?);
        }
        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    pub fn parse_const_declaration(&mut self) -> OptResult<()> {
        try_value!(self.keyword("const"));

        eat_value!(self.parse_lexical_declarator(true)?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_value!(self.parse_lexical_declarator(true)?);
        }
        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    pub fn parse_lexical_declarator(&mut self, initializer_required: bool) -> OptResult<()> {
        if let TokenResult::Some(_) = self.parse_binding_pattern()? {
            eat_value!(self.parse_initializer()?);
        } else {
            eat_value!(self.binding_identifier());

            if initializer_required {
                eat_value!(self.parse_initializer()?);
            } else {
                opt_value!(self.parse_initializer()?);
            }
        }
        Ok(TokenResult::Some(()))
    }
}
