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
macro_rules! try_token {
    ($self:expr, $p:path) => {
        if let None = $self.peek(|tok| match tok {
            &$p => {
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
macro_rules! try_identifier {
    ($self:expr) => {
        $self.peek(
            |tok| if let &TokenType::IdentifierName { ref value, ref raw } = tok { Some((value.clone(), raw.clone())) } else { None }
        )
    }
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
macro_rules! try_if_token {
    ($self:expr, $p:path) => {
        if let Some(_) = $self.peek(|tok| match tok {
            &$p => {
                Some(())
            }
            _ => {
                None
            }
        }) {
            true
        } else {
            false
        }
    };
}

macro_rules! eat_token {
    ($self:expr, $p:path) => {
        if let Some(err) = $self.peek(|tok| match tok {
            &$p => {
                Some(None)
            }
            _ => {
                Some(Some(Err(ParseError::new(format!("unexpected token {:?}", tok)))))
            }
        }).unwrap() {
            return err;
        }
    };
}
macro_rules! eat_keyword {
    ($self:expr, $e:expr) => {
        if let Some(err) = $self.peek(|tok| match tok {
            &TokenType::IdentifierName { ref value, .. } if value == $e => {
                Some(None)
            }
            _ => {
                Some(Some(Err(ParseError::new(format!("unexpected token {:?}", tok)))))
            }
        }).unwrap() {
            return err;
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
        try_token!(self, TokenType::LCurly);

        let body = {
            let mut items: Vec<ast::alias::StatementItem> = vec![];

            while let Some(item) = self.parse_statement_item()? {
                items.push(item);
            }

            items
        };

        eat_token!(self, TokenType::RCurly);

        Ok(Some(ast::statement::BlockStatement {
            body,
            position: None,
        }))
    }
    fn parse_empty_statement(&mut self) -> MaybeResult<ast::statement::EmptyStatement> {
        // ASI is not allowed to create empty statements.
        try_token!(self, TokenType::Semicolon);

        Ok(Some(ast::statement::EmptyStatement {
            position: None,
        }))
    }
    fn parse_function_statement(&mut self) -> MaybeResult<ast::functions::FunctionDeclaration> {
        try_keyword!(self, "function");
        let kind = if try_if_token!(self, TokenType::Star) {
            ast::functions::FunctionKind::Normal
        } else {
            ast::functions::FunctionKind::Generator
        };

        let id = eat_parse!(self, self.parse_binding_identifier()?);
        let params = self.parse_function_params()?;
        let body = self.parse_function_body()?;

        Ok(Some(ast::functions::FunctionDeclaration {
            kind,
            id,
            params,
            body,
            position: None,
        }))
    }
    fn parse_function_params(&mut self) -> Result<ast::functions::FunctionParams> {
        eat_token!(self, TokenType::LParen);
        let mut rest = None;
        let mut params = vec![];
        loop {
            if try_if_token!(self, TokenType::Ellipsis) {
                let id = self.parse_binding_pattern()?;
                rest = Some(ast::functions::FunctionRestParam {
                    id,
                    position: None,
                });
                break;
            }
            let id = self.parse_binding_pattern()?;
            let init = if try_if_token!(self, TokenType::Eq) {
                Some(self.parse_assignment_expression()?.into())
            } else {
                None
            };

            params.push(ast::functions::FunctionParam {
                decorators: vec![],
                id,
                init,
                position: None,
            });

            if !try_if_token!(self, TokenType::Comma) {
                break;
            }
        }
        eat_token!(self, TokenType::RParen);

        Ok(ast::functions::FunctionParams {
            params,
            rest,
            position: None,
        })
    }

    fn parse_function_body(&mut self) -> Result<ast::functions::FunctionBody> {
        eat_token!(self, TokenType::LCurly);
        let directives = self.parse_directives()?;
        let body = parse_list(|| self.parse_statement_item())?;
        eat_token!(self, TokenType::RCurly);

        Ok(ast::functions::FunctionBody {
            directives,
            body,
            position: None,
        })
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
        let decorators = self.parse_class_decorator_list()?;
        try_keyword!(self, "class");

        let id = eat_parse!(self, self.parse_binding_identifier()?);

        let extends = if try_if_keyword!(self, "extends") {
            Some(self.parse_left_hand_expression()?.into())
        } else {
            None
        };

        let body = self.parse_class_body()?;

        Ok(Some(ast::classes::ClassDeclaration {
            decorators,
            id,
            extends,
            body,
            position: None,
        }))
    }

    fn parse_class_body(&mut self) -> Result<ast::classes::ClassBody> {
        let mut items = vec![];

        eat_token!(self, TokenType::LCurly);
        while let Some(item) = self.parse_class_body_item()? {
            items.push(item);
        }
        eat_token!(self, TokenType::RCurly);

        Ok(ast::classes::ClassBody {
            items,
            position: None,
        })
    }
    fn parse_class_body_item(&mut self) -> MaybeResult<ast::classes::ClassItem> {
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
    fn parse_class_decorator_list(&mut self) -> Result<Vec<ast::classes::ClassDecorator>> {
        unimplemented!();
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
        let test = self.parse_expression()?.into();
        eat_token!(self, TokenType::RParen);

        let consequent = eat_parse!(self, self.parse_statement()?);

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
        eat_token!(self, TokenType::LParen);
        let discriminant = self.parse_expression()?.into();
        eat_token!(self, TokenType::RParen);

        eat_token!(self, TokenType::LCurly);

        let mut cases = vec![];
        loop {
            let test = if try_if_keyword!(self, "case") {
                let test = self.parse_expression()?.into();
                eat_token!(self, TokenType::Colon);
                Some(test)
            } else if try_if_keyword!(self, "default") {
                eat_token!(self, TokenType::Colon);
                None
            } else {
                break;
            };

            cases.push(ast::statement::SwitchCase {
                test,
                consequent: parse_list(|| self.parse_statement_item())?,
                position: None,
            });
        }

        eat_token!(self, TokenType::RCurly);

        Ok(Some(ast::statement::SwitchStatement {
            discriminant,
            cases,
            position: None,
        }))
    }
    fn parse_forish_loop_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        try_keyword!(self, "for");


        unimplemented!();
        // ast::statement::ForStatement | ast::statement::ForInStatement | ast::statement::ForOfStatement | ast::statement::ForAwaitStatement
    }
    fn parse_do_while_statement(&mut self) -> MaybeResult<ast::statement::DoWhileStatement> {
        try_keyword!(self, "do");

        let body = eat_parse!(self, self.parse_statement()?);

        eat_keyword!(self, "while");

        eat_token!(self, TokenType::LParen);
        let test = self.parse_expression()?.into();
        eat_token!(self, TokenType::RParen);

        Ok(Some(ast::statement::DoWhileStatement {
            test,
            body,
            position: None,
        }))
    }
    fn parse_while_statement(&mut self) -> MaybeResult<ast::statement::WhileStatement> {
        try_keyword!(self, "while");
        eat_token!(self, TokenType::LParen);
        let test = self.parse_expression()?.into();
        eat_token!(self, TokenType::RParen);

        let body = eat_parse!(self, self.parse_statement()?);

        Ok(Some(ast::statement::WhileStatement {
            test,
            body,
            position: None,
        }))
    }
    fn parse_break_statement(&mut self) -> MaybeResult<ast::statement::BreakStatement> {
        try_keyword!(self, "break");
        let label = self.parse_label()?;
        self.eat_semicolon()?;

        Ok(Some(ast::statement::BreakStatement {
            label,
            position: None,
        }))
    }
    fn parse_continue_statement(&mut self) -> MaybeResult<ast::statement::ContinueStatement> {
        try_keyword!(self, "continue");
        let label = self.parse_label()?;
        self.eat_semicolon()?;

        Ok(Some(ast::statement::ContinueStatement {
            label,
            position: None,
        }))
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
        eat_token!(self, TokenType::LParen);
        let object = self.parse_expression()?.into();
        eat_token!(self, TokenType::RParen);

        let body = eat_parse!(self, self.parse_statement()?);

        Ok(Some(ast::statement::WithStatement {
            object,
            body,
            position: None,
        }))
    }
    fn parse_throw_statement(&mut self) -> MaybeResult<ast::statement::ThrowStatement> {
        try_keyword!(self, "throw");

        // if next token is not newline and not semicolon, parse expression


        unimplemented!();
        // let expr = self.parse_expression()?;
        // self.eat_semicolon();

        // Ok(ast::statement::ThrowStatement::new(expr))
    }
    fn parse_try_statement(&mut self) -> MaybeResult<ast::alias::Statement> {
        try_keyword!(self, "try");

        // ast::statement::TryCatchStatement | ast::statement::TryFinallyStatement | ast::statement::TryCatchFinallyStatement

        let body = eat_parse!(self, self.parse_block_statement()?);

        let catch_clause = if try_if_keyword!(self, "catch") {
            let param = if try_if_token!(self, TokenType::LParen) {
                let param = self.parse_binding_pattern()?;
                eat_token!(self, TokenType::RParen);
                Some(param)
            } else {
                None
            };

            let body = eat_parse!(self, self.parse_block_statement()?);

            Some(ast::statement::CatchClause {
                param,
                body,
                position: None,
            })
        } else {
            None
        };

        let finally_clause = if try_if_keyword!(self, "finally") {
            Some(eat_parse!(self, self.parse_block_statement()?))
        } else {
            None
        };

        if let Some(catch) = catch_clause {
            if let Some(finalizer) = finally_clause {
                Ok(Some(ast::statement::TryCatchFinallyStatement {
                    body,
                    catch,
                    finalizer,
                    position: None,
                }.into()))
            } else {
                Ok(Some(ast::statement::TryCatchStatement {
                    body,
                    catch,
                    position: None,
                }.into()))
            }
        } else {
            if let Some(finalizer) = finally_clause {
                Ok(Some(ast::statement::TryFinallyStatement {
                    body,
                    finalizer,
                    position: None,
                }.into()))
            } else {
                Err(ParseError::new(format!("'try' statements must be followed by a catch and/or finally")))
            }
        }
    }
    fn parse_debugger_statement(&mut self) -> MaybeResult<ast::statement::DebuggerStatement> {
        try_keyword!(self, "debugger");
        self.eat_semicolon()?;

        Ok(Some(ast::statement::DebuggerStatement {
            position: None,
        }))
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

    fn parse_assignment_expression(&mut self) -> Result<ast::alias::Expression> {
        unimplemented!();
    }

    fn parse_left_hand_expression(&mut self) -> Result<ast::alias::Expression> {
        unimplemented!();
    }

    fn parse_label(&mut self) -> Result<Option<ast::statement::LabelIdentifier>> {
        unimplemented!();
    }

    fn parse_binding_pattern(&mut self) -> Result<ast::patterns::BindingPattern> {
        if let Some(obj) = self.parse_binding_object()? {
            return Ok(From::from(obj));
        }
        if let Some(obj) = self.parse_binding_array()? {
            return Ok(From::from(obj));
        }
        if let Some(obj) = self.parse_binding_identifier()? {
            return Ok(From::from(obj));
        }

        Err(ParseError::new("expected binding pattern"))
    }

    fn parse_binding_identifier(&mut self) -> MaybeResult<ast::general::BindingIdentifier> {
        Ok(if let Some((value, raw)) = try_identifier!(self) {
            Some(ast::general::BindingIdentifier {
                value,
                raw,
                position: None,
            })
        } else {
            None
        })
    }

    fn parse_binding_object(&mut self) -> MaybeResult<ast::patterns::ObjectBindingPattern> {
        let mut properties = vec![];
        let mut rest = None;

        try_token!(self, TokenType::LCurly);
        loop {
            if try_if_token!(self, TokenType::Ellipsis) {
                rest = Some(eat_parse!(self, self.parse_binding_identifier()?));
                break;
            }

            if try_if_token!(self, TokenType::RCurly) {
                break;
            } else {
                properties.push(if let Some((value, raw)) = try_identifier!(self) {
                    // a,
                    // a: PAT
                    // a = 4,
                    // a: PAT = 4,

                    let pat = if try_if_token!(self, TokenType::Colon) {
                        Some(self.parse_binding_pattern()?)
                    } else {
                        None
                    };

                    let init = if try_if_token!(self, TokenType::Eq) {
                        Some(self.parse_assignment_expression()?)
                    } else {
                        None
                    };

                    if let Some(pattern) = pat {
                        ast::patterns::ObjectBindingPatternPatternProperty {
                            name: ast::general::PropertyIdentifier {
                                value,
                                raw,
                                position: None,
                            }.into(),
                            pattern,
                            init,
                            position: None,
                        }.into()
                    } else {
                        ast::patterns::ObjectBindingPatternIdentifierProperty {
                            id: ast::general::BindingIdentifier {
                                value,
                                raw,
                                position: None,
                            }.into(),
                            init,
                            position: None,
                        }.into()
                    }
                } else {
                    // [omg]: PAT
                    // [omg]: PAT = 4,
                    let name = self.parse_property_name()?;

                    eat_token!(self, TokenType::Colon);

                    let pattern = self.parse_binding_pattern()?;

                    let init = if try_if_token!(self, TokenType::Eq) {
                        Some(self.parse_assignment_expression()?)
                    } else {
                        None
                    };

                    ast::patterns::ObjectBindingPatternPatternProperty {
                        name,
                        pattern,
                        init,
                        position: None,
                    }.into()
                });

                // TODO: This token logic is wrong
                try_if_token!(self, TokenType::Comma);
            }
        }

        Ok(Some(ast::patterns::ObjectBindingPattern {
            properties,
            rest,
            position: None,
        }))
    }

    fn parse_binding_array(&mut self) -> MaybeResult<ast::patterns::ArrayBindingPattern> {
        let mut items = vec![];
        let mut rest = None;

        try_token!(self, TokenType::LParen);
        loop {
            if try_if_token!(self, TokenType::Ellipsis) {
                rest = Some(self.parse_binding_pattern()?.into());
                break;
            }

            if try_if_token!(self, TokenType::RParen) {
                break;
            } else if try_if_token!(self, TokenType::Comma) {
                items.push(None);
            } else {
                let id = self.parse_binding_pattern()?;
                let init = if try_if_token!(self, TokenType::Eq) {
                    Some(self.parse_assignment_expression()?.into())
                } else {
                    None
                };

                items.push(Some(ast::patterns::ArrayBindingPatternElement {
                    id,
                    init,
                    position: None,
                }));

                // TODO: This token logic is wrong
                try_if_token!(self, TokenType::Comma);
            }
        }

        Ok(Some(ast::patterns::ArrayBindingPattern {
            items,
            rest,
            position: None,
        }))
    }

    fn parse_property_name(&mut self) -> Result<ast::general::PropertyName> {
        unimplemented!();
    }
}

