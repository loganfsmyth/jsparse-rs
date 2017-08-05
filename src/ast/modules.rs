use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   FirstSpecialToken, SpecialToken};

use ast::statement::{VariableStatement, LetDeclaration, ConstDeclaration};
use ast::classes::ClassDeclaration;
use ast::functions::FunctionDeclaration;
use ast::literal::String;
use ast::alias;

use ast::general::BindingIdentifier;


// identifiers used as names of imports and exports
node!(pub struct ModuleIdentifier {
    // Identifier with "default"
    pub value: string::String,
    pub raw: Option<string::String>,
});

impl NodeDisplay for ModuleIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl ModuleIdentifier {
    pub fn new<T: Into<string::String>>(s: T) -> ModuleIdentifier {
        ModuleIdentifier {
            value: s.into(),
            raw: None,
            position: None,
        }
    }
}


node!(pub struct ImportSpecifier {
    pub local: BindingIdentifier,
    pub imported: Option<ModuleIdentifier>,
});
impl NodeDisplay for ImportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.local)?;

        if let Some(ref imported) = self.imported {
            f.keyword(Keyword::As);
            f.node(imported)?;
        }
        Ok(())
    }
}

// import foo from "";
node!(pub struct ImportNamedDeclaration {
    pub default: BindingIdentifier,
    pub source: String,
});
impl NodeDisplay for ImportNamedDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.node(&self.default)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// import foo, * as bar from "";
node!(pub struct ImportNamedAndNamespaceDeclaration {
    pub default: BindingIdentifier,
    pub namespace: BindingIdentifier,
    pub source: String,
});
impl NodeDisplay for ImportNamedAndNamespaceDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::As);
        f.node(&self.namespace)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// import * as bar from "";
node!(pub struct ImportNamespaceDeclaration {
    pub namespace: BindingIdentifier,
    pub source: String,
});
impl NodeDisplay for ImportNamespaceDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::As);
        f.node(&self.namespace)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// import foo, {bar} from "";
// import foo, {bar as bar} from "";
node!(pub struct ImportNamedAndSpecifiersDeclaration {
    pub default: BindingIdentifier,
    pub specifiers: Vec<ImportSpecifier>,
    pub source: String,
});
impl NodeDisplay for ImportNamedAndSpecifiersDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        f.wrap_curly().comma_list(&self.specifiers)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// import {bar} from "";
// import {bar as bar} from "";
node!(pub struct ImportSpecifiersDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: String,
});
impl NodeDisplay for ImportSpecifiersDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.wrap_curly().comma_list(&self.specifiers)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}




// export default 4;
node!(pub struct ExportDefaultExpression {
    pub expression: alias::Expression,
});
impl NodeDisplay for ExportDefaultExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        f.keyword(Keyword::Export);
        f.keyword(Keyword::Default);

        if let SpecialToken::Declaration = self.expression.first_special_token() {
            f.wrap_parens().node(&self.expression)?;
        } else {
            f.require_precedence(Precedence::Assignment).node(
                &self.expression,
            )?;
        }
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}


// export class foo {}
node!(pub struct ExportClassDeclaration {
    pub exported: ClassDeclaration,
});
impl NodeDisplay for ExportClassDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.node(&self.exported)
    }
}


// export function foo() {}
node!(pub struct ExportFunctionDeclaration {
    pub exported: FunctionDeclaration,
});
impl NodeDisplay for ExportFunctionDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.node(&self.exported)
    }
}


// export var foo;
node!(pub struct ExportVarStatement {
    pub exported: VariableStatement,
});
impl NodeDisplay for ExportVarStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.node(&self.exported)
    }
}


// export let foo;
node!(pub struct ExportLetDeclaration {
    pub exported: LetDeclaration,
});
impl NodeDisplay for ExportLetDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.node(&self.exported)
    }
}


// export const foo;
node!(pub struct ExportConstDeclaration {
    pub exported: ConstDeclaration,
});
impl NodeDisplay for ExportConstDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.node(&self.exported)
    }
}


// export {foo};
// export {foo as bar};
node!(#[derive(Default)] pub struct ExportLocalBindings {
    pub specifiers: Vec<LocalExportSpecifier>,
});
impl NodeDisplay for ExportLocalBindings {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.wrap_curly().comma_list(&self.specifiers)?;
        Ok(())
    }
}


node!(pub struct LocalExportSpecifier {
    pub local: BindingIdentifier,
    pub exported: Option<ModuleIdentifier>,
});
impl NodeDisplay for LocalExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.local)?;

        if let Some(ref exported) = self.exported {
            f.keyword(Keyword::As);
            f.node(exported)?;
        }
        Ok(())
    }
}


// export {foo} from "";
// export {foo as bar} from "";
node!(pub struct ExportSourceSpecifiers {
    pub specifiers: Vec<SourceExportSpecifier>,
    pub source: String,
});
impl NodeDisplay for ExportSourceSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        f.wrap_curly().comma_list(&self.specifiers)?;

        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


node!(pub struct SourceExportSpecifier {
    pub imported: ModuleIdentifier,
    pub exported: Option<ModuleIdentifier>,
});
impl NodeDisplay for SourceExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.imported)?;

        if let Some(ref exported) = self.exported {
            f.keyword(Keyword::As);
            f.node(exported)?;
        }
        Ok(())
    }
}


// export * from "";
node!(pub struct ExportAllSpecifiers {
    pub source: String,
});
impl NodeDisplay for ExportAllSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// export foo from "";
node!(pub struct ExportNamedSpecifier {
    pub default: ModuleIdentifier,
    pub source: String,
});
impl NodeDisplay for ExportNamedSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.node(&self.default)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// export foo, * as foo from "";
node!(pub struct ExportNamedAndNamespace {
    pub default: ModuleIdentifier,
    pub namespace: ModuleIdentifier,
    pub source: String,
});
impl NodeDisplay for ExportNamedAndNamespace {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::As);
        f.node(&self.namespace)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// export * as foo from "";
node!(pub struct ExportNamespace {
    pub namespace: ModuleIdentifier,
    pub source: String,
});
impl NodeDisplay for ExportNamespace {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::As);
        f.node(&self.namespace)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}


// export foo, {foo} from "";
// export foo, {foo as bar} from "";
node!(pub struct ExportNamedAndSpecifiers {
    pub default: ModuleIdentifier,
    pub specifiers: Vec<SourceExportSpecifier>,
    pub source: String,
});
impl NodeDisplay for ExportNamedAndSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        f.wrap_curly().comma_list(&self.specifiers)?;
        f.keyword(Keyword::From);
        f.node(&self.source)
    }
}
