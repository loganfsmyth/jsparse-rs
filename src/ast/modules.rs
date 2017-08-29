use std::string;

use ast::{MaybeTokenPosition, KeywordData, KeywordWrappedData};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadRestriction};

use ast::statement::{VariableStatement, LetDeclaration, ConstDeclaration};
use ast::classes::ClassDeclaration;
use ast::functions::FunctionDeclaration;
use ast::literal::String;
use ast::alias;

use ast::general::{BindingIdentifier, ReferenceIdentifier};


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
impl<T: Into<string::String>> From<T> for ModuleIdentifier {
    fn from(value: T) -> ModuleIdentifier {
        ModuleIdentifier {
            value: value.into(),
            raw: None,
            position: None,
        }
    }
}

node_enum!(@node_display pub enum ImportSpecifier {
    Normal(NormalImportSpecifier),
    Aliased(AliasedImportSpecifier),
});


node!(pub struct NormalImportSpecifier {
    pub local: BindingIdentifier,
});
impl NodeDisplay for NormalImportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.local)
    }
}
impl From<BindingIdentifier> for NormalImportSpecifier {
    fn from(b: BindingIdentifier) -> NormalImportSpecifier {
        NormalImportSpecifier {
            local: b,
            position: None,
        }
    }
}


node!(pub struct AliasedImportSpecifier {
    pub imported: ModuleIdentifier,
    pub token_as: KeywordWrappedData,
    pub local: BindingIdentifier,
});
impl NodeDisplay for AliasedImportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.imported)?;
        f.keyword(Keyword::As, &self.token_as);
        f.node(&self.local)
    }
}

// import foo from "";
node!(pub struct ImportNamedDeclaration {
    pub token_import: KeywordData,
    pub default: BindingIdentifier,
    pub token_from: KeywordData,
    pub source: String,
});
impl NodeDisplay for ImportNamedDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import, &self.token_import);
        f.node(&self.default)?;
        f.keyword(Keyword::From, &self.token_from);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_import_named {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ImportNamedDeclaration {
                default: "foo".into(),
                source: "file.js".into(),
                position: None,
            },
            "import foo from'file.js';"
        );
    }
}


// import foo, * as bar from "";
node!(pub struct ImportNamedAndNamespaceDeclaration {
    pub token_import: KeywordData,
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
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_import_named_and_namespace {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ImportNamedAndNamespaceDeclaration {
                default: "foo".into(),
                namespace: "namespaceObj".into(),
                source: "file.js".into(),
                position: None,
            },
            "import foo,*as namespaceObj from'file.js';"
        );
    }
}


// import * as bar from "";
node!(pub struct ImportNamespaceDeclaration {
    pub token_import: KeywordData,
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
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_import_namespace {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ImportNamespaceDeclaration {
                namespace: "namespaceObj".into(),
                source: "file.js".into(),
                position: None,
            },
            "import*as namespaceObj from'file.js';"
        );
    }
}


// import foo, {bar} from "";
// import foo, {bar as bar} from "";
node!(pub struct ImportNamedAndSpecifiersDeclaration {
    pub token_import: KeywordData,
    pub default: BindingIdentifier,
    pub specifiers: Vec<(ImportSpecifier, KeywordData)>,
    pub last_specifier: Option<ImportSpecifier>,
    pub source: String,
});
impl NodeDisplay for ImportNamedAndSpecifiersDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        {
            let mut f = f.wrap_curly();
            f.comma_list(&self.specifiers)?;
            f.node(&self.last_specifier)?;
        }
        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_import_named_and_specifiers {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ImportNamedAndSpecifiersDeclaration {
                default: "foo".into(),
                specifiers: vec![],
                source: "file.js".into(),
                position: None,
            },
            "import foo,{}from'file.js';"
        );
    }

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ImportNamedAndSpecifiersDeclaration {
                default: "foo".into(),
                specifiers: vec![
                    BindingIdentifier::from("spec1").into(),
                    BindingIdentifier::from("spec2").into(),
                    ImportSpecifier {
                        local: BindingIdentifier::from("spec3"),
                        imported: ModuleIdentifier::from("fooImport").into(),
                        position: None,
                    },
                ],
                source: "file.js".into(),
                position: None,
            },
            "import foo,{spec1,spec2,spec3 as fooImport}from'file.js';"
        );
    }
}

// import {bar} from "";
// import {bar as bar} from "";
node!(pub struct ImportSpecifiersDeclaration {
    pub token_import: KeywordData,
    pub specifiers: Vec<(ImportSpecifier, KeywordData)>,
    pub last_specifier: ImportSpecifier,
    pub source: String,
});
impl NodeDisplay for ImportSpecifiersDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        {
            let mut f = f.wrap_curly();
            f.comma_list(&self.specifiers)?;
            f.node(&self.last_specifier)?;
        }
        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_import_specifiers {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ImportSpecifiersDeclaration {
                specifiers: vec![],
                source: "file.js".into(),
                position: None,
            },
            "import{}from'file.js';"
        );
    }

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ImportSpecifiersDeclaration {
                specifiers: vec![
                    BindingIdentifier::from("spec1").into(),
                    BindingIdentifier::from("spec2").into(),
                    ImportSpecifier {
                        local: BindingIdentifier::from("spec3"),
                        imported: ModuleIdentifier::from("fooImport").into(),
                        position: None,
                    },
                ],
                source: "file.js".into(),
                position: None,
            },
            "import{spec1,spec2,spec3 as fooImport}from'file.js';"
        );
    }
}



