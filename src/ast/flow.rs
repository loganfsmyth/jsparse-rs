use super::alias;
use super::misc;
use super::literal;

pub enum Annotation {
	Primitive(PrimitiveAnnotation),
	Literal(LiteralAnnotation),
	Special(SpecialAnnotation),
	Maybe(MaybeAnnotation),
	Function(FunctionAnnotation),
	Object(ObjectAnnotation),
	ArrayShorthand(ArrayShorthandAnnotation),
	Tuple(TupleAnnotation),
	Binding(BindingIdentifierAnnotation),
	Union(UnionAnnotation),
	Intersection(IntersectionAnnotation),
	Typeof(TypeofAnnotation),
}

// (foo: number)
pub struct CastExpression {
	expression: Box<alias::Expression>,
	type_annotation: Annotation,
	position: MaybePosition,
}


// <T, U>
pub struct Parameters {
	parameters: Vec<Parameter>,
	position: misc::MaybePosition,
}
struct Parameter {
	variance: Variance,
	id: misc::BindingIdentifier,
	bound: Option<Box<Annotation>>,
	default: Option<Box<Annotation>>,
	position: misc::MaybePosition,
}
pub enum PrimitiveAnnotation {
	Boolean,
	String,
	Number,
	Null,
	Void,
}
pub enum LiteralAnnotation {
	Boolean(literal::Boolean),
	String(literal::String),
	Number(literal::Numeric),
}

pub enum SpecialAnnotation {
	Any {
		position: MaybePosition,
	},
	Mixed {
		position: MaybePosition,
	},
	Existential {
		position: MaybePosition,
	},
}

pub struct MaybeAnnotation {
	argument: Box<Annotation>,
	position: misc::MaybePosition,
}

pub struct FunctionAnnotation {
	params: FunctionParams,
	return_type: Box<Annotation>,
	position: misc::MaybePosition,
}

struct FunctionParams {
	params: Vec<FunctionParam>,
	rest: Option<FunctionRestParam>,
	position: misc::MaybePosition,
}
struct FunctionParam {
	type_annotation: Box<Annotation>,
	id: Option<misc::BindingIdentifier>,
	position: misc::MaybePosition,

	optional: bool,
}
struct FunctionRestParam {
	type_annotation: Box<Annotation>,
	id: Option<misc::BindingIdentifier>,
	position: misc::MaybePosition,
}


pub struct ObjectAnnotation {
	exact: bool,
	properties: Vec<ObjectItem>,
	position: misc::MaybePosition,
}

enum ObjectItem {
	Method(ObjectMethod),
	Property(ObjectProperty),
	MapProperty(ObjectMapProperty),
}
struct ObjectMethod {
	id: ObjectMethodId,
	params: FunctionParams,
	return_type: Box<Annotation>,
}
enum ObjectMethodId {
	Literal(misc::PropertyIdentifier),
	String(literal::String),
}

struct ObjectProperty {
	variance: Variance,
	id: ObjectMethodId,
	optional: bool,
	value: Box<Annotation>,
	position: misc::MaybePosition,
}

struct ObjectMapProperty {
	id: ObjectMethodId,
	key: Box<Annotation>,
	value: Box<Annotation>,
	position: misc::MaybePosition,
}

// foo[]
pub struct ArrayShorthandAnnotation {
	type_annotation: Box<Annotation>,
	position: misc::MaybePosition,
}

// [number, string]
pub struct TupleAnnotation {
	items: Vec<Annotation>,
	position: misc::MaybePosition,
}

// 
pub enum BindingIdentifierAnnotationList {
	Identifier(BindingIdentifierAnnotation),
	List(BindingIdentifierAnnotation, Box<BindingIdentifierAnnotationList>),
}

// Foo<T>
pub struct BindingIdentifierAnnotation {
	id: misc::BindingIdentifier,
	type_parameters: Option<Parameters>,
	position: misc::MaybePosition,
}

pub struct UnionAnnotation {
	left: Box<Annotation>,
	right: Box<Annotation>,
	position: misc::MaybePosition,
}
pub struct IntersectionAnnotation {
	left: Box<Annotation>,
	right: Box<Annotation>,
	position: misc::MaybePosition,
}

pub enum TypeofAnnotation {
	Literal(LiteralAnnotation),
	Identifier(misc::BindingIdentifier),
	Member(MemberExpression),
}

