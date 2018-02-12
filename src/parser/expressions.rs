use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{OptResult, TokenResult};
use parser::utils;

use failure;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_left_hand_side_expression(&mut self) -> OptResult<()> {
        self.parse_left_hand_expression(true)
    }

    pub fn parse_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_assignment_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_value!(self.parse_assignment_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    pub fn parse_assignment_expression(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.parse_yield_expression()? {
            return Ok(TokenResult::Some(()));
        }

        // TODO: Alternatively this can kick off the cover lookahead here then skip reify

        let left = try_value!(self.parse_conditional_expression()?);

        use self::tokens::PunctuatorToken::*;

        let result = try_sequence!(
            self.punc(Arrow),
            self.punc(Eq),
            self.punc(StarEq),
            self.punc(SlashEq),
            self.punc(PercentEq),
            self.punc(PlusEq),
            self.punc(MinusEq),
            self.punc(LAngleAngleEq),
            self.punc(RAngleAngleEq),
            self.punc(RAngleAngleAngleEq),
            self.punc(AmpEq),
            self.punc(CaretEq),
            self.punc(BarEq),
            self.punc(StarStarEq),
        );

        if let TokenResult::Some(p) = result {
            match p {
                Arrow => {
                    // TODO: No LineTerminator allowed before arrow.
                    eat_value!(self.reify_arrow(left)?);
                }
                o@Eq | o@StarEq | o@SlashEq | o@PercentEq | o@PlusEq | o@MinusEq |
                o@LAngleAngleEq | o@RAngleAngleEq | o@RAngleAngleAngleEq |
                o@AmpEq | o@CaretEq | o@BarEq | o@StarStarEq => {
                    eat_value!(self.reify_assignment(left, o)?);
                }
                _ => unreachable!(),
            }
        } else {
            eat_value!(self.reify_expression(left)?);
        }

        Ok(TokenResult::Some(()))
    }
    fn reify_expression(&mut self, left: ()) -> OptResult<()> {
        Ok(TokenResult::Some(()))
    }
    fn reify_arrow(&mut self, left: ()) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_block_statement()?,
            self.parse_assignment_expression()?,
        ))
    }
    fn reify_assignment(&mut self, left: (), op: tokens::PunctuatorToken) -> OptResult<()> {
        self.expect_expression();

        eat_value!(self.parse_assignment_expression()?);

        Ok(TokenResult::Some(()))
    }

    fn parse_yield_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("yield"));

        if self.no_line_terminator() {
            opt_value!(self.punc(tokens::PunctuatorToken::Star));

            opt_value!(self.parse_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_conditional_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_logical_or_expression()?);

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Question) {
            eat_value!(self.with(Flag::In).parse_assignment_expression()?);
            eat_value!(self.punc(tokens::PunctuatorToken::Colon));
            eat_value!(self.parse_assignment_expression()?);
        }
        Ok(TokenResult::Some(()))
    }
    fn parse_logical_or_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_logical_and_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::BarBar) {
            eat_value!(self.parse_logical_and_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_logical_and_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_bitwise_or_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::AmpAmp) {
            eat_value!(self.parse_bitwise_or_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_bitwise_or_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_bitwise_xor_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Bar) {
            eat_value!(self.parse_bitwise_xor_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_bitwise_xor_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_bitwise_and_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Caret) {
            eat_value!(self.parse_bitwise_and_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_bitwise_and_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_equality_expression()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Amp) {
            eat_value!(self.parse_equality_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_equality_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_relational_expression()?);

        while let TokenResult::Some(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::EqEq),
            self.punc(tokens::PunctuatorToken::EqEqEq),
            self.punc(tokens::PunctuatorToken::ExclamEq),
            self.punc(tokens::PunctuatorToken::ExclamEqEq),
        ) {
            eat_value!(self.parse_relational_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_relational_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_shift_expression()?);

        while let TokenResult::Some(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::LAngle).map(tokens::Token::Punctuator),
            self.punc(tokens::PunctuatorToken::LAngleEq).map(tokens::Token::Punctuator),
            self.punc(tokens::PunctuatorToken::RAngle).map(tokens::Token::Punctuator),
            self.punc(tokens::PunctuatorToken::RAngleEq).map(tokens::Token::Punctuator),
            self.keyword("instanceof").map(tokens::Token::IdentifierName),
            if self.flags.allow_in { self.keyword("in").map(tokens::Token::IdentifierName) } else { TokenResult::None },
        ) {
            eat_value!(self.parse_shift_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_shift_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_additive_expression()?);

        while let TokenResult::Some(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::LAngleAngle),
            self.punc(tokens::PunctuatorToken::RAngleAngle),
            self.punc(tokens::PunctuatorToken::RAngleAngle),
        ) {
            eat_value!(self.parse_additive_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_additive_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_multiplicative_expression()?);

        while let TokenResult::Some(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::Plus),
            self.punc(tokens::PunctuatorToken::Minus),
        ) {
            eat_value!(self.parse_multiplicative_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_multiplicative_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_exponential_expression()?);

        while let TokenResult::Some(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::Star),
            self.punc(tokens::PunctuatorToken::Slash),
            self.punc(tokens::PunctuatorToken::Percent),
        ) {
            eat_value!(self.parse_exponential_expression()?);
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_exponential_expression(&mut self) -> OptResult<()> {
        try_value!(self.parse_unary_expression()?);

        // TODO:
        // if is_update_expression(expr) {
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::StarStar) {
                eat_value!(self.parse_exponential_expression()?);
            }
        // }

        Ok(TokenResult::Some(()))
    }
    fn parse_unary_expression(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_delete_expression()?,
            self.parse_void_expression()?,
            self.parse_typeof_expression()?,
            self.parse_plus_expression()?,
            self.parse_minus_expression()?,
            self.parse_tilde_expression()?,
            self.parse_exclam_expression()?,
            self.parse_await_expression()?,
            self.parse_update_expression()?,
        ))
    }
    fn parse_delete_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("delete"));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_void_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("void"));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_typeof_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("typeof"));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_plus_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Plus));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_minus_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Minus));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_tilde_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Tilde));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_exclam_expression(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Exclam));

        eat_value!(self.parse_unary_expression()?);

        Ok(TokenResult::Some(()))
    }
    fn parse_await_expression(&mut self) -> OptResult<()> {
        if !self.flags.allow_await {
            return Ok(TokenResult::None);
        }

        try_value!(self.keyword("await"));

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
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::SquareOpen) {
                eat_value!(self.with(Flag::In).parse_expression()?);
                eat_value!(self.punc(tokens::PunctuatorToken::SquareClose));
            } else if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Period) {
                eat_value!(self.with(Flag::In).identifier());
            } else {
                // TODO: skip if super`foo`
                if let TokenResult::Some(_) = self.parse_template_literal_expression()? {

                } else if allow_call {
                    if let TokenResult::Some(_) = self.parse_call_arguments()? {
                        // println!("got args");
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        Ok(TokenResult::Some(()))
    }


    fn parse_call_arguments(&mut self) -> OptResult<()> {
        let mut parser = self.with(Flag::In);

        try_value!(parser.punc(tokens::PunctuatorToken::ParenOpen));

        // println!("osdf");
        loop {
            // println!("inside");
            if let TokenResult::Some(_) = parser.punc(tokens::PunctuatorToken::Ellipsis) {
                println!("4");
                eat_value!(parser.parse_assignment_expression()?);
                break;
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
        Ok(try_sequence!(
            self.parse_this_expression()?,
            self.parse_identifier_reference_expression()?,
            self.parse_literal_expression()?,
            self.parse_array_literal_expression()?,
            self.parse_object_literal_expression()?,
            self.parse_function_expression()?,
            self.parse_class_expression()?,
            self.parse_regular_expression_literal_expression()?,
            self.parse_template_literal_expression()?,
            self.parse_cover_parenthesized_expression()?,
        ))
    }

    fn parse_this_expression(&mut self) -> OptResult<()> {
        try_value!(self.keyword("this"));

        Ok(TokenResult::Some(()))
    }
    fn parse_identifier_reference_expression(&mut self) -> OptResult<()> {
        try_value!(self.reference_identifier());

        Ok(TokenResult::Some(()))
    }
    fn parse_literal_expression(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_null_expression()?,
            self.parse_true_expression()?,
            self.parse_false_expression()?,
            self.parse_numeric_expression()?,
            self.parse_string_expression()?,
        ))
    }
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
        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }

    fn parse_object_property(&mut self) -> OptResult<()> {

        // TODO: Disallow static
        let head = try_value!(self.parse_method_head(true)?);

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Colon) {
            eat_value!(self.parse_assignment_expression()?);
        } else {
            eat_value!(self.parse_method_tail(head)?);
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
                eat_value!(parser.parse_expression()?);
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

        // TODO

        let expr = opt_value!(self.with(Flag::In).parse_expression()?);

        let expect_more = if let Some(_) = expr {
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
                true
            } else {
                false
            }
        } else {
            true
        };

        if expect_more {
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
