use tokenizer::{Tokenizer, tokens};
use parser::{Parser, Flag, LookaheadResult};
use parser::utils::OptResult;
use parser::utils;

enum ForInit {
    // Can occur in any type of for-init.
    SingleVar,
    SingleLet,
    SingleConst,

    // Allowed in for and for..in
    SingleInitializedVar,

    // Allowed in for
    SingleInitializedLet,
    SingleInitializedConst,

    // Allowed in for
    MultiVar,
    MultiLet,
    MultiConst,

    // allowed in any for-init
    LeftHandExpression,

    // allowed in for
    Expression,
    None,
}

#[derive(Debug)]
enum LoopType {
    Any,
    For,
    ForAndForIn,
    ForX
}

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
            self.parse_expression_statement()?,
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
        ))
    }

    fn parse_statement_list_item(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_statement()?,
            self.parse_declaration()?,
        ))
    }

    pub fn parse_block_statement(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);
        try_token!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        loop {
            match parser.parse_statement_list_item()? {
                Ok(item) => body.push(item),
                Err(utils::NotFound) => { break; }
            }
        }

        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Ok(()))
    }

    pub fn parse_variable_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("var"));

        eat_fn!(self.with(Flag::In).parse_var_declarator()?);

        while let Ok(_) = self.punc(tokens::PunctuatorToken::Comma) {
            eat_fn!(self.with(Flag::In).parse_var_declarator()?);
        }

        eat_token!(self.semicolon());

        Ok(Ok(()))
    }

    pub fn parse_var_declarator(&mut self) -> OptResult<()> {
        if let Ok(_) = self.parse_binding_pattern()? {
            eat_fn!(self.parse_initializer()?);
        } else {
            eat_token!(self.binding_identifier());
            self.parse_initializer();
        }
        Ok(Ok(()))
    }

    pub fn parse_binding_pattern(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_object_binding_pattern()?,
            self.parse_array_binding_pattern()?,
        ))
    }

    pub fn parse_object_binding_pattern(&mut self) -> OptResult<()> {
        let mut parser = self.without(Flag::Template);

        try_token!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        loop {
            if let Ok(_) = parser.parse_binding_property()? {
                if let Err(utils::NotFound) = parser.punc(tokens::PunctuatorToken::Comma) {
                    break;
                }
            } else {
                break;
            }

        }

        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Ok(()))
    }

    // foo = 4
    // foo: bar = 4
    // foo: {bar} = 4
    // "foo": bar = 4
    fn parse_binding_property(&mut self) -> OptResult<()> {
        let _name = try_fn!(self.parse_property_name()?);

        if let Ok(_) = self.punc(tokens::PunctuatorToken::Colon) {
            eat_fn!(self.parse_binding_element()?);
        } else {
            self.with(Flag::In).parse_initializer();
        }

        Ok(Ok(()))
    }

    pub fn parse_property_name(&mut self) -> OptResult<()> {
        Ok(try_sequence!(
            self.parse_literal_property_name()?,
            self.parse_computed_property_name()?,
        ))
    }

    fn parse_literal_property_name(&mut self) -> OptResult<()> {
        if let Ok(_) = self.identifier() {
        } else if let Ok(_) = self.string() {
        } else if let Ok(_) = self.numeric() {
        } else {
            return Ok(Err(utils::NotFound));
        }

        Ok(Ok(()))
    }

    fn parse_computed_property_name(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::SquareOpen));

        self.expect_expression();
        self.with(Flag::In).parse_assignment_expression()?;

        eat_token!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(Ok(()))
    }

    pub fn parse_array_binding_pattern(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::SquareOpen));

        loop {
            if let Ok(_) = self.parse_binding_rest_element()? {
                break;
            }

            if let Ok(_) = self.parse_binding_element()? {

            }

            if let Err(utils::NotFound) = self.punc(tokens::PunctuatorToken::Comma) {
                break;
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::SquareClose));

        Ok(Ok(()))
    }

    pub fn parse_binding_rest_element(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Ellipsis));

        if let Err(utils::NotFound) = self.parse_binding_pattern()? {
            eat_token!(self.binding_identifier());
        }

        Ok(Ok(()))
    }
    fn parse_binding_element(&mut self) -> OptResult<()> {
        if let Err(utils::NotFound) = self.parse_binding_pattern()? {
            try_token!(self.binding_identifier());
        }
        self.with(Flag::In).parse_initializer()?;

        Ok(Ok(()))
    }

    pub fn parse_initializer(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Eq));

        self.expect_expression();
        eat_fn!(self.parse_assignment_expression()?);

        Ok(Ok(()))
    }

    fn parse_empty_statement(&mut self) -> OptResult<()> {
        try_token!(self.punc(tokens::PunctuatorToken::Semicolon));

        Ok(Ok(()))
    }

    fn parse_expression_statement(&mut self) -> OptResult<()> {
        let result = self.with(Flag::In).parse_expression();

        try_fn!(result?);

        eat_token!(self.semicolon());

        Ok(Ok(()))
    }

    fn parse_if_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("if"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);

        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement()?);

        if let Ok(_) = self.keyword("else") {
            eat_fn!(self.parse_statement()?);
        }

        Ok(Ok(()))
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
        try_token!(self.keyword("do"));

        eat_fn!(self.parse_statement()?);

        eat_token!(self.keyword("while"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_token!(self.semicolon_dowhile());

        Ok(Ok(()))
    }

    fn parse_while_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("while"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement()?);

        Ok(Ok(()))
    }

    fn parse_for_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("for"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

        self.expect_expression();

        let (kind, pattern, initialized, multiple, bracket, left_hand) = if let Ok(_) = self.keyword("var") {
            let mut pattern = false;
            let mut initialized = false;
            if let Ok(_) = self.parse_binding_pattern()? {
                pattern = true;

                if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            } else {
                eat_token!(self.binding_identifier());
                if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            }

            let mut multiple = false;
            if !pattern || initialized {
                while let Ok(_) = self.punc(tokens::PunctuatorToken::Comma) {
                    multiple = true;
                    eat_fn!(self.without(Flag::In).parse_var_declarator()?);
                }
            }

            ("var", pattern, initialized, multiple, false, false)
        } else if let Ok(_) = self.keyword("const") {
            let mut pattern = false;
            let mut initialized = false;
            if let Ok(_) = self.parse_binding_pattern()? {
                pattern = true;

                if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            } else {
                eat_token!(self.binding_identifier());
                if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                    initialized = true;
                }
            }

            let mut multiple = false;
            if !pattern || initialized {
                while let Ok(_) = self.punc(tokens::PunctuatorToken::Comma) {
                    multiple = true;
                    eat_fn!(self.without(Flag::In).parse_lexical_declarator(true)?);
                }
            }

            ("const", pattern, initialized, multiple, false, false)
        } else {
            let (maybe_decl, bracket) = if let Some(&LookaheadResult { line, ref token }) = self.ident_lookahead() {
                match *token {
                    tokens::Token::IdentifierName(_) => (true, false),
                    tokens::Token::Punctuator(tokens::PunctuatorToken::SquareOpen) => (true, true),
                    tokens::Token::Punctuator(tokens::PunctuatorToken::CurlyOpen) => (true, false),
                    _ => (false, false),
                }
            } else {
                (false, false)
            };


            let decl = if maybe_decl {
                if let Ok(_) = self.keyword("let") {
                    let mut pattern = false;
                    let mut initialized = false;
                    if let Ok(_) = self.parse_binding_pattern()? {
                        pattern = true;

                        if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                            initialized = true;
                        }
                    } else {
                        eat_token!(self.binding_identifier());
                        if let Ok(_) = self.without(Flag::In).parse_initializer()? {
                            initialized = true;
                        }
                    }

                    let mut multiple = false;
                    if !pattern || initialized {
                        while let Ok(_) = self.punc(tokens::PunctuatorToken::Comma) {
                            multiple = true;
                            eat_fn!(self.without(Flag::In).parse_lexical_declarator(true)?);
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
                // TODO: What to do here? If this is a LeftHandSideExpression,
                // the for can be any type, otherwise it _must_ be 'ForStatement'
                self.without(Flag::In).parse_expression()?;
                ("expression", false, false, false, false, false)
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

        let found = if maybe_for {
            if let Ok(_) = self.punc(tokens::PunctuatorToken::Semicolon) {
                self.expect_expression();
                self.parse_expression()?;
                eat_token!(self.punc(tokens::PunctuatorToken::Semicolon));

                self.expect_expression();
                self.parse_expression()?;
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found && maybe_x {
            if let Ok(_) = self.keyword("in") {
                self.expect_expression();
                eat_fn!(self.with(Flag::In).parse_assignment_expression()?);
            } else if let Ok(_) = self.keyword("of") {
                self.expect_expression();
                eat_fn!(self.with(Flag::In).parse_assignment_expression()?);
            } else {
                bail!("Invalid for loop");
            }
        }

        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));


        Ok(Ok(()))
    }
/*

enum LoopType {
    Any,
    For,
    ForAndForIn,
    ForX
}

enum ForInit {
    // Can occur in any type of for-init.
    SingleVar,
    SingleLet,
    SingleConst,

    // Allowed in for and for..in
    SingleInitializedVar,

    // Allowed in for
    SingleInitializedLet,
    SingleInitializedConst,

    // Allowed in for
    MultiVar,
    MultiLet,
    MultiConst,

    // allowed in any for-init
    LeftHandExpression,

    // allowed in for
    Expression,
    None,
}


*/
    fn parse_switch_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("switch"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));


        let mut parser = self.without(Flag::Template);
        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyOpen));

        let mut body = vec![];
        let mut has_default = false;
        loop {
            if let Ok(_) = parser.parse_default_clause()? {
                if has_default {
                    return bail!("duplicate default statements");
                }
                has_default = true;
            } else {
                match parser.parse_case_clause()? {
                    Ok(item) => body.push(item),
                    Err(utils::NotFound) => { break; }
                }
            }
        }

        eat_token!(parser.punc(tokens::PunctuatorToken::CurlyClose));

        Ok(Ok(()))
    }
    fn parse_default_clause(&mut self) -> OptResult<()> {
        try_token!(self.keyword("default"));
        eat_token!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        loop {
            match self.parse_statement_list_item()? {
                Ok(item) => body.push(item),
                Err(utils::NotFound) => { break; }
            }
        }

        Ok(Ok(()))
    }

    fn parse_case_clause(&mut self) -> OptResult<()> {
        try_token!(self.keyword("case"));
        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);
        eat_token!(self.punc(tokens::PunctuatorToken::Colon));

        let mut body = vec![];
        loop {
            match self.parse_statement_list_item()? {
                Ok(item) => body.push(item),
                Err(utils::NotFound) => { break; }
            }
        }

        Ok(Ok(()))
    }


    fn parse_continue_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("continue"));

        if self.no_line_terminator() {
            self.label_identifier();
        }

        self.semicolon();

        Ok(Ok(()))
    }
    fn parse_break_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("break"));

        if self.no_line_terminator() {
            self.label_identifier();
        }

        self.semicolon();
        Ok(Ok(()))
    }

    fn parse_return_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("return"));

        if self.no_line_terminator() {
            self.expect_expression();
            self.with(Flag::In).parse_expression();
        }
        self.semicolon();

        Ok(Ok(()))
    }

    fn parse_with_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("with"));

        eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));
        self.expect_expression();
        eat_fn!(self.with(Flag::In).parse_expression()?);
        eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

        eat_fn!(self.parse_statement()?);

        Ok(Ok(()))
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
            eat_fn!(self.parse_statement()?);
            Ok(Ok(()))
        } else {
            Ok(Err(utils::NotFound))
        }

    }

    fn parse_throw_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("throw"));

        if self.no_line_terminator() {
            self.expect_expression();
            eat_fn!(self.with(Flag::In).parse_expression()?);
        }

        Ok(Ok(()))
    }

    fn parse_try_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("try"));

        eat_fn!(self.parse_block_statement()?);

        if let Ok(_) = self.keyword("catch") {
            eat_token!(self.punc(tokens::PunctuatorToken::ParenOpen));

            if let Err(utils::NotFound) = self.parse_binding_pattern()? {
                eat_token!(self.binding_identifier());
            }

            eat_token!(self.punc(tokens::PunctuatorToken::ParenClose));

            eat_fn!(self.parse_block_statement()?);
        }

        if let Ok(_) = self.keyword("finally") {
            eat_fn!(self.parse_block_statement()?);
        }

        Ok(Ok(()))
    }

    fn parse_debugger_statement(&mut self) -> OptResult<()> {
        try_token!(self.keyword("debugger"));
        eat_token!(self.semicolon());

        Ok(Ok(()))
    }
}
