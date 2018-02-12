use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{OptResult};
use parser::utils;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_class_declaration(&mut self) -> OptResult<()> {
        try_token!(self.keyword("class"));

        let id = if self.flags.allow_default {
            self.binding_identifier()
        } else {
            Ok(eat_token!(self.binding_identifier()))
        };

        let parent = self.parse_class_heritage()?;

        self.parse_class_body()?;

        Ok(Ok(()))
    }
    pub fn parse_class_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("class"));

        Ok(Ok(()))
    }

    fn parse_class_heritage(&mut self) -> OptResult<()> {
        try_token!(self.keyword("extends"));

        self.expect_expression();
        eat_fn!(self.parse_left_hand_side_expression()?);

        Ok(Ok(()))
    }

    fn parse_class_body(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);
        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        while let Ok(_) = parser.parse_class_item()? {
        }

        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Ok(()))
    }

    fn parse_class_item(&mut self) -> OptResult<()> {
        if let Ok(_) = self.punc(tokens::PunctuatorToken::Semicolon) {
            return Ok(Ok(()));
        }

        let head = try_fn!(self.parse_method_head(true)?);

        eat_fn!(self.parse_method_tail(head)?);

        Ok(Ok(()))
    }

    pub fn parse_method_head(&mut self, allow_static: bool) -> OptResult<MethodHead> {
        let mut stat = if allow_static {
            self.keyword("static")
        } else {
            Err(utils::NotFound)
        };

        let mut kind = if let Ok(_) = self.keyword("get") {
            MethodKind::Get
        } else if let Ok(_) = self.keyword("set") {
            MethodKind::Set
        } else {
            MethodKind::None
        };

        let mut async = if let MethodKind::None = kind {
            if let Ok(_) = self.keyword("async") {
                true
            } else {
                false
            }
        } else {
            false
        };

        let star = if async {
            Err(utils::NotFound)
        } else {
            self.punc(tokens::PunctuatorToken::Star)
        };

        let mut is_ident = false;
        let name = if async && self.no_line_terminator() {
            if let Ok(_) = self.parse_property_name()? {
                // async fn method
            } else {
                // method named  "async"
            }
        } else {
            if async {
                // has lineterminator
                // method named  "async"
            } else if let Ok(_) = self.parse_property_name()? {

            } else {
                match (stat, &kind, &star) {
                    (_, _, &Ok(_)) => bail!("expected method name"),
                    (_, &MethodKind::Get, _) => {
                        kind = MethodKind::None;
                    }
                    (_, &MethodKind::Set, _) => {
                        kind = MethodKind::None;
                    },
                    (Ok(_), _, _) => {
                        stat = Err(utils::NotFound);
                    }
                    _ => return Ok(Err(utils::NotFound)),
                }
            }
        };

        Ok(Ok(MethodHead {
            kind,
            is_ident,
            generator: star.is_ok(),
            async,
        }))
    }

    pub fn parse_method_tail(&mut self, head: MethodHead) -> OptResult<()> {
        println!("{:?}", head);

        let parser = match head.kind {
            MethodKind::Get => {
                try_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
                eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_fn!(self.parse_function_body()?);
            }
            MethodKind::Set => {
                try_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

                if let Err(utils::NotFound) = self.parse_binding_pattern()? {
                    eat_token!(self.binding_identifier());
                }

                eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_fn!(self.parse_function_body()?);
            }
            MethodKind::None => {
                if head.async {
                    let mut parser = self.without(Flag::Yield);

                    try_fn!(parser.parse_function_params()?);
                    eat_fn!(parser.with(Flag::Await).parse_function_body()?);
                } else if head.generator {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.with(Flag::Yield);

                    try_fn!(parser.parse_function_params()?);
                    eat_fn!(parser.parse_function_body()?);
                } else {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.without(Flag::Yield);

                    try_fn!(parser.parse_function_params()?);
                    eat_fn!(parser.parse_function_body()?);
                }
            }
        };

        Ok(Ok(()))
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
