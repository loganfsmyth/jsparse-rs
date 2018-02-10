use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag, LookaheadResult};
use parser::utils::OptResult;

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_statement(&mut self) -> OptResult<()> {
        self.expect_expression();

        try_sequence!(
            self.parse_block_statement(),
            self.parse_variable_statement(),
            self.parse_empty_statement(),
            self.parse_expression_statement(),
            self.parse_if_statement(),
            self.parse_breakable_statement(),
            self.parse_continue_statement(),
            self.parse_break_statement(),
            self.parse_return_statement(),
            self.parse_with_statement(),
            self.parse_labelled_statement(),
            self.parse_throw_statement(),
            self.parse_try_statement(),
            self.parse_debugger_statement(),
        )
    }

    fn parse_statement_list_item(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_statement(),
            self.parse_declaration(),
        )
    }

    fn parse_block_statement(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        loop {
            match self.parse_statement_list_item()? {
                Some(item) => body.push(item),
                None => { break; }
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Some(()))
    }

    pub fn parse_variable_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("var"));

        eat_fn!(self.with(Flag::In).parse_var_declarator());

        while let Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_fn!(self.with(Flag::In).parse_var_declarator());
        }

        eat_token!(self.semicolon());

        Ok(Some(()))
    }

    pub fn parse_var_declarator(&mut self) -> OptResult<()> {
        if let Some(_) = self.parse_binding_pattern()? {
            eat_fn!(self.parse_initializer());
        } else {
            eat_token!(self.binding_identifier());
            self.parse_initializer();
        }
        Ok(Some(()))
    }

    pub fn parse_binding_pattern(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_object_binding_pattern(),
            self.parse_array_binding_pattern(),
        )
    }

    pub fn parse_object_binding_pattern(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));


        loop {
            if let Some(_) = self.parse_binding_property()? {
                if let None = self.punc(tokens::PunctuatorToken::Comma) {
                    break;
                }
            } else {
                break;
            }

        }

        eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Some(()))
    }

    // foo = 4
    // foo: bar = 4
    // foo: {bar} = 4
    // "foo": bar = 4
    fn parse_binding_property(&mut self) -> OptResult<()> {
        let _name = try_fn!(self.parse_property_name());

        if let Some(_) = self.punc(tokens::PunctuatorToken::Colon) {
            eat_fn!(self.parse_binding_element());
        } else {
            self.with(Flag::In).parse_initializer();
        }

        Ok(Some(()))
    }

    pub fn parse_property_name(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_literal_property_name(),
            self.parse_computed_property_name(),
        )
    }

    fn parse_literal_property_name(&mut self) -> OptResult<()> {
        if let Some(_) = self.identifier() {
        } else if let Some(_) = self.string() {
        } else if let Some(_) = self.numeric() {
        } else {
            return Ok(None);
        }

        Ok(Some(()))
    }

    fn parse_computed_property_name(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::SquareOpen));

        self.with(Flag::In).parse_assignment_expression()?;

        eat_token!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(Some(()))
    }

    pub fn parse_array_binding_pattern(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::SquareOpen));

        loop {
            if let Some(_) = self.parse_binding_rest_element()? {
                break;
            }

            if let Some(_) = self.parse_binding_element()? {

            }

            if let None = self.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(Some(()))
    }

    fn parse_binding_rest_element(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Ellipsis));

        if let None = self.parse_binding_pattern()? {
            eat_token!(self.binding_identifier());
        }

        Ok(Some(()))
    }
    fn parse_binding_element(&mut self) -> OptResult<()> {
        if let None = self.parse_binding_pattern()? {
            try_token!(self.binding_identifier());
        }
        self.with(Flag::In).parse_initializer()?;

        Ok(Some(()))
    }

    pub fn parse_initializer(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Eq));

        eat_fn!(self.parse_assignment_expression());

        Ok(Some(()))
    }

    fn parse_empty_statement(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Semicolon));

        Ok(Some(()))
    }

    fn parse_expression_statement(&mut self) -> OptResult<()> {
        try_fn!(self.with(Flag::In).parse_expression());

        try_token!(self.semicolon());

        Ok(Some(()))
    }

    fn parse_if_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("if"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

        eat_fn!(self.with(Flag::In).parse_expression());

        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement());

        if let Some(_) = self.keyword("else") {
            eat_fn!(self.parse_statement());
        }

        Ok(Some(()))
    }

    fn parse_breakable_statement(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_iteration_statement(),
            self.parse_switch_statement(),
        )
    }

    fn parse_iteration_statement(&mut self) -> OptResult<()> {
        try_sequence!(
            self.parse_do_while_statement(),
            self.parse_while_statement(),
            self.parse_for_statement(),
        )
    }

    fn parse_do_while_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("do"));

        eat_fn!(self.parse_statement());

        eat_token!(self.keyword("while"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        eat_fn!(self.with(Flag::In).parse_expression());
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_token!(self.semicolon_dowhile());

        Ok(Some(()))
    }

    fn parse_while_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("while"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        eat_fn!(self.with(Flag::In).parse_expression());
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement());

        Ok(Some(()))
    }

    fn parse_for_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("for"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

        eat_token!(self.punc(tokens::PunctuatorToken::Semicolon));
        eat_token!(self.punc(tokens::PunctuatorToken::Semicolon));

        // TODO

        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));


        Ok(Some(()))
    }

    fn parse_switch_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("switch"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        eat_fn!(self.with(Flag::In).parse_expression());
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_token!(self.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        let mut has_default = false;
        loop {
            if let Some(_) = self.parse_default_clause()? {
                if has_default {
                    return bail!("duplicate default statements");
                }
                has_default = true;
            } else {
                match self.parse_case_clause()? {
                    Some(item) => body.push(item),
                    None => { break; }
                }
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Some(()))
    }
    fn parse_default_clause(&mut self) -> OptResult<()> {
        try_token!(self.keyword("default"));
        eat_token!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        loop {
            match self.parse_statement_list_item()? {
                Some(item) => body.push(item),
                None => { break; }
            }
        }

        Ok(Some(()))
    }

    fn parse_case_clause(&mut self) -> OptResult<()> {
        try_token!(self.keyword("case"));
        eat_fn!(self.with(Flag::In).parse_expression());
        eat_token!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        loop {
            match self.parse_statement_list_item()? {
                Some(item) => body.push(item),
                None => { break; }
            }
        }

        Ok(Some(()))
    }


    fn parse_continue_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("continue"));

        if self.no_line_terminator() {
            self.label_identifier();
        }

        self.semicolon();

        Ok(Some(()))
    }
    fn parse_break_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("break"));

        if self.no_line_terminator() {
            self.label_identifier();
        }

        self.semicolon();
        Ok(Some(()))
    }

    fn parse_return_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("return"));

        if self.no_line_terminator() {
            self.with(Flag::In).parse_expression();
        }
        self.semicolon();

        Ok(Some(()))
    }

    fn parse_with_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("with"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        eat_fn!(self.with(Flag::In).parse_expression());
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement());

        Ok(Some(()))
    }

    fn parse_labelled_statement(&mut self) -> OptResult<()> {
        let is_label = if let Some(&LookaheadResult {
            line,
            token: tokens::Token::Punctuator(tokens::PunctuatorToken::Colon),
        }) = self.ident_lookahead() {
            true
        } else {
            false
        };

        if is_label {
            eat_token!(self.identifier());
            eat_token!(self.punc(tokens::PunctuatorToken::Colon));
            eat_fn!(self.parse_statement());
            Ok(Some(()))
        } else {
            Ok(None)
        }

    }

    fn parse_throw_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("throw"));

        if self.no_line_terminator() {
            eat_fn!(self.with(Flag::In).parse_expression());
        }

        Ok(Some(()))
    }

    fn parse_try_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("try"));

        eat_fn!(self.parse_block_statement());

        if let Some(_) = self.keyword("catch") {
            eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

            if let None = self.parse_binding_pattern()? {
                eat_token!(self.binding_identifier());
            }

            eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

            eat_fn!(self.parse_block_statement());
        }

        if let Some(_) = self.keyword("finally") {
            eat_fn!(self.parse_block_statement());
        }

        Ok(Some(()))
    }

    fn parse_debugger_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("debugger"));
        eat_token!(self.semicolon());

        Ok(Some(()))
    }
}
