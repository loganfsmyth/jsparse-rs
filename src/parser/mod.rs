use std::result;
use std::fmt;
use std::error;
use std::ops;

use ast;
use tokenizer::TokenType;

pub type Result<T> = result::Result<T, ParseError>;
pub type MaybeResult<T> = Result<Option<T>>;

macro_rules! try_parse {
    ($e:expr) => {
        if let Some(val) = $e {
            return Ok(Some(From::from(val)));
        }
    };
}
macro_rules! try_keyword {
    ($self:expr, $e:expr) => {
        if let None = $self.peek(|tok| match tok {
            &TokenType::IdentifierName { ref value, .. } if value == $e => {
                Some(())
            }
            _ => {
                None
            }
        }) {
            return Ok(None);
        }
    };
}
macro_rules! try_if_keyword {
    ($self:expr, $e:expr) => {
        if let None = $self.peek(|tok| match tok {
            &TokenType::IdentifierName { ref value, .. } if value == $e => {
                Some(())
            }
            _ => {
                None
            }
        }) {
            false
        } else {
            true
        }
    };
}
// macro_rules! try_token {
//     ($self:expr, $p:ty) => {
//         $self.peek(|tok| match tok {
//             $p => {
//                 Ok(Some(()))
//             }
//             _ => {
//                 Ok(None)
//             }
//         })
//     };
// }
macro_rules! eat_token {
    ($self:expr, $p:path) => {
        if let Some(tok) = $self.peek(|tok| match tok {
            &$p => {
                None
            }
            _ => {
                Some(tok.clone())
            }
        }) {
            return Err(ParseError::new(format!("unexpected token {:?}", tok)));
        }
    };
}
macro_rules! eat_parse {
    ($self:expr, $e:expr) => {
        if let Some(val) = $e {
            From::from(val)
        } else {
            let tok = $self.peek(|tok| Some(tok.clone())).unwrap();
            return Err(ParseError::new(format!("unexpected token {:?}", tok)));
        }
    };
}


pub struct ParseError {
    message: String,

}
impl ParseError {
    fn new<T: Into<String>>(s: T) -> ParseError {
        ParseError {
            message: s.into(),
        }
    }
}
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.message[..]
    }
}



pub fn script(code: &str) -> Result<ast::root::Script> {
    let mut parser = Parser {
        code,
        module: false,
        annexb: false,

        flags: Default::default(),
        flags_stack: Vec::with_capacity(32),
    };

    if let ast::root::Ast::Script(s) = parser.parse()? {
        Ok(s)
    } else {
        unreachable!();
    }
}

// pub fn script_annexb(code: &str) -> parser::Result<ast::root::Script> {
//     if let ast::root::Ast::Script(s) = parser::parse(code, parser::ParserFlags::with_annexb())? {
//         Ok(s)
//     } else {
//         unreachable!();
//     }
// }

// pub fn module(code: &str) -> parser::Result<ast::root::Module> {
//     if let ast::root::Ast::Module(m) = parser::parse(code, parser::ParserFlags::with_module())? {
//         Ok(m)
//     } else {
//         unreachable!();
//     }
// }


// pub fn (code: T) {

// }
// pub fn with_annexb(code: T) -> Parser {
//     let mut f = Parser::new(code);
//     f.annexb = true;
//     f
// }
// pub fn with_module(code: T) -> Parser {
//     let mut f = Parser::new(code);
//     f.module = true;
//     f
// }

pub struct ExpressionContext {
    // Allow super.foo
    method: bool,

    // Allow super()
    subclass_constructor: bool,

    generator: bool,
    async: bool,
    function: bool,

    module: bool,
    strict: bool,
}

// pub fn expression(code: &str, context: &ExpressionContext) -> parser::Result<ast::alias::Expression> {
// }




pub struct EvalContext {
    // Allow super.foo
    method: bool,

    // Allow super()
    subclass_constructor: bool,

