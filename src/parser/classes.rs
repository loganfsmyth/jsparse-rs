use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{OptResult, TokenResult};
use parser::utils;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> OptResult<()> {
        try_value!(self.keyword("class"));

        let id = if self.flags.allow_default {
            opt_value!(self.binding_identifier())
        } else {
            Some(eat_value!(self.binding_identifier()))
        };

        let parent = opt_value!(self.parse_class_heritage()?);

        eat_value!(self.parse_class_body()?);

        Ok(TokenResult::Some(()))
    }
    pub fn parse_class_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("class"));

        Ok(TokenResult::Some(()))
    }

    fn parse_class_heritage(&mut self) -> OptResult<()> {
        try_value!(self.keyword("extends"));

        self.expect_expression();
        eat_value!(self.parse_left_hand_side_expression()?);

        Ok(TokenResult::Some(()))
    }

    fn parse_class_body(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);
        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        while let TokenResult::Some(_) = parser.parse_class_item()? {
        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }

    fn parse_class_item(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Semicolon) {
            return Ok(TokenResult::Some(()));
        }

        let head = try_value!(self.parse_method_head(true)?);

        eat_value!(self.parse_method_tail(head)?);

        Ok(TokenResult::Some(()))
    }

    pub fn parse_method_head(&mut self, allow_static: bool) -> OptResult<MethodHead> {
        let mut stat = if allow_static {
            opt_value!(self.keyword("static"))
        } else {
            None
        };

        let mut kind = if let TokenResult::Some(_) = self.keyword("get") {
            MethodKind::Get
        } else if let TokenResult::Some(_) = self.keyword("set") {
            MethodKind::Set
        } else {
            MethodKind::None
        };

        let async = if let MethodKind::None = kind {
            if let TokenResult::Some(_) = self.keyword("async") {
                true
            } else {
                false
            }
        } else {
            false
        };

        let star = if async {
            None
        } else {
            opt_value!(self.punc(tokens::PunctuatorToken::Star))
        };

        let is_ident = false;
        let name = if async && self.no_line_terminator() {
            if let TokenResult::Some(_) = self.parse_property_name()? {
                // async fn method
            } else {
                // method named  "async"
            }
        } else {
            if async {
                // has lineterminator
                // method named  "async"
            } else if let TokenResult::Some(_) = self.parse_property_name()? {

            } else {
                match (stat, &kind, &star) {
                    (_, _, &Some(_)) => bail!("expected method name"),
                    (_, &MethodKind::Get, _) => {
                        kind = MethodKind::None;
                    }
                    (_, &MethodKind::Set, _) => {
                        kind = MethodKind::None;
                    },
                    (Some(_), _, _) => {
                        stat = None;
                    }
                    _ => return Ok(TokenResult::None),
                }
            }
        };

        Ok(TokenResult::Some(MethodHead {
            kind,
            is_ident,
            generator: star.is_some(),
            async,
        }))
    }

    pub fn parse_method_tail(&mut self, head: MethodHead) -> OptResult<()> {
        println!("{:?}", head);

        match head.kind {
            MethodKind::Get => {
                try_value!(self.punc(tokens::PunctuatorToken::ParenOpen));
                eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_value!(self.parse_function_body()?);
            }
            MethodKind::Set => {
                try_value!(self.punc(tokens::PunctuatorToken::ParenOpen));

                if let TokenResult::None = self.parse_binding_pattern()? {
                    eat_value!(self.binding_identifier());
                }

                eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_value!(self.parse_function_body()?);
            }
            MethodKind::None => {
                if head.async {
                    let mut parser = self.without(Flag::Yield);

                    try_value!(parser.parse_function_params()?);
                    eat_value!(parser.with(Flag::Await).parse_function_body()?);
                } else if head.generator {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.with(Flag::Yield);

                    try_value!(parser.parse_function_params()?);
                    eat_value!(parser.parse_function_body()?);
                } else {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.without(Flag::Yield);

                    try_value!(parser.parse_function_params()?);
                    eat_value!(parser.parse_function_body()?);
                }
            }
        };

        Ok(TokenResult::Some(()))
    }
}

#[derive(Debug)]
pub enum MethodKind {
    Get,
    Set,
    None,
}

#[derive(Debug)]
pub struct MethodHead {
    kind: MethodKind,
    is_ident: bool,
    generator: bool,
    async: bool,
}
