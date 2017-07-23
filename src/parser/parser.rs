use ast;

pub struct ParserFlags {
    // "yield" toggle
    generator: bool,

    // "await" toggle
    async: bool,

    // "return" toggle
    function: bool,

    // "in" toggle
    for_in: bool,

    strict: bool,

    // Static flags for this parser pass
    module: bool,
    annexb: bool,
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {}


    fn get_position() -> (usize, (usize, usize)) {
        (   
            // Character offset
            123
            // Line/column offset
            (456, 789),
        )
    }

    fn peek(&mut self) -> TokenType {

    }

    fn peek2(&mut self) -> TokenType {

    }

    fn eat(&mut self) {
        // 
    }

    fn eat_token(&mut self, tok: &TokenType) -> Result<(), ParseError> {

    }

    fn eat_token_maybe(&mut self, tok: &TokenType) -> bool {
        self.eat_token(tok).is_ok()
    }

    fn eat_keyword(&mut self, s: &str) -> Result<(), ParseError> {

    }

    fn eat_keyword_maybe(&mut self, s: &str) -> bool {
        self.eat_token(s).is_ok()
    }

    fn eat_semicolon(&mut self) -> Result<(), ParseError> {
        match self.peek() {
            TokenType::Semicolon => {
                self.eat();
            }
            TokenType::LineTerminator => {
                self.eat();
            }
        }
        if let  = self.peek() {
            self.eat();
            Ok(())
        } else {

        }
    }

    fn parse_script(&mut self) {
        let directives = {
            let mut directives = vec![];

            while let Some(directive) = self.parse_directive()? {
                directives.push(directive);
            }

            directives
        };

        let body = {
            let mut directives: Vec<alias::StatementItem> = vec![];

            while let Some(directive) = self.parse_statement_item()? {
                directives.push(directive);
            }

            directives
        };
    }

    fn parse_directive(&mut self) -> Result<Option<ast::misc::Directive>> {
        Ok(if let TokenType::StringLiteral { raw, value } = self.peek() {
            self.eat();

            Some(ast::misc::Directive::new(raw))
        } else {
            None
        })
    }