// export default 4;
node!(pub struct ExportDefaultExpression {
    pub token_export: KeywordData,
    pub token_default: KeywordData,
    pub expression: alias::Expression,
});
impl NodeDisplay for ExportDefaultExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.keyword(Keyword::Default);

        {
            let mut f = f.restrict_lookahead(LookaheadRestriction::ExportDefault);
            f.require_precedence(Precedence::Assignment).node(
                &self.expression,
            )?;
        }

        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl<T: Into<alias::Expression>> From<T> for ExportDefaultExpression {
    fn from(val: T) -> ExportDefaultExpression {
        ExportDefaultExpression {
            token_export: Default::default(),
            token_default: Default::default(),
            expression: val.into(),
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_export_default_expression {
    use super::*;
    use ast::literal;
    use ast::functions;
    use ast::classes;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ExportDefaultExpression::from(literal::Numeric::from(65.0)),
            "export default 65;"
        );
    }

    #[test]
    fn it_prints_with_class_parens() {
        assert_serialize!(
            ExportDefaultExpression::from(classes::ClassExpression::default()),
            "export default(class{});"
        );
    }

    #[test]
    fn it_prints_with_functino_parens() {
        assert_serialize!(
            ExportDefaultExpression::from(functions::FunctionExpression::default()),
            "export default(function(){});"
        );
    }
}



// export class foo {}
node!(pub struct ExportClassDeclaration {
    pub token_export: KeywordData,
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
    pub token_export: KeywordData,
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
    pub token_export: KeywordData,
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
    pub token_export: KeywordData,
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
    pub token_export: KeywordData,
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
    pub token_export: KeywordData,
    pub token_curly_left: MaybeTokenPosition,
    pub specifiers: Vec<(LocalExportSpecifier, KeywordData)>,
    pub last_specifier: Option<LocalExportSpecifier>,
    pub token_curly_right: MaybeTokenPosition,
});
impl NodeDisplay for ExportLocalBindings {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        {
            let mut f = f.wrap_curly();
            f.comma_list(&self.specifiers)?;
            f.node(&self.last_specifier);
        }
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl From<Vec<LocalExportSpecifier>> for ExportLocalBindings {
    fn from(v: Vec<LocalExportSpecifier>) -> ExportLocalBindings {
        ExportLocalBindings {
            token_export: Default::default(),
            token_curly_left: Default::default(),
            specifiers: v.into_iter().map(|s| (s, Default::default())).collect(),
            last_specifier: Default::default(),
            token_curly_right: Default::default(),
            position: None,
        }
    }
}

#[cfg(test)]
mod tests_export_specifiers {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ExportLocalBindings::from(vec![
                ReferenceIdentifier::from("someName").into(),
                ReferenceIdentifier::from("someOtherName").into(),
                LocalExportSpecifier {
                    local: "local".into(),
                    exported: Some("exp".into()),
                    position: None,
                },
            ]),
            "export{someName,someOtherName,local as exp};"
        );
    }
}

node_enum!(@node_display pub enum LocalExportSpecifier {
    Normal(NormalLocalExportSpecifier),
    Aliased(AliasedLocalExportSpecifier),
});

node!(pub struct NormalLocalExportSpecifier {
    pub local: ReferenceIdentifier,
});
impl NodeDisplay for NormalLocalExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.local)
    }
}
impl From<ReferenceIdentifier> for NormalLocalExportSpecifier {
    fn from(b: ReferenceIdentifier) -> NormalLocalExportSpecifier {
        NormalLocalExportSpecifier {
            local: b,
            position: None,
        }
    }
}

node!(pub struct AliasedLocalExportSpecifier {
    pub local: ReferenceIdentifier,
    pub exported: ModuleIdentifier,
});
impl NodeDisplay for AliasedLocalExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.local)?;
        f.keyword(Keyword::As);
        f.node(&self.exported)
    }
}


// export {foo} from "";
// export {foo as bar} from "";
node!(pub struct ExportSourceSpecifiers {
    pub token_export: MaybeTokenPosition,
    pub token_curly_left: MaybeTokenPosition,
    pub specifiers: Vec<(SourceExportSpecifier, KeywordData)>,
    pub last_specifier: Option<SourceExportSpecifier>,
    pub token_curly_right: MaybeTokenPosition,
    pub token_from: MaybeTokenPosition,
    pub source: String,
});
impl NodeDisplay for ExportSourceSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);

        {
            let mut f = f.wrap_curly();
            f.comma_list(&self.specifiers)?;
            f.node(&self.last_specifier)?;
        }

        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
