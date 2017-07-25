use super::misc;
use super::alias;
use super::declaration;

nodes!{
	// { ... }
	pub struct BlockStatement {
		body: Vec<alias::StatementItem>,
	}
  impl misc::NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::CurlyL)?;
    	for item in self.body.iter() {
    		f.node(item)?;
    	}
    	f.token(misc::Token::CurlyR)?;
    }
  }
  impl misc::HasOrphanIf for BlockStatement {}

	// var foo, bar;
	pub struct VariableStatement {
		declarations: VariableDeclaratorList,
	}
	pub enum VariableDeclaratorList {
		Declarator(VariableDeclarator),
		List(VariableDeclarator, Box<VariableDeclaratorList>),
	}
	pub struct VariableDeclarator {
		id: misc::Pattern,
		init: Option<alias::Expression>,
	}
  impl misc::HasOrphanIf for VariableStatement {}

	// foo;
	pub struct ExpressionStatement {
		expression: alias::Expression,
	}
  impl misc::NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	if let misc::SpecialToken::None = self.expression.first_special_token()
    		f.node(self.expression)?;
    	} else {
    		f.with_parens(|&mut f| f.node(self.expression))?;
    	}
    	f.token(misc::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for ExpressionStatement {}

	// if () {}
	pub struct IfStatement {
		test: alias::Expression,
		consequent: Box<alias::Statement>,
		alternate: Option<Box<alias::Statement>>,
	}
  impl misc::NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::If)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(self.test)?;
    	f.token(misc::Token::ParenR)?;

    	if self.consequent.orphan_if() {
    		f.token(misc::Token::CurlyL)?;
    		f.node(self.consequent)?;
    		f.token(misc::Token::CurlyR)?;
    	} else {
    		f.node(self.consequent)?;
    	}

    	if let Some(stmt) = self.alternate {
    		f.node(stmt)?;
    	}
    	Ok(())
    }
  }
  impl misc::HasOrphanIf for IfStatement {
  	fn orphan_if(&self) -> bool {
  		self.consequent.orphan_if()
  	}
  }

	// for( ; ; ) {}
	pub struct ForStatement {
		init: Option<ForInit>,
		test: Option<Box<alias::Expression>>,
		update: Option<Box<alias::Expression>>,
		body: Box<alias::Statement>,
	}
	pub enum ForInit {
		Variable(VariableStatement),
		Lexical(declaration::LexicalDeclaration),
		// Pattern(misc::Pattern),
		Expression(alias::Expression),
	}
  impl misc::NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::For)?;
    	f.token(misc::Token::ParenL)?;
    	if let Some(init) = self.init {
    		// TODO: has_in_operator usage here won't quite work
    		f.node(init)?;
    	}
    	f.token(misc::Token::Semicolon)?;
    	if let Some(test) = self.test {
    		f.node(test)?;
    	}
    	f.token(misc::Token::Semicolon)?;
    	if let Some(update) = self.update {
    		f.node(update)?;
    	}
    	f.token(misc::Token::ParenR)?;
    	misc::NodeDisplay::fmt(self.body)
    }
  }
  impl misc::HasOrphanIf for ForStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// for ... in
	pub struct ForInStatement {
		left: ForInInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
	pub enum ForInInit {
		Variable(VariableDeclarator),
		Lexical(declaration::LexicalDeclarator),
		Pattern(misc::Pattern),

		// May result in runtime errors, even if it parses
		Expression(Box<alias::Expression>),
	}
  impl misc::NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::For)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(misc::Token::In)?;
    	f.node(&self.right)?;
    	f.token(misc::Token::ParenR)?;

    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForInStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// for ... of
	pub struct ForOfStatement {
		left: ForOfInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl misc::NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::For)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(misc::Token::Of)?;
    	f.node(&self.right)?;
    	f.token(misc::Token::ParenR)?;

    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForOfStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// for await .. of
	pub struct ForAwaitStatement {
		left: ForOfInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
	pub enum ForOfInit {
		Variable(misc::Pattern),
		Lexical(misc::Pattern),
		Pattern(misc::Pattern),
		Expression(Box<alias::Expression>),
	}
  impl misc::NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::For)?;
    	f.token(misc::Token::Await)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(misc::Token::In)?;
    	f.node(&self.right)?;
    	f.token(misc::Token::ParenR)?;

    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForAwaitStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// while(...) ;
	pub struct WhileStatement {
		test: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl misc::NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::While)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.test)?;
    	f.token(misc::Token::ParenR)?;
    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for WhileStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// do ; while(...) ;
	pub struct DoWhileStatement {
		test: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl misc::NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Do)?;

    	// TODO: Does this need special logic to wrap body in curlies?
    	f.node(&self.body)?;
    	f.token(misc::Token::While)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.test)?;
    	f.token(misc::Token::ParenR)?;
    	f.token(misc::Token::Semicolon)?;
    }
  }

  // TODO: Does this need a special implementation?
  impl misc::HasOrphanIf for DoWhileStatement {}

	// switch (...) { ...  }
	pub struct SwitchStatement {
		discriminant: Box<alias::Expression>,
		cases: Vec<SwitchCase>,
	}
  impl misc::NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Switch)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.discriminant)?;
    	f.token(misc::Token::ParenR)?;
    	f.token(misc::Token::CurlyL)?;
    	for c in self.cases.iter() {
    		f.node(c)?;
    	}
    	f.token(misc::Token::CurlyR)?;
    }
  }
  impl misc::HasOrphanIf for SwitchStatement {}

	// case foo:
	// default:
	pub struct SwitchCase {
		test: Option<Box<alias::Expression>>,
		consequent: Vec<alias::StatementItem>,
	}
  impl misc::NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	if let Some(expr) = self.test {
    		f.token(misc::Token::Case);
    		f.node(expr)?;
    	} else {
    		f.token(misc::Token::Default);
    	}
    	f.token(misc::Token::Colon);
    }
  }

	// continue;
	// continue foo;
	pub struct ContinueStatement {
		label: Option<misc::LabelIdentifier>,
	}
  impl misc::NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Continue)?;
    	if let Some(label) = self.label {
    		f.node(label)?;
    	}
    	Ok(())
    }
  }
  impl misc::HasOrphanIf for ContinueStatement {}

	// break;
	// break foo;
	pub struct BreakStatement {
		label: Option<misc::LabelIdentifier>,
	}
  impl misc::NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Break)?;
    	if let Some(label) = self.label {
    		f.node(label)?;
    	}
    	Ok(())
    }
  }
  impl misc::HasOrphanIf for BreakStatement {}

	// return;
	// return foo;
	pub struct ReturnStatement {
		argument: Option<Box<alias::Expression>>,
	}
  impl misc::NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Return)?;
    	if let Some(expr) = self.argument {
    		f.node(expr)?;
    	}
    	Ok(())
    }
  }
  impl misc::HasOrphanIf for ReturnStatement {}

	// with(...) ;
	pub struct WithStatement {
		object: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl misc::NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::With)?;
    	f.token(misc::Token::ParenL)?;
    	f.node(&self.object)?;
    	f.token(misc::Token::ParenR)?;
    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for WhileStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// foo: while(false) ;
	pub struct LabelledStatement {
		label: misc::LabelIdentifier,
		body: Box<alias::Statement>,
	}
  impl misc::NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.node(self.label)?;
    	f.token(misc::Token::Colon)?;
    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for LabelledStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// throw foo;
	pub struct ThrowStatement {
		argument: Box<alias::Expression>,
	}
  impl misc::NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Throw)?;
    	f.node(&self.argument)
    }
  }
  impl misc::HasOrphanIf for ThrowStatement {}

	// try {} catch(foo) {}
	// try {} catch(foo) {} finally {}
	// try {} finally {}
	pub struct TryStatement {
		block: BlockStatement,
		handler: Option<CatchClause>,
		finalizer: Option<BlockStatement>,

		// TODO: Include type-system checks for requiring at least handler or finalizer
	}
  impl misc::NodeDisplay for TryStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Try)?;
    	f.node(&self.block)?;

    	if let Some(c) = self.handler {
    		f.node(c)?;
    	}
    	if let Some(fin) = self.finalizer {
    		f.token(misc::Token::Finally)?;
    		f.node(fin)?;
    	}
    }
  }
  impl misc::HasOrphanIf for TryStatement {}

	pub struct CatchClause {
		// Missing param is experimental
		param: Option<misc::Pattern>,
		body: BlockStatement,
	}
  impl misc::NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Catch)?;
    	if let Some(pat) = self.param {
    		f.token(misc::Token::ParenL)?;
    		f.node(pat)?;
    		f.token(misc::Token::ParenR)?;
    	}
    	f.node(self.body)
    }
  }

	// debugger;
	pub struct DebuggerStatement {}
  impl misc::NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Debugger)?;
    	f.token(misc::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for DebuggerStatement {}

	// ;
	pub struct EmptyStatement {}
  impl misc::NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for EmptyStatement {}
}
