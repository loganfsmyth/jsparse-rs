use std::string;
use super::misc;
use super::alias;
use super::display;

use super::statement::{VariableStatement};
use super::declaration::{ClassDeclaration, FunctionDeclaration, LetDeclaration, ConstDeclaration};
use super::literal::{String};
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


    pub enum ImportSpecifier {
        Named(misc::BindingIdentifier),
        NamedAndAliased(ModuleIdentifier, misc::BindingIdentifier),
    }
    impl display::NodeDisplay for ImportSpecifier {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            match *self {
                ImportSpecifier::Named(ref id) => {
                    f.node(id)
                }
                ImportSpecifier::NamedAndAliased(ref module, ref id) => {
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
        source: String,
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
        source: String,
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
        source: String,
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
    pub struct ImportNamedAndSpecifiersDeclaration {
        default: misc::BindingIdentifier,
        specifiers: Vec<ImportSpecifier>,
        source: String,
    }
    impl display::NodeDisplay for ImportNamedAndSpecifiersDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Import)?;
            f.node(&self.default)?;
            f.token(display::Token::Comma)?;
            f.token(display::Token::CurlyL)?;
            f.comma_list(&self.specifiers)?;
            f.token(display::Token::CurlyR)?;
            f.token(display::Token::From)?;
            f.node(&self.source)
        }
    }


    // import {bar} from "";
    // import {bar as bar} from "";
    pub struct ImportSpecifiersDeclaration {
        specifiers: Vec<ImportSpecifier>,
        source: String,
    }
    impl display::NodeDisplay for ImportSpecifiersDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Import)?;
            f.token(display::Token::CurlyL)?;
            f.comma_list(&self.specifiers)?;
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
    }
    impl display::NodeDisplay for ExportDefaultFunctionDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;
            f.token(display::Token::Default)?;
            f.token(display::Token::Function)?;
            if let Some(ref id) = self.id {
                f.node(id)?;
            }
            f.node(&self.params)?;
            f.node(&self.body)
        }
    }


    // export default class name {}
    pub struct ExportDefaultClassDeclaration {
        decorators: Vec<misc::Decorator>, // experimental
        id: Option<misc::BindingIdentifier>,
        extends: Option<Box<alias::Expression>>,
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
            if let Some(ref extends) = self.extends {
                f.token(display::Token::Extends)?;
                f.require_precedence(display::Precedence::LeftHand).node(extends)?;
            }
            f.node(&self.body)
        }
    }


    // export default 4;
    pub struct ExportDefaultExpression {
        expression: alias::Expression,
    }
    impl display::NodeDisplay for ExportDefaultExpression {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            let mut f = f.allow_in();
            f.token(display::Token::Export)?;
            f.token(display::Token::Default)?;

            if let misc::SpecialToken::Declaration = self.expression.first_special_token() {
                f.wrap_parens().node(&self.expression)?;
            } else {
                f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
            }
            f.token(display::Token::Semicolon)
        }
    }


    // export class foo {}
    pub struct ExportClassDeclaration {
        exported: ClassDeclaration,
    }
    impl display::NodeDisplay for ExportClassDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;

            f.node(&self.exported)
        }
    }


    // export function foo() {}
    pub struct ExportFunctionDeclaration {
        exported: FunctionDeclaration,
    }
    impl display::NodeDisplay for ExportFunctionDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;

            f.node(&self.exported)
        }
    }


    // export var foo;
    pub struct ExportVarStatement {
        exported: VariableStatement,
    }
    impl display::NodeDisplay for ExportVarStatement {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;

            f.node(&self.exported)
        }
    }


    // export let foo;
    pub struct ExportLetDeclaration {
        exported: LetDeclaration,
    }
    impl display::NodeDisplay for ExportLetDeclaration {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;

            f.node(&self.exported)
        }
    }


    // export const foo;
    pub struct ExportConstDeclaration {
        exported: ConstDeclaration,
    }
    impl display::NodeDisplay for ExportConstDeclaration {
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
            f.comma_list(&self.specifiers)?;
            f.token(display::Token::CurlyR)
        }
    }


    pub enum LocalExportSpecifier {
        Named(misc::BindingIdentifier),
        NamedAndAliased(misc::BindingIdentifier, ModuleIdentifier),
    }
    impl display::NodeDisplay for LocalExportSpecifier {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            match *self {
                LocalExportSpecifier::Named(ref id) => f.node(id),
                LocalExportSpecifier::NamedAndAliased(ref id, ref mod_id) => {
                    f.node(id)?;
                    f.token(display::Token::As)?;
                    f.node(mod_id)
                }
            }
        }
    }


    // export {foo} from "";
    // export {foo as bar} from "";
    pub struct ExportSourceSpecifiers {
            specifiers: Vec<SourceExportSpecifier>,
            source: String,
    }
    impl display::NodeDisplay for ExportSourceSpecifiers {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;

            f.token(display::Token::CurlyL)?;
            f.comma_list(&self.specifiers)?;
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
            match *self {
                SourceExportSpecifier::Named(ref id) => f.node(id),
                SourceExportSpecifier::NamedAndAliased(ref id, ref mod_id) => {
                    f.node(id)?;
                    f.token(display::Token::As)?;
                    f.node(mod_id)
                }
            }
        }
    }


    // export * from "";
    pub struct ExportAllSpecifiers {
        source: String,
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
        source: String,
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
        source: String,
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
        source: String,
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
    pub struct ExportNamedAndSpecifiers {
            default: ModuleIdentifier,
            specifiers: Vec<SourceExportSpecifier>,
            source: String,
    }
    impl display::NodeDisplay for ExportNamedAndSpecifiers {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.token(display::Token::Export)?;
            f.node(&self.default)?;
            f.token(display::Token::Comma)?;

            f.token(display::Token::CurlyL)?;
            f.comma_list(&self.specifiers)?;
            f.token(display::Token::CurlyR)?;

            f.token(display::Token::From)?;
            f.node(&self.source)
        }
    }
}
