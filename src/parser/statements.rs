use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag, LookaheadResult};
use parser::utils::{OptResult, TokenResult};
use parser::utils;

// enum ForInit {
//     // Can occur in any type of for-init.
//     SingleVar,
//     SingleLet,
//     SingleConst,

//     // Allowed in for and for..in
//     SingleInitializedVar,

//     // Allowed in for
//     SingleInitializedLet,
//     SingleInitializedConst,

//     // Allowed in for
//     MultiVar,
//     MultiLet,
//     MultiConst,

//     // allowed in any for-init
//     LeftHandExpression,

//     // allowed in for
//     Expression,
//     None,
// }

// #[derive(Debug)]
// enum LoopType {
//     Any,
//     For,
//     ForAndForIn,
//     ForX
// }

impl<'code, T> Parser<'code, T>
where
    T: Tokenizer<'code>
{
    pub fn parse_statement(&mut self) -> OptResult<()> {
        self.expect_expression();

        Ok(try_sequence!(
            self.parse_block_statement()?,
            self.parse_variable_statement()?,
            self.parse_empty_statement()?,
            // self.parse_expression_statement()?, // TODO: This is an ambiguity
            self.parse_if_statement()?,
            self.parse_breakable_statement()?,
            self.parse_continue_statement()?,
            self.parse_break_statement()?,
            self.parse_return_statement()?,
            self.parse_with_statement()?,
            self.parse_labelled_statement()?,
            self.parse_throw_statement()?,
            self.parse_try_statement()?,
            self.parse_debugger_statement()?,
            self.parse_expression_statement()?,
        ))
    }

    fn parse_statement_list_item(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_declaration()?,
            self.parse_statement()?,
        ))
    }

    pub fn parse_block_statement(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);
        try_value!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        while let TokenResult::Some(item) = parser.parse_statement_list_item()? {
            body.push(item);
        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }

    pub fn parse_variable_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("var"));

        eat_value!(self.with(Flag::In).parse_var_declarator()?);

        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_value!(self.with(Flag::In).parse_var_declarator()?);
        }

        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    pub fn parse_var_declarator(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.parse_binding_pattern()? {
            eat_value!(self.parse_initializer()?);
        } else {
            eat_value!(self.binding_identifier());
            opt_value!(self.parse_initializer()?);
        }
        Ok(TokenResult::Some(()))
    }

    pub fn parse_binding_pattern(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_object_binding_pattern()?,
            self.parse_array_binding_pattern()?,
        ))
    }

    pub fn parse_object_binding_pattern(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);

        try_value!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        loop {
            if let TokenResult::Some(_) = parser.parse_binding_property()? {
                if let TokenResult::None = parser.punc(tokens::PunctuatorToken::Comma) {
                    break;
                }
            } else {
                break;
            }

        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }

    // foo = 4
    // foo: bar = 4
    // foo: {bar} = 4
    // "foo": bar = 4
    fn parse_binding_property(&mut self) -> OptResult<()> {
        let _name = try_value!(self.parse_property_name()?);

        if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Colon) {
            eat_value!(self.parse_binding_element()?);
        } else {
            opt_value!(self.with(Flag::In).parse_initializer()?);
        }

        Ok(TokenResult::Some(()))
    }

    pub fn parse_property_name(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_literal_property_name()?,
            self.parse_computed_property_name()?,
        ))
    }

    fn parse_literal_property_name(&mut self) -> OptResult<()> {
        if let TokenResult::Some(_) = self.identifier() {
        } else if let TokenResult::Some(_) = self.string() {
        } else if let TokenResult::Some(_) = self.numeric() {
        } else {
            return Ok(TokenResult::None);
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_computed_property_name(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::SquareOpen));

        self.expect_expression();
        opt_value!(self.with(Flag::In).parse_assignment_expression()?);

        eat_value!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(TokenResult::Some(()))
    }

    pub fn parse_array_binding_pattern(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::SquareOpen));

        loop {
            if let TokenResult::Some(_) = self.parse_binding_rest_element()? {
                break;
            }

            if let TokenResult::Some(_) = self.parse_binding_element()? {

            }

            if let TokenResult::None = self.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
        }

        eat_value!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(TokenResult::Some(()))
    }

    pub fn parse_binding_rest_element(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Ellipsis));

        if let TokenResult::None = self.parse_binding_pattern()? {
            eat_value!(self.binding_identifier());
        }

        Ok(TokenResult::Some(()))
    }
    fn parse_binding_element(&mut self) -> OptResult<()> {
        if let TokenResult::None = self.parse_binding_pattern()? {
            try_value!(self.binding_identifier());
        }
        opt_value!(self.with(Flag::In).parse_initializer()?);

        Ok(TokenResult::Some(()))
    }

    pub fn parse_initializer(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Eq));

        self.expect_expression();
        eat_value!(self.parse_assignment_expression()?);

        Ok(TokenResult::Some(()))
    }

    fn parse_empty_statement(&mut self) -> OptResult<()> {
        try_value!(self.punc(tokens::PunctuatorToken::Semicolon));

        Ok(TokenResult::Some(()))
    }

    fn parse_expression_statement(&mut self) -> OptResult<()> {
        try_value!(self.with(Flag::In).parse_expression()?);

        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    fn parse_if_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("if"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));

        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);

        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_value!(self.parse_statement()?);

        if let TokenResult::Some(_) = self.keyword("else") {
            eat_value!(self.parse_statement()?);
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_breakable_statement(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_iteration_statement()?,
            self.parse_switch_statement()?,
        ))
    }

    fn parse_iteration_statement(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_do_while_statement()?,
            self.parse_while_statement()?,
            self.parse_for_statement()?,
        ))
    }

    fn parse_do_while_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("do"));

        eat_value!(self.parse_statement()?);

        eat_value!(self.keyword("while"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);
        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_value!(self.semicolon_dowhile());

        Ok(TokenResult::Some(()))
    }

    fn parse_while_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("while"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);
        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_value!(self.parse_statement()?);

        Ok(TokenResult::Some(()))
    }

    fn parse_for_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("for"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));

        self.expect_expression();

        let (kind, pattern, initialized, multiple, bracket, left_hand) = if let TokenResult::Some(_) = self.keyword("var") {
            let mut pattern = false;
            let mut initialized = false;
            if let TokenResult::Some(_) = self.parse_binding_pattern()? {
                pattern = true;

                if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            } else {
                eat_value!(self.binding_identifier());
                if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            }

            let mut multiple = false;
            if !pattern || initialized {
                while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
                    multiple = true;
                    eat_value!(self.without(Flag::In).parse_var_declarator()?);
                }
            }

            ("var", pattern, initialized, multiple, false, false)
        } else if let TokenResult::Some(_) = self.keyword("const") {
            let mut pattern = false;
            let mut initialized = false;
            if let TokenResult::Some(_) = self.parse_binding_pattern()? {
                pattern = true;

                if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            } else {
                eat_value!(self.binding_identifier());
                if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            }

            let mut multiple = false;
            if !pattern || initialized {
                while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
                    multiple = true;
                    eat_value!(self.without(Flag::In).parse_lexical_declarator(true)?);
                }
            }

            ("const", pattern, initialized, multiple, false, false)
        } else {
            let (maybe_decl, bracket) = if let Some(&LookaheadResult { line, ref token }) = self.ident_lookahead() {
                match *token {
                    tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) => (name != "in" && name != "of", false),
                    tokens::Token::Punctuator(tokens::PunctuatorToken::SquareOpen) => (true, true),
                    tokens::Token::Punctuator(tokens::PunctuatorToken::CurlyOpen) => (true, false),
                    _ => (false, false),
                }
            } else {
                (false, false)
            };

            println!("maybe_decl: {}, {:?}", maybe_decl, self.token());


            let decl = if maybe_decl {
                if let TokenResult::Some(_) = self.keyword("let") {
                    let mut pattern = false;
                    let mut initialized = false;
                    if let TokenResult::Some(_) = self.parse_binding_pattern()? {
                        pattern = true;

                        if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                            initialized = true;
                        }
                    } else {
                        eat_value!(self.binding_identifier());
                        if let TokenResult::Some(_) = self.without(Flag::In).parse_initializer()? {
                            initialized = true;
                        }
                    }

                    let mut multiple = false;
                    if !pattern || initialized {
                        while let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Comma) {
                            multiple = true;
                            eat_value!(self.without(Flag::In).parse_lexical_declarator(true)?);
                        }
                    }

                    Some(("let", pattern, initialized, multiple, bracket, false))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(decl) = decl {
                decl
            } else {
                println!("expr pre {:?}", self.token());
                // TODO: What to do here? If this is a LeftHandSideExpression,
                // the for can be any type, otherwise it _must_ be 'ForStatement'
                opt_value!(self.without(Flag::In).parse_expression()?);

                println!("expr post {:?}", self.token());
                ("expression", false, false, false, false, true /* tmp */)
            }
        };

        let (maybe_for, maybe_x) = match kind {
            "var" | "let" => {
                let maybe_for = !pattern || initialized;
                let maybe_x = !multiple && !initialized;

                (maybe_for, maybe_x)
            }
            "const" => {
                let maybe_for = initialized;
                let maybe_x = !multiple && !initialized;

                (maybe_for, maybe_x)
            }
            "expression" => {
                if left_hand {
                    // for/in/of
                    (true, true)
                } else {
                    // for
                    (true, false)
                }
            }
            _ => unreachable!(),
        };

        println!("{}, {}, {:?}", maybe_for, maybe_x, self.token());

        let found = if maybe_for {
            if let TokenResult::Some(_) = self.punc(tokens::PunctuatorToken::Semicolon) {
                self.expect_expression();
                opt_value!(self.parse_expression()?);

                eat_value!(self.punc(tokens::PunctuatorToken::Semicolon));

                self.expect_expression();
                opt_value!(self.parse_expression()?);
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found {
            if maybe_x {
                if let TokenResult::Some(_) = self.keyword("in") {
                    self.expect_expression();
                    eat_value!(self.with(Flag::In).parse_assignment_expression()?);
                } else if let TokenResult::Some(_) = self.keyword("of") {
                    self.expect_expression();
                    eat_value!(self.with(Flag::In).parse_assignment_expression()?);
                } else {
                    bail!("Invalid for loop");
                }
            } else {
                bail!("bad for loop");
            }
        }

        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));


        Ok(TokenResult::Some(()))
    }

    fn parse_switch_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("switch"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);
        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));


        let mut parser = self.without(Flag::Template);
        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        let mut has_default = false;
        loop {
            if let TokenResult::Some(_) = parser.parse_default_clause()? {
                if has_default {
                    return bail!("duplicate default statements");
                }
                has_default = true;
            } else if let TokenResult::Some(item) = parser.parse_case_clause()? {
                body.push(item);
            } else {
                break;
            }
        }

        eat_value!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(TokenResult::Some(()))
    }
    fn parse_default_clause(&mut self) -> OptResult<()> {
        try_value!(self.keyword("default"));
        eat_value!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        while let TokenResult::Some(item) = self.parse_statement_list_item()? {
            body.push(item);
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_case_clause(&mut self) -> OptResult<()> {
        try_value!(self.keyword("case"));
        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);
        eat_value!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        while let TokenResult::Some(item) = self.parse_statement_list_item()? {
            body.push(item);
        }

        Ok(TokenResult::Some(()))
    }


    fn parse_continue_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("continue"));

        if self.no_line_terminator() {
            opt_value!(self.label_identifier());
        }

        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }
    fn parse_break_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("break"));

        if self.no_line_terminator() {
            opt_value!(self.label_identifier());
        }

        eat_value!(self.semicolon());
        Ok(TokenResult::Some(()))
    }

    fn parse_return_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("return"));

        if self.no_line_terminator() {
            println!("had linetermiantor");

            self.expect_expression();
            opt_value!(self.with(Flag::In).parse_expression()?);

            println!("{:?}", self.token());
        } else {
            println!("had linetermiantor");
        }

        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }

    fn parse_with_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("with"));

        eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_value!(self.with(Flag::In).parse_expression()?);
        eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_value!(self.parse_statement()?);

        Ok(TokenResult::Some(()))
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
            eat_value!(self.identifier());
            eat_value!(self.punc(tokens::PunctuatorToken::Colon));
            eat_value!(self.parse_statement()?);
            Ok(TokenResult::Some(()))
        } else {
            Ok(TokenResult::None)
        }

    }

    fn parse_throw_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("throw"));

        if self.no_line_terminator() {
            self.expect_expression();
            eat_value!(self.with(Flag::In).parse_expression()?);
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_try_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("try"));

        eat_value!(self.parse_block_statement()?);

        if let TokenResult::Some(_) = self.keyword("catch") {
            eat_value!(self.punc(tokens::PunctuatorToken::ParenOpen));

            if let TokenResult::None = self.parse_binding_pattern()? {
                eat_value!(self.binding_identifier());
            }

            eat_value!(self.punc(tokens::PunctuatorToken::ParenClose));

            eat_value!(self.parse_block_statement()?);
        }

        if let TokenResult::Some(_) = self.keyword("finally") {
            eat_value!(self.parse_block_statement()?);
        }

        Ok(TokenResult::Some(()))
    }

    fn parse_debugger_statement(&mut self) -> OptResult<()> {
        try_value!(self.keyword("debugger"));
        eat_value!(self.semicolon());

        Ok(TokenResult::Some(()))
    }
}
