use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
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

        self.expect_expression();
        eat_fn!(self.parse_left_hand_side_expression());

        Ok(Some(()))
    }

    fn parse_class_body(&mut self) -> OptResult<()> {
        eat_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

        while let Some(_) = self.parse_class_item()? {
        }

        eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Some(()))
    }

    fn parse_class_item(&mut self) -> OptResult<()> {
        if let Some(_) = self.punc(tokens::PunctuatorToken::Semicolon) {
            return Ok(Some(()));
        }

        let head = try_fn!(self.parse_method_head(true));

        eat_fn!(self.parse_method_tail(head));

        Ok(Some(()))
    }

    fn parse_method_head(&mut self, allow_static: bool) -> OptResult<MethodHead> {
        let mut stat = if allow_static {
            self.keyword("static")
        } else {
            None
        };

        let mut kind = if let Some(_) = self.keyword("get") {
            MethodKind::Get
        } else if let Some(_) = self.keyword("set") {
            MethodKind::Set
        } else {
            MethodKind::None
        };

        let mut async = if let MethodKind::None = kind {
            if let Some(_) = self.keyword("async") {
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
            self.punc(tokens::PunctuatorToken::Star)
        };

        let name = if async && self.no_line_terminator() {
            if let Some(_) = self.parse_property_name()? {
                // async fn method
            } else {
                // method named  "async"
            }
        } else {
            if async {
                // has lineterminator
                // method named  "async"
            } else if let Some(_) = self.parse_property_name()? {

            } else {
                match (stat, &kind, star) {
                    (_, _, Some(_)) => bail!("expected method name"),
                    (_, &MethodKind::Get, _) => {
                        kind = MethodKind::None;
                    }
                    (_, &MethodKind::Set, _) => {
                        kind = MethodKind::None;
                    },
                    (Some(_), _, _) => {
                        stat = None;
                    }
                    _ => return Ok(None),
                }
            }
        };

        Ok(Some(MethodHead {
            kind,
            generator: star.is_some(),
            async,
        }))
    }
    fn parse_method_tail(&mut self, head: MethodHead) -> OptResult<()> {
        println!("{:?}", head);

        let parser = match head.kind {
            MethodKind::Get => {
                try_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
                eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_fn!(self.parse_function_body());
            }
            MethodKind::Set => {
                try_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

                if let None = self.parse_binding_pattern()? {
                    eat_token!(self.binding_identifier());
                }

                eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

                eat_fn!(self.parse_function_body());
            }
            MethodKind::None => {
                if head.async {
                    let mut parser = self.without(Flag::Yield);

                    try_fn!(parser.parse_function_params());
                    eat_fn!(parser.with(Flag::Await).parse_function_body());
                } else if head.generator {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.with(Flag::Yield);

                    try_fn!(parser.parse_function_params());
                    eat_fn!(parser.parse_function_body());
                } else {
                    let mut parser = self.without(Flag::Await);
                    let mut parser = parser.without(Flag::Yield);

                    try_fn!(parser.parse_function_params());
                    eat_fn!(parser.parse_function_body());
                }
            }
        };

        Ok(Some(()))
    }
}

#[derive(Debug)]
enum MethodKind {
    Get,
    Set,
    None,
}

#[derive(Debug)]
struct MethodHead {
    kind: MethodKind,
    generator: bool,
    async: bool,
}
