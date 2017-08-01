use std::string;
use super::misc;
use super::alias;
use super::display;

use super::statement::{VariableStatement};
use super::declaration::{ClassDeclaration, FunctionDeclaration, LetDeclaration, ConstDeclaration};
use super::literal::{String};
use super::misc::FirstSpecialToken;

nodes!(pub struct ModuleIdentifier {
    // Identifier with "default"
    id: string::String,
});
impl display::NodeDisplay for ModuleIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.identifier(&self.id, None)
    }
}


nodes!(pub struct ImportSpecifier {
    local: misc::BindingIdentifier,
    imported: Option<ModuleIdentifier>,
});
impl display::NodeDisplay for ImportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.local);

        if let Some(ref imported) = self.imported {
            f.keyword(display::Keyword::As)?;
            f.node(imported)?;
        }
        Ok(())
    }
}

// import foo from "";
nodes!(pub struct ImportNamedDeclaration {
    default: misc::BindingIdentifier,
    source: String,
});
impl display::NodeDisplay for ImportNamedDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.node(&self.default)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// import foo, * as bar from "";
nodes!(pub struct ImportNamedAndNamespaceDeclaration {
    default: misc::BindingIdentifier,
    namespace: misc::BindingIdentifier,
    source: String,
});
impl display::NodeDisplay for ImportNamedAndNamespaceDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.node(&self.default)?;
        f.punctuator(display::Punctuator::Comma)?;
        f.punctuator(display::Punctuator::Star)?;
        f.keyword(display::Keyword::As)?;
        f.node(&self.namespace)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// import * as bar from "";
nodes!(pub struct ImportNamespaceDeclaration {
    namespace: misc::BindingIdentifier,
    source: String,
});
impl display::NodeDisplay for ImportNamespaceDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.punctuator(display::Punctuator::Star)?;
        f.keyword(display::Keyword::As)?;
        f.node(&self.namespace)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// import foo, {bar} from "";
// import foo, {bar as bar} from "";
nodes!(pub struct ImportNamedAndSpecifiersDeclaration {
    default: misc::BindingIdentifier,
    specifiers: Vec<ImportSpecifier>,
    source: String,
});
impl display::NodeDisplay for ImportNamedAndSpecifiersDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.node(&self.default)?;
        f.punctuator(display::Punctuator::Comma)?;
        f.punctuator(display::Punctuator::CurlyL)?;
        f.comma_list(&self.specifiers)?;
        f.punctuator(display::Punctuator::CurlyR)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// import {bar} from "";
// import {bar as bar} from "";
nodes!(pub struct ImportSpecifiersDeclaration {
    specifiers: Vec<ImportSpecifier>,
    source: String,
});
impl display::NodeDisplay for ImportSpecifiersDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.punctuator(display::Punctuator::CurlyL)?;
        f.comma_list(&self.specifiers)?;
        f.punctuator(display::Punctuator::CurlyR)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// export default function name() {}
nodes!(pub struct ExportDefaultFunctionDeclaration {
    id: Option<misc::BindingIdentifier>,
    params: misc::FunctionParams,
    body: misc::FunctionBody,
    fn_kind: misc::FunctionKind,
});
impl display::NodeDisplay for ExportDefaultFunctionDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.keyword(display::Keyword::Default)?;
        f.keyword(display::Keyword::Function)?;
        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        f.node(&self.params)?;
        f.node(&self.body)
    }
}


// export default class name {}
nodes!(pub struct ExportDefaultClassDeclaration {
    decorators: Vec<misc::Decorator>, // experimental
    id: Option<misc::BindingIdentifier>,
    extends: Option<Box<alias::Expression>>,
    body: misc::ClassBody,
});
impl display::NodeDisplay for ExportDefaultClassDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.keyword(display::Keyword::Default)?;
        f.keyword(display::Keyword::Class)?;
        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        if let Some(ref extends) = self.extends {
            f.keyword(display::Keyword::Extends)?;
            f.require_precedence(display::Precedence::LeftHand).node(extends)?;
        }
        f.node(&self.body)
    }
}


// export default 4;
nodes!(pub struct ExportDefaultExpression {
    expression: alias::Expression,
});
impl display::NodeDisplay for ExportDefaultExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();
        f.keyword(display::Keyword::Export)?;
        f.keyword(display::Keyword::Default)?;

        if let misc::SpecialToken::Declaration = self.expression.first_special_token() {
            f.wrap_parens().node(&self.expression)?;
        } else {
            f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
        }
        f.punctuator(display::Punctuator::Semicolon)
    }
}


