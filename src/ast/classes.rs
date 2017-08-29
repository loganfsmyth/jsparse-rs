use ast::{SeparatorTokens, KeywordData, CommentIterator, LeadingComments};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadSequence};

// TODO: Should we have a MethodBody?
use ast::functions::{FunctionParams, FunctionBody};
use ast::decorators::DecoratorValue;
use ast::objects::MethodKind;
use ast::general::{BindingIdentifier, PropertyIdentifier, PropertyName};

use ast::alias;

node!(pub struct ClassHeritage {
    pub token_extends: KeywordData,
    pub tokens_extends_post: SeparatorTokens,
    pub expression: Option<Box<alias::Expression>>,
});
impl NodeDisplay for ClassHeritage {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Extends, &self.token_extends);
        // f.separator(&self.tokens_extends_post);
        f.require_precedence(Precedence::LeftHand).node(&self.expression)?;

        Ok(())
    }
}

node!(pub struct ClassName {
    pub tokens_prefix: SeparatorTokens,
    pub id: BindingIdentifier,
});
impl NodeDisplay for ClassName {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // f.separator(&self.tokens_prefix);
        f.node(&self.id)
    }
}

// export default class name {}
node!(#[derive(Default)] pub struct ExportDefaultClassDeclaration {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub token_export: KeywordData,
    pub token_default: KeywordData,
    pub token_class: KeywordData,
    pub id: Option<ClassName>,
    pub heritage: Option<ClassHeritage>,
    pub body: ClassBody,
});
impl NodeDisplay for ExportDefaultClassDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export, &self.token_export);
        f.keyword(Keyword::Default, &self.token_default);

        f.node_list(&self.decorators)?;
        f.keyword(Keyword::Class, &self.token_class);

        f.node(&self.id)?;
        f.node(&self.heritage)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_class_export_default {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(
            ExportDefaultClassDeclaration::default(),
            "export default class{}"
        );
    }

    #[test]
    fn it_prints_name() {
        assert_serialize!(
            ExportDefaultClassDeclaration {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: Default::default(),
                body: Default::default(),
                position: None,
            },
            "export default class someName{}"
        );
    }

    #[test]
    fn it_prints_extends() {
        assert_serialize!(
            ExportDefaultClassDeclaration {
                decorators: Default::default(),
                id: Default::default(),
                extends: ReferenceIdentifier::from("baseClass").into(),
                body: Default::default(),
                position: None,
            },
            "export default class extends baseClass{}"
        );
    }

    #[test]
    fn it_prints_name_extends() {
        assert_serialize!(
            ExportDefaultClassDeclaration {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: ReferenceIdentifier::from("baseClass").into(),
                body: Default::default(),
                position: None,
            },
            "export default class someName extends baseClass{}"
        );
    }
}


// class name {}
node!(pub struct ClassDeclaration {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub token_class: KeywordData,
    pub id: ClassName,
    pub heritage: Option<ClassHeritage>,
    pub body: ClassBody,
});
impl NodeDisplay for ClassDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node_list(&self.decorators)?;

        f.keyword(Keyword::Class, &self.token_class);

        f.node(&self.id)?;
        f.node(&self.heritage)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_class_declaration {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_name() {
        assert_serialize!(
            ClassDeclaration {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: Default::default(),
                body: Default::default(),
                position: None,
            },
            "class someName{}"
        );
    }

    #[test]
    fn it_prints_name_extends() {
        assert_serialize!(
            ClassDeclaration {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: ReferenceIdentifier::from("baseClass").into(),
                body: Default::default(),
                position: None,
            },
            "class someName extends baseClass{}"
        );
    }
}


// (class {})
node!(#[derive(Default)] pub struct ClassExpression {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub token_class: KeywordData,
    pub id: Option<ClassName>,
    pub heritage: Option<ClassHeritage>,
    pub body: ClassBody,
});
impl NodeDisplay for ClassExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(LookaheadSequence::Declaration);

        f.node_list(&self.decorators)?;

        f.keyword(Keyword::Class, &self.token_class);

        f.node(&self.id)?;
        f.node(&self.heritage)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_class_expression {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ClassExpression::default(), "class{}");
    }

    #[test]
    fn it_prints_name() {
        assert_serialize!(
            ClassExpression {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: Default::default(),
                body: Default::default(),
                position: None,
            },
            "class someName{}"
        );
    }

    #[test]
    fn it_prints_extends() {
        assert_serialize!(
            ClassExpression {
                decorators: Default::default(),
                id: Default::default(),
                extends: ReferenceIdentifier::from("baseClass").into(),
                body: Default::default(),
                position: None,
            },
            "class extends baseClass{}"
        );
    }

    #[test]
    fn it_prints_name_extends() {
        assert_serialize!(
            ClassExpression {
                decorators: Default::default(),
                id: BindingIdentifier::from("someName").into(),
                extends: ReferenceIdentifier::from("baseClass").into(),
                body: Default::default(),
                position: None,
            },
            "class someName extends baseClass{}"
        );
    }
}


node!(#[derive(Default)] pub struct ClassBody {
    pub token_curly_l: KeywordData,
    pub items: Vec<ClassItem>,
    pub token_curly_r: KeywordData,
});
impl NodeDisplay for ClassBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_curly().node_list(&self.items)?;

        Ok(())
    }
}

