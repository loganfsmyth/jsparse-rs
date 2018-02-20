use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag, LookaheadResult, is_binding_identifier};
use parser::utils::{OptResult, TokenResult};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_left_hand_side_expression(&mut self) -> OptResult<()> {
        self.parse_left_hand_expression(true)
    }

    pub fn parse_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_assignment_expression()?);

        self.expect_expression();
        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            self.expect_expression();
            // println!("starting comma");
            eat_value!(self.parse_assignment_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    pub fn parse_assignment_expression(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.parse_yield_expression()? {
            return Ok(TokenResult::Some(()));
        }

        let flags = self.flags;

        let maybe_async_arrow = if let Some(&LookaheadResult {
            line: false,
            token: tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name })
        }) = self.ident_lookahead() {
            if is_binding_identifier(&flags, name) {
                true
            } else {
                false
            }
        } else {
            false
        };

        if maybe_async_arrow {
            if let TokenResult::Some(_) = self.keyword("async") {
                if !self.no_line_terminator() {
                    unreachable!();
                }
                eat_value!(self.binding_identifier());
                if !self.no_line_terminator() {
                    bail!("Unexpected line terminator between async arrow argument and arrow");
                }

                eat_value!(self.punc(tokens::PunctuatorToken::Arrow));
                eat_value!(self.reify_arrow(())?);

                return Ok(TokenResult::Some(()));
            }
        }

        // TODO: Alternatively this can kick off the cover lookahead here then skip reify

        let left = try_value!(self.parse_conditional_expression()?);

        #[derive(Debug)]
        enum Reify {
            Arrow,
            Eq,
            StarEq,
            SlashEq,
            PercentEq,
            PlusEq,
            MinusEq,
            LAngleAngleEq,
            RAngleAngleEq,
            RAngleAngleAngleEq,
            AmpEq,
            CaretEq,
            BarEq,
            StarStarEq,
        }

        let t = match *self.token() {
            tokens::Token::Punctuator(tokens::PunctuatorToken::Arrow) => Reify::Arrow,
            tokens::Token::Punctuator(tokens::PunctuatorToken::Eq) => Reify::Eq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::StarEq) => Reify::StarEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::SlashEq) => Reify::SlashEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::PercentEq) => Reify::PercentEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::PlusEq) => Reify::PlusEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::MinusEq) => Reify::MinusEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::LAngleAngleEq) => Reify::LAngleAngleEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::RAngleAngleEq) => Reify::RAngleAngleEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::RAngleAngleAngleEq) => Reify::RAngleAngleAngleEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::AmpEq) => Reify::AmpEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::CaretEq) => Reify::CaretEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::BarEq) => Reify::BarEq,
            tokens::Token::Punctuator(tokens::PunctuatorToken::StarStarEq) => Reify::StarStarEq,
            _ => return Ok(TokenResult::Some(())),
        };

        self.pop();

        eat_value!(match t {
            Reify::Arrow => {
                // TODO: No LineTerminator allowed before arrow.
                // TODO: This needs to know if the left was an "async(foo)" to decide if "await" is allowed.
                self.reify_arrow(left)?
            }
            Reify::Eq => self.reify_assignment(left, tokens::PunctuatorToken::Eq)?,
            Reify::StarEq => self.reify_assignment(left, tokens::PunctuatorToken::StarEq)?,
            Reify::SlashEq => self.reify_assignment(left, tokens::PunctuatorToken::SlashEq)?,
            Reify::PercentEq => self.reify_assignment(left, tokens::PunctuatorToken::PercentEq)?,
            Reify::PlusEq => self.reify_assignment(left, tokens::PunctuatorToken::PlusEq)?,
            Reify::MinusEq => self.reify_assignment(left, tokens::PunctuatorToken::MinusEq)?,
            Reify::LAngleAngleEq => self.reify_assignment(left, tokens::PunctuatorToken::LAngleAngleEq)?,
            Reify::RAngleAngleEq => self.reify_assignment(left, tokens::PunctuatorToken::RAngleAngleEq)?,
            Reify::RAngleAngleAngleEq => self.reify_assignment(left, tokens::PunctuatorToken::RAngleAngleAngleEq)?,
            Reify::AmpEq => self.reify_assignment(left, tokens::PunctuatorToken::AmpEq)?,
            Reify::CaretEq => self.reify_assignment(left, tokens::PunctuatorToken::CaretEq)?,
            Reify::BarEq => self.reify_assignment(left, tokens::PunctuatorToken::BarEq)?,
            Reify::StarStarEq => self.reify_assignment(left, tokens::PunctuatorToken::StarStarEq)?,
        });

        Ok(TokenResult::Some(()))
    }
    fn reify_arrow(&mut self, _left: ()) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_block_statement()?,
            self.parse_assignment_expression()?,
        ))
    }
    fn reify_assignment(&mut self, _left: (), _op: tokens::PunctuatorToken) -> OptResult<()> {
        self.expect_expression();

        // println!("starting expr");
        eat_value!(self.parse_assignment_expression()?);

        // println!("ending expr");
        Ok(TokenResult::Some(()))
    }

    fn parse_yield_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("yield"));

        if self.no_line_terminator() {
            self.expect_expression();
            opt_value!(self.punc(tokens::PunctuatorToken::Star));

            self.expect_expression();
            opt_value!(self.parse_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_conditional_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_logical_or_expression()?);

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Question) {
            self.expect_expression();
            eat_value!(self.with(Flag::In).parse_assignment_expression()?);
            eat_value!(self.punc(tokens::PunctuatorToken::Colon));

            self.expect_expression();
            eat_value!(self.parse_assignment_expression()?);
        }
        Ok(TokenResult::Some(()))
    }

    fn parse_logical_or_expression(&mut self) -> OptResult<()> {
        self.parse_fancy(0)
    }
    fn parse_fancy(&mut self, mut precedence: u8) -> OptResult<()> {
        try_value!(self.parse_exponential_expression()?);

        let allow_in = self.flags.allow_in;

        loop {
            let new_precedence = match *self.token() {
                tokens::Token::Punctuator(tokens::PunctuatorToken::BarBar) => 1,
                tokens::Token::Punctuator(tokens::PunctuatorToken::AmpAmp) => 2,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Bar) => 3,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Caret) => 4,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Amp) => 5,
                tokens::Token::Punctuator(tokens::PunctuatorToken::EqEq) => 6,
                tokens::Token::Punctuator(tokens::PunctuatorToken::EqEqEq) => 6,
                tokens::Token::Punctuator(tokens::PunctuatorToken::ExclamEq) => 6,
                tokens::Token::Punctuator(tokens::PunctuatorToken::ExclamEqEq) => 6,
                tokens::Token::Punctuator(tokens::PunctuatorToken::LAngle) => 7,
                tokens::Token::Punctuator(tokens::PunctuatorToken::RAngle) => 7,
                tokens::Token::Punctuator(tokens::PunctuatorToken::LAngleEq) => 7,
                tokens::Token::Punctuator(tokens::PunctuatorToken::RAngleEq) => 7,
                tokens::Token::Punctuator(tokens::PunctuatorToken::LAngleAngle) => 8,
                tokens::Token::Punctuator(tokens::PunctuatorToken::RAngleAngle) => 8,
                tokens::Token::Punctuator(tokens::PunctuatorToken::RAngleAngleAngle) => 8,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Plus) => 9,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Minus) => 9,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Star) => 10,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Percent) => 10,
                tokens::Token::Punctuator(tokens::PunctuatorToken::Slash) => 10,
                tokens::Token::Punctuator(tokens::PunctuatorToken::StarStar) => 11,

                tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) => {
                    match &**name {
                        "in" if allow_in => 7,
                        "instanceof" => 7,
                        _ => break,
                    }
                }
                _ => break,
            };

            self.pop();

            self.expect_expression();


            // if new_precedence === 12 {
            //     self.parse_fancy()
            // }
            if new_precedence >= precedence && new_precedence != 11 {
                precedence = new_precedence;

                eat_value!(self.parse_unary_expression()?);
            } else {
                eat_value!(self.parse_fancy(new_precedence)?);
            }
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_exponential_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_unary_expression()?);

        // TODO:
        // if is_update_expression(expr) {
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::StarStar) {
                self.expect_expression();
                eat_value!(self.parse_exponential_expression()?);
            }
        // }

        Ok(TokenResult::Some(()))
    }
    fn parse_unary_expression(&mut self) -> OptResult<()> {
        enum UnaryType {
            Delete,
            Void,
            Typeof,
            Plus,
            Minus,
            Tilde,
            Exclam,
            Await,
            Unknown,
        }

        let t = match *self.token() {
            tokens::Token::Punctuator(tokens::PunctuatorToken::Plus) => UnaryType::Plus,
            tokens::Token::Punctuator(tokens::PunctuatorToken::Minus) => UnaryType::Minus,
            tokens::Token::Punctuator(tokens::PunctuatorToken::Tilde) => UnaryType::Tilde,
            tokens::Token::Punctuator(tokens::PunctuatorToken::Exclam) => UnaryType::Exclam,
            tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) => {
                match &**name {
                    "delete" => UnaryType::Delete,
                    "void" => UnaryType::Void,
                    "typeof" => UnaryType::Typeof,
                    "await" => UnaryType::Await,
                    _ => UnaryType::Unknown,
                }
            }
            _ => UnaryType::Unknown,
        };

        match t {
            UnaryType::Delete => eat_value!(self.parse_delete_expression()?),
            UnaryType::Void => eat_value!(self.parse_void_expression()?),
            UnaryType::Typeof => eat_value!(self.parse_typeof_expression()?),
            UnaryType::Plus => eat_value!(self.parse_plus_expression()?),
            UnaryType::Minus => eat_value!(self.parse_minus_expression()?),
            UnaryType::Tilde => eat_value!(self.parse_tilde_expression()?),
            UnaryType::Exclam => eat_value!(self.parse_exclam_expression()?),
            UnaryType::Await => eat_value!(self.parse_await_expression()?),
            UnaryType::Unknown => try_value!(self.parse_update_expression()?),
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_delete_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("delete"));

        self.expect_expression();

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_void_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("void"));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_typeof_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("typeof"));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_plus_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Plus));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_minus_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Minus));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_tilde_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Tilde));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_exclam_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Exclam));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_await_expression(&mut self) -> OptResult<()> {
        if !self.flags.allow_await {
            return Ok(TokenResult::None);
        }

        try_value!(self.keyword("await"));

        self.expect_expression();
        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_update_expression(&mut self) -> OptResult<()> {
        let op = if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::PlusPlus) {
            true
        } else if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::MinusMinus) {
            true
        } else {
            false
        };

        if op {
            self.expect_expression();
            eat_value!(self.parse_update_expression()?);
        } else {
            try_value!(self.parse_left_hand_expression(true)?);

            if self.no_line_terminator() {
                if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::PlusPlus) {

                } else if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::MinusMinus) {

                }
            }
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_left_hand_expression(&mut self, allow_call: bool) -> OptResult<()> {
        if let TokenResult::Some(_) = self.keyword("new") {
            eat_value!(self.parse_left_hand_expression(false)?);
        } else {
            if let TokenResult::None = try_sequence!(
                self.parse_primary_expression()?,
                self.parse_meta_property_expression()?,
                self.parse_super_expression()?,
            ) {
                return Ok(TokenResult::None);
            }
        }

        // println!("starting member");

        loop {
            enum LeftType {
                Ident,
                Call,
                Computed,
                Template,
            }

            let t = match *self.token() {
                tokens::Token::Punctuator(tokens::PunctuatorToken::Period) => LeftType::Ident,
                tokens::Token::Punctuator(tokens::PunctuatorToken::SquareOpen) => LeftType::Computed,
                tokens::Token::Punctuator(tokens::PunctuatorToken::ParenOpen) => LeftType::Call,
                tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::NoSubstitution, .. }) |
                tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::Head, .. }) => LeftType::Template,
                _ => break,
            };

            match t {
                LeftType::Ident => {
                    self.pop();
                    // eat_value!(self.punc(tokens::PunctuatorToken::Period));
                    eat_value!(self.with(Flag::In).identifier());
                }
                LeftType::Call if allow_call => eat_value!(self.parse_call_arguments()?),
                LeftType::Computed => {
                    self.pop();
                    // eat_value!(self.punc(tokens::PunctuatorToken::SquareOpen));
                    eat_value!(self.with(Flag::In).parse_expression()?);
                    eat_value!(self.punc(tokens::PunctuatorToken::SquareClose));
                },

                // TODO: Not allowed for 'super'.
                LeftType::Template => eat_value!(self.parse_template_literal_expression()?),
                _ => break,
            }
        }

        Ok(TokenResult::Some(()))
    }


    fn parse_call_arguments(&mut self) -> OptResult<()> {
        let mut parser = self.with(Flag::In);

        try_value!(parser.punc(tokens::PunctuatorToken::ParenOpen));

        // println!("osdf");
        loop {
            parser.expect_expression();

            // println!("inside");
            if let TokenResult::Some(_) = parser.punc(tokens::PunctuatorToken::Ellipsis) {
                // println!("4");
                parser.expect_expression();
                eat_value!(parser.parse_assignment_expression()?);

                if let TokenResult::Some(_) = parser.punc(tokens::PunctuatorToken::Comma) {
                } else {
                    break;
                }
            }

            // println!("1");

            if let TokenResult::Some(_) = parser.parse_assignment_expression()? {
            // println!("2");
                if let TokenResult::Some(_) = parser.punc(tokens::PunctuatorToken::Comma) {
            // println!("3");
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // println!("done");

        eat_value!(parser.punc(tokens::PunctuatorToken::ParenClose));

        Ok(TokenResult::Some(()))
    }

    fn parse_meta_property_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("new"));
        eat_value!(self.punc(tokens::PunctuatorToken::Period));
        eat_value!(self.keyword("target"));

        Ok(TokenResult::Some(()))
    }
    fn parse_super_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("super"));

        Ok(TokenResult::Some(()))
    }

    fn parse_primary_expression(&mut self) -> OptResult<()> {
        enum PrimaryType {
            This,
            Ident,
            Number,
            String,
            True,
            False,
            Null,
            Array,
            Object,
            Regex,
            Template,
            Paren,

            Function,
            Class,
        }

        let flags = self.flags;

        let t = match *self.token() {
            tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) => {
                match &**name {
                    "this" => PrimaryType::This,
                    "true" => PrimaryType::True,
                    "false" => PrimaryType::False,
                    "null" => PrimaryType::Null,

                    // TODO: Ignores async functions
                    "function" => PrimaryType::Function,
                    "class" => PrimaryType::Class,
                    _ => {
                        if is_binding_identifier(&flags, name) {
                            PrimaryType::Ident
                        } else {
                            return Ok(TokenResult::None)
                        }
                    }
                }
            }
            tokens::Token::StringLiteral(_) => PrimaryType::String,
            tokens::Token::NumericLiteral(_) => PrimaryType::Number,
            tokens::Token::Template(_) => PrimaryType::Template,
            tokens::Token::RegularExpressionLiteral(_) => PrimaryType::Regex,
            tokens::Token::Punctuator(tokens::PunctuatorToken::SquareOpen) => PrimaryType::Array,
            tokens::Token::Punctuator(tokens::PunctuatorToken::CurlyOpen) => PrimaryType::Object,
            tokens::Token::Punctuator(tokens::PunctuatorToken::ParenOpen) => PrimaryType::Paren,
            _ => return Ok(TokenResult::None),
        };

        match t {
            PrimaryType::This => eat_value!(self.parse_this_expression()?),
            PrimaryType::Ident => eat_value!(self.parse_identifier_reference_expression()?),
            PrimaryType::Number => eat_value!(self.parse_numeric_expression()?),
            PrimaryType::String => eat_value!(self.parse_string_expression()?),
            PrimaryType::True => eat_value!(self.parse_true_expression()?),
            PrimaryType::False => eat_value!(self.parse_false_expression()?),
            PrimaryType::Null => eat_value!(self.parse_null_expression()?),
            PrimaryType::Array => eat_value!(self.parse_array_literal_expression()?),
            PrimaryType::Object => eat_value!(self.parse_object_literal_expression()?),
            PrimaryType::Regex => eat_value!(self.parse_regular_expression_literal_expression()?),
            PrimaryType::Template => eat_value!(self.parse_template_literal_expression()?),
            PrimaryType::Paren => eat_value!(self.parse_cover_parenthesized_expression()?),
            PrimaryType::Function => eat_value!(self.parse_function_expression()?),
            PrimaryType::Class => eat_value!(self.parse_class_expression()?),
        }

        Ok(TokenResult::Some(()))



        // Ok(try_sequence!(
        //     self.parse_this_expression()?,
        //     self.parse_identifier_reference_expression()?,
        //     self.parse_literal_expression()?,
        //     self.parse_array_literal_expression()?,
        //     self.parse_object_literal_expression()?,
        //     self.parse_function_expression()?,
        //     self.parse_class_expression()?,
        //     self.parse_regular_expression_literal_expression()?,
        //     self.parse_template_literal_expression()?,
        //     self.parse_cover_parenthesized_expression()?,
        // ))
    }

    fn parse_this_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("this"));

        Ok(TokenResult::Some(()))
    }
    fn parse_identifier_reference_expression(&mut self) -> OptResult<()> {
        if let Some(ahead) = self.ident_lookahead() {
            match ahead.token {
                tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name}) => {
                    if name == "function" {
                        return Ok(TokenResult::None);
                    }
                }
                _ => {}
            }

        }

        try_value!(self.reference_identifier());

        Ok(TokenResult::Some(()))
    }
    // fn parse_literal_expression(&mut self) -> OptResult<()> {
    //     Ok(try_sequence!(
    //         self.parse_null_expression()?,
    //         self.parse_true_expression()?,
    //         self.parse_false_expression()?,
    //         self.parse_numeric_expression()?,
    //         self.parse_string_expression()?,
    //     ))
    // }
    fn parse_null_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("null"));

        Ok(TokenResult::Some(()))
    }
    fn parse_true_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("true"));

        Ok(TokenResult::Some(()))
    }
    fn parse_false_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("false"));

        Ok(TokenResult::Some(()))
    }
    fn parse_numeric_expression(&mut self) -> OptResult<()> {
        try_value!(self.numeric());

        Ok(TokenResult::Some(()))
    }
    fn parse_string_expression(&mut self) -> OptResult<()> {
        try_value!(self.string());

        Ok(TokenResult::Some(()))
    }
    fn parse_array_literal_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::SquareOpen));

        loop {
            if let TokenResult::Some(_) = self.parse_array_item()? {
            }

            if let TokenResult::None = self.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
        }

        eat_value!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(TokenResult::Some(()))
    }
    fn parse_object_literal_expression(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);
        try_value!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        loop {
            if let TokenResult::Some(_) = parser.parse_object_property()? {
            }

            if let TokenResult::None = parser.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
            // println!("again");
        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }

    fn parse_object_property(&mut self) -> OptResult<()> {

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Ellipsis) {
            self.expect_expression();
            eat_value!(self.parse_assignment_expression()?);
            return Ok(TokenResult::Some(()));
        }

        // TODO: Disallow static
        let head = try_value!(self.parse_method_head(true)?);

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Colon) {
            self.expect_expression();
            eat_value!(self.parse_assignment_expression()?);
        } else {
            // TODO: This needs to handle singlename prop access
            // eat_value!(self.parse_method_tail(head)?);
            opt_value!(self.parse_method_tail(head)?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_array_item(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Ellipsis) {
            eat_value!(self.parse_assignment_expression()?);
        } else if let TokenResult::Some(_) = self.parse_assignment_expression()? {
        }
        Ok(TokenResult::Some(()))
    }

    fn parse_regular_expression_literal_expression(&mut self) -> OptResult<()> {
        try_value!(self.regex());

        Ok(TokenResult::Some(()))
    }
    fn parse_template_literal_expression(&mut self) -> OptResult<()> {
        let tok = try_value!(self.template());

        if tok.format == tokens::TemplateFormat::Head {
            let mut parser = self.with(Flag::Template);
            loop {
                // println!("In template1");
                eat_value!(parser.parse_expression()?);

                // println!("In template2");

                let next = eat_value!(parser.template_tail());

                if next.format == tokens::TemplateFormat::Tail {
                    break;
                }
            }
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_cover_parenthesized_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::ParenOpen));

        self.expect_expression();

        if let Some(_) = opt_value!(self.parse_assignment_expression()?) {
            while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
                self.expect_expression();
                if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Ellipsis) {
                    if let TokenResult::Some(_) = self.parse_binding_pattern()? {
                    } else {
                        eat_value!(self.binding_identifier());
                    }
                } else {
                    eat_value!(self.parse_assignment_expression()?)
                }
            }
        } else {
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Ellipsis) {
                if let TokenResult::Some(_) = self.parse_binding_pattern()? {
                } else {
                    eat_value!(self.binding_identifier());
                }
            }
        }

        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

        Ok(TokenResult::Some(()))
    }
}
