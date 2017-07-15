use super::statement;
use super::declaration;
use super::expression;
use super::flow;
use super::jsx;
use super::misc;
use super::literal;

pub enum Function {
	DefaultDeclaration(declaration::ExportDefaultFunctionDeclaration),
	Declaration(declaration::FunctionDeclaration),
	Expression(expression::FunctionExpression),
	ClassMethod(misc::ClassMethod),
	ObjectMethod(expression::ObjectMethod),
	Arrow(expression::ArrowFunctionExpression),
}
pub enum Method {
	ClassMethod(misc::ClassMethod),
	ObjectMethod(expression::ObjectMethod),
}

pub enum ModuleStatementItem {
	Statement(Statement),
	Declaration(Declaration),
	Import(declaration::ImportDeclaration),
	Export(declaration::ExportDeclaration),
}

pub enum StatementItem {
	Statement(Statement),
	Declaration(Declaration),
}

pub enum Declaration {
	Function(declaration::FunctionDeclaration),
	Class(declaration::ClassDeclaration),
	Lexical(declaration::LexicalDeclaration),
	
	// Flow extension
	FlowTypeDeclareModule(flow::DeclareModuleDeclaration),
	FlowTypeDeclareFunction(flow::DeclareFunctionDeclaration),
	FlowTypeDeclareClass(flow::DeclareClassDeclaration),
	FlowTypeDeclareVariable(flow::DeclareVariableDeclaration),
	FlowTypeDeclareAlias(flow::DeclareAliasDeclaration),
	FlowTypeAlias(flow::AliasDeclaration),
}

pub enum Statement {
	Block(statement::BlockStatement),
	Variable(statement::VariableStatement),
	Empty(statement::EmptyStatement),
	Expression(statement::ExpressionStatement),
	If(statement::IfStatement),
	For(statement::ForStatement),
	ForIn(statement::ForInStatement),
	ForOf(statement::ForOfStatement),
	While(statement::WhileStatement),
	DoWhile(statement::DoWhileStatement),
	Switch(statement::SwitchStatement),
	Continue(statement::ContinueStatement),
	Break(statement::BreakStatement),
	Return(statement::ReturnStatement),
	With(statement::WithStatement),
	Labelled(statement::LabelledStatement),
	Throw(statement::ThrowStatement),
	Try(statement::TryStatement),
	Debugger(statement::DebuggerStatement),
}

pub enum Expression {
	Binding(misc::BindingIdentifier),
	This(expression::ThisExpression),
	Array(expression::ArrayExpression),
	Object(expression::ObjectExpression),
	Null(literal::Null),
	Boolean(literal::Boolean),
	Numeric(literal::Numeric),
	String(literal::String),

	Function(expression::FunctionExpression),
	Class(expression::ClassExpression),
	Regex(expression::RegularExpressionLiteral),
	Template(expression::TemplateLiteral),

	Member(expression::MemberExpression),
	SuperMember(expression::SuperMemberExpression),
	Binary(expression::BinaryExpression),
	Unary(expression::UnaryExpression),
	Update(expression::UpdateExpression),

	Call(expression::CallExpression),
	New(expression::NewExpression),

	Conditional(expression::ConditionalExpression),
	Sequence(expression::SequenceExpression),
	Arrow(expression::ArrowFunctionExpression),

	Do(expression::DoExpression),

	FlowTypeCast(flow::CastExpression),

	JSX(jsx::Element),
}



