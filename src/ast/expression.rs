use std::string;

use super::misc;
use super::alias;
use super::flow;
use super::literal;
use super::statement;

nodes!{
	// this
	pub struct ThisExpression {}

	// [1, 2, 3, ...4]
	pub struct ArrayExpression {
		elements: Vec<Option<Box<alias::Expression>>>,
		spread: Option<Box<alias::Expression>>,
	}

	// {a: 1, ...b}
	pub struct ObjectExpression {
		properties: Vec<ObjectProperty>,
		spread: Option<Box<alias::Expression>>, // experimental
	}
	pub enum ObjectProperty {
		Method(ObjectMethod),
		Value(misc::PropertyId, Box<alias::Expression>),
	}
	pub struct ObjectMethod {
		kind: misc::MethodKind,
		id: misc::PropertyId,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		return_type: Option<Box<flow::Annotation>>,
	}

	// (function(){})
	pub struct FunctionExpression {
		decorators: Vec<misc::Decorator>, // experimental
		id: Option<misc::BindingIdentifier>,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

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

		// Flow extension
		type_parameters: Option<flow::Parameters>,
	}

	// /foo/g
	pub struct RegularExpressionLiteral {
		value: string::String,
		flags: Vec<char>,
	}

	// fn`content`
	pub struct TaggedTemplateLiteral {
		tag: Option<Box<alias::Expression>>,
		template: TemplateLiteral,

	}

	// `content`
	pub enum TemplateLiteral {
		Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteral>),
		End(TemplatePart),
	}
	pub struct TemplatePart {
		value: string::String,
	}

	// foo()
	pub struct CallExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
		optional: bool,
	}

	// new foo()
	pub struct NewExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
	}

	// experimental
	// import(foo)
	pub struct ImportExpression {
		argument: Box<alias::Expression>,
	}

	// foo.bar
	// foo?.bar
	// foo.#bar
	// foo?.#bar
	pub struct MemberExpression {
		object: Box<alias::Expression>,
		property: MemberProperty,
		optional: bool,
	}
	pub enum MemberProperty {
		Normal(misc::PropertyId),
		Private(PrivateProperty),
	}
	pub struct PrivateProperty {
		property: misc::PropertyIdentifier,
	}

	// #bar
	pub struct PrivateExpression {
		property: misc::PropertyIdentifier,
	}

	// i++
	// i--
	// ++i
	// --i
	pub struct UpdateExpression {
		value: LeftHandExpression,
		operator: UpdateOperator,
	}
	pub enum UpdateOperator {
		PreIncrement,
		PreDecrement,
		PostIncrement,
		PostDecrement,
	}

	// void foo
	pub struct UnaryExpression {
		value: Box<alias::Expression>,
		operator: UnaryOperator,
	}
	pub enum UnaryOperator {
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
	}
	pub enum BinaryOperator {
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
	}

	// foo OP= bar
	pub struct AssignmentExpression {
		operator: AssignmentOperator,
		left: Box<LeftHandExpression>,
		value: Box<alias::Expression>,
	}
	pub enum AssignmentOperator {
		None,

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
	}

	pub enum LeftHandExpression {
		Pattern(misc::Pattern),
		MemberExpression(MemberExpression),
		SuperProperty(SuperMemberExpression),

		// "yield" is disallowed in strict mode
		// "await" is disallowed in module
		BindingIdentifier(misc::BindingIdentifier), // May not be "eval" or "arguments" in strict
	}

	// foo, bar
	pub struct SequenceExpression {
		left: Box<alias::Expression>,
		right: Box<alias::Expression>,
	}

	// (foo) => bar
	pub struct ArrowFunctionExpression {
		params: misc::FunctionParams,
		body: ArrowFunctionBody,
		fn_kind: ArrowFunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	pub enum ArrowFunctionKind {
		Normal,
		Async,

		Generator, // experimental
		AsyncGenerator, // experimental
	}
	pub enum ArrowFunctionBody {
		Expression(Box<alias::Expression>),
		Block(misc::FunctionBody),
	}

	// do { foo; }
	pub struct DoExpression {
		body: statement::BlockStatement,
	}

	// new.target
	pub struct MetaProperty {
		kind: MetaPropertyKind,
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
	}
}