// export class foo {}
nodes!(pub struct ExportClassDeclaration {
    exported: ClassDeclaration,
});
impl display::NodeDisplay for ExportClassDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.node(&self.exported)
    }
}


// export function foo() {}
nodes!(pub struct ExportFunctionDeclaration {
    exported: FunctionDeclaration,
});
impl display::NodeDisplay for ExportFunctionDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.node(&self.exported)
    }
}


// export var foo;
nodes!(pub struct ExportVarStatement {
    exported: VariableStatement,
});
impl display::NodeDisplay for ExportVarStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.node(&self.exported)
    }
}


// export let foo;
nodes!(pub struct ExportLetDeclaration {
    exported: LetDeclaration,
});
impl display::NodeDisplay for ExportLetDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.node(&self.exported)
    }
}


// export const foo;
nodes!(pub struct ExportConstDeclaration {
    exported: ConstDeclaration,
});
impl display::NodeDisplay for ExportConstDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.node(&self.exported)
    }
}


// export {foo};
// export {foo as bar};
nodes!(pub struct ExportLocalBindings {
    specifiers: Vec<LocalExportSpecifier>,
});
impl display::NodeDisplay for ExportLocalBindings {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.punctuator(display::Punctuator::CurlyL)?;
        f.comma_list(&self.specifiers)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}


nodes!(pub struct LocalExportSpecifier {
    local: misc::BindingIdentifier,
    exported: Option<ModuleIdentifier>,
});
impl display::NodeDisplay for LocalExportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.local)?;

        if let Some(ref exported) = self.exported {
            f.keyword(display::Keyword::As)?;
            f.node(exported)?;
        }
        Ok(())
    }
}


// export {foo} from "";
// export {foo as bar} from "";
nodes!(pub struct ExportSourceSpecifiers {
        specifiers: Vec<SourceExportSpecifier>,
        source: String,
});
impl display::NodeDisplay for ExportSourceSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;

        f.punctuator(display::Punctuator::CurlyL)?;
        f.comma_list(&self.specifiers)?;
        f.punctuator(display::Punctuator::CurlyR)?;

        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


nodes!(pub struct SourceExportSpecifier {
    imported: ModuleIdentifier,
    exported: Option<ModuleIdentifier>,
});
impl display::NodeDisplay for SourceExportSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.imported)?;

        if let Some(ref exported) = self.exported {
            f.keyword(display::Keyword::As)?;
            f.node(exported)?;
        }
        Ok(())
    }
}


// export * from "";
nodes!(pub struct ExportAllSpecifiers {
    source: String,
});
impl display::NodeDisplay for ExportAllSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.punctuator(display::Punctuator::Star)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// export foo from "";
nodes!(pub struct ExportNamedSpecifier {
    default: ModuleIdentifier,
    source: String,
});
impl display::NodeDisplay for ExportNamedSpecifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.node(&self.default)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// export foo, * as foo from "";
nodes!(pub struct ExportNamedAndNamespace {
    default: ModuleIdentifier,
    namespace: ModuleIdentifier,
    source: String,
});
impl display::NodeDisplay for ExportNamedAndNamespace {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.node(&self.default)?;
        f.punctuator(display::Punctuator::Comma)?;
        f.punctuator(display::Punctuator::Star)?;
        f.keyword(display::Keyword::As)?;
        f.node(&self.namespace)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// export * as foo from "";
nodes!(pub struct ExportNamespace {
    namespace: ModuleIdentifier,
    source: String,
});
impl display::NodeDisplay for ExportNamespace {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.punctuator(display::Punctuator::Star)?;
        f.keyword(display::Keyword::As)?;
        f.node(&self.namespace)?;
        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}


// export foo, {foo} from "";
// export foo, {foo as bar} from "";
nodes!(pub struct ExportNamedAndSpecifiers {
    default: ModuleIdentifier,
    specifiers: Vec<SourceExportSpecifier>,
    source: String,
});
impl display::NodeDisplay for ExportNamedAndSpecifiers {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Export)?;
        f.node(&self.default)?;
        f.punctuator(display::Punctuator::Comma)?;

        f.punctuator(display::Punctuator::CurlyL)?;
        f.comma_list(&self.specifiers)?;
        f.punctuator(display::Punctuator::CurlyR)?;

        f.keyword(display::Keyword::From)?;
        f.node(&self.source)
    }
}
