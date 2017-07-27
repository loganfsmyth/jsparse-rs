use super::alias;
use super::misc;
use super::literal;
use super::display;

use super::misc::FirstSpecialToken;

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
	impl display::NodeDisplay for Annotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
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
	impl display::NodeDisplay for CastExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::ParenL)?;
			f.node(&self.expression)?;
			f.node(&self.type_annotation)?;
			f.token(display::Token::ParenR)
		}
	}
	impl misc::HasInOperator for CastExpression {
	    fn has_in_operator(&self) -> bool {
	        false
	    }
	}
	impl misc::FirstSpecialToken for CastExpression {}


	// <T, U>
	pub struct Parameters {
		parameters: Vec<Parameter>,
	}
	impl display::NodeDisplay for Parameters {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::AngleL)?;

			for (i, param) in self.parameters.iter().enumerate() {
				if i != 0 {
					f.token(display::Token::Comma)?;
				}

				f.node(param)?;
			}
			f.token(display::Token::AngleR)
		}
	}
	pub struct Parameter {
		variance: Variance,
		id: misc::BindingIdentifier,
		bound: Option<Box<Annotation>>,
		default: Option<Box<Annotation>>,
	}
	impl display::NodeDisplay for Parameter {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.variance)?;

			// TODO
			Ok(())
		}
	}
	pub enum PrimitiveAnnotation {
		Boolean,
		String,
		Number,
		Null,
		Void,
	}
	impl display::NodeDisplay for PrimitiveAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				PrimitiveAnnotation::Boolean => f.token(display::Token::Boolean),
				PrimitiveAnnotation::String => f.token(display::Token::String),
				PrimitiveAnnotation::Number => f.token(display::Token::Number),
				PrimitiveAnnotation::Null => f.token(display::Token::Null),
				PrimitiveAnnotation::Void => f.token(display::Token::Void),
			}
		}
	}
	pub enum LiteralAnnotation {
		Boolean(literal::Boolean),
		String(literal::String),
		Number(literal::Numeric),
	}
	impl display::NodeDisplay for LiteralAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				LiteralAnnotation::Boolean(ref id) => f.node(id),
				LiteralAnnotation::String(ref id) => f.node(id),
				LiteralAnnotation::Number(ref id) => f.node(id),
			}
		}
	}

	pub enum SpecialAnnotation {
		// any
		Any,
		// mixed
		Mixed,
		// *
		Existential,
	}
	impl display::NodeDisplay for SpecialAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				SpecialAnnotation::Any => f.token(display::Token::Any),
				SpecialAnnotation::Mixed => f.token(display::Token::Mixed),
				SpecialAnnotation::Existential => f.token(display::Token::Star),
			}
		}
	}

	pub struct MaybeAnnotation {
		argument: Box<Annotation>,
	}
	impl display::NodeDisplay for MaybeAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.argument)?;
			f.token(display::Token::Question)
		}
	}

	pub struct FunctionAnnotation {
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl display::NodeDisplay for FunctionAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.params)?;
			f.node(&self.return_type)?;
			f.token(display::Token::Semicolon)
		}
	}

	pub struct FunctionParams {
		params: Vec<FunctionParam>,
		rest: Option<FunctionRestParam>,
	}
	impl display::NodeDisplay for FunctionParams {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			for param in self.params.iter() {
				f.node(param)?;

				f.token(display::Token::Comma)?;
			}

			if let Some(ref rest) = self.rest {
				f.token(display::Token::Ellipsis)?;
				f.node(rest)?;
			}
			Ok(())
		}
	}

	pub struct FunctionParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,

		optional: bool,
	}
	impl display::NodeDisplay for FunctionParam {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			if let Some(ref id) = self.id {
				f.node(id)?;

				if self.optional {
					f.token(display::Token::Question)?;
				}
				f.token(display::Token::Colon)?;
			}
			f.node(&self.type_annotation)?;

			Ok(())
		}
	}
	pub struct FunctionRestParam {
		type_annotation: Box<Annotation>,
		id: Option<misc::BindingIdentifier>,
	}
	impl display::NodeDisplay for FunctionRestParam {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Ellipsis)?;

			if let Some(ref id) = self.id {
				f.node(id)?;

				f.token(display::Token::Colon)?;
			}
			f.node(&self.type_annotation)?;

			Ok(())
		}
	}


	pub struct ObjectAnnotation {
		exact: bool,
		properties: Vec<ObjectItem>,
	}
	impl display::NodeDisplay for ObjectAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Ellipsis)?;
			for prop in self.properties.iter() {
				f.node(prop)?;

				f.token(display::Token::Comma)?;
			}

			f.token(display::Token::Ellipsis)
		}
	}

	pub enum ObjectItem {
		Method(ObjectMethod),
		Property(ObjectProperty),
		MapProperty(ObjectMapProperty),
	}
	impl display::NodeDisplay for ObjectItem {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
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
	impl display::NodeDisplay for ObjectMethod {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			f.node(&self.params)?;
			f.node(&self.return_type)
		}
	}
	pub enum ObjectMethodId {
		Literal(misc::PropertyIdentifier),
		String(literal::String),
	}
	impl display::NodeDisplay for ObjectMethodId {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
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
	impl display::NodeDisplay for ObjectProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.variance)?;
			f.node(&self.id)?;
			if self.optional {
				f.token(display::Token::Question)?;
			}
			f.token(display::Token::Colon)?;
			f.node(&self.value)
		}
	}

	pub struct ObjectMapProperty {
		id: Option<ObjectMethodId>,
		key: Box<Annotation>,
		value: Box<Annotation>,
	}
	impl display::NodeDisplay for ObjectMapProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::SquareL)?;

			if let Some(ref id) = self.id {
				f.node(id)?;
				f.token(display::Token::Colon)?;
			}
			f.node(&self.key)?;

			f.token(display::Token::SquareR)?;
			f.token(display::Token::Colon)?;

			f.node(&self.value)
		}
	}

	// foo[]
	pub struct ArrayShorthandAnnotation {
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for ArrayShorthandAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.type_annotation)?;
			f.token(display::Token::SquareL)?;
			f.token(display::Token::SquareR)
		}
	}

	// [number, string]
	pub struct TupleAnnotation {
		items: Vec<Annotation>,
	}
	impl display::NodeDisplay for TupleAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::SquareL)?;
			for (i, anno) in self.items.iter().enumerate() {
				if i != 0 {
					f.token(display::Token::Comma)?;
				}
				f.node(anno)?;
			}
			f.token(display::Token::SquareR)
		}
	}

	//
	pub enum BindingIdentifierAnnotationList {
		Identifier(BindingIdentifierAnnotation),
		List(BindingIdentifierAnnotation, Box<BindingIdentifierAnnotationList>),
	}
	impl display::NodeDisplay for BindingIdentifierAnnotationList {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				BindingIdentifierAnnotationList::Identifier(ref id) => f.node(id),
				BindingIdentifierAnnotationList::List(ref id, ref next) => {
					f.node(id)?;
					f.token(display::Token::Comma)?;
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
	impl display::NodeDisplay for BindingIdentifierAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			Ok(())
		}
	}

	pub struct UnionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}
	impl display::NodeDisplay for UnionAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.left)?;
			f.token(display::Token::Amp)?;
			f.node(&self.right)
		}
	}
	pub struct IntersectionAnnotation {
		left: Box<Annotation>,
		right: Box<Annotation>,
	}
	impl display::NodeDisplay for IntersectionAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.left)?;
			f.token(display::Token::Bar)?;
			f.node(&self.right)
		}
	}

	pub enum TypeofAnnotation {
		Literal(LiteralAnnotation),
		Identifier(misc::BindingIdentifier),
		Member(MemberExpression),
	}
	impl display::NodeDisplay for TypeofAnnotation {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Typeof)?;

			match *self {
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
	impl display::NodeDisplay for MemberExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				MemberExpression::Identifier(ref id) => f.node(id),
				MemberExpression::Member(ref obj, ref id) => {
					f.node(obj)?;
					f.token(display::Token::Period)?;
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
	impl display::NodeDisplay for Variance {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				Variance::None => Ok(()),
				Variance::Covariant => f.token(display::Token::Plus),
				Variance::Contravariant => f.token(display::Token::Minus),
			}
		}
	}

	pub struct Interface {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,
		items: Vec<ObjectItem>,
	}
	impl display::NodeDisplay for Interface {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Interface)?;
			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			f.token(display::Token::CurlyL)?;

			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(display::Token::CurlyR)
		}
	}

	// type Foo = {};
	pub struct AliasDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,

		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for AliasDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Type)?;
			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			f.token(display::Token::Eq)?;

			f.node(&self.type_annotation)
		}
	}

	// declare function fn(){}
	pub struct DeclareFunctionDeclaration {
		id: misc::BindingIdentifier,
		params: FunctionParams,
		type_parameters: Option<Parameters>,
		return_type: Box<Annotation>,

	}
	impl display::NodeDisplay for DeclareFunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO
			Ok(())
		}
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
	impl display::NodeDisplay for DeclareClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO
			Ok(())
		}
	}

	// declare var foo: number;
	pub struct DeclareVariableDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareVariableDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Var)?;
			f.node(&self.id)?;
			f.token(display::Token::Colon)?;
			f.node(&self.type_annotation)
		}
	}

	// declare type foo = {};
	pub struct DeclareAliasDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Box<Parameters>>,
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareAliasDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Type)?;

			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}

			f.token(display::Token::Eq)?;
			f.node(&self.type_annotation)
		}
	}

	// declare module "foo" {}
	pub struct DeclareModuleDeclaration {
		source: literal::String,
		items: Vec<ModuleItem>,
	}
	impl display::NodeDisplay for DeclareModuleDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Module)?;
			f.node(&self.source)?;

			f.token(display::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(display::Token::CurlyR)
		}
	}

	// declare export function fn(){}
	pub struct DeclareExportFunctionDeclaration {
		id: misc::BindingIdentifier,
		type_parameters: Option<Parameters>,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportFunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Function)?;
			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			f.node(&self.params)?;
			f.token(display::Token::Colon)?;
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
	impl display::NodeDisplay for DeclareExportClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Class)?;
			f.node(&self.id)?;
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			if let Some(ref extends) = self.extends {
				f.token(display::Token::Extends)?;
				f.node(extends)?;
			}
			if let Some(ref implements) = self.implements {
				f.token(display::Token::Implements)?;
				f.node(implements)?;
			}
			f.token(display::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(display::Token::CurlyR)
		}
	}

	// declare export default function fn(){}
	pub struct DeclareExportDefaultFunctionDeclaration {
		id: Option<misc::BindingIdentifier>,
		type_parameters: Option<Parameters>,
		params: FunctionParams,
		return_type: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportDefaultFunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Default)?;
			f.token(display::Token::Function)?;
			if let Some(ref id) = self.id {
				f.node(id)?;
			}
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			f.node(&self.params)?;
			f.token(display::Token::Colon)?;
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
	impl display::NodeDisplay for DeclareExportDefaultClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Default)?;
			f.token(display::Token::Class)?;
			if let Some(ref id) = self.id {
				f.node(id)?;
			}
			if let Some(ref param) = self.type_parameters {
				f.node(param)?;
			}
			if let Some(ref extends) = self.extends {
				f.token(display::Token::Extends)?;
				f.node(extends)?;
			}
			if let Some(ref implements) = self.implements {
				f.token(display::Token::Implements)?;
				f.node(implements)?;
			}
			f.token(display::Token::CurlyL)?;
			for item in self.items.iter() {
				f.node(item)?;
			}
			f.token(display::Token::CurlyR)
		}
	}


	// declare export var foo: number;
	pub struct DeclareExportVariableDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportVariableDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Var)?;
			f.node(&self.id)?;
			f.token(display::Token::Colon)?;
			f.node(&self.type_annotation)?;
			f.token(display::Token::Semicolon)
		}
	}

	// declare export type foo = {};
	pub struct DeclareExportAliasDeclaration {
		id: misc::BindingIdentifier,
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportAliasDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Type)?;
			f.node(&self.id)?;
			f.token(display::Token::Eq)?;
			f.node(&self.type_annotation)?;
			f.token(display::Token::Semicolon)
		}
	}

	// declare export default foo;
	pub struct DeclareExportDefaultTypeDeclaration {
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportDefaultTypeDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Export)?;
			f.token(display::Token::Default)?;
			f.node(&self.type_annotation)?;
			f.token(display::Token::Semicolon)
		}
	}

	// declare module.exports: {};
	pub struct DeclareExportCommonJS {
		type_annotation: Box<Annotation>,
	}
	impl display::NodeDisplay for DeclareExportCommonJS {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Declare)?;
			f.token(display::Token::Module)?;
			f.token(display::Token::Period)?;
			f.token(display::Token::Exports)?;
			f.token(display::Token::Colon)?;
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
	impl display::NodeDisplay for ModuleItem {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match *self {
				ModuleItem::Function(ref item) => f.node(item),
				ModuleItem::Class(ref item) => f.node(item),
				ModuleItem::Variable(ref item) => f.node(item),
				ModuleItem::Alias(ref item) => f.node(item),
				ModuleItem::ExportFunction(ref item) => f.node(item),
				ModuleItem::ExportClass(ref item) => f.node(item),
				ModuleItem::ExportDefaultFunction(ref item) => f.node(item),
				ModuleItem::ExportDefaultClass(ref item) => f.node(item),
				ModuleItem::ExportVariable(ref item) => f.node(item),
				ModuleItem::ExportAlias(ref item) => f.node(item),
				ModuleItem::ExportDefaultType(ref item) => f.node(item),
				ModuleItem::CommonJSExport(ref item) => f.node(item),
			}
		}
	}
}