node!(#[derive(Default)] pub struct ClassEmpty {
    pub token: KeywordData,
});
impl NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}


// TODO: Should the class constructor be it's own item type to make "super()" checks easier?
node_enum!(@node_display pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
});

// experimental
node_enum!(@node_display @leading_comments pub enum ClassFieldId {
    Public(PropertyName),
    Private(PropertyIdentifier),
});

// experimental
node!(pub struct ClassField {
    pub decorators: Vec<ClassItemDecorator>,
    pub pos: Option<StaticPosition>,
    pub id: ClassFieldId,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ClassField {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node_list(&self.decorators)?;

        f.node(&self.pos)?;
        f.node(&self.id)?;
        f.node(&self.init)?;

        f.punctuator(Punctuator::Semicolon);

        Ok(())
    }
}
// impl LeadingComments for ClassField {
//     fn leading_comments(&self) -> CommentIterator {
//         if let Some(ref dec) = self.decorators.get(0) {
//             dec.leading_comments()
//         } else if let Some(ref pos) = self.pos {
//             pos.leading_comments()
//         } else {
//             self.id.leading_comments()
//         }
//     }
// }


#[cfg(test)]
mod tests_class_field {
    use super::*;
    use ast::literal;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ClassField {
                decorators: Default::default(),
                pos: Default::default(),
                id: PropertyIdentifier::from("someName").into(),
                value: Default::default(),
                position: None,
            },
            "someName;"
        );
    }

    #[test]
    fn it_prints_with_pos() {
        assert_serialize!(
            ClassField {
                decorators: Default::default(),
                pos: FieldPosition::Static,
                id: PropertyIdentifier::from("someName").into(),
                value: Default::default(),
                position: None,
            },
            "static someName;"
        );
    }

    #[test]
    fn it_prints_with_value() {
        assert_serialize!(
            ClassField {
                decorators: Default::default(),
                pos: Default::default(),
                id: PropertyIdentifier::from("someName").into(),
                value: literal::Boolean::from(true).into(),
                position: None,
            },
            "someName=true;"
        );
    }

    #[test]
    fn it_prints_with_value_and_static() {
        assert_serialize!(
            ClassField {
                decorators: Default::default(),
                pos: FieldPosition::Static,
                id: PropertyIdentifier::from("someName").into(),
                value: literal::Boolean::from(true).into(),
                position: None,
            },
            "static someName=true;"
        );
    }
}

node!(pub struct ClassMethod {
    pub decorators: Vec<ClassItemDecorator>,
    pub pos: Option<StaticPosition>,
    pub kind: MethodKind,
    pub id: ClassFieldId,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for ClassMethod {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node_list(&self.decorators)?;

        f.node(&self.pos)?;
        f.node(&self.kind)?;

        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)?;

        Ok(())
    }
}
#[cfg(test)]
mod tests_class_method {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ClassMethod {
                decorators: Default::default(),
                pos: Default::default(),
                kind: Default::default(),
                id: PropertyIdentifier::from("someName").into(),
                params: Default::default(),
                body: Default::default(),
                position: None,
            },
            "someName(){}"
        );
    }

    #[test]
    fn it_prints_with_pos() {
        assert_serialize!(
            ClassMethod {
                decorators: Default::default(),
                pos: FieldPosition::Static,
                kind: Default::default(),
                id: PropertyIdentifier::from("someName").into(),
                params: Default::default(),
                body: Default::default(),
                position: None,
            },
            "static someName(){}"
        );
    }

    #[test]
    fn it_prints_with_async() {
        assert_serialize!(
            ClassMethod {
                decorators: Default::default(),
                pos: Default::default(),
                kind: MethodKind::Async,
                id: PropertyIdentifier::from("someName").into(),
                params: Default::default(),
                body: Default::default(),
                position: None,
            },
            "async someName(){}"
        );
    }
}



node!(#[derive(Default)] pub struct StaticPosition {
    pub token_static: KeywordData,
});
impl NodeDisplay for StaticPosition {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Static);
        Ok(())
    }
}
// impl LeadingComments for StaticPosition {
//     fn leading_comments(&self) -> CommentIterator {
//         CommentIterator::new(&self.token_static)
//     }
// }

node!(pub struct ClassDecorator {
    pub tokens_prefix: SeparatorTokens,
    pub value: DecoratorValue,
});
impl NodeDisplay for ClassDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}
impl<T: Into<DecoratorValue>> From<T> for ClassDecorator {
    fn from(obj: T) -> ClassDecorator {
        ClassDecorator {
            tokens_prefix: Default::default(),
            value: obj.into(),
            position: None,
        }
    }
}

node!(pub struct ClassItemDecorator {
    pub tokens_prefix: SeparatorTokens,
    pub value: DecoratorValue,
});
impl NodeDisplay for ClassItemDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}
// impl LeadingComments for ClassItemDecorator {
//     fn leading_comments(&self) -> CommentIterator {
//         CommentIterator::new(&self.tokens_prefix)
//     }
// }
impl<T: Into<DecoratorValue>> From<T> for ClassItemDecorator {
    fn from(obj: T) -> ClassItemDecorator {
        ClassItemDecorator {
            tokens_prefix: Default::default(),
            value: obj.into(),
            position: None,
        }
    }
}
