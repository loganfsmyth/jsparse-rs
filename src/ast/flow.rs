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
	impl misc::NodeDisplay for Annotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				Annotation::Primitive(ref anno) => f.node(anno),
				Annotation::Literal(ref anno) => f.node(anno),
				Annotation::Special(ref anno) => f.node(anno),
				Annotation::Maybe(ref anno) => f.node(anno),
				Annotation::Function(ref anno) => f.node(anno),
				Annotation::Object(ref anno) => f.node(anno),
				Annotation::ArrayShorthand(ref anno) => f.node(anno),
				Annotation::Tuple(ref anno) => f.node(anno),
				Annotation::Binding(ref anno) => f.node(anno),
				Annotation::Union(ref anno) => f.node(anno),
				Annotation::Intersection(ref anno) => f.node(anno),
				Annotation::Typeof(ref anno) => f.node(anno),
			}
		}
	}

	// (foo: number)
	pub struct CastExpression {
		expression: Box<alias::Expression>,
		type_annotation: Annotation,
	}
	impl misc::NodeDisplay for CastExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::ParenL)?;
			f.node(self.expression)?;
			f.node(self.type_annotation)?;
			f.token(misc::Token::ParenR)
		}
	}


	// <T, U>
	pub struct Parameters {
		parameters: Vec<Parameter>,
	}
	impl misc::NodeDisplay for Parameters {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::AngleL)?;

			for (i, param) in self.parameters.iter().enumerate() {
				if i != 0 {
					f.token(misc::Token::Comma)?;
				}

				f.node(param)?;
			}
			f.token(misc::Token::AngleR)?;
		}
	}
	pub struct Parameter {
		variance: Variance,
		id: misc::BindingIdentifier,
		bound: Option<Box<Annotation>>,
		default: Option<Box<Annotation>>,
	}
	impl misc::NodeDisplay for Parameter {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.variance)?;


		}
	}
	pub enum PrimitiveAnnotation {
		Boolean,
		String,
		Number,
		Null,
		Void,
	}
	impl misc::NodeDisplay for PrimitiveAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				PrimitiveAnnotation::Boolean => f.token(misc::Token::Boolean),
				PrimitiveAnnotation::String => f.token(misc::Token::String),
				PrimitiveAnnotation::Number => f.token(misc::Token::Number),
				PrimitiveAnnotation::Null => f.token(misc::Token::Null),
				PrimitiveAnnotation::Void => f.token(misc::Token::Void),
			}
		}
	}
	pub enum LiteralAnnotation {
		Boolean(literal::Boolean),
		String(literal::String),
		Number(literal::Numeric),
	}
	impl misc::NodeDisplay for LiteralAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				LiteralAnnotation::Boolean(ref id) => f.node(id),
				LiteralAnnotation::String(ref id) => f.node(id),
				LiteralAnnotation::Number(ref id) => f.node(id),
			}
		}
	}

	pub enum SpecialAnnotation {
		// any
		Any {
		},
		// mixed
		Mixed {
		},
		// *
		Existential {
		},
	}
	impl misc::NodeDisplay for SpecialAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				SpecialAnnotation::Any => f.token(misc::Token::Any),
				SpecialAnnotation::Mixed => f.token(misc::Token::Mixed),
				SpecialAnnotation::Existential => f.token(misc::Token::Star),
			}
		}
	}

	pub struct MaybeAnnotation {
		argument: Box<Annotation>,
	}
	impl misc::NodeDisplay for MaybeAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.argument)?;
			f.token(misc::Token::Question)
		}
	}

	pub struct FunctionAnnotation {
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl misc::NodeDisplay for FunctionAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.params)?;
			f.node(self.return_type)?;
			f.token(misc::Token::Semicolon)
		}
	}

	pub struct FunctionParams {
		params: Vec<FunctionParam>,
		rest: Option<FunctionRestParam>,
	}
	impl misc::NodeDisplay for FunctionParams {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			for param in self.params.iter() {
				f.node(param)?;

				f.token(misc::Token::Comma)?;
			}

			if let Some(rest) = self.rest {
				f.token(misc::Token::Ellipsis)?;
				f.node(rest)?;
			}
		}
	}

	pub struct FunctionParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,

		optional: bool,
	}
	impl misc::NodeDisplay for FunctionParam {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			if let Some(id) = self.id {
				f.node(id)?;

				if self.optional {
					f.token(misc::Token::Question)?;
				}
				f.token(misc::Token::Colon)?;
			}
			f.node(self.type_annotation)?;

			Ok(())
		}
	}
	pub struct FunctionRestParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,
	}
	impl misc::NodeDisplay for FunctionRestParam {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Ellipsis)?;

			if let Some(id) = self.id {
				f.node(id)?;

				f.token(misc::Token::Colon)?;
			}
			f.node(self.type_annotation)?;

			Ok(())
		}
	}


	pub struct ObjectAnnotation {
		exact: bool,
		properties: Vec<ObjectItem>,
	}
	impl misc::NodeDisplay for ObjectAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Ellipsis)?;
			for prop in self.properties.iter() {
				f.node(prop)?;

				f.token(misc::Token::Comma)?;
			}

			f.token(misc::Token::Ellipsis)
		}
	}

	pub enum ObjectItem {
		Method(ObjectMethod),
		Property(ObjectProperty),
		MapProperty(ObjectMapProperty),
	}
	impl misc::NodeDisplay for ObjectItem {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				ObjectItem::Method(ref item) => f.node(item),
				ObjectItem::Property(ref item) => f.node(item),
				ObjectItem::MapProperty(ref item) => f.node(item),
			}
		}
	}
	pub struct ObjectMethod {
		id: ObjectMethodId,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl misc::NodeDisplay for ObjectMethod {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.id)?;
			f.node(self.params)?;
			f.node(self.return_type)
		}
	}
	pub enum ObjectMethodId {
		Literal(misc::PropertyIdentifier),
		String(literal::String),
	}
	impl misc::NodeDisplay for ObjectMethodId {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				ObjectMethodId::Literal(ref id) => f.node(id),
				ObjectMethodId::String(ref id) => f.node(id),
			}
		}
	}

	pub struct ObjectProperty {
		variance: Variance,
		id: ObjectMethodId,
		optional: bool,
		value: Box<Annotation>,
	}
	impl misc::NodeDisplay for ObjectProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.variance)?;
			f.node(self.id)?;
			if self.optional {
				f.token(misc::Token::Question)?;
			}
			f.token(misc::Token::Colon)?;
			f.node(self.value)
		}
	}

	pub struct ObjectMapProperty {
		id: Option<ObjectMethodId>,
		key: Box<Annotation>,
		value: Box<Annotation>,
	}
	impl misc::NodeDisplay for ObjectMapProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::SquareL)?;

			if let Some(id) = self.id {
				f.node(id)?;
				f.token(misc::Token::Colon)?;
			}
			f.node(self.key)?;

			f.token(misc::Token::SquareR)?;
			f.token(misc::Token::Colon)?;

			f.node(self.value)
		}
	}

	// foo[]
	pub struct ArrayShorthandAnnotation {
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for ArrayShorthandAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.type_annotation)?;
			f.token(misc::Token::SquareL)?;
			f.token(misc::Token::SquareR)
		}
	}

	// [number, string]
	pub struct TupleAnnotation {
		items: Vec<Annotation>,
	}
	impl misc::NodeDisplay for TupleAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::SquareL)?;
			for (i, anno) in self.items.iter().enumerate() {
				if i != 0 {
					f.token(misc::Token::Comma)?;
				}
				f.node(anno)?;
			}
			f.token(misc::Token::SquareR)?;
		}
	}

	//
	pub enum BindingIdentifierAnnotationList {
		Identifier(BindingIdentifierAnnotation),
		List(BindingIdentifierAnnotation, Box<BindingIdentifierAnnotationList>),
	}
	impl misc::NodeDisplay for BindingIdentifierAnnotationList {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				BindingIdentifierAnnotationList::Identifier(ref id) => f.node(id),
				BindingIdentifierAnnotationList::List(ref id, ref next) => {
					f.node(id)?;
					f.token(misc::Token::Comma)?;
					f.node(next)
				}
			}
		}
	}

	// Foo<T>
	pub struct BindingIdentifierAnnotation {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,
	}
	impl misc::NodeDisplay for BindingIdentifierAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			Ok(())
		}
	}

	pub struct UnionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}
	impl misc::NodeDisplay for UnionAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.left)?;
			f.token(misc::Token::Amp)?;
			f.node(self.right)?;
		}
	}
	pub struct IntersectionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}
	impl misc::NodeDisplay for IntersectionAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node(self.left)?;
			f.token(misc::Token::Bar)?;
			f.node(self.right)?;
		}
	}

	pub enum TypeofAnnotation {
		Literal(LiteralAnnotation),
		Identifier(misc::BindingIdentifier),
		Member(MemberExpression),
	}
	impl misc::NodeDisplay for TypeofAnnotation {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Typeof)?;

			match self {
				TypeofAnnotation::Literal(ref id) => f.node(id),
				TypeofAnnotation::Identifier(ref id) => f.node(id),
				TypeofAnnotation::Member(ref id) => f.node(id),
			}
		}
	}

	pub enum MemberExpression {
		Identifier(misc::BindingIdentifier),
		Member(Box<MemberExpression>, misc::PropertyIdentifier),
	}
	impl misc::NodeDisplay for MemberExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				MemberExpression::Identifier(ref id) => f.node(id),
				MemberExpression::Member(ref obj, ref id) => {
					f.node(obj)?;
					f.token(misc::Token::Period)?;
					f.node(id)
				}
			}
		}
	}

	pub enum Variance {
		None,
		Covariant,
		Contravariant,
	}
	impl misc::NodeDisplay for Variance {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				Variance::None => Ok(()),
				Variance::Covariant => f.token(misc::Token::Plus),
				Variance::Contravariant => f.token(misc::Token::Minus),
			}
		}
	}

	pub struct Interface {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,
		items: Vec<ObjectItem>,
	}
	impl misc::NodeDisplay for Interface {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Interface)?;
			f.node(self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			f.token(misc::Token::CurlyL)?;

			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(misc::Token::CurlyR)
		}
	}

	// type Foo = {};
	pub struct AliasDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,

		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for AliasDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Type)?;
			f.node(self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			f.token(misc::Token::Eq)?;
			f.token(misc::Token::CurlyL)?;

			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(misc::Token::CurlyR)
		}
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
	impl misc::NodeDisplay for DeclareVariableDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Var)?;
			f.node(&self.id)?;
			f.token(misc::Token::Colon)?;
			f.node(&self.type_annotation)
		}
	}

	// declare type foo = {};
	pub struct DeclareAliasDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Box<Parameters>>,
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareAliasDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Type)?;

			f.node(&self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}

			f.token(misc::Token::Eq)?;
			f.node(&self.type_annotation)
		}
	}

	// declare module "foo" {}
	pub struct DeclareModuleDeclaration {
		source: literal::String,
		items: Vec<ModuleItem>,
	}
	impl misc::NodeDisplay for DeclareModuleDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Module)?;
			misc::NodeDisplay::fmt(&self.source)?;

			f.token(misc::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(misc::Token::CurlyR)
		}
	}

	// declare export function fn(){}
	pub struct DeclareExportFunctionDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportFunctionDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Function)?;
			f.node(&self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			f.node(&self.params)?;
			f.token(misc::Token::Colon)?;
			f.node(&self.return_type)
		}
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
	impl misc::NodeDisplay for DeclareExportClassDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Class)?;
			f.node(&self.id)?;
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			if let Some(extends) = self.extends {
				f.token(misc::Token::Extends)?;
				f.node(extends)?;
			}
			if let Some(implements) = self.implements {
				f.token(misc::Token::Implements)?;
				f.node(implements)?;
			}
			f.token(misc::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(misc::Token::CurlyR)
		}
	}

	// declare export default function fn(){}
	pub struct DeclareExportDefaultFunctionDeclaration {
		id: Option<misc::BindingIdentifier>,
		type_parameters: Option<Parameters>,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportDefaultFunctionDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Default)?;
			f.token(misc::Token::Function)?;
			if let Some(id) = self.id {
				f.node(id)?;
			}
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			f.node(&self.params)?;
			f.token(misc::Token::Colon)?;
			f.node(&self.return_type)
		}
	}

	// declare export default class Foo {}
	pub struct DeclareExportDefaultClassDeclaration {
		id: Option<misc::BindingIdentifier>,
		// TODO: Lifetime of this should be tied to identifier.
		type_parameters: Option<Parameters>,

		// TODO: Probably more specific than "Expression"
		extends: Option<Box<alias::Expression>>,
		implements: Option<BindingIdentifierAnnotationList>,
		items: Vec<ObjectItem>,
	}
	impl misc::NodeDisplay for DeclareExportDefaultClassDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Default)?;
			f.token(misc::Token::Class)?;
			if let Some(id) = self.id {
				f.node(id)?;
			}
			if let Some(param) = self.type_parameters {
				f.node(param)?;
			}
			if let Some(extends) = self.extends {
				f.token(misc::Token::Extends)?;
				f.node(extends)?;
			}
			if let Some(implements) = self.implements {
				f.token(misc::Token::Implements)?;
				f.node(implements)?;
			}
			f.token(misc::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(misc::Token::CurlyR)
		}
	}


	// declare export var foo: number;
	pub struct DeclareExportVariableDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportVariableDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Var)?;
			f.node(&self.id)?;
			f.token(misc::Token::Colon)?;
			f.node(&self.type_annotation)?;
			f.token(misc::Token::Semicolon)
		}
	}

	// declare export type foo = {};
	pub struct DeclareExportAliasDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportAliasDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Type)?;
			f.node(&self.id)?;
			f.token(misc::Token::Eq)?;
			f.node(&self.type_annotation)?;
			f.token(misc::Token::Semicolon)
		}
	}

	// declare export default foo;
	pub struct DeclareExportDefaultTypeDeclaration {
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportDefaultTypeDeclaration {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Export)?;
			f.token(misc::Token::Default)?;
			f.node(&self.type_annotation)?;
			f.token(misc::Token::Semicolon)
		}
	}

	// declare module.exports: {};
	pub struct DeclareExportCommonJS {
		type_annotation: Box<Annotation>,
	}
	impl misc::NodeDisplay for DeclareExportCommonJS {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Declare)?;
			f.token(misc::Token::Module)?;
			f.token(misc::Token::Period)?;
			f.token(misc::Token::Exports)?;
			f.token(misc::Token::Colon)?;
			f.node(&self.type_annotation)
		}
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