enum MemberExpression {
	Identifier {
		id: misc::BindingIdentifier,
		position: misc::MaybePosition,
	},
	Member {
		object: Box<MemberExpression>,
		property: misc::PropertyIdentifier,
		position: misc::MaybePosition,
	},
}

pub enum Variance {
	None,
	Covariant,
	Contravariant,
}

pub struct Interface {
	id: misc::BindingIdentifier,
	type_parameters: Option<Parameters>,
	items: Vec<ObjectItem>,
	position: misc::MaybePosition,
}

// type Foo = {};
pub struct AliasDeclaration {
	id: misc::BindingIdentifier,
	type_parameters: Option<Parameters>,
	position: misc::MaybePosition,

	type_annotation: Box<Annotation>,
}

// declare function fn(){}
pub struct DeclareFunctionDeclaration {
	id: misc::BindingIdentifier,
	params: FunctionParams,
	type_parameters: Option<Parameters>,
	return_type: Box<Annotation>,

	position: misc::MaybePosition,
}
// declare class Foo {}
pub struct DeclareClassDeclaration {
	id: misc::BindingIdentifier,
	type_parameters: Option<Parameters>,


	// TODO: Probably more specific than "Expression"
	extends: Option<Box<alias::Expression>>,
	implements: Option<BindingIdentifierAnnotationList>,
	items: Vec<ObjectItem>,

	position: misc::MaybePosition,
}
// declare var foo: number;
pub struct DeclareVariableDeclaration {
	id: misc::BindingIdentifier,
	type_annotation: Box<Annotation>,

	position: misc::MaybePosition,
}
// declare type foo = {};
pub struct DeclareAliasDeclaration {
	id: misc::BindingIdentifier,
	type_annotation: Box<Annotation>,

	position: misc::MaybePosition,
}
// declare module "foo" {}
pub struct DeclareModuleDeclaration {
	source: literal::String,
	items: Vec<ModuleItem>,

	position: misc::MaybePosition,
}

// declare export function fn(){}
struct DeclareExportFunctionDeclaration {
	id: misc::BindingIdentifier,
	params: FunctionParams,
	type_parameters: Option<Parameters>,
	return_type: Box<Annotation>,

	position: misc::MaybePosition,
}

// declare export class Foo {}
struct DeclareExportClassDeclaration {
	id: misc::BindingIdentifier,
	type_parameters: Option<Parameters>,

	// TODO: Probably more specific than "Expression"
	extends: Option<Box<alias::Expression>>,
	implements: Option<BindingIdentifierAnnotationList>,
	items: Vec<ObjectItem>,

	position: misc::MaybePosition,
}

// declare export default function fn(){}
struct DeclareExportDefaultFunctionDeclaration {
	id: Option<misc::BindingIdentifier>,
	params: FunctionParams,
	type_parameters: Option<Parameters>,
	return_type: Box<Annotation>,

	position: misc::MaybePosition,
}

// declare export default class Foo {}
struct DeclareExportDefaultClassDeclaration {
	id: Option<misc::BindingIdentifier>,
	type_parameters: Option<Parameters>,
	items: Vec<ObjectItem>,

	position: misc::MaybePosition,
}
// declare export var foo: number;
struct DeclareExportVariableDeclaration {
	id: misc::BindingIdentifier,
	type_annotation: Box<Annotation>,

	position: misc::MaybePosition,
}


// declare export type foo = {};
struct DeclareExportAliasDeclaration {
	id: misc::BindingIdentifier,
	type_annotation: Box<Annotation>,

	position: misc::MaybePosition,
}

// declare export default foo;
struct DeclareExportDefaultTypeDeclaration {
	type_annotation: Box<Annotation>,

	position: misc::MaybePosition,
}

// declare module.exports: {};
struct DeclareExportCommonJS {
	type_annotation: Box<Annotation>,
	
	position: misc::MaybePosition,
}


enum ModuleItem {
	Function(DeclareFunctionDeclaration),
	Class(DeclareClassDeclaration),
	Variable(DeclareVariableDeclaration),
	Alias(DeclareAliasDeclaration),

	ExportFunction(DeclareExportFunctionDeclaration),
	ExportClass(DeclareExportClassDeclaration),
	ExportDefaultFunction(DeclareExportDefaultFunctionDeclaration),
	ExportDefaultClass(DeclareExportDefaultClassDeclaration),
	ExportVariable(DeclareExportVariableDeclaration),
	ExportAlias(DeclareExportAliasDeclaration),
	ExportDefaultType(DeclareExportDefaultTypeDeclaration),
	CommonJSExport(DeclareExportCommonJS),
}
