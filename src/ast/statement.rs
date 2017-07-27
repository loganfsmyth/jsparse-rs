use super::misc;
use super::alias;
use super::display;
use super::declaration;
use super::expression;
use super::misc::HasOrphanIf;
use super::misc::FirstSpecialToken;

nodes!{
	// { ... }
	pub struct BlockStatement {
		body: Vec<alias::StatementItem>,
	}
  impl display::NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::CurlyL)?;
    	for item in self.body.iter() {
    		f.node(item)?;
    	}
    	f.token(display::Token::CurlyR)
    }
  }
  impl misc::HasOrphanIf for BlockStatement {}

	// var foo, bar;
	pub struct VariableStatement {
		declarations: VariableDeclaratorList,
	}
  impl display::NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.declarations)
    }
  }
  impl misc::HasOrphanIf for VariableStatement {}

	pub enum VariableDeclaratorList {
		Declarator(VariableDeclarator),
		List(VariableDeclarator, Box<VariableDeclaratorList>),
	}
  impl display::NodeDisplay for VariableDeclaratorList {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&VariableDeclaratorList::Declarator(ref item) => f.node(item),
    		&VariableDeclaratorList::List(ref item, ref list) => {
    			f.node(item)?;
    			f.node(list)
    		}
    	}
    }
  }

	pub struct VariableDeclarator {
		id: misc::Pattern,
		init: Option<alias::Expression>,
	}
  impl display::NodeDisplay for VariableDeclarator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.id);
    	if let Some(ref init) = self.init {
    		f.token(display::Token::Eq)?;
    		f.node(init)?;
    	}
    	Ok(())
    }
  }

	// foo;
	pub struct ExpressionStatement {
		expression: alias::Expression,
	}
  impl display::NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	if let misc::SpecialToken::None = self.expression.first_special_token() {
    		f.node(&self.expression)?;
    	} else {
    		f.with_parens(|f| f.node(&self.expression))?;
    	}
    	f.token(display::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for ExpressionStatement {}

	// if () {}
	pub struct IfStatement {
		test: alias::Expression,
		consequent: Box<alias::Statement>,
		alternate: Option<Box<alias::Statement>>,
	}
  impl display::NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::If)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.test)?;
    	f.token(display::Token::ParenR)?;

    	if self.consequent.orphan_if() {
    		f.token(display::Token::CurlyL)?;
    		f.node(&self.consequent)?;
    		f.token(display::Token::CurlyR)?;
    	} else {
    		f.node(&self.consequent)?;
    	}

    	if let Some(ref stmt) = self.alternate {
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
  impl display::NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::For)?;
    	f.token(display::Token::ParenL)?;
    	if let Some(ref init) = self.init {
    		// TODO: has_in_operator usage here won't quite work
    		f.node(init)?;
    	}
    	f.token(display::Token::Semicolon)?;
    	if let Some(ref test) = self.test {
    		f.node(test)?;
    	}
    	f.token(display::Token::Semicolon)?;
    	if let Some(ref update) = self.update {
    		f.node(update)?;
    	}
    	f.token(display::Token::ParenR)?;
    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	pub enum ForInit {
		Var(VariableStatement),
		Let(declaration::LetDeclaration),
		Const(declaration::ConstDeclaration),
		Expression(alias::Expression),
	}
  impl display::NodeDisplay for ForInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ForInit::Var(ref item) => f.node(item),
    		&ForInit::Let(ref item) => f.node(item),
    		&ForInit::Const(ref item) => f.node(item),
    		&ForInit::Expression(ref item) => f.node(item),
    	}
    }
  }

	// for ... in
	pub struct ForInStatement {
		left: ForInInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl display::NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::For)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(display::Token::In)?;
    	f.node(&self.right)?;
    	f.token(display::Token::ParenR)?;

    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForInStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }
	pub enum ForInInit {
		Var(VariableDeclarator),
		Let(misc::Pattern),
		Const(misc::Pattern),
		Complex(misc::LeftHandComplexAssign),
	}
  impl display::NodeDisplay for ForInInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ForInInit::Var(ref decl) => f.node(decl),
    		&ForInInit::Let(ref pat) => f.node(pat),
    		&ForInInit::Const(ref pat) => f.node(pat),
    		&ForInInit::Complex(ref pat) => f.node(pat),
    	}
    }
  }


	// for ... of
	pub struct ForOfStatement {
		left: ForOfInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl display::NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::For)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(display::Token::Of)?;
    	f.node(&self.right)?;
    	f.token(display::Token::ParenR)?;

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
  impl display::NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::For)?;
    	f.token(display::Token::Await)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.left)?;
    	f.token(display::Token::In)?;
    	f.node(&self.right)?;
    	f.token(display::Token::ParenR)?;

    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for ForAwaitStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }
	pub enum ForOfInit {
		Var(misc::Pattern),
		Let(misc::Pattern),
		Const(misc::Pattern),
		Complex(misc::LeftHandComplexAssign),
	}
  impl display::NodeDisplay for ForOfInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ForOfInit::Var(ref pat) => f.node(pat),
    		&ForOfInit::Let(ref pat) => f.node(pat),
    		&ForOfInit::Const(ref pat) => f.node(pat),
    		&ForOfInit::Complex(ref pat) => f.node(pat),
    	}
    }
  }

	// while(...) ;
	pub struct WhileStatement {
		test: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}
  impl display::NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::While)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.test)?;
    	f.token(display::Token::ParenR)?;
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
  impl display::NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Do)?;

    	// TODO: Does this need special logic to wrap body in curlies?
    	f.node(&self.body)?;
    	f.token(display::Token::While)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.test)?;
    	f.token(display::Token::ParenR)?;
    	f.token(display::Token::Semicolon)
    }
  }

  // TODO: Does this need a special implementation?
  impl misc::HasOrphanIf for DoWhileStatement {}

	// switch (...) { ...  }
	pub struct SwitchStatement {
		discriminant: Box<alias::Expression>,
		cases: Vec<SwitchCase>,
	}
  impl display::NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Switch)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.discriminant)?;
    	f.token(display::Token::ParenR)?;
    	f.token(display::Token::CurlyL)?;
    	for c in self.cases.iter() {
    		f.node(c)?;
    	}
    	f.token(display::Token::CurlyR)
    }
  }
  impl misc::HasOrphanIf for SwitchStatement {}

	// case foo:
	// default:
	pub struct SwitchCase {
		test: Option<Box<alias::Expression>>,
		consequent: Vec<alias::StatementItem>,
	}
  impl display::NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	if let Some(ref expr) = self.test {
    		f.token(display::Token::Case)?;
    		f.node(expr)?;
    	} else {
    		f.token(display::Token::Default)?;
    	}
    	f.token(display::Token::Colon)
    }
  }

	// continue;
	// continue foo;
	pub struct ContinueStatement {
		label: Option<misc::LabelIdentifier>,
	}
  impl display::NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Continue)?;
    	if let Some(ref label) = self.label {
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
  impl display::NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Break)?;
    	if let Some(ref label) = self.label {
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
  impl display::NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Return)?;
    	if let Some(ref expr) = self.argument {
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
  impl display::NodeDisplay for WithStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::With)?;
    	f.token(display::Token::ParenL)?;
    	f.node(&self.object)?;
    	f.token(display::Token::ParenR)?;
    	f.node(&self.body)
    }
  }
  impl misc::HasOrphanIf for WithStatement {
  	fn orphan_if(&self) -> bool {
  		self.body.orphan_if()
  	}
  }

	// foo: while(false) ;
	pub struct LabelledStatement {
		label: misc::LabelIdentifier,
		body: Box<alias::Statement>,
	}
  impl display::NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.label)?;
    	f.token(display::Token::Colon)?;
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
  impl display::NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Throw)?;
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
  impl display::NodeDisplay for TryStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Try)?;
    	f.node(&self.block)?;

    	if let Some(ref c) = self.handler {
    		f.node(c)?;
    	}
    	if let Some(ref fin) = self.finalizer {
    		f.token(display::Token::Finally)?;
    		f.node(fin)?;
    	}
    	Ok(())
    }
  }
  impl misc::HasOrphanIf for TryStatement {}

	pub struct CatchClause {
		// Missing param is experimental
		param: Option<misc::Pattern>,
		body: BlockStatement,
	}
  impl display::NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Catch)?;
    	if let Some(ref pat) = self.param {
    		f.token(display::Token::ParenL)?;
    		f.node(pat)?;
    		f.token(display::Token::ParenR)?;
    	}
    	f.node(&self.body)
    }
  }

	// debugger;
	pub struct DebuggerStatement {}
  impl display::NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Debugger)?;
    	f.token(display::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for DebuggerStatement {}

	// ;
	pub struct EmptyStatement {}
  impl display::NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Semicolon)
    }
  }
  impl misc::HasOrphanIf for EmptyStatement {}
}
