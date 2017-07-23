use super::alias;
use super::misc;
use super::literal;


nodes!{
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
	}


	// <T, U>
	pub struct Parameters {
		parameters: Vec<Parameter>,
	}
	pub struct Parameter {
		variance: Variance,
		id: misc::BindingIdentifier,
		bound: Option<Box<Annotation>>,
		default: Option<Box<Annotation>>,
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
		},
		Mixed {
		},
		Existential {
		},
	}

	pub struct MaybeAnnotation {
		argument: Box<Annotation>,
	}

	pub struct FunctionAnnotation {
		params: FunctionParams,
		return_type: Box<Annotation>,
	}

	pub struct FunctionParams {
		params: Vec<FunctionParam>,
		rest: Option<FunctionRestParam>,
	}
	pub struct FunctionParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,

		optional: bool,
	}
	pub struct FunctionRestParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,
	}


	pub struct ObjectAnnotation {
		exact: bool,
		properties: Vec<ObjectItem>,
	}

	pub enum ObjectItem {
		Method(ObjectMethod),
		Property(ObjectProperty),
		MapProperty(ObjectMapProperty),
	}
	pub struct ObjectMethod {
		id: ObjectMethodId,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	pub enum ObjectMethodId {
		Literal(misc::PropertyIdentifier),
		String(literal::String),
	}

	pub struct ObjectProperty {
		variance: Variance,
		id: ObjectMethodId,
		optional: bool,
		value: Box<Annotation>,
	}

	pub struct ObjectMapProperty {
		id: ObjectMethodId,
		key: Box<Annotation>,
		value: Box<Annotation>,
	}

	// foo[]
	pub struct ArrayShorthandAnnotation {
		type_annotation: Box<Annotation>,
	}

	// [number, string]
	pub struct TupleAnnotation {
		items: Vec<Annotation>,
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
	}

	pub struct UnionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}
	pub struct IntersectionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}

	pub enum TypeofAnnotation {
		Literal(LiteralAnnotation),
		Identifier(misc::BindingIdentifier),
		Member(MemberExpression),
	}

	pub enum MemberExpression {
		Identifier {
			id: misc::BindingIdentifier,
		},
		Member {
			object: Box<MemberExpression>,
			property: misc::PropertyIdentifier,
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
	}

	// type Foo = {};
	pub struct AliasDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,

		type_annotation: Box<Annotation>,
	}

	// declare function fn(){}
	pub struct DeclareFunctionDeclaration {
		id: misc::BindingIdentifier,
		params: FunctionParams,
		type_parameters: Option<Parameters>,
		return_type: Box<Annotation>,

	}
	// declare class Foo {}
	pub struct DeclareClassDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,


		// TODO: Probably more specific than "Expression"
		extends: Option<Box<alias::Expression>>,
		implements: Option<BindingIdentifierAnnotationList>,
		items: Vec<ObjectItem>,

	}
	// declare var foo: number;
	pub struct DeclareVariableDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,

	}
	// declare type foo = {};
	pub struct DeclareAliasDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,

	}
	// declare module "foo" {}
	pub struct DeclareModuleDeclaration {
		source: literal::String,
		items: Vec<ModuleItem>,

	}

	// declare export function fn(){}
	pub struct DeclareExportFunctionDeclaration {
		id: misc::BindingIdentifier,
		params: FunctionParams,
		type_parameters: Option<Parameters>,
		return_type: Box<Annotation>,

	}

	// declare export class Foo {}
	pub struct DeclareExportClassDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,

		// TODO: Probably more specific than "Expression"
		extends: Option<Box<alias::Expression>>,
		implements: Option<BindingIdentifierAnnotationList>,
		items: Vec<ObjectItem>,

	}

	// declare export default function fn(){}
	pub struct DeclareExportDefaultFunctionDeclaration {
		id: Option<misc::BindingIdentifier>,
		params: FunctionParams,
		type_parameters: Option<Parameters>,
		return_type: Box<Annotation>,

	}

	// declare export default class Foo {}
	pub struct DeclareExportDefaultClassDeclaration {
		id: Option<misc::BindingIdentifier>,
		type_parameters: Option<Parameters>,
		items: Vec<ObjectItem>,

	}
	// declare export var foo: number;
	pub struct DeclareExportVariableDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,

	}


	// declare export type foo = {};
	pub struct DeclareExportAliasDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,

	}

	// declare export default foo;
	pub struct DeclareExportDefaultTypeDeclaration {
		type_annotation: Box<Annotation>,

	}

	// declare module.exports: {};
	pub struct DeclareExportCommonJS {
		type_annotation: Box<Annotation>,
		
	}

	pub enum ModuleItem {
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
}