    generator: bool,
    async: bool,
    function: bool,

    module: bool,
    strict: bool,
}

pub fn eval(code: &str, context: &EvalContext) {}




pub struct Parser<'a> {
    code: &'a str,
    // Static flags for this parser pass
    module: bool,
    annexb: bool,

    flags: ParserStateFlags,
    flags_stack: Vec<ParserStateFlags>,
}

#[derive(Clone, Copy, Debug, Default)]
struct ParserStateFlags {
    // "yield" toggle
    generator: bool,
    // "await" toggle
    async: bool,
    // "return" toggle
    function: bool,
    // "in" toggle
    for_in: bool,
    strict: bool,
}


fn parse_list<R, F>(mut f: F) -> Result<Vec<R>>
where
    F: FnMut() -> Result<Option<R>>
{
    let mut items = vec![];
    while let Some(item) = f()? {
        items.push(item);
    }
    Ok(items)
}

struct ParserFlagsLock<'a, 'p: 'a> {
    parser: &'a mut Parser<'p>,
}
impl<'a, 'p: 'a> ParserFlagsLock<'a, 'p> {
    fn new(parser: &'a mut Parser<'p>) -> ParserFlagsLock<'a, 'p> {
        parser.flags_stack.push(parser.flags);
        ParserFlagsLock { parser }
    }
}
impl<'a, 'p: 'a> Drop for ParserFlagsLock<'a, 'p> {
    fn drop(&mut self) {
        self.parser.flags = self.parser.flags_stack.pop().unwrap();
    }
}
impl<'a, 'p: 'a> ops::Deref for ParserFlagsLock<'a, 'p> {
    type Target = Parser<'p>;

    fn deref(&self) -> &Parser<'p> {
        self.parser
    }
}
impl<'a, 'p: 'a> ops::DerefMut for ParserFlagsLock<'a, 'p> {
    fn deref_mut(&mut self) -> &mut Parser<'p> {
        self.parser
    }
}


impl<'a> Parser<'a> {

