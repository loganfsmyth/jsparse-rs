use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag, LookaheadResult};
use parser::utils::{OptResult};
use parser::utils;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_function_declaration(&mut self) -> OptResult<()> {
        let maybe_async = if let Some(&LookaheadResult {
            line: false,
            token: tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name })
        }) = self.ident_lookahead() {
            name == "function"
        } else {
            false
        };

        let star = if maybe_async {
            try_token!(self.keyword("async"));
            eat_token!(self.keyword("function"));

            Err(utils::NotFound)
        } else {
            try_token!(self.keyword("function"));

            self.punc(tokens::PunctuatorToken::Star)
        };

        if self.flags.allow_default {
            self.binding_identifier();
        } else {
            eat_token!(self.binding_identifier());
        }

        if maybe_async {
            let mut parser = self.without(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.with(Flag::Await).parse_function_body()?);
        } else if let Ok(_) = star {
            let mut parser = self.without(Flag::Await);
            let mut parser = parser.with(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.parse_function_body()?);
        } else {
            let mut parser = self.without(Flag::Await);
            let mut parser = parser.without(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.parse_function_body()?);
        }

        Ok(Ok(()))
    }

    pub fn parse_function_expression(&mut self) -> OptResult<()> {
        let maybe_async = if let Some(&LookaheadResult {
            line: false,
            token: tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name })
        }) = self.ident_lookahead() {
            name == "function"
        } else {
            false
        };

        let star = if maybe_async {
            try_token!(self.keyword("async"));
            eat_token!(self.keyword("function"));

            Err(utils::NotFound)
        } else {
            try_token!(self.keyword("function"));

            self.punc(tokens::PunctuatorToken::Star)
        };

        self.binding_identifier();

        if maybe_async {
            let mut parser = self.without(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.with(Flag::Await).parse_function_body()?);
        } else if let Ok(_) = star {
            let mut parser = self.without(Flag::Await);
            let mut parser = parser.with(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.parse_function_body()?);
        } else {
            let mut parser = self.without(Flag::Await);
            let mut parser = parser.without(Flag::Yield);

            eat_fn!(parser.parse_function_params()?);
            eat_fn!(parser.parse_function_body()?);
        }

        Ok(Ok(()))
    }

    pub fn parse_function_params(&mut self) -> OptResult<()> {
        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

        loop {
            if let Ok(_) = self.parse_binding_rest_element()? {
                break;
            }

            if let Ok(_) = self.parse_binding_pattern()? {
                self.parse_initializer();
            } else if let Ok(_) = self.binding_identifier() {
                self.parse_initializer();
            } else {
                break;
            }

            if let Err(utils::NotFound) = self.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        Ok(Ok(()))
    }
    pub fn parse_function_body(&mut self) -> OptResult<()> {
        self.parse_block_statement()
    }
}
