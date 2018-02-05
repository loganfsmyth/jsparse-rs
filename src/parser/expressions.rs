use parser::Parser;
use parser::utils::{InnerResult, InnerError};

impl<'a, T> Parser<'a, T> {
    pub fn parse_expression(&mut self) -> InnerResult<()> {
        return self.parse_primary_expression();

        // return Err(InnerError::NotFound);
    }
    pub fn parse_assignment_expression(&mut self) -> InnerResult<()> {
        return self.parse_primary_expression();

        // return Err(InnerError::NotFound);
    }
    fn parse_conditional_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_logical_or_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_logical_and_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_bitwise_or_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_bitwise_xor_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_bitwise_and_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_equality_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_relational_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_shift_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_additive_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_multiplicative_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_exponential_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_unary_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_update_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    pub fn parse_left_hand_expression(&mut self) -> InnerResult<()> {
        return self.parse_primary_expression();

        // return Err(InnerError::NotFound);
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
        self.try_identifier("this")?;

        Ok(())
    }
    fn parse_identifier_reference_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_literal_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_array_literal_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_object_literal_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_regular_expression_literal_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_template_literal_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
    fn parse_cover_parenthesized_expression(&mut self) -> InnerResult<()> {
        return Err(InnerError::NotFound);
    }
}
