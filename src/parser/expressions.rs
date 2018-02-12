use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag};
use parser::utils::{OptResult};

use failure;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_left_hand_side_expression(&mut self) -> OptResult<()> {
        self.parse_left_hand_expression(true)
    }

    pub fn parse_expression(&mut self) -> OptResult<()> {
        // println!("woot: {:?}", failure::err_msg("HERE"));

        try_fn!(self.parse_assignment_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_fn!(self.parse_assignment_expression());
        }

        Ok(Some(()))
    }
    pub fn parse_assignment_expression(&mut self) -> OptResult<()> {
        match self.parse_yield_expression()? {
            Some(expr) => return Ok(Some(expr)),
            _ => {}
        }

        // Alternatively this can kick off the cover lookahead here then skip reify

        let left = try_fn!(self.parse_conditional_expression());

        use self::tokens::PunctuatorToken::*;

        let result: OptResult<_> = try_sequence!(
            Ok(self.punc(Arrow)) as OptResult<_>,
            Ok(self.punc(Eq)) as OptResult<_>,
            Ok(self.punc(StarEq)) as OptResult<_>,
            Ok(self.punc(SlashEq)) as OptResult<_>,
            Ok(self.punc(PercentEq)) as OptResult<_>,
            Ok(self.punc(PlusEq)) as OptResult<_>,
            Ok(self.punc(MinusEq)) as OptResult<_>,
            Ok(self.punc(LAngleAngleEq)) as OptResult<_>,
            Ok(self.punc(RAngleAngleEq)) as OptResult<_>,
            Ok(self.punc(RAngleAngleAngleEq)) as OptResult<_>,
            Ok(self.punc(AmpEq)) as OptResult<_>,
            Ok(self.punc(CaretEq)) as OptResult<_>,
            Ok(self.punc(BarEq)) as OptResult<_>,
            Ok(self.punc(StarStarEq)) as OptResult<_>,
        );

        if let Ok(Some(p)) = result {
            match p {
                Arrow => {
                    // TODO: No LineTerminator allowed before arrow.
                    eat_fn!(self.reify_arrow(left));
                }
                o@Eq | o@StarEq | o@SlashEq | o@PercentEq | o@PlusEq | o@MinusEq |
                o@LAngleAngleEq | o@RAngleAngleEq | o@RAngleAngleAngleEq |
                o@AmpEq | o@CaretEq | o@BarEq | o@StarStarEq => {
                    eat_fn!(self.reify_assignment(left, o));
                }
                _ => unreachable!(),
            }
        } else {
            eat_fn!(self.reify_expression(left));
        }

        Ok(Some(()))
    }
    fn reify_expression(&mut self, left: ()) -> OptResult<()> {
        Ok(Some(()))
    }
    fn reify_arrow(&mut self, left: ()) -> OptResult<()> {
        try_sequence!(
            self.parse_block_statement(),
            self.parse_expression(),
        )
    }
    fn reify_assignment(&mut self, left: (), op: tokens::PunctuatorToken) -> OptResult<()> {
        eat_fn!(self.parse_assignment_expression());

        Ok(Some(()))
    }

    fn parse_yield_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("yield"));

        if self.no_line_terminator() {
            self.punc(tokens::PunctuatorToken::Star);

            self.parse_expression()?;
        }

        Ok(Some(()))
    }
    fn parse_conditional_expression(&mut self) -> OptResult<()> {
        let test = try_fn!(self.parse_logical_or_expression());

        if let Some(_) = self.punc(tokens::PunctuatorToken::Question) {
            eat_fn!(self.with(Flag::In).parse_assignment_expression());

            eat_token!(self.punc(tokens::PunctuatorToken::Colon));

            eat_fn!(self.parse_assignment_expression());

            Ok(Some(()))
        } else {
            Ok(Some(test))
        }
    }
    fn parse_logical_or_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_logical_and_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::BarBar) {
            eat_fn!(self.parse_logical_and_expression());
        }

        Ok(Some(()))
    }
    fn parse_logical_and_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_bitwise_or_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::AmpAmp) {
            eat_fn!(self.parse_bitwise_or_expression());
        }

        Ok(Some(()))
    }
    fn parse_bitwise_or_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_bitwise_xor_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::Bar) {
            eat_fn!(self.parse_bitwise_xor_expression());
        }

        Ok(Some(()))
    }
    fn parse_bitwise_xor_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_bitwise_and_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::Caret) {
            eat_fn!(self.parse_bitwise_and_expression());
        }

        Ok(Some(()))
    }
    fn parse_bitwise_and_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_equality_expression());

        while let Some(_) = self.punc(tokens::PunctuatorToken::Amp) {
            eat_fn!(self.parse_equality_expression());
        }

        Ok(Some(()))
    }
    fn parse_equality_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_relational_expression());

        while let Ok(Some(_)) = try_sequence!(
            Ok(self.punc(tokens::PunctuatorToken::EqEq)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::EqEqEq)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::ExclamEq)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::ExclamEqEq)) as OptResult<_>,
        ) as OptResult<_> {
            eat_fn!(self.parse_relational_expression());
        }

        Ok(Some(()))
    }
    fn parse_relational_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_shift_expression());

        while let Ok(Some(_)) = try_sequence!(
            Ok(self.punc(tokens::PunctuatorToken::LAngle).map(tokens::Token::Punctuator)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::LAngleEq).map(tokens::Token::Punctuator)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::RAngle).map(tokens::Token::Punctuator)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::RAngleEq).map(tokens::Token::Punctuator)) as OptResult<_>,
            Ok(self.keyword("instanceof").map(tokens::Token::IdentifierName)) as OptResult<_>,
            Ok(if self.flags.allow_in { self.keyword("in").map(tokens::Token::IdentifierName) } else { None }) as OptResult<_>,
        ) as OptResult<_> {
            eat_fn!(self.parse_shift_expression());
        }

        Ok(Some(()))
    }
    fn parse_shift_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_additive_expression());

        while let Ok(Some(_)) = try_sequence!(
            Ok(self.punc(tokens::PunctuatorToken::LAngleAngle)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::RAngleAngle)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::RAngleAngle)) as OptResult<_>,
        ) as OptResult<_> {
            eat_fn!(self.parse_additive_expression());
        }

        Ok(Some(()))
    }
    fn parse_additive_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_multiplicative_expression());

        while let Ok(Some(_)) = try_sequence!(
            Ok(self.punc(tokens::PunctuatorToken::Plus)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::Minus)) as OptResult<_>,
        ) as OptResult<_> {
            eat_fn!(self.parse_multiplicative_expression());
        }

        Ok(Some(()))
    }
    fn parse_multiplicative_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_exponential_expression());

        while let Ok(Some(_)) = try_sequence!(
            Ok(self.punc(tokens::PunctuatorToken::Star)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::Slash)) as OptResult<_>,
            Ok(self.punc(tokens::PunctuatorToken::Percent)) as OptResult<_>,
        ) as OptResult<_> {
            eat_fn!(self.parse_exponential_expression());
        }

        Ok(Some(()))
    }
    fn parse_exponential_expression(&mut self) -> OptResult<()> {
        try_fn!(self.parse_unary_expression());

        // TODO:
        // if is_update_expression(expr) {
            if let Some(_) = self.punc(tokens::PunctuatorToken::StarStar) {
                eat_fn!(self.parse_exponential_expression());
            }
        // }

        Ok(Some(()))
    }
    fn parse_unary_expression(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_delete_expression(),
            self.parse_void_expression(),
            self.parse_typeof_expression(),
            self.parse_plus_expression(),
            self.parse_minus_expression(),
            self.parse_tilde_expression(),
            self.parse_exclam_expression(),
            self.parse_await_expression(),
            self.parse_update_expression(),
        )
    }
    fn parse_delete_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("delete"));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_void_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("void"));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_typeof_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("typeof"));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_plus_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Plus));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_minus_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Minus));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_tilde_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Tilde));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_exclam_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Exclam));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_await_expression(&mut self) -> OptResult<()> {
        if !self.flags.allow_await {
            return Ok(None);
        }

        try_token!(self.keyword("await"));

        eat_fn!(self.parse_unary_expression());

        Ok(Some(()))
    }
    fn parse_update_expression(&mut self) -> OptResult<()> {
        let op = if let Some(_) = self.punc(tokens::PunctuatorToken::PlusPlus) {
            true
        } else if let Some(_) = self.punc(tokens::PunctuatorToken::MinusMinus) {
            true
        } else {
            false
        };

        if op {
            eat_fn!(self.parse_update_expression());
        } else {
            try_fn!(self.parse_left_hand_expression(true));

            if self.no_line_terminator() {
                if let Some(_) = self.punc(tokens::PunctuatorToken::PlusPlus) {

                } else if let Some(_) = self.punc(tokens::PunctuatorToken::MinusMinus) {

                }
            }
        }

        Ok(Some(()))
    }
    fn parse_left_hand_expression(&mut self, allow_call: bool) -> OptResult<()> {
        if let Some(_) = self.keyword("new") {
            eat_fn!(self.parse_left_hand_expression(false));
        } else {
            let result = try_sequence!(
                self.parse_primary_expression(),
                self.parse_meta_property_expression(),
                self.parse_super_expression(),
            ) as OptResult<_>;

            try_fn!(result);
        }

        loop {
            if let Some(_) = self.punc(tokens::PunctuatorToken::SquareOpen) {
                eat_fn!(self.with(Flag::In).parse_expression());
                eat_token!(self.punc(tokens::PunctuatorToken::SquareClose));
            } else if let Some(_) = self.punc(tokens::PunctuatorToken::Period) {
                eat_token!(self.with(Flag::In).reference_identifier());
            } else {
                // TODO: skip if super`foo`
                if let Some(_) = self.parse_template_literal_expression()? {

                } else if allow_call {
                    if let Some(_) = self.parse_call_arguments()? {

                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        Ok(Some(()))
    }

    fn parse_call_arguments(&mut self) -> OptResult<()> {
        let mut parser = self.with(Flag::In);

        try_token!(parser.punc(tokens::PunctuatorToken::ParenOpen));

        loop {
            if let Some(_) = parser.punc(tokens::PunctuatorToken::Ellipsis) {
                eat_fn!(parser.parse_assignment_expression());
                break;
            }

            if let Some(_) = parser.parse_assignment_expression()? {
                if let Some(_) = parser.punc(tokens::PunctuatorToken::Comma) {
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        eat_token!(parser.punc(tokens::PunctuatorToken::ParenClose));

        Ok(Some(()))
    }

    fn parse_meta_property_expression(&mut self) -> OptResult<()> {
        // TODO:
        Ok(None)
    }
    fn parse_super_expression(&mut self) -> OptResult<()> {
        // TODO:
        Ok(None)
    }

    fn parse_primary_expression(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_this_expression(),
            self.parse_identifier_reference_expression(),
            self.parse_literal_expression(),
            self.parse_array_literal_expression(),
            self.parse_object_literal_expression(),
            self.parse_function_expression(),
            self.parse_class_expression(),
            self.parse_regular_expression_literal_expression(),
            self.parse_template_literal_expression(),
            self.parse_cover_parenthesized_expression(),
        )
    }

    fn parse_this_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("this"));

        Ok(Some(()))
    }
    fn parse_identifier_reference_expression(&mut self) -> OptResult<()> {
        try_token!(self.reference_identifier());

        Ok(Some(()))
    }
    fn parse_literal_expression(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_null_expression(),
            self.parse_true_expression(),
            self.parse_false_expression(),
            self.parse_numeric_expression(),
            self.parse_string_expression(),
        )
    }
    fn parse_null_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("null"));

        Ok(Some(()))
    }
    fn parse_true_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("true"));

        Ok(Some(()))
    }
    fn parse_false_expression(&mut self) -> OptResult<()> {
        try_token!(self.keyword("false"));

        Ok(Some(()))
    }
    fn parse_numeric_expression(&mut self) -> OptResult<()> {
        try_token!(self.numeric());

        Ok(Some(()))
    }
    fn parse_string_expression(&mut self) -> OptResult<()> {
        try_token!(self.string());

        Ok(Some(()))
    }
    fn parse_array_literal_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::SquareOpen));

        Ok(Some(()))
    }
    fn parse_object_literal_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

        Ok(Some(()))
    }
    fn parse_regular_expression_literal_expression(&mut self) -> OptResult<()> {
        try_token!(self.regex());

        Ok(Some(()))
    }
    fn parse_template_literal_expression(&mut self) -> OptResult<()> {
        try_token!(self.template());

        Ok(Some(()))
    }
    fn parse_cover_parenthesized_expression(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::ParenOpen));


        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        Ok(Some(()))
    }
}
