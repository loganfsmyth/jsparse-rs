use super::misc;
use super::alias;
use super::declaration;

nodes!{
	// { ... }
	pub struct BlockStatement {
		body: Vec<alias::StatementItem>,
	}

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

	// foo;
	pub struct ExpressionStatement {
		expression: alias::Expression,
	}

	// if () {}
	pub struct IfStatement {
		test: alias::Expression,
		consequence: Box<alias::Statement>,
		alternate: Option<Box<alias::Statement>>,
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
		Pattern(misc::Pattern),
		Expression(alias::Expression),
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

	// for ... of
	pub struct ForOfStatement {
		left: ForOfInit,
		right: Box<alias::Expression>,
		body: Box<alias::Statement>,
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

	// while(...) ;
	pub struct WhileStatement {
		test: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}

	// do ; while(...) ;
	pub struct DoWhileStatement {
		test: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}

	// switch (...) { ...  }
	pub struct SwitchStatement {
		discriminant: Box<alias::Expression>,
		cases: Vec<SwitchCase>,
	}

	// case foo:
	// default:
	pub struct SwitchCase {
		test: Option<Box<alias::Expression>>,
		consequent: Vec<alias::StatementItem>,
	}

	// continue;
	// continue foo;
	pub struct ContinueStatement {
		label: Option<misc::LabelIdentifier>,
	}

	// break;
	// break foo;
	pub struct BreakStatement {
		label: Option<misc::LabelIdentifier>,
	}

	// return;
	// return foo;
	pub struct ReturnStatement {
		argument: Option<Box<alias::Expression>>,
	}

	// with(...) ;
	pub struct WithStatement {
		object: Box<alias::Expression>,
		body: Box<alias::Statement>,
	}

	// foo: while(false) ;
	pub struct LabelledStatement {
		label: misc::LabelIdentifier,
		body: Box<alias::Statement>,
	}

	// throw foo;
	pub struct ThrowStatement {
		argument: Box<alias::Expression>,
	}

	// try {} catch(foo) {}
	// try {} catch(foo) {} finally {}
	// try {} finally {}
	pub struct TryStatement {
		block: BlockStatement,
		handler: Option<CatchClause>,
		finalizer: Option<BlockStatement>,
	}
	pub struct CatchClause {
		param: misc::Pattern,
		body: BlockStatement,
	}

	// debugger;
	pub struct DebuggerStatement {
	}

	// ;
	pub struct EmptyStatement {
	}
}
