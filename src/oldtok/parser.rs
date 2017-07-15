mod node;

const err = "AHHH";

#[derive(Debug)]
struct ParserError {

}

type Result<T> = std::Result<AST, ParserError>;

type MaybeResult<T> = Result<Option<T>>;

impl Error for ParserError {
    fn description(&self) -> &str {
    	&err
    }
}

struct Parser<T: Iterator<Item=Token>> {
	it: Peekable<T>,
}

impl<T: Iterator<Item=Token>> Parser<T> {
	fn new(it: T) -> Parser {
		Parser {
			it,
		}
	}

	fn end(&mut self) -> Result<AST> {

	}

	fn write(&mut self, chars: &str) -> Result<()> {



	}

	fn script(&mut self) -> Result<Script> {
		Script {
			body: self.statement_list(false, false, false),
		}
	}

	fn statement_list(&mut self, yld: bool, await: bool, ret: bool) -> Result<Vec<ScriptStatementItem>> {
		let mut items = vec![];
		while let Some(item) = statement_list_item(self, yld, await, ret)? {
			items.push(item);
		}
		return items;
	}

	fn statement_list_item(&mut self, yld: bool, await: bool, ret: bool) -> MaybeResult<ScriptStatementItem> {
		let token = self.it.peek();
		Ok(Some(From::from(match token {
			Token::RCurly => self.block_statement(yld, await, ret)?,
			Token::Ident("var") => self.variable_statement(yld, await)?,
			Token::Semi => node::EmptyStatement,
			Token::Ident("if") => self.if_statement(yld, await, ret)?,

			Token::Ident("for") if ret => self.for_statement(yld, await, ret)?,
			Token::Ident("while") if ret => self.while_statement(yld, await, ret)?,
			Token::Ident("do") if ret => self.do_statement(yld, await, ret)?,
			Token::Ident("switch") if ret => self.do_statement(yld, await, ret)?,

			Token::Ident("break") => self.break_statement(yld, await)?,
			Token::Ident("continue") => self.continue_statement(yld, await)?,
			Token::Ident("return") if ret => self.return_statement(yld, await)?,

			Token::Ident("with") => self.with_statement(yld, await, ret)?,
			Token::Ident("throw") => self.throw_statement(yld, await)?,
			Token::Ident("try") => self.try_statement(yld, await)?,
			Token::Ident("debugger") => self.debugger_statement(yld, await)?,

			// expression statement, or labelled statement
			_ => => self.return_statement(yld, await)?,
		})))
	}

	fn statement(&mut self, yld: bool, await: bool, ret: bool) -> MaybeResult<node::Statement> {
		let token = self.it.peek();
		Ok(Some(From::from(match token {
			Token::RCurly => self.block_statement(yld, await, ret)?,
			Token::Ident("var") => self.variable_statement(yld, await)?,
			Token::Semi => node::EmptyStatement,
			Token::Ident("if") => self.if_statement(yld, await, ret)?,

			Token::Ident("for") if ret => self.for_statement(yld, await, ret)?,
			Token::Ident("while") if ret => self.while_statement(yld, await, ret)?,
			Token::Ident("do") if ret => self.do_statement(yld, await, ret)?,
			Token::Ident("switch") if ret => self.do_statement(yld, await, ret)?,

			Token::Ident("break") => self.break_statement(yld, await)?,
			Token::Ident("continue") => self.continue_statement(yld, await)?,
			Token::Ident("return") if ret => self.return_statement(yld, await)?,

			Token::Ident("with") => self.with_statement(yld, await, ret)?,
			Token::Ident("throw") => self.throw_statement(yld, await)?,
			Token::Ident("try") => self.try_statement(yld, await)?,
			Token::Ident("debugger") => self.debugger_statement(yld, await)?,
			Token::EOF => Ok(None),

			// expression statement, or labelled statement
			_ => => self.expression_or_labelled_statement(yld, await)?,
		})))
	}

	fn declaration(&mut self, yld: bool, await: bool) -> MaybeResult<node::Declaration> {
		let token = self.it.peek();
		Ok(Some(From::from(match token {
			Token::Ident("async") => self.function_declaration(yld, await, false)?,
			Token::Ident("function") => self.function_declaration(yld, await, false)?,
			Token::Ident("class") => self.class_declaration(yld, await, false)?,
			Token::Ident("let") => self.lexical_declaration(true, yld, await)?,
			Token::Ident("const") => self.lexical_declaration(true, yld, await)?,
			_ => Result(None),
		})))
	}

	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
	fn block_statement(&mut self, yld: bool, await: bool, ret: bool) -> Result<node::BlockStatement> {

	}
}
