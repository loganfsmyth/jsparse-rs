use tokenizer::{Tokenizer, tokens};
use parser::Parser;
use parser::utils::{OptResult};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("class"));

        let id = if self.flags.allow_default {
            self.binding_identifier()
        } else {
            Some(eat_token!(self.binding_identifier()))
        };

        let parent = self.parse_class_heritage()?;

        self.parse_class_body()?;

        Ok(Some(()))
    }
    pub fn parse_class_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("class"));

        Ok(Some(()))
    }

    fn parse_class_heritage(&mut self) -> OptResult<()> {
        try_token!(self.keyword("extends"));

        eat_fn!(self.parse_left_hand_side_expression());

        Ok(Some(()))
    }

    fn parse_class_body(&mut self) -> OptResult<()> {
        eat_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));
        eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Some(()))
    }
}
