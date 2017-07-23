use std::string;
use super::alias;
use super::flow;
use super::literal;

pub struct MaybePosition(Option<Box<NodePosition>>);

pub struct NodePosition {
	start: usize,
	end: usize,
	range: PositionRange,
}
pub struct PositionRange {
	start: (usize, usize),
	end: (usize, usize),
}

pub trait WithPosition {
	set_position(&mut self, pos: NodePosition);
	clear_position(&mut self);
}


pub struct Directive {
	value: string::String,
	position: MaybePosition,
}

pub enum Pattern {
	Identifier(BindingIdentifier),
	Object(ObjectPattern),
	Array(ArrayPattern),
}

// identifiers used as labels
pub struct LabelIdentifier {
	value: string::String,
	position: MaybePosition,
}

// identifiers used as variables
pub struct BindingIdentifier {
	id: string::String,
	position: MaybePosition,
}

// identifiers used as properties
pub struct PropertyIdentifier {
	id: string::String,
	position: MaybePosition,
}

// ({   } = 
pub struct ObjectPattern {
	properties: Vec<ObjectPatternProperty>,
	rest: Option<Box<Pattern>>,
	position: MaybePosition,
}
struct ObjectPatternProperty {
	// foo (= expr)?
	// prop: foo (= expr)?
	// prop: {a} (= expr)?
	name: Option<PropertyIdentifier>,
	id: Pattern,
	init: Option<alias::Expression>,
	position: MaybePosition,
}

pub struct ArrayPattern {
	elements: Vec<Option<ArrayPatternElement>>,
	rest: Option<Box<Pattern>>,
	position: MaybePosition,
}
struct ArrayPatternElement {
	// foo (= expr)?
	// {a} (= expr)?
	id: Pattern,
	init: Option<alias::Expression>,
	position: MaybePosition,
}



pub struct FunctionBody {
	directives: Vec<Directive>,
	body: Vec<alias::StatementItem>,
	position: MaybePosition,
}



// experimental
pub enum Decorator {
	Property(DecoratorMemberExpression),
	Call(DecoratorCallExpression),

	Expression(alias::Expression), // Backward-compat for older decorator spec
}

// experimental
enum DecoratorMemberExpression {
	Identifier(BindingIdentifier),
	Member(Box<DecoratorMemberExpression>, PropertyIdentifier),
}
// experimental
struct DecoratorCallExpression {
	callee: DecoratorMemberExpression,
	arguments: CallArguments,
	position: MaybePosition,
}

pub struct CallArguments {
	args: Vec<Box<alias::Expression>>,
	spread: Option<Box<alias::Expression>>,
	position: MaybePosition,
}


pub struct ClassBody {
	items: Vec<ClassItem>,
	position: MaybePosition,
}
enum ClassItem {
	Method(ClassMethod),
	Field(ClassField),
	Empty {
		position: MaybePosition,
	},
}

pub enum FunctionKind {
	Normal,
	Generator,
	Async,
	AsyncGenerator, // experimental
}

pub enum MethodKind {
	Normal,
	Get,
	Set,
}
pub enum PropertyId {
	Literal(PropertyIdentifier),
	String(literal::String),
	Number(literal::Numeric),
	Computed(Box<alias::Expression>),
}

struct ClassMethod {
	pos: FieldPosition,
	kind: MethodKind,
	id: ClassFieldId,
	params: FunctionParams,
	body: FunctionBody,
	fn_kind: FunctionKind,
	decorators: Vec<Decorator>,
	position: MaybePosition,

	return_type: Option<Box<flow::Annotation>>,
}
enum FieldPosition {
    Instance,
    Static,
}

// experimental
enum ClassFieldId {
	Public(PropertyId),
	Private(PropertyIdentifier),
}

// experimental
struct ClassField {
	pos: FieldPosition,
	decorators: Vec<Decorator>,

	// This is limited to >= 1 item
	items: Vec<ClassFieldPair>,
	position: MaybePosition,
}
struct ClassFieldPair {
	id: ClassFieldId,
	value: alias::Expression,
	position: MaybePosition,

	// Flow extension
	type_variance: flow::Variance,
}


pub struct FunctionParams {
	params: Vec<FunctionParam>,
	rest: Option<FunctionRestParam>,
	position: MaybePosition,
}
struct FunctionParam {
	decorators: Vec<Decorator>, // experimental
	id: Pattern,
	init: Option<Box<alias::Expression>>,
	position: MaybePosition,

	// Flow extension
	type_annotation: Option<Box<flow::Annotation>>,
	optional: bool,
}
struct FunctionRestParam {
	decorators: Vec<Decorator>, // experimental
	id: Pattern,
	position: MaybePosition,

	// Flow extensionF
	type_annotation: Option<Box<flow::Annotation>>,
}
