use super::misc;
use super::alias;
use super::declaration;

// { ... }
pub struct BlockStatement {
	body: Vec<alias::StatementItem>,
	position: misc::MaybePosition,
}

// var foo, bar;
pub struct VariableStatement {
	declarations: VariableDeclaratorList,
	position: misc::MaybePosition,
}
struct VariableDeclarator {
	id: misc::Pattern,
	init: Option<alias::Expression>,
	position: misc::MaybePosition,
}
enum VariableDeclaratorList {
	Declarator(VariableDeclarator),
	List(VariableDeclarator, Box<VariableDeclaratorList>),
}


// foo;
pub struct ExpressionStatement {
	expression: alias::Expression,
	position: misc::MaybePosition,
}


// if () {}
pub struct IfStatement {
	test: alias::Expression,
	consequence: Box<alias::Statement>,
	alternate: Option<Box<alias::Statement>>,
	position: misc::MaybePosition,
}

// for( ; ; ) {}
pub struct ForStatement {
	init: Option<ForInit>,
	test: Option<Box<alias::Expression>>,
	update: Option<Box<alias::Expression>>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}
enum ForInit {
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
	position: misc::MaybePosition,
}
enum ForInInit {
	Variable(VariableDeclarator),
	Lexical(declaration::LexicalDeclarator),
	Pattern(misc::Pattern),
	Expression(Box<alias::Expression>), // May result in runtime errors, even if it parses
}

// for ... of
pub struct ForOfStatement {
	left: ForOfInit,
	right: Box<alias::Expression>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}
// for await .. of
pub struct ForAwaitStatement {
	left: ForOfInit,
	right: Box<alias::Expression>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}
enum ForOfInit {
	Variable(misc::Pattern),
	Lexical(misc::Pattern),
	Pattern(misc::Pattern),
	Expression(Box<alias::Expression>),
}

// while(...) ;
pub struct WhileStatement {
	test: Box<alias::Expression>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}

// do ; while(...) ;
pub struct DoWhileStatement {
	test: Box<alias::Expression>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}

// switch (...) { ...  }
pub struct SwitchStatement {
	discriminant: Box<alias::Expression>,
	cases: Vec<SwitchCase>,
	position: misc::MaybePosition,
}

// case foo:
// default:
pub struct SwitchCase {
	test: Option<Box<alias::Expression>>,
	consequent: Vec<alias::StatementItem>,
	position: misc::MaybePosition,
}

// continue;
// continue foo;
pub struct ContinueStatement {
	label: Option<misc::LabelIdentifier>,
	position: misc::MaybePosition,
}

// break;
// break foo;
pub struct BreakStatement {
	label: Option<misc::LabelIdentifier>,
	position: misc::MaybePosition,
}

// return;
// return foo;
pub struct ReturnStatement {
	argument: Option<Box<alias::Expression>>,
	position: misc::MaybePosition,
}

// with(...) ;
pub struct WithStatement {
	object: Box<alias::Expression>,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}

// foo: while(false) ;
pub struct LabelledStatement {
	label: misc::LabelIdentifier,
	body: Box<alias::Statement>,
	position: misc::MaybePosition,
}

// throw foo;
pub struct ThrowStatement {
	argument: Box<alias::Expression>,
	position: misc::MaybePosition,
}

// try {} catch(foo) {}
// try {} catch(foo) {} finally {}
// try {} finally {}
pub struct TryStatement {
	block: BlockStatement,
	handler: Option<CatchClause>,
	finalizer: Option<BlockStatement>,
	position: misc::MaybePosition,
}
struct CatchClause {
	param: misc::Pattern,
	body: BlockStatement,
	position: misc::MaybePosition,
}

// debugger;
pub struct DebuggerStatement {
	position: misc::MaybePosition,
}

// ;
pub struct EmptyStatement {
	position: misc::MaybePosition,
}
