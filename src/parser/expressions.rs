use tokenizer::{Tokenizer, tokens};
use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_left_hand_side_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }

    pub fn parse_expression(&mut self) -> InnerResult<()> {
        self.parse_assignment_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::Comma) {
            self.parse_assignment_expression()?
        }

        Ok(())
    }
    pub fn parse_assignment_expression(&mut self) -> InnerResult<()> {

        match self.parse_yield_expression() {
            Err(InnerError::NotFound) => {}
            Ok(expr) => return Ok(expr),
            Err(e) => return Err(e),
        }

        // Alternatively this can kick off the cover lookahead here then skip reify

        let left = self.parse_conditional_expression()?;


        use tokens::PunctuatorToken::*;
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

        match result {
            Err(InnerError::NotFound) => {
                self.reify_expression(left)?
            }
            Err(e) => Err(e),
            Ok(punc) => {
                match punc {
                    Arrow => {
                        // TODO: No LineTerminator allowed before arrow.
                        self.reify_arrow(left)?
                    }
                    o@Eq | o@StarEq | o@SlashEq | o@PercentEq | o@PlusEq | o@MinusEq |
                    o@LAngleAngleEq | o@RAngleAngleEq | o@RAngleAngleAngleEq |
                    o@AmpEq | o@CaretEq | o@BarEq | o@StarStarEq => {
                        self.reify_assignment(left, o)?
                    }
                }
            }
        }
    }
    fn reify_expression(&mut self, left: ()) -> InnerResult<()> {
        Ok(())
    }
    fn reify_arrow(&mut self, left: ()) -> InnerResult<()> {
        try_sequence!(
            self.parse_block_statement(),
            self.parse_expression(),
        )
    }
    fn reify_assignment(&mut self, left: (), op: tokens::PunctuatorToken) -> InnerResult<()> {
        self.parse_assignment_expression()?;

        Ok(())
    }

    fn parse_yield_expression(&mut self) -> InnerResult<()> {
        self.keyword("yield")?;
        Ok(())
    }
    fn parse_conditional_expression(&mut self) -> InnerResult<()> {
        let test = self.parse_logical_or_expression()?;

        if let Ok(_) = self.punc(tokens::PunctuatorToken::Question) {
            // TODO: Throw if NotFound
            self.with(Flag::In).parse_assignment_expression()?;

            self.punc(tokens::PunctuatorToken::Colon)?;

            self.parse_assignment_expression()?;
        } else {
            Ok(test)
        }
    }
    fn parse_logical_or_expression(&mut self) -> InnerResult<()> {
        self.parse_logical_and_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::BarBar) {
            // TODO: Throw if NotFound
            self.parse_logical_and_expression()?;
        }

        Ok(())
    }
    fn parse_logical_and_expression(&mut self) -> InnerResult<()> {
        self.parse_bitwise_or_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::AmpAmp) {
            // TODO: Throw if NotFound
            self.parse_bitwise_or_expression()?;
        }

        Ok(())
    }
    fn parse_bitwise_or_expression(&mut self) -> InnerResult<()> {
        self.parse_bitwise_xor_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::Bar) {
            // TODO: Throw if NotFound
            self.parse_bitwise_xor_expression()?;
        }

        Ok(())
    }
    fn parse_bitwise_xor_expression(&mut self) -> InnerResult<()> {
        self.parse_bitwise_and_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::Caret) {
            // TODO: Throw if NotFound
            self.parse_bitwise_and_expression()?;
        }

        Ok(())
    }
    fn parse_bitwise_and_expression(&mut self) -> InnerResult<()> {
        self.parse_equality_expression()?;

        while let Ok(_) = self.punc(tokens::PunctuatorToken::Amp) {
            // TODO: Throw if NotFound
            self.parse_equality_expression()?;
        }

        Ok(())
    }
    fn parse_equality_expression(&mut self) -> InnerResult<()> {
        self.parse_relational_expression()?;

        while let Ok(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::EqEq),
            self.punc(tokens::PunctuatorToken::EqEqEq),
            self.punc(tokens::PunctuatorToken::ExclamEq),
            self.punc(tokens::PunctuatorToken::ExclamEqEq),
        ) {
            // TODO: Throw if NotFound
            self.parse_relational_expression()?;
        }

        Ok(())
    }
    fn parse_relational_expression(&mut self) -> InnerResult<()> {
        self.parse_shift_expression()?;

        while let Ok(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::LAngle),
            self.punc(tokens::PunctuatorToken::LAngleEq),
            self.punc(tokens::PunctuatorToken::RAngle),
            self.punc(tokens::PunctuatorToken::RAngleEq),
            self.keyword("instanceof"),
            if self.flags.allow_in { self.keyword("in") } else { Err(InnerError::NotFound) },
        ) {
            // TODO: Throw if NotFound
            self.parse_shift_expression()?;
        }

        Ok(())
    }
    fn parse_shift_expression(&mut self) -> InnerResult<()> {
        self.parse_additive_expression()?;

        while let Ok(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::LAngleAngle),
            self.punc(tokens::PunctuatorToken::RAngleAngle),
            self.punc(tokens::PunctuatorToken::RAngleAngle),
        ) {
            // TODO: Throw if NotFound
            self.parse_additive_expression()?;
        }

        Ok(())
    }
    fn parse_additive_expression(&mut self) -> InnerResult<()> {
        self.parse_multiplicative_expression()?;

        while let Ok(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::Plus),
            self.punc(tokens::PunctuatorToken::Minus),
        ) {
            // TODO: Throw if NotFound
            self.parse_multiplicative_expression()?;
        }

        Ok(())
    }
    fn parse_multiplicative_expression(&mut self) -> InnerResult<()> {
        self.parse_exponential_expression()?;

        while let Ok(_) = try_sequence!(
            self.punc(tokens::PunctuatorToken::Star),
            self.punc(tokens::PunctuatorToken::Slash),
            self.punc(tokens::PunctuatorToken::Percent),
        ) {
            // TODO: Throw if NotFound
            self.parse_exponential_expression()?;
        }

        Ok(())
    }
    fn parse_exponential_expression(&mut self) -> InnerResult<()> {
        let expr = self.parse_unary_expression()?;

        if is_update_expression(expr) {
            if let Ok(_) = self.punc(tokens::PunctuatorToken::StarStar) {
                // TODO: Throw if NotFound
                self.parse_exponential_expression()?;
            }
        }

        Ok(())
    }
    fn parse_unary_expression(&mut self) -> InnerResult<()> {
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
    fn parse_delete_expression(&mut self) -> InnerResult<()> {
        self.keyword("delete")?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_void_expression(&mut self) -> InnerResult<()> {
        self.keyword("void")?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_typeof_expression(&mut self) -> InnerResult<()> {
        self.keyword("typeof")?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_plus_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::Plus)?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_minus_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::Minus)?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_tilde_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::Tilde)?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_exclam_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::Exclam)?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_await_expression(&mut self) -> InnerResult<()> {
        if !self.flags.allow_await {
            return Err(InnerError::NotFound);
        }

        self.keyword("await")?;

        // TODO: NotFound
        self.parse_unary_expression()?;

        Ok(())
    }
    fn parse_update_expression(&mut self) -> InnerResult<()> {
        let op = try_sequence!(
            self.punc(tokens::PunctuatorToken::PlusPlus),
            self.punc(tokens::PunctuatorToken::MinusMinus),
        );

        match op {
            Ok(_) => {
                return self.parse_update_expression();
            }
            Err(InnerError::NotFound) => {
                self.parse_left_hand_expression(true)?;

                try_sequence!(
                    self.punc(tokens::PunctuatorToken::PlusPlus),
                    self.punc(tokens::PunctuatorToken::MinusMinus),
                );
            }
            Err(e) => return Err(e),
        }

    }
    fn parse_left_hand_expression(&mut self, allow_call: bool) -> InnerResult<()> {
        if let Ok(_) = self.keyword("new") {
            self.parse_left_hand_expression(false)
        } else {
            try_sequence!(
                self.parse_primary_expression(),
                self.parse_meta_property_expression(),
                self.parse_super_expression(),
            )
        }

        loop {
            if let Ok(_) = self.punc(tokens::PunctuatorToken::SquareOpen) {
                self.with(Flag::In).parse_expression()?;
                self.eat_punc(tokens::PunctuatorToken::SquareClose)?;
            } else if let Ok(_) = self.punc(tokens::PunctuatorToken::Period) {
                self.with(Flag::In).parse_identifier()?;
            } else {
                // TODO: skip if super`foo`
                match self.parse_template_literal_expression() {
                    Err(InnerError::NotFound) => {
                        if !allow_call { break }

                        match self.parse_call_arguments() {
                            Err(InnerError::NotFound) => {
                                break;
                            }
                            Err(v) => {
                                return Err(v);
                            }
                            Ok(item) => {

                            }
                        }
                    }
                    Err(v) => {
                        return Err(v);
                    }
                    Ok(item) => {

                    }
                }
            }

        }
    }
    fn parse_meta_property_expression(&mut self) -> InnerResult<()> {}
    fn parse_super_expression(&mut self) -> InnerResult<()> {

    }

    fn parse_new_call_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_primary_expression(&mut self) -> InnerResult<()> {
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

    fn parse_this_expression(&mut self) -> InnerResult<()> {
        self.keyword("this")?;

        Ok(())
    }
    fn parse_identifier_reference_expression(&mut self) -> InnerResult<()> {
        self.nonreserved_identifier()?;

        Ok(())
    }
    fn parse_literal_expression(&mut self) -> InnerResult<()> {
        try_sequence!(
            self.parse_null_expression(),
            self.parse_true_expression(),
            self.parse_false_expression(),
            self.parse_numeric_expression(),
            self.parse_string_expression(),
        )
    }
    fn parse_null_expression(&mut self) -> InnerResult<()> {
        self.keyword("null")?;

        Ok(())
    }
    fn parse_true_expression(&mut self) -> InnerResult<()> {
        self.keyword("true")?;

        Ok(())
    }
    fn parse_false_expression(&mut self) -> InnerResult<()> {
        self.keyword("false")?;

        Ok(())
    }
    fn parse_numeric_expression(&mut self) -> InnerResult<()> {
        self.numeric()?;

        Ok(())
    }
    fn parse_string_expression(&mut self) -> InnerResult<()> {
        self.string()?;

        Ok(())
    }
    fn parse_array_literal_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::SquareOpen)?;

        Ok(())
    }
    fn parse_object_literal_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::CurlyOpen)?;

        Ok(())
    }
    fn parse_regular_expression_literal_expression(&mut self) -> InnerResult<()> {
        self.regex()?;

        Ok(())
    }
    fn parse_template_literal_expression(&mut self) -> InnerResult<()> {
        self.template()?;

        Ok(())
    }
    fn parse_cover_parenthesized_expression(&mut self) -> InnerResult<()> {
        self.punc(tokens::PunctuatorToken::ParenOpen)?;


        self.punc(tokens::PunctuatorToken::ParenClose)?;

        Ok(())
    }
}
