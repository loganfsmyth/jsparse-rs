use std::string;
use super::misc;
use super::flow;
use super::alias;
use super::statement;
use super::literal;
use super::display;

enum DeclaratorList<T: display::NodeDisplay> {
	Last(T),
	List(T, Box<DeclaratorList<T>>),
}
impl<T: display::NodeDisplay> display::NodeDisplay for DeclaratorList<T> {
	fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
		match self {
			&DeclaratorList::Last(ref decl) => f.node(decl),
			&DeclaratorList::List(ref decl, ref list) => {
				f.node(decl)?;
				f.token(display::Token::Comma)?;
				f.node(list)
			}
		}
	}
}

nodes!{
	// let foo, bar;
	pub struct LetDeclaration {
		declarators: DeclaratorList<LetDeclarator>,
	}
	impl display::NodeDisplay for LetDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Let)?;
			f.node(&self.declarators)
		}
	}
	pub struct LetDeclarator {
		id: misc::Pattern,
		init: Option<alias::Expression>,
	}
	impl display::NodeDisplay for LetDeclarator {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			if let Some(ref init) = self.init {
				f.token(display::Token::Eq)?;
				f.node(init)?;
			}
			Ok(())
		}
	}


	// const foo = 4, bar = 5;
	pub struct ConstDeclaration {
		declarators: DeclaratorList<ConstDeclarator>,
	}
	impl display::NodeDisplay for ConstDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Const)?;
			f.node(&self.declarators)
		}
	}
	pub struct ConstDeclarator {
		id: misc::Pattern,
		init: alias::Expression,
	}
	impl display::NodeDisplay for ConstDeclarator {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			f.token(display::Token::Eq)?;
			f.node(&self.init)
		}
	}


	// export default function name() {}
	pub struct ExportDefaultFunctionDeclaration {
		id: Option<misc::BindingIdentifier>,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	impl display::NodeDisplay for ExportDefaultFunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Export)?;
			f.token(display::Token::Default)?;
			f.token(display::Token::Function)?;
			if let Some(ref id) = self.id {
				f.node(id)?;
			}
			if let Some(ref type_parameters) = self.type_parameters {
				f.node(type_parameters)?;
			}
			f.node(&self.params)?;
			if let Some(ref return_type) = self.return_type {
				f.node(return_type)?;
			}
			f.node(&self.body)
		}
	}

	// function name() {}
	pub struct FunctionDeclaration {
		id: misc::BindingIdentifier,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	impl display::NodeDisplay for FunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Function)?;
			f.node(&self.id)?;
			if let Some(ref type_parameters) = self.type_parameters {
				f.node(type_parameters)?;
			}
			f.node(&self.params)?;
			if let Some(ref return_type) = self.return_type {
				f.node(return_type)?;
			}
			f.node(&self.body)
		}
	}

	// export default class name {}
	pub struct ExportDefaultClassDeclaration {
		decorators: Vec<misc::Decorator>, // experimental
		id: Option<misc::BindingIdentifier>,
		type_parameters: Option<Box<flow::Parameters>>,
		extends: Option<Box<alias::Expression>>,
		implements: Option<flow::BindingIdentifierAnnotationList>,
		body: misc::ClassBody,
	}
	impl display::NodeDisplay for ExportDefaultClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Export)?;
			f.token(display::Token::Default)?;
			f.token(display::Token::Class)?;
			if let Some(ref id) = self.id {
				f.node(id)?;
			}
			if let Some(ref type_parameters) = self.type_parameters {
	 			f.node(type_parameters)?;
	 		}
			f.node(&self.body)
		}
	}

	// class name {}
	pub struct ClassDeclaration {
		decorators: Vec<misc::Decorator>, // experimental
		id: misc::BindingIdentifier,
		extends: Option<Box<alias::Expression>>,
		implements: Option<flow::BindingIdentifierAnnotationList>,
		body: misc::ClassBody,

		type_parameters: Option<flow::Parameters>,
	}
	impl display::NodeDisplay for ClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO
			Ok(())
		}
	}


	// import ... from "";
	pub struct ImportDeclaration {
		specifiers: ImportSpecifiers,
		source: literal::String,
	}
	pub enum FlowImportKind {
		// Flow extension
		Type,
		Typeof,
	}
	pub enum ImportNamespaceKind {
		Normal,

		// Flow extension
		Typeof,
	}
	impl display::NodeDisplay for ImportDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Import)?;
			f.node(&self.specifiers)?;
			f.token(display::Token::From)?;
			f.node(&self.source)
		}
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
	impl display::NodeDisplay for ImportSpecifiers {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			Ok(())
		}
	}
	pub enum ImportSpecifier {
		Named {
			local: misc::BindingIdentifier,
		},
		NamedAndAliased {
			name: ModuleIdentifier,
			local: misc::BindingIdentifier,
		},
	}
	pub struct ModuleIdentifier {
		// Identifier with "default"
		id: string::String,
	}

	pub struct ExportDeclaration {
		decl_type: ExportType,
	}
	impl display::NodeDisplay for ExportDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO
			Ok(())
		}
	}

	pub enum ExportType {
		// export default class {}
		DefaultClass(ExportDefaultClassDeclaration),
		// export default function() {}
		DefaultFunction(ExportDefaultFunctionDeclaration),

		// export default 4;
		// TODO: Whatever expression here can't start with "function" or "class" or "async"
		Default(alias::Expression),

		// export class foo {}
		Class(ClassDeclaration),
		// export function foo() {}
		Function(FunctionDeclaration),
		// export var foo;
		Variable(statement::VariableStatement),
		Let(LetDeclaration),
		Const(ConstDeclaration),

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
		// experimental
		NamedAndSpecifiers(
			ModuleIdentifier,
			Vec<(SourceExportSpecifier, FlowImportKind)>,
			literal::String
		),
	}
	pub enum LocalExportSpecifier {
		Named(misc::BindingIdentifier),
		NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
	}
	pub enum SourceExportSpecifier {
		Named(misc::BindingIdentifier),
		NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
	}
}
