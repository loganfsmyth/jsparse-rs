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
    	let mut f = f.allow_in();

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
    	match *self {
    		VariableDeclaratorList::Declarator(ref item) => f.node(item),
    		VariableDeclaratorList::List(ref item, ref list) => {
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
    		f.require_precedence(display::Precedence::Assignment).node(init)?;
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
    	let mut f = f.allow_in();

    	if let misc::SpecialToken::None = self.expression.first_special_token() {
    		f.require_precedence(display::Precedence::Normal).node(&self.expression)?;
    	} else {
    		f.wrap_parens().node(&self.expression)?;
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
    	{
	    	let mut f = f.allow_in();
	    	f.require_precedence(display::Precedence::Normal).node(&self.test)?;
	    }
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
    		f.node(init)?;
    	}
    	f.token(display::Token::Semicolon)?;
    	if let Some(ref test) = self.test {
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(test)?;
    	}
    	f.token(display::Token::Semicolon)?;
    	if let Some(ref update) = self.update {
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(update)?;
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
    	let mut f = f.disallow_in();

    	match *self {
    		ForInit::Var(ref item) => f.node(item),
    		ForInit::Let(ref item) => f.node(item),
    		ForInit::Const(ref item) => f.node(item),
    		ForInit::Expression(ref item) => {
				  // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
				  // so we need parens here for that too.
    			f.require_precedence(display::Precedence::Normal).node(item)
    		}
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
    	{
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(&self.right)?;
    	}
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
    	match *self {
    		ForInInit::Var(ref decl) => f.node(decl),
    		ForInInit::Let(ref pat) => f.node(pat),
    		ForInInit::Const(ref pat) => f.node(pat),
    		ForInInit::Complex(ref pat) => {
				  // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
				  // so we need parens here for that too.
    			f.node(pat)
    		}
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
    	f.require_precedence(display::Precedence::Normal).node(&self.right)?;
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
    	{
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(&self.right)?;
    	}
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
    	match *self {
    		ForOfInit::Var(ref pat) => f.node(pat),
    		ForOfInit::Let(ref pat) => f.node(pat),
    		ForOfInit::Const(ref pat) => f.node(pat),
    		ForOfInit::Complex(ref pat) => {
				  // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
				  // so we need parens here for that too.
    			f.node(pat)
    		}
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
    	{
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(&self.test)?;
    	}
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

    	f.node(&self.body)?;
    	f.token(display::Token::While)?;
    	f.token(display::Token::ParenL)?;
    	{
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(&self.test)?;
    	}
    	f.token(display::Token::ParenR)?;
    	f.token(display::Token::Semicolon)
    }
  }
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
    	{
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(&self.discriminant)?;
    	}
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
	    let mut f = f.allow_in();

    	if let Some(ref expr) = self.test {
    		f.token(display::Token::Case)?;
    		f.require_precedence(display::Precedence::Normal).node(expr)?;
    	} else {
    		f.token(display::Token::Default)?;
    	}
    	f.token(display::Token::Colon)?;

    	for stmt in self.consequent.iter() {
    		f.node(stmt)?;
    	}

    	Ok(())
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
	    	let mut f = f.allow_in();
    		f.require_precedence(display::Precedence::Normal).node(expr)?;
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
    	f.require_precedence(display::Precedence::Normal).node(&self.object)?;
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
    	let mut f = f.allow_in();
    	f.token(display::Token::Throw)?;
    	f.require_precedence(display::Precedence::Normal).node(&self.argument)?;

    	Ok(())
    }
  }
  impl misc::HasOrphanIf for ThrowStatement {}

	// try {} catch(foo) {}
  pub struct TryCatchStatement {
		block: BlockStatement,
		handler: CatchClause,
  }
  impl display::NodeDisplay for TryCatchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Try)?;
    	f.node(&self.block)?;
    	f.node(&self.handler)
    }
  }
  impl misc::HasOrphanIf for TryCatchStatement {}

	// try {} catch(foo) {} finally {}
  pub struct TryCatchFinallyStatement {
		block: BlockStatement,
		handler: CatchClause,
		finalizer: BlockStatement,
  }
  impl display::NodeDisplay for TryCatchFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Try)?;
    	f.node(&self.block)?;

    	f.node(&self.handler)?;

  		f.token(display::Token::Finally)?;
  		f.node(&self.finalizer)
    }
  }
  impl misc::HasOrphanIf for TryCatchFinallyStatement {}

	// try {} finally {}
  pub struct TryFinallyStatement {
		block: BlockStatement,
		finalizer: BlockStatement,
  }
  impl display::NodeDisplay for TryFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Try)?;
    	f.node(&self.block)?;

  		f.token(display::Token::Finally)?;
  		f.node(&self.finalizer)
    }
  }
  impl misc::HasOrphanIf for TryFinallyStatement {}

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