    fn peek<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&TokenType) -> Option<R> + 'static
    {
        unimplemented!();
        // if let Some(result) = f() {
        //     self.skip();
        //     Some(result)
        // } else {
        //     None
        // }
    }

    fn next(&mut self) -> TokenType {
        unimplemented!();
    }

    fn skip(&mut self) {
        unimplemented!();
    }

    fn eat_semicolon(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn cache<'l>(&'l mut self) -> ParserFlagsLock<'l, 'a> {
        ParserFlagsLock::new(self)
    }

    fn parse(&mut self) -> Result<ast::root::Ast> {
        let directives = self.parse_directives()?;
        for directive in &directives {
            match directive.value.value.as_str() {
                "\"use module\"" | "'use module'" => {
                    self.module = true;
                    self.flags.strict = true;
                }
                _ => {}
            }
        }

        Ok(if self.module {
            ast::root::Module {
                directives,
                body: parse_list(|| self.parse_module_statement_item())?,
                position: None,
            }.into()
        } else {
            ast::root::Script {
                directives,
                body: parse_list(|| self.parse_statement_item())?,
                position: None,
            }.into()
        })
    }

    fn parse_directives(&mut self) -> Result<Vec<ast::functions::Directive>> {
        let directives = parse_list(|| self.parse_directive())?;
        for directive in &directives {
            match directive.value.value.as_str() {
                "\"use strict\"" | "'use strict'" => {
                    self.flags.strict = true;
                }
                _ => {}
            }

        }
        Ok(directives)
    }

    fn parse_directive(&mut self) -> MaybeResult<ast::functions::Directive> {
        Ok(if let Some(value) = self.parse_directive_literal()? {
            self.eat_semicolon()?;

            Some(ast::functions::Directive {
                value,
                position: None,
            })
        } else {
            None
        })
    }

    fn parse_directive_literal(&mut self) -> MaybeResult<ast::functions::DirectiveLiteral> {
        Ok(if let Some((value, raw)) = self.peek(
            |tok| if let &TokenType::StringLiteral { ref value, ref raw } = tok { Some((value.clone(), raw.clone())) } else { None }
        ) {
            Some(ast::functions::DirectiveLiteral {
                value: raw.unwrap(),
                position: None,
            })
        } else {
            None
        })
    }

    fn parse_module_statement_item(&mut self) -> MaybeResult<ast::alias::ModuleStatementItem> {
        try_parse!(self.parse_import()?);
        try_parse!(self.parse_export()?);
        try_parse!(self.parse_statement_item()?);

        unreachable!();
    }

    fn parse_statement_item(&mut self) -> MaybeResult<ast::alias::StatementItem> {
        try_parse!(self.parse_function_statement()?);
        try_parse!(self.parse_async_function_statement()?);
        try_parse!(self.parse_class_statement()?);
        try_parse!(self.parse_let_statement()?);
        try_parse!(self.parse_const_statement()?);
        try_parse!(self.parse_empty_statement()?);
        try_parse!(self.parse_block_statement()?);
        try_parse!(self.parse_variable_statement()?);
        try_parse!(self.parse_if_statement()?);
        try_parse!(self.parse_switch_statement()?);
        try_parse!(self.parse_forish_loop_statement()?);
        try_parse!(self.parse_do_while_statement()?);
        try_parse!(self.parse_while_statement()?);
        try_parse!(self.parse_break_statement()?);
        try_parse!(self.parse_continue_statement()?);
        try_parse!(self.parse_return_statement()?);
        try_parse!(self.parse_with_statement()?);
        try_parse!(self.parse_throw_statement()?);
        try_parse!(self.parse_try_statement()?);
        try_parse!(self.parse_debugger_statement()?);
        try_parse!(self.parse_labelled_statement()?);
        try_parse!(self.parse_expression_statement()?);

        Err(ParseError::new(format!("Unexpected token: {:?}", self.peek(|tok| Some(tok.clone())))))
    }

    fn parse_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        try_parse!(self.parse_empty_statement()?);
        try_parse!(self.parse_block_statement()?);
        try_parse!(self.parse_variable_statement()?);
        try_parse!(self.parse_if_statement()?);
        try_parse!(self.parse_switch_statement()?);
        try_parse!(self.parse_forish_loop_statement()?);
        try_parse!(self.parse_do_while_statement()?);
        try_parse!(self.parse_while_statement()?);
        try_parse!(self.parse_break_statement()?);
        try_parse!(self.parse_continue_statement()?);
        try_parse!(self.parse_return_statement()?);
        try_parse!(self.parse_with_statement()?);
        try_parse!(self.parse_throw_statement()?);
        try_parse!(self.parse_try_statement()?);
        try_parse!(self.parse_debugger_statement()?);
        try_parse!(self.parse_labelled_statement()?);
        try_parse!(self.parse_expression_statement()?);

        Err(ParseError::new(format!("Unexpected token: {:?}", self.peek(|tok| Some(tok.clone())))))
    }

    fn parse_import(&mut self) -> MaybeResult<ast::alias::ModuleStatementItem> {
        try_keyword!(self, "import");
        if let Some(_) = self.peek(|tok| match tok { &TokenType::LParen => { Some(()) } _ => { None }}) {

        }

        unimplemented!();
    }
    fn parse_export(&mut self) -> MaybeResult<ast::alias::ModuleStatementItem> {
        try_keyword!(self, "export");
        unimplemented!();
    }

    fn parse_block_statement(&mut self) -> MaybeResult<ast::statement::BlockStatement> {
        unimplemented!();
        // self.eat_token(TokenType::LCurly);

        // let body = {
        //     let mut directives: Vec<ast::alias::StatementItem> = vec![];

        //     while let Some(directive) = self.parse_statement_item()? {
        //         directives.push(directive);
        //     }

        //     directives
        // };

        // self.eat_token(TokenType::RCurly);

        // ast::statement::BlockStatement::new(body);
    }
    fn parse_empty_statement(&mut self) -> MaybeResult<ast::statement::EmptyStatement> {
        unimplemented!();
        // ASI is not allowed to create empty statements.
        // self.eat_token(TokenType::Semicolon);

        // Ok(ast::statement::EmptyStatement::new())
    }
    fn parse_function_statement(&mut self) -> MaybeResult<ast::functions::FunctionDeclaration> {
        try_keyword!(self, "function");
        unimplemented!();
        // self.eat_keyword("function");
        // let is_generator = self.eat_token_maybe(TokenType::Star)?;

        // let name = self.parse_binding_identifier()?;
        // let params = self.parse_function_params()?;
        // let body = self.parse_function_body()?;

        // Ok(ast::functions::FunctionDeclaration::new(name, params, body))
    }
    fn parse_async_function_statement(&mut self) -> MaybeResult<ast::functions::FunctionDeclaration> {
        try_keyword!(self, "async");
        unimplemented!();
        // self.eat_keyword("async");
        // self.eat_keyword("function");
        // let is_generator = self.eat_token_maybe(TokenType::Star)?;

        // let name = self.parse_binding_identifier()?;
        // let params = self.parse_function_params()?;

        // let body = self.parse_function_body()?;

        // Ok(ast::functions::FunctionDeclaration::new(name, params, body))
    }
    fn parse_class_statement(&mut self) -> MaybeResult<ast::classes::ClassDeclaration> {
        try_keyword!(self, "class");
        unimplemented!();
        // let decorators = self.parse_decorator_list()?;

        // self.eat_keyword("class");

        // let name = self.parse_binding_identifier()?;

        // let parent = if self.eat_keyword_maybe("extends") {
        //     Some(self.parse_expression())
        // } else {
        //     None
        // };

        // let body = self.parse_class_body()?;

        // Ok(ast::classes::ClassDeclaration::new(name, parent, body, decorators))
    }

    fn parse_class_body(&mut self) -> Result<ast::classes::ClassBody> {
        unimplemented!();
        // self.eat_token(TokenType::LCurly);

        // let items = vec![];
        // while !self.eat_token_maybe(TokenType::RCurly) {
        //     if self.eat_token_maybe(TokenType::Semicolon) {
        //         continue;
        //     }

        //     items.push(self.parse_class_body_item()?);
        // }

        // ast::classes::ClassBody::new(items)
    }
    fn parse_class_body_item(&mut self) {
        unimplemented!();
        // self.node(|&mut self| {
        //     let is_static = self.eat_keyword_maybe("static");

        //     let keyword_prefix = match self.peek() {
        //         TokenType::IdentifierName { value } => {
        //             if value == "get" || value == "set" || value === "async" {
        //                 match self.peek2() {
        //                     TokenType::LParen => false,
        //                     _ => true,
        //                 }
        //             } else {
        //                 false
        //             }
        //         }
        //         _ => false,
        //     }

        //     if keyword_prefix && self.eat_keyword_maybe("get") {

        //     }
        //     if keyword_prefix && self.eat_keyword_maybe("set") {

        //     }
        //     if keyword_prefix && self.eat_keyword_maybe("async") {

        //     }
        // })
    }

    fn parse_let_statement(&mut self) -> MaybeResult<ast::statement::LetDeclaration> {
        try_keyword!(self, "let");
        unimplemented!();
        // self.eat_keyword("let");

        // let declarators = self.parse_declarator_list()?;

        // Ok(ast::statement::LetDeclaration::new())
    }
    fn parse_const_statement(&mut self) -> MaybeResult<ast::statement::ConstDeclaration> {
        try_keyword!(self, "const");
        unimplemented!();
        // self.eat_keyword("let");

        // let declarators = self.parse_declarator_list()?;

        // Ok(ast::statement::ConstDeclaration::new())
    }
    fn parse_variable_statement(&mut self) -> MaybeResult<ast::statement::VariableStatement> {
        try_keyword!(self, "var");
        unimplemented!();
        // self.eat_keyword("var");

        // let declarators = self.parse_declarator_list()?;

        // Ok(ast::statement::VariableStatement::new())
    }
    // fn parse_declarator_list(&mut self) -> Result<Vec<ast::declaration::Declarator>> {
    //     unimplemented!();
        // let items = vec![];
        // loop {
        //     let id = self.parse_pattern()?;

        //     let init = if self.eat_token_maybe(TokenType::Eq) {
        //         Some(self.parse_assignment_pattern()?)
        //     } else {
        //         None
        //     };

        //     // items.push(ast::declaration::Declarator::new(id, init));

        //     if !self.eat_token_maybe(TokenType::Comma) {
        //         break;
        //     }
        // }
        // items
    // }

    fn parse_if_statement(&mut self) -> MaybeResult<ast::statement::IfStatement> {
        try_keyword!(self, "if");
        eat_token!(self, TokenType::LParen);
        let test = self.parse_expression()?;
        eat_token!(self, TokenType::RParen);

        let consequent = eat_parse!(self, self.parse_block_statement()?);

        let alternate = if try_if_keyword!(self, "else") {
            Some(eat_parse!(self, self.parse_statement()?))
        } else {
            None
        };

        Ok(Some(ast::statement::IfStatement {
            test,
            consequent,
            alternate,
            position: None,
        }))
    }
    fn parse_switch_statement(&mut self) -> MaybeResult<ast::statement::SwitchStatement> {
        try_keyword!(self, "switch");
        unimplemented!();
        // self.eat_keyword("switch");
        // self.eat_token(TokenType::LParen);
        // let discriminant = self.parse_expression()?;
        // self.eat_token(TokenType::RParen);

        // self.eat_token(TokenType::LCurly);

        // let mut cases = vec![];
        // loop {
        //     let test = if self.eat_keyword_maybe("case") {
        //         let test = self.parse_expression()?;
        //         self.eat_token(TokenType::Colon);
        //         Some(test)
        //     } else if self.eat_keyword_maybe("default") {
        //         self.eat_token(TokenType::Colon);
        //         None
        //     } else {
        //         break;
        //     };

        //     let statements = self.parse_statement_items()?;

        //     cases.push(ast::statement::SwitchCase::new(test, statements));
        // }

        // self.eat_token(TokenType::RCurly);

        // Ok(ast::statement::SwitchStatement::new(discriminant, cases))
    }
    fn parse_forish_loop_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        try_keyword!(self, "for");
        unimplemented!();
        // self.eat_keyword("for");
        // ast::statement::ForStatement | ast::statement::ForInStatement | ast::statement::ForOfStatement | ast::statement::ForAwaitStatement



    }
    fn parse_do_while_statement(&mut self) -> MaybeResult<ast::statement::DoWhileStatement> {
        try_keyword!(self, "do");
        unimplemented!();
        // self.eat_keyword("do");

        // let statement = self.parse_statement();

        // Ok(if self.eat_keyword_maybe("while") {
        //     self.eat_token(TokenType::LParen);
        //     let test = self.parse_expression()?;
        //     self.eat_token(TokenType::RParen);

        //     ast::statement::DoWhileStatement::new(test, statement)
        // } else {
        //     ast::statement::DoStatement::new(test, statement)
        // })
    }
    fn parse_while_statement(&mut self) -> MaybeResult<ast::statement::WhileStatement> {
        try_keyword!(self, "while");
        unimplemented!();
        // self.eat_keyword("while")?;
        // self.eat_token(TokenType::LParen);
        // let test = self.parse_expression()?;
        // self.eat_token(TokenType::RParen);

        // let statement = self.parse_statement()?;

        // Ok(ast::statement::WhileStatement::new(test, statement));
    }
    fn parse_break_statement(&mut self) -> MaybeResult<ast::statement::BreakStatement> {
        try_keyword!(self, "break");
        unimplemented!();
        // self.eat_keyword("break");
        // let id = self.parse_label()?;
        // self.eat_semicolon()?;

        // Ok(ast::statement::BreakStatement::new(id));
    }
    fn parse_continue_statement(&mut self) -> MaybeResult<ast::statement::ContinueStatement> {
        try_keyword!(self, "break");
        unimplemented!();
        // self.eat_keyword("continue");
        // let id = self.parse_label()?;
        // self.eat_semicolon()?;

        // Ok(ast::statement::BreakStatement::new(id));
    }
    fn parse_return_statement(&mut self) -> MaybeResult<ast::statement::ReturnStatement> {
        try_keyword!(self, "return");

        if !self.flags.function {
            return Err(ParseError::new("'return' only allowed inside functions"));
        }
        unimplemented!();

        // self.eat_keyword("return");

        // let argument = match self.eat_semicolon() {
        //     Err(_) => {
        //         let expr = self.parse_expression()?;
        //         self.eat_semicolon()?;

        //         Ok(expr)
        //     }
        //     _ => None,
        // };

        // Ok(ast::statement::ReturnStatement::new(argument))
    }
    fn parse_with_statement(&mut self) -> MaybeResult<ast::statement::WithStatement> {
        try_keyword!(self, "with");
        unimplemented!();
        // self.eat_keyword("with");
        // self.eat_token(TokenType::LParen);
        // let test = self.parse_expression()?;
        // self.eat_token(TokenType::RParen);

        // let statement = self.parse_statement()?;

        // Ok(ast::statement::WithStatement::new(test, statement).into())
    }
    fn parse_throw_statement(&mut self) -> MaybeResult<ast::statement::ThrowStatement> {
        try_keyword!(self, "throw");
        unimplemented!();
        // self.eat_keyword("throw");
        // let expr = self.parse_expression()?;
        // self.eat_semicolon();

        // Ok(ast::statement::ThrowStatement::new(expr))
    }
    fn parse_try_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        try_keyword!(self, "try");

        // ast::statement::TryCatchStatement | ast::statement::TryFinallyStatement | ast::statement::TryCatchFinallyStatement
        unimplemented!();
        // self.eat_keyword("try");
        // let body = self.parse_block_statement();

        // let catch_clause = if self.eat_keyword("catch") {
        //     self.eat_token(TokenType::LParen);
        //     let test = self.parse_binding_pattern()?;
        //     self.eat_token(TokenType::RParen);

        //     let body = self.parse_block_statement()?;

        //     Some(ast::statement::CatchClause::new(test, body))
        // } else {
        //     None
        // };

        // let finally_clause = if self.eat_keyword("finally") {
        //     Some(self.parse_block_statement()?)
        // } else {
        //     None
        // };

        // Ok(ast::statement::TryCatchStatement::new(body, catch_clause, finally_clause))
    }
    fn parse_debugger_statement(&mut self) -> MaybeResult<ast::statement::DebuggerStatement> {
        try_keyword!(self, "debugger");
        unimplemented!();
        // self.eat_keyword("debugger");
        // self.eat_semicolon();

        // Ok(ast::statement::DebuggerStatement::new())
    }
    fn parse_labelled_statement(&mut self) -> MaybeResult<ast::statement::LabelledStatement> {
        unimplemented!();
        // let id = self.parse_label()?;
        // self.eat_token(TokenType::Colon);

        // let body = self.parse_statement()?;

        // Ok(ast::statement::LabelledStatement::new(id, body))
    }
    fn parse_expression_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        unimplemented!();
        // let expr = self.parse_expression()?;
        // self.eat_semicolon();

        // Ok(ast::statement::ExpressionStatement::new(expr))
    }



    fn parse_expression(&mut self) -> Result<ast::alias::Expression> {
        unimplemented!();
    }
}

