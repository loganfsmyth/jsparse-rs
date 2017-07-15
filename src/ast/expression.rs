use std::string;
use super::alias;
use super::misc;
use super::flow;
use super::literal;
use super::statement;

// this
pub struct ThisExpression {
	position: misc::MaybePosition,
}

// [1, 2, 3, ...4]
pub struct ArrayExpression {
	elements: Vec<Option<Box<alias::Expression>>>,
	spread: Option<Box<alias::Expression>>,
	position: misc::MaybePosition,
}

// {a: 1, ...b}
pub struct ObjectExpression {
	properties: Vec<ObjectProperty>,
	spread: Option<Box<alias::Expression>>, // experimental
	position: misc::MaybePosition,
}
enum ObjectProperty {
	Method(ObjectMethod),
	Value(misc::PropertyId, Box<alias::Expression>),
}
pub struct ObjectMethod {
	kind: misc::MethodKind,
	id: misc::PropertyId,
	params: misc::FunctionParams,
	body: misc::FunctionBody,
	fn_kind: misc::FunctionKind,
	position: misc::MaybePosition,

	return_type: Option<Box<flow::Annotation>>,
}

// (function(){})
pub struct FunctionExpression {
	decorators: Vec<misc::Decorator>, // experimental
	id: Option<misc::BindingIdentifier>,
	params: misc::FunctionParams,
	body: misc::FunctionBody,
	fn_kind: misc::FunctionKind,
	position: misc::MaybePosition,

	// Flow extension
	type_parameters: Option<flow::Parameters>,
	return_type: Option<Box<flow::Annotation>>,
}

// (class {})
pub struct ClassExpression {
	decorators: Vec<misc::Decorator>, // experimental
	id: Option<misc::BindingIdentifier>,
	extends: Option<Box<alias::Expression>>,
	implements: Option<flow::BindingIdentifierAnnotationList>,
	body: misc::ClassBody,
	position: misc::MaybePosition,

	// Flow extension
	type_parameters: Option<flow::Parameters>,
}

// /foo/g
pub struct RegularExpressionLiteral {
	value: string::String,
	flags: Vec<char>,
	position: misc::MaybePosition,
}

// fn`content`
pub struct TaggedTemplateLiteral {
	tag: Option<Box<alias::Expression>>,
	template: TemplateLiteral,

	position: misc::MaybePosition,
}

// `content`
pub enum TemplateLiteral {
	Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteral>),
	End(TemplatePart),
}
struct TemplatePart {
	value: string::String,
	position: misc::MaybePosition,

	position: misc::MaybePosition,
}

// foo()
pub struct CallExpression {
	callee: Box<alias::Expression>,
	arguments: misc::CallArguments,
	optional: bool,
	position: misc::MaybePosition,
}

// new foo()
pub struct NewExpression {
	callee: Box<alias::Expression>,
	arguments: misc::CallArguments,
	position: misc::MaybePosition,
}

// experimental
// import(foo)
pub struct ImportExpression {
	argument: Box<alias::Expression>,
	position: misc::MaybePosition,
}

// foo.bar
// foo?.bar
// foo.#bar
// foo?.#bar
pub struct MemberExpression {
	object: Box<alias::Expression>,
	property: MemberProperty,
	optional: bool,
	position: misc::MaybePosition,
}
enum MemberProperty {
	Normal(misc::PropertyId),
	Private(PrivateProperty),
}
struct PrivateProperty {
	property: misc::PropertyIdentifier,
	position: misc::MaybePosition,
}

// #bar
pub struct PrivateExpression {
	property: misc::PropertyIdentifier,
	position: misc::MaybePosition,
}

// i++
// i--
// ++i
// --i
pub struct UpdateExpression {
	value: LeftHandExpression,
	operator: UpdateOperator,
	position: misc::MaybePosition,
}
enum UpdateOperator {
	PreIncrement,
	PreDecrement,
	PostIncrement,
	PostDecrement,
}

// void foo
pub struct UnaryExpression {
	value: Box<alias::Expression>,
	operator: UnaryOperator,
	position: misc::MaybePosition,
}
enum UnaryOperator {
	Delete,
	Void,
	Typeof,
	Positive,
	Negative,
	BitNegate,
	Negate,
	Await,
	Yield,
}

// foo OP bar
pub struct BinaryExpression {
	value: Box<alias::Expression>,
	operator: BinaryOperator,
	position: misc::MaybePosition,
}
enum BinaryOperator {
	Add,
	Subtract,
	LeftShift,
	RightShift,
	RightShiftSigned,
	Divide,
	Multiply,
	Modulus,
	BitAnd,
	BitOr,
	BitXor,
	Power,

	Compare,
	StrictCompare,
	NegateCompare,
	NegateStrictCompare,
	LessThan,
	LessThanEq,
	GreaterThan,
	GreaterThanEq,
	In,
	Instanceof,

	And,
	Or,

	Bind, // experimental
}

// foo ? bar : baz
pub struct ConditionalExpression {
	test: Box<alias::Expression>,
	alternate: Box<alias::Expression>,
	consequent: Box<alias::Expression>,
	position: misc::MaybePosition,
}

// foo OP= bar
pub struct AssignmentExpression {
	operator: AssignmentOperator,
	left: Box<LeftHandExpression>,
	value: Box<alias::Expression>,
	position: misc::MaybePosition,
}
enum AssignmentOperator {
	Add,
	Subtract,
	LeftShift,
	RightShift,
	RightShiftSigned,
	Divide,
	Multiply,
	Modulus,
	BitAnd,
	BitOr,
	BitXor,
	Power,

	None,
}

pub enum LeftHandExpression {
	Pattern(misc::Pattern),
	MemberExpression(expression::MemberExpression),
	SuperProperty(expression::SuperMemberExpression),

	// "yield" is disallowed in strict mode
	// "await" is disallowed in module
	BindingIdentifier(BindingIdentifier), // May not be "eval" or "arguments" in strict
}

// foo, bar
pub struct SequenceExpression {
	left: Box<alias::Expression>,
	right: Box<alias::Expression>,
	position: misc::MaybePosition,
}

// (foo) => bar
pub struct ArrowFunctionExpression {
	params: misc::FunctionParams,
	body: ArrowFunctionBody,
	fn_kind: ArrowFunctionKind,
	position: misc::MaybePosition,

	// Flow extension
	type_parameters: Option<flow::Parameters>,
	return_type: Option<Box<flow::Annotation>>,
}
enum ArrowFunctionKind {
	Normal,
	Async,
	
	Generator, // experimental
	AsyncGenerator, // experimental
}
enum ArrowFunctionBody {
	Expression(Box<alias::Expression>),
	Block(misc::FunctionBody),
}

// do { foo; }
pub struct DoExpression {
	body: statement::BlockStatement,
	position: misc::MaybePosition,
}

// new.target
pub struct MetaProperty {
	kind: MetaPropertyKind,
	position: misc::MaybePosition,
}
pub enum MetaPropertyKind {
	NewTarget,
	ImportMeta, // experimental
	FunctionSent, // experimental
	FunctionArguments, // experimental
}

// super.foo
// super[foo]
pub struct SuperMemberExpression {
	property: misc::PropertyId,
	position: misc::MaybePosition,
}