use tokenizer::{Tokenizer, tokens};
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> InnerResult<()> {
        self.keyword("class")?;

        let id = if self.flags.allow_default {
            try!(self.binding_identifier())
        } else {
            Some(eat!(self.binding_identifier()))
        };

        let parent = self.parse_class_heritage()?;

        self.parse_class_body()?;

        Ok(())
    }
    pub fn parse_class_expression(&mut self) -> InnerResult<()> {
        self.keyword("class")?;

        Ok(())
    }

    fn parse_class_heritage(&mut self) -> InnerResult<Option<()>> {
        self.keyword("extends")?;

        eat!(self.parse_left_hand_side_expression());

        Ok(Some(()))
    }

    fn parse_class_body(&mut self) -> InnerResult<()> {
        eat!(self.punc(tokens::PunctuatorToken::CurlyOpen));
        eat!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(())
    }
}
