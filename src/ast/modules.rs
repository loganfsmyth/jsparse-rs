use std::string;
use super::misc;
use super::flow;
use super::alias;
use super::statement;
use super::declaration;
use super::literal;
use super::display;

use super::misc::FirstSpecialToken;

nodes!{
  pub struct ModuleIdentifier {
    // Identifier with "default"
    id: string::String,
  }
  impl display::NodeDisplay for ModuleIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.identifier(&self.id, None)
    }
  }

  pub enum FlowImportKind {
    // Flow extension
    Type,
    Typeof,
  }
  impl display::NodeDisplay for FlowImportKind {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &FlowImportKind::Type => f.token(display::Token::Type),
        &FlowImportKind::Typeof => f.token(display::Token::Typeof),
      }
    }
  }

  pub enum ImportSpecifier {
    Named(misc::BindingIdentifier),
    NamedAndAliased(ModuleIdentifier, misc::BindingIdentifier),
  }
  impl display::NodeDisplay for ImportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &ImportSpecifier::Named(ref id) => {
          f.node(id)
        }
        &ImportSpecifier::NamedAndAliased(ref module, ref id) => {
          f.node(module)?;
          f.token(display::Token::As)?;
          f.node(id)
        }
      }
    }
  }
  pub enum TypedImportSpecifier {
    Named(Option<FlowImportKind>, ModuleIdentifier),
    NamedAndAliased(Option<FlowImportKind>, ModuleIdentifier, ModuleIdentifier),
  }
  impl display::NodeDisplay for TypedImportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &TypedImportSpecifier::Named(ref kind, ref id) => {
          if let &Some(ref kind) = kind {
            f.node(kind)?;
          }
          f.node(id)
        }
        &TypedImportSpecifier::NamedAndAliased(ref kind, ref module, ref id) => {
          if let &Some(ref kind) = kind {
            f.node(kind)?;
          }
          f.node(module)?;
          f.token(display::Token::As)?;
          f.node(id)
        }
      }
    }
  }

  // import foo from "";
  pub struct ImportNamedDeclaration {
    default: misc::BindingIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamedDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.node(&self.default)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import foo, * as bar from "";
  pub struct ImportNamedAndNamespaceDeclaration {
    default: misc::BindingIdentifier,
    namespace: misc::BindingIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamedAndNamespaceDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.node(&self.default)?;
      f.token(display::Token::Comma)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::As)?;
      f.node(&self.namespace)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import * as bar from "";
  pub struct ImportNamespaceDeclaration {
    namespace: misc::BindingIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamespaceDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::As)?;
      f.node(&self.namespace)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import foo, {bar} from "";
  // import foo, {bar as bar} from "";
  // import foo, {type bar} from "";
  // import foo, {type bar as bar} from "";
  // import foo, {typeof bar} from "";
  // import foo, {typeof bar as bar} from "";
  pub struct ImportNamedAndSpecifiersDeclaration {
    default: misc::BindingIdentifier,
    specifiers: Vec<TypedImportSpecifier>,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamedAndSpecifiersDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.node(&self.default)?;
      f.token(display::Token::Comma)?;
      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import {bar} from "";
  // import {bar as bar} from "";
  // import {type bar} from "";
  // import {type bar as bar} from "";
  // import {typeof bar} from "";
  // import {typeof bar as bar} from "";
  pub struct ImportSpecifiersDeclaration {
    specifiers: Vec<TypedImportSpecifier>,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportSpecifiersDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import type foo from "";
  // import typeof foo from "";
  pub struct ImportNamedTypeDeclaration {
    kind: FlowImportKind,
    default: misc::BindingIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamedTypeDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;

      f.node(&self.kind)?;
      f.node(&self.default)?;

      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import typeof * as bar from "";
  pub struct ImportNamespaceTypeofDeclaration {
    namespace: misc::BindingIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamespaceTypeofDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.token(display::Token::Typeof)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::As)?;
      f.node(&self.namespace)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import type foo, {bar} from "";
  // import type foo, {bar as bar} from "";
  // import typeof foo, {bar} from "";
  // import typeof foo, {bar as bar} from "";
  pub struct ImportNamedAndSpecifiersTypeDeclaration {
    default: misc::BindingIdentifier,
    specifiers: Vec<ImportSpecifier>,
    kind: FlowImportKind,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportNamedAndSpecifiersTypeDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.node(&self.kind)?;
      f.node(&self.default)?;
      f.token(display::Token::Comma)?;
      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // import type {bar} from "";
  // import type {bar as bar} from "";
  // import typeof {bar} from "";
  // import typeof {bar as bar} from "";
  pub struct ImportSpecifiersTypeDeclaration {
    specifiers: Vec<ImportSpecifier>,
    kind: FlowImportKind,
    source: literal::String,
  }
  impl display::NodeDisplay for ImportSpecifiersTypeDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Import)?;
      f.node(&self.kind)?;
      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
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

  // TODO: Whatever expression here can't start with "function" or "class" or "async"
  // export default 4;
  pub struct ExportDefaultExpression {
    expression: alias::Expression,
  }
  impl display::NodeDisplay for ExportDefaultExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.token(display::Token::Default)?;

      if let misc::SpecialToken::Declaration = self.expression.first_special_token() {
        f.with_parens(|f| f.node(&self.expression))?;
      } else {
        f.node(&self.expression)?;
      }
      f.token(display::Token::Semicolon)
    }
  }

  // export class foo {}
  pub struct ExportClassDeclaration {
    exported: declaration::ClassDeclaration,
  }
  impl display::NodeDisplay for ExportClassDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }

  // export function foo() {}
  pub struct ExportFunctionDeclaration {
    exported: declaration::FunctionDeclaration,
  }
  impl display::NodeDisplay for ExportFunctionDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }

  // export var foo;
  pub struct ExportVarStatement {
    exported: statement::VariableStatement,
  }
  impl display::NodeDisplay for ExportVarStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }
  // export let foo;
  pub struct ExportLetDeclaration {
    exported: declaration::LetDeclaration,
  }
  impl display::NodeDisplay for ExportLetDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }
  // export const foo;
  pub struct ExportConstDeclaration {
    exported: declaration::ConstDeclaration,
  }
  impl display::NodeDisplay for ExportConstDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }
  // export type Foo = {};
  pub struct ExportFlowAlias {
    exported: flow::AliasDeclaration,
  }
  impl display::NodeDisplay for ExportFlowAlias {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.node(&self.exported)
    }
  }

  // export {foo};
  // export {foo as bar};
  pub struct ExportLocalBindings {
    specifiers: Vec<LocalExportSpecifier>,
  }
  impl display::NodeDisplay for ExportLocalBindings {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)
    }
  }
  pub enum LocalExportSpecifier {
    Named(misc::BindingIdentifier),
    NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
  }
  impl display::NodeDisplay for LocalExportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &LocalExportSpecifier::Named(ref id) => f.node(id),
        &LocalExportSpecifier::NamedAndAliased(ref id, ref mod_id) => {
          f.node(id)?;
          f.token(display::Token::As)?;
          f.node(mod_id)
        }
      }
    }
  }

  // export {foo} from "";
  // export {foo as bar} from "";
  // export {type foo} from "";
  // export {type foo as bar} from "";
  // export {typeof foo} from "";
  // export {typeof foo as bar} from "";
  pub struct ExportSourceSpecifiers {
      specifiers: Vec<TypedSourceExportSpecifier>,
      source: literal::String,
  }
  impl display::NodeDisplay for ExportSourceSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;

      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;

      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  pub enum SourceExportSpecifier {
    Named(ModuleIdentifier),
    NamedAndAliased(ModuleIdentifier, ModuleIdentifier),
  }
  impl display::NodeDisplay for SourceExportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &SourceExportSpecifier::Named(ref id) => f.node(id),
        &SourceExportSpecifier::NamedAndAliased(ref id, ref mod_id) => {
          f.node(id)?;
          f.token(display::Token::As)?;
          f.node(mod_id)
        }
      }
    }
  }

  pub enum TypedSourceExportSpecifier {
    Named(Option<FlowImportKind>, ModuleIdentifier),
    NamedAndAliased(Option<FlowImportKind>, ModuleIdentifier, ModuleIdentifier),
  }
  impl display::NodeDisplay for TypedSourceExportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      match self {
        &TypedSourceExportSpecifier::Named(ref kind, ref id) => {
          if let &Some(ref kind) = kind {
            f.node(kind)?;
          }
          f.node(id)
        }
        &TypedSourceExportSpecifier::NamedAndAliased(ref kind, ref id, ref mod_id) => {
          if let &Some(ref kind) = kind {
            f.node(kind)?;
          }
          f.node(id)?;
          f.token(display::Token::As)?;
          f.node(mod_id)
        }
      }
    }
  }

  // export type {foo} from "";
  // export type {foo as bar} from "";
  pub struct ExportFlowtypeSourceSpecifiers {
      kind: FlowImportKind,
      specifiers: Vec<SourceExportSpecifier>,
      source: literal::String,
  }
  impl display::NodeDisplay for ExportFlowtypeSourceSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.node(&self.kind)?;

      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;

      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // export * from "";
  pub struct ExportAllSpecifiers {
    source: literal::String,
  }
  impl display::NodeDisplay for ExportAllSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // export foo from "";
  pub struct ExportNamedSpecifier {
    default: ModuleIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ExportNamedSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.node(&self.default)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // export foo, * as foo from "";
  pub struct ExportNamedAndNamespace {
    default: ModuleIdentifier,
    namespace: ModuleIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ExportNamedAndNamespace {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.node(&self.default)?;
      f.token(display::Token::Comma)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::As)?;
      f.node(&self.namespace)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // export * as foo from "";
  pub struct ExportNamespace {
    namespace: ModuleIdentifier,
    source: literal::String,
  }
  impl display::NodeDisplay for ExportNamespace {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.token(display::Token::Star)?;
      f.token(display::Token::As)?;
      f.node(&self.namespace)?;
      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }

  // export foo, {foo} from "";
  // export foo, {foo as bar} from "";
  // export foo, {type foo} from "";
  // export foo, {type foo as bar} from "";
  pub struct ExportNamedAndSpecifiers {
      default: ModuleIdentifier,
      specifiers: Vec<TypedSourceExportSpecifier>,
      source: literal::String,
  }
  impl display::NodeDisplay for ExportNamedAndSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Export)?;
      f.node(&self.default)?;
      f.token(display::Token::Comma)?;

      f.token(display::Token::CurlyL)?;
      for (i, spec) in self.specifiers.iter().enumerate() {
        if i != 0 {
          f.token(display::Token::Comma)?;
        }

        f.node(spec)?;
      }
      f.token(display::Token::CurlyR)?;

      f.token(display::Token::From)?;
      f.node(&self.source)
    }
  }
}
