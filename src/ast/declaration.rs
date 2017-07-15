use std::string;
use super::misc;
use super::flow;
use super::alias;
use super::statement;
use super::literal;

// let foo, bar;
pub struct LexicalDeclaration {
	kind: LexicalKind,
	declarations: Vec<LexicalDeclarator>,
	position: misc::MaybePosition,
}
pub struct LexicalDeclarator {
	id: misc::Pattern,
	init: Option<alias::Expression>,
	position: misc::MaybePosition,
}
enum LexicalDeclaratorList {
	Declarator(LexicalDeclarator),
	List(LexicalDeclarator, Box<LexicalDeclaratorList>),
}
enum LexicalKind {
	Let,
	Const,
}





// export default function name() {}
pub struct ExportDefaultFunctionDeclaration {
	id: Option<misc::BindingIdentifier>,
	params: misc::FunctionParams,
	body: misc::FunctionBody,
	fn_kind: misc::FunctionKind,
	position: misc::MaybePosition,

	// Flow extension
	type_parameters: Option<flow::Parameters>,
	return_type: Option<Box<flow::Annotation>>,
}

// function name() {}
pub struct FunctionDeclaration {
	id: misc::BindingIdentifier,
	params: misc::FunctionParams,
	body: misc::FunctionBody,
	fn_kind: misc::FunctionKind,
	position: misc::MaybePosition,

	// Flow extension
	type_parameters: Option<flow::Parameters>,
	return_type: Option<Box<flow::Annotation>>,
}

// export default class name {}
pub struct ExportDefaultClassDeclaration {
	decorators: Vec<misc::Decorator>, // experimental
	id: Option<misc::BindingIdentifier>,
	extends: Option<Box<alias::Expression>>,
	implements: Option<flow::BindingIdentifierAnnotationList>,
	body: misc::ClassBody,
	position: misc::MaybePosition,

	type_parameters: Option<flow::Parameters>,
}

// class name {}
pub struct ClassDeclaration {
	decorators: Vec<misc::Decorator>, // experimental
	id: misc::BindingIdentifier,
	extends: Option<Box<alias::Expression>>,
	implements: Option<flow::BindingIdentifierAnnotationList>,
	body: misc::ClassBody,
	position: misc::MaybePosition,

	type_parameters: Option<flow::Parameters>,
}


// import ... from "";
pub struct ImportDeclaration {
	specifiers: ImportSpecifiers,
	source: literal::String,
	position: misc::MaybePosition,
}
enum FlowImportKind {
	// Flow extension
	Type,
	Typeof,
}
enum ImportNamespaceKind {
	Normal,

	// Flow extension
	Typeof,
}

// TODO: This is really hard to read
pub enum ImportSpecifiers {
	// foo
	Named {
		default_id: misc::BindingIdentifier,
	},
	// foo, * as bar
	NamedAndNamespace {
		default_id: misc::BindingIdentifier,
		namespace_id: misc::BindingIdentifier,
	},
	// * as bar
	Namespace {
		namespace_id: misc::BindingIdentifier,
	},
	// foo, {bar}
	// foo, {bar as bar}
	NamedAndSpecifiers {
		default_id: misc::BindingIdentifier,
		specifiers: Vec<(FlowImportKind, ImportSpecifier)>,
	},
	// {bar}
	// {bar as bar}
	Specifiers {
		specifiers: Vec<(FlowImportKind, ImportSpecifier)>,
	},

	// type foo
	// typeof foo
	NamedType {
		default_id: misc::BindingIdentifier,
		kind: FlowImportKind,
	},
	// typeof * as bar
	NamespaceTypeof {
		namespace_id: misc::BindingIdentifier,
	},
	// type foo, {bar}
	// type foo, {bar as bar}
	// typeof foo, {bar}
	// typeof foo, {bar as bar}
	NamedAndSpecifiersType {
		default_id: misc::BindingIdentifier,
		specifiers: Vec<ImportSpecifier>,
		kind: FlowImportKind,
	},
	// type {bar}
	// type {bar as bar}
	// typeof {bar}
	// typeof {bar as bar}
	SpecifiersType {
		specifiers: Vec<ImportSpecifier>,
		kind: FlowImportKind,
	},
}
enum ImportSpecifier {
	Named {
		local: misc::BindingIdentifier,
	},
	NamedAndAliased {
		name: ModuleIdentifier,
		local: misc::BindingIdentifier,
	},
}
struct ModuleIdentifier {
	// Identifier with "default"
	id: string::String,
	position: misc::MaybePosition,
}

pub struct ExportDeclaration {
	decl_type: ExportType,
	position: misc::MaybePosition,
}
enum ExportType {
	// export default class {}
	DefaultClass(ExportDefaultClassDeclaration),
	// export default function() {}
	DefaultFunction(ExportDefaultFunctionDeclaration),

	// export default 4;
	Default(alias::Expression),

	// export class foo {}
	Class(ClassDeclaration),
	// export function foo() {}
	Function(FunctionDeclaration),
	// export var foo;
	Variable(statement::VariableStatement),
	Lexical(LexicalDeclaration),

	// export {foo}
	// export {foo as bar}
	LocalSpecifiers(Vec<LocalExportSpecifier>),

	// export type Foo = {};
	FlowDeclaration(flow::AliasDeclaration),

	// export {foo} from "";
	// export {foo as bar} from "";
	SourceSpecifiers(Vec<(SourceExportSpecifier, FlowImportKind)>, literal::String),

	// export type {foo} from "";
	// export type {foo as bar} from "";
	SourceSpecifiersFlow(Vec<SourceExportSpecifier>, literal::String),

	// export * from "";
	All(literal::String),

	// export foo from "";
	Named(ModuleIdentifier, literal::String), // experimental
	// export foo, * as foo from "";
	NamedAndNamespace(ModuleIdentifier, ModuleIdentifier, literal::String), // experimental
	// export * as foo from "";
	Namespace(ModuleIdentifier, literal::String), // experimental
	// export foo, {foo} from "";
	// export foo, {foo as bar} from "";
	// export foo, {type foo} from "";
	// export foo, {type foo as bar} from "";
	NamedAndSpecifiers(ModuleIdentifier, Vec<(SourceExportSpecifier, FlowImportKind)>, literal::String), // experimental
}
enum LocalExportSpecifier {
	Named(misc::BindingIdentifier),
	NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
}
enum SourceExportSpecifier {
	Named(misc::BindingIdentifier),
	NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
}