#[cfg(test)]
mod tests_export_source_specifiers {
    use super::*;

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ExportSourceSpecifiers {
                specifiers: vec![
                    ModuleIdentifier::from("someName").into(),
                    ModuleIdentifier::from("someOtherName").into(),
                    SourceExportSpecifier {
                        imported: "local".into(),
                        exported: Some("exp".into()),
                        position: None,
                    },
                ],
                source: "file.js".into(),
                position: None,
            },
            "export{someName,someOtherName,local as exp}from'file.js';"
        );
    }
}

node_enum!(@node_display pub enum SourceExportSpecifier {
    Normal(NormalSourceExportSpecifier),
    Aliased(AliasedSourceExportSpecifier),
});


node!(pub struct NormalSourceExportSpecifier {
    pub imported: ModuleIdentifier,
});
impl NodeDisplay for NormalSourceExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.imported)
    }
}
impl From<ModuleIdentifier> for NormalSourceExportSpecifier {
    fn from(b: ModuleIdentifier) -> NormalSourceExportSpecifier {
        NormalSourceExportSpecifier {
            imported: b,
            position: None,
        }
    }
}

node!(pub struct AliasedSourceExportSpecifier {
    pub imported: ModuleIdentifier,
    pub exported: ModuleIdentifier,
});
impl NodeDisplay for AliasedSourceExportSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.imported)?;
        f.keyword(Keyword::As);
        f.node(&self.exported)
    }
}


// export * from "";
node!(pub struct ExportAllSpecifiers {
    pub token_export: MaybeTokenPosition,
    pub token_star: MaybeTokenPosition,
    pub token_from: MaybeTokenPosition,
    pub source: String,
});
impl NodeDisplay for ExportAllSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.punctuator(Punctuator::Star);
        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
#[cfg(test)]
mod tests_export_source_all {
    use super::*;

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ExportAllSpecifiers {
                source: "file.js".into(),
                position: None,
            },
            "export*from'file.js';"
        );
    }
}


// export foo from "";
node!(pub struct ExportNamedSpecifier {
    pub token_export: MaybeTokenPosition,
    pub default: ModuleIdentifier,
    pub token_from: MaybeTokenPosition,
    pub source: String,
});
impl NodeDisplay for ExportNamedSpecifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.node(&self.default)?;
        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
#[cfg(test)]
mod tests_export_named_default {
    use super::*;

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ExportNamedSpecifier {
                default: "fooExp".into(),
                source: "file.js".into(),
                position: None,
            },
            "export fooExp from'file.js';"
        );
    }
}


// export foo, * as foo from "";
node!(pub struct ExportNamedAndNamespace {
    pub token_export: MaybeTokenPosition,
    pub default: ModuleIdentifier,
    pub token_comma: MaybeTokenPosition,
    pub namespace: ModuleIdentifier,
    pub token_from: MaybeTokenPosition,
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
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}


// export * as foo from "";
node!(pub struct ExportNamespace {
    pub token_export: MaybeTokenPosition,
    pub token_star: MaybeTokenPosition,
    pub token_as: MaybeTokenPosition,
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
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_export_namespace {
    use super::*;

    #[test]
    fn it_prints_without_specifiers() {
        assert_serialize!(
            ExportNamespace {
                namespace: "foo".into(),
                source: "file.js".into(),
                position: None,
            },
            "export*as foo from'file.js';"
        );
    }
}


// export foo, {foo} from "";
// export foo, {foo as bar} from "";
node!(pub struct ExportNamedAndSpecifiers {
    pub token_export: MaybeTokenPosition,
    pub default: ModuleIdentifier,
    pub token_comma: MaybeTokenPosition,
    pub token_curly_left: MaybeTokenPosition,
    pub specifiers: Vec<(SourceExportSpecifier, KeywordData)>,
    pub last_specifier: Option<SourceExportSpecifier>,
    pub token_curly_right: MaybeTokenPosition,
    pub token_from: MaybeTokenPosition,
    pub source: String,
});
impl NodeDisplay for ExportNamedAndSpecifiers {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.node(&self.default)?;
        f.punctuator(Punctuator::Comma);
        {
            let mut f = f.wrap_curly();
            f.comma_list(&self.specifiers)?;
            f.node(&self.last_specifier)?;
        }
        f.keyword(Keyword::From);
        f.node(&self.source)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

#[cfg(test)]
mod tests_export_named_and_specifiers {
    use super::*;

    #[test]
    fn it_prints_without_specifiers() {
        assert_serialize!(
            ExportNamedAndSpecifiers {
                default: "foo".into(),
                specifiers: vec![],
                source: "file.js".into(),
                position: None,
            },
            "export foo,{}from'file.js';"
        );
    }

    #[test]
    fn it_prints_with_specifiers() {
        assert_serialize!(
            ExportNamedAndSpecifiers {
                default: "foo".into(),
                specifiers: vec![
                    ModuleIdentifier::from("someName").into(),
                    ModuleIdentifier::from("someOtherName").into(),
                    SourceExportSpecifier {
                        imported: "local".into(),
                        exported: Some("exp".into()),
                        position: None,
                    },
                ],
                source: "file.js".into(),
                position: None,
            },
            "export foo,{someName,someOtherName,local as exp}from'file.js';"
        );
    }
}