    fn parse_statement_item(&mut self) -> Result<Option<alias::StatementItem>> {
        match self.peek() {
            TokenType::LCurly => self.parse_block_statement(),
            TokenType::Semicolon => self.parse_empty_statement(),
            TokenType::IdentifierName { value } => {
                match value {
                    // Declaration nodes
                    "function" => self.parse_function_statement(),
                    "async" => self.parse_async_function_statement(),
                    "class" => self.parse_class_statement(),
                    "let" => self.parse_let_statement(),
                    "const" => self.parse_const_statement(),

                    // Statement nodes
                    "var" => self.parse_variable_statement(),
                    "if" => self.parse_if_statement(),
                    "switch" => self.parse_switch_statement(),
                    "for" => self.parse_forish_loop_statement(),
                    "do" => self.parse_do_statement(),
                    "while" => self.parse_while_statement(),
                    "break" => self.parse_break_statement(),
                    "continue" => self.parse_continue_statement(),
                    "return" if self.flags.in_function => self.parse_return_statement(),
                    "with" => self.parse_with_statement(),
                    "throw" => self.parse_throw_statement(),
                    "try" => self.parse_try_statement(),
                    "debugger" => self.parse_debugger_statement(),
                    _ => {
                        match self.peek2() {
                            TokenType::Colon => self.parse_labelled_statement(),
                            _ => self.parse_expression_statement(),
                        }
                    }
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_block_statement(&mut self) -> Result<ast::statement::BlockStatement> {
        self.eat_token(TokenType::LCurly);

        let body = {
            let mut directives: Vec<alias::StatementItem> = vec![];

            while let Some(directive) = self.parse_statement_item()? {
                directives.push(directive);
            }

            directives
        };

        self.eat_token(TokenType::RCurly);

        ast::statement::BlockStatement::new(body);
    }
    fn parse_empty_statement(&mut self) -> Result<ast::statement::EmptyStatement> {
        // ASI is not allowed to create empty statements.
        self.eat_token(TokenType::Semicolon);

        Ok(ast::statement::EmptyStatement::new())
    }
    fn parse_function_statement(&mut self) -> Result<ast::statement::FunctionStatement> {
        self.eat_keyword("function");
        let is_generator = self.eat_token_maybe(TokenType::Star)?;

        let name = self.parse_binding_identifier()?;
        let params = self.parse_function_params()?;
        let body = self.parse_function_body()?;

        Ok(ast::statement::FunctionStatement::new(name, params, body))
    }
    fn parse_async_function_statement(&mut self) -> Result<ast::statement::FunctionStatement> {
        self.eat_keyword("async");
        self.eat_keyword("function");
        let is_generator = self.eat_token_maybe(TokenType::Star)?;

        let name = self.parse_binding_identifier()?;
        let params = self.parse_function_params()?;

        let body = self.parse_function_body()?;

        Ok(ast::statement::FunctionStatement::new(name, params, body))
    }
    fn parse_class_statement(&mut self) -> Result<ast::statement::ClassStatement> {
        let decorators = self.parse_decorator_list()?;

        self.eat_keyword("class");

        let name = self.parse_binding_identifier()?;

        let parent = if self.eat_keyword_maybe("extends") {
            Some(self.parse_expression())
        } else {
            None
        }

        let body = self.parse_class_body()?;

        Ok(ast::statement::ClassStatement::new(name, parent, body, decorators))
    }

    fn node<T: ast::misc::WithPosition>(&mut self, cb: FnOnce(&mut Self) -> T) -> T {
        let (start_offset, start_position) = self.get_position();

        let mut node = cb(&mut self);

        let (end_offset, end_position) = self.get_position();

        node.set_position(ast::misc::NodePosition {
            start: start_offset,
            end: end_offset,
            range: ast::misc::PositionRange {
                start: start_position,
                end: end_position,
            },
        });

        node
    }

    fn parse_class_body(&mut self) -> Result<ast::misc::ClassBody> {
        self.eat_token(TokenType::LCurly);

        let items = vec![];
        while !self.eat_token_maybe(TokenType::RCurly) {
            if self.eat_token_maybe(TokenType::Semicolon) {
                continue;
            }

            items.push(self.parse_class_body_item()?);
        }

        ast::misc::ClassBody::new(items)
    }
    fn parse_class_body_item(&mut self) {
        self.node(|&mut self| {
            let is_static = self.eat_keyword_maybe("static");

            let keyword_prefix = match self.peek() {
                TokenType::IdentifierName { value } => {
                    if value == "get" || value == "set" || value === "async" {
                        match self.peek2() {
                            TokenType::LParen => false,
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }

            if keyword_prefix && self.eat_keyword_maybe("get") {
                
            }
            if keyword_prefix && self.eat_keyword_maybe("set") {
                
            }
            if keyword_prefix && self.eat_keyword_maybe("async") {
                
            }
        })
    }

    fn parse_let_statement(&mut self) -> Result<ast::statement::LexicalStatement> {
        self.eat_keyword("let");

        let declarators = self.parse_declarator_list()?;

        Ok(ast::statement::LexicalStatement::new())
    }
    fn parse_const_statement(&mut self) -> Result<ast::statement::LexicalStatement> {
        self.eat_keyword("let");

        let declarators = self.parse_declarator_list()?;

        Ok(ast::statement::LexicalStatement::new())
    }
    fn parse_variable_statement(&mut self) -> Result<ast::statement::VariableStatement> {
        self.eat_keyword("var");

        let declarators = self.parse_declarator_list()?;

        Ok(ast::statement::LexicalStatement::new())
    }
    fn parse_declarator_list(&mut self) -> Result<Vec<ast::declaration::Declarator>> {
        let items = vec![];
        loop {
            let id = self.parse_pattern()?;

            let init = if self.eat_token_maybe(TokenType::Eq) {
                Some(self.parse_assignment_pattern()?)
            } else {
                None
            }

            items.push(ast::declaration::Declarator::new(id, init));

            if !self.eat_token_maybe(TokenType::Comma) {
                break;
            }
        }
        items
    }

    fn parse_if_statement(&mut self) -> Result<ast::statement::IfStatement> {
        self.eat_keyword("if");
        self.eat_token(TokenType::LParen);
        let test = self.parse_expression()?;
        self.eat_token(TokenType::RParen);

        let statement = self.parse_block_statement()?;

        let alternate = if self.eat_keyword_maybe("else") {
            Some(self.parse_statement()?);
        } else {
            None
        }

        Ok(ast::statement::IfStatement::new(test, statement, alternate))
    }
    fn parse_switch_statement(&mut self) -> Result<ast::statement::SwitchStatement> {
        self.eat_keyword("switch");
        self.eat_token(TokenType::LParen);
        let discriminant = self.parse_expression()?;
        self.eat_token(TokenType::RParen);

        self.eat_token(TokenType::LCurly);

        let mut cases = vec![];
        loop {
            let test = if self.eat_keyword_maybe("case") {
                let test = self.parse_expression()?;
                self.eat_token(TokenType::Colon);
                Some(test)
            } else if self.eat_keyword_maybe("default") {
                self.eat_token(TokenType::Colon);
                None
            } else {
                break;
            }

            let statements = self.parse_statement_items()?;

            cases.push(ast::statement::CaseStatement::new(test, statements));
        }

        self.eat_token(TokenType::RCurly);

        Ok(ast::statement::SwitchStatement::new(discriminant, cases))
    }
    fn parse_forish_loop_statement(&mut self) -> Result<ast::statement::ForStatement | ast::statement::ForInStatement | ast::statement::ForOfStatement | ast::statement::ForAwaitStatement> {
        self.eat_keyword("for");



    }
    fn parse_do_statement(&mut self) -> Result<ast::statement::DoStatement | ast::statement::DoWhileStatement> {
        self.eat_keyword("do");

        let statement = self.parse_statement();

        Ok(if self.eat_keyword_maybe("while") {
            self.eat_token(TokenType::LParen);
            let test = self.parse_expression()?;
            self.eat_token(TokenType::RParen);

            ast::statement::DoWhileStatement::new(test, statement)
        } else {
            ast::statement::DoStatement::new(test, statement)
        })
    }
    fn parse_while_statement(&mut self) -> Result<ast::statement::WhileStatement> {
        self.eat_keyword("while")?;
        self.eat_token(TokenType::LParen);
        let test = self.parse_expression()?;
        self.eat_token(TokenType::RParen);

        let statement = self.parse_statement()?;

        Ok(ast::statement::WhileStatement::new(test, statement));
    }
    fn parse_break_statement(&mut self) -> Result<ast::statement::BreakStatement> {
        self.eat_keyword("break");
        let id = self.parse_label()?;
        self.eat_semicolon()?;

        Ok(ast::statement::BreakStatement::new(id));
    }
    fn parse_continue_statement(&mut self) -> Result<ast::statement::ContinueStatement> {
        self.eat_keyword("continue");
        let id = self.parse_label()?;
        self.eat_semicolon()?;

        Ok(ast::statement::BreakStatement::new(id));
    }
    fn parse_return_statement(&mut self) -> Result<ast::statement::ReturnStatement> {
        self.eat_keyword("return");

        let argument = match self.eat_semicolon() {
            Err(_) => {
                let expr = self.parse_expression()?;
                self.eat_semicolon()?;

                Ok(expr)
            }
            _ => None,
        }

        Ok(ast::statement::ReturnStatement::new(argument));
    }
    fn parse_with_statement<T>(&mut self) -> Result<T> where T: From<ast::statement::WithStatement> {
        self.eat_keyword("with");
        self.eat_token(TokenType::LParen);
        let test = self.parse_expression()?;
        self.eat_token(TokenType::RParen);

        let statement = self.parse_statement()?;

        Ok(ast::statement::WithStatement::new(test, statement).into())
    }
    fn parse_throw_statement(&mut self) -> Result<ast::statement::ThrowStatement> {
        self.eat_keyword("throw");
        let expr = self.parse_expression()?;
        self.eat_semicolon();

        Ok(ast::statement::ThrowStatement::new(expr))
    }
    fn parse_try_statement(&mut self) -> Result<ast::statement::TryStatement> {
        self.eat_keyword("try");
        let body = self.parse_block_statement();

        let catch_clause = if self.eat_keyword("catch") {
            self.eat_token(TokenType::LParen);
            let test = self.parse_binding_pattern()?;
            self.eat_token(TokenType::RParen);

            let body = self.parse_block_statement()?;

            Some(ast::statement::CatchClause::new(test, body))
        } else {
            None
        }

        let finally_clause = if self.eat_keyword("finally") {
            Some(self.parse_block_statement()?)
        } else {
            None
        }

        Ok(ast::statement::TryStatement(body, catch_clause, finally_clause))
    }
    fn parse_debugger_statement(&mut self) -> Result<ast::statement::DebuggerStatement> {
        self.eat_keyword("debugger");
        self.eat_semicolon();

        Ok(ast::statement::DebuggerStatement::new())
    }
    fn parse_labelled_statement(&mut self) -> Result<ast::statement::LabelledStatement> {
        let id = self.parse_label()?;
        self.eat_token(TokenType::Colon);

        let body = self.parse_statement()?;

        Ok(ast::statement::LabelledStatement::new(id, body))
    }
    fn parse_expression_statement(&mut self) -> Result<ast::statement::ExpressionStatement> {
        let expr = self.parse_expression()?;
        self.eat_semicolon();

        Ok(ast::statement::ExpressionStatement::new(expr))
    }
}
