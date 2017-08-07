use std::default;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadSequence};

use ast::alias;

use ast::general::PropertyName;
use ast::functions::{FunctionParams, FunctionBody};

// {a: 1, ...b}
node!(#[derive(Default)] pub struct ObjectExpression {
    pub properties: Vec<ObjectItem>,
    pub spread: Option<Box<alias::Expression>>, // experimental
});
impl NodeDisplay for ObjectExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(LookaheadSequence::Curly);
        let mut f = f.wrap_curly();

        f.comma_list(&self.properties)?;

        if let Some(ref expr) = self.spread {
            if !self.properties.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);
            f.require_precedence(Precedence::Assignment).node(expr)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_object_expression {
    use super::*;
    use ast::literal;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ObjectExpression::default(), "{}");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectExpression {
            properties: vec![
                ObjectProperty {
                    name: PropertyIdentifier::from("fooProp").into(),
                    value: literal::Boolean::from(false).into(),
                    position: None,
                }.into(),
                ObjectMethod {
                    kind: Default::default(),
                    id: PropertyIdentifier::from("fooMethod").into(),
                    params: Default::default(),
                    body: Default::default(),
                    position: None,
                }.into(),
            ],
            spread: Default::default(),
            position: None,
        }, "{fooProp:false,fooMethod(){}}");
    }

    #[test]
    fn it_prints_prop_and_spread() {
        assert_serialize!(ObjectExpression {
            properties: vec![
                ObjectProperty {
                    name: PropertyIdentifier::from("fooProp").into(),
                    value: literal::Boolean::from(false).into(),
                    position: None,
                }.into(),
            ],
            spread: literal::Boolean::from(true).into(),
            position: None,
        }, "{fooProp:false,...true}");
    }

    #[test]
    fn it_prints_spread() {
        assert_serialize!(ObjectExpression {
            properties: vec![],
            spread: literal::Boolean::from(true).into(),
            position: None,
        }, "{...true}");
    }
}


node_enum!(@node_display pub enum ObjectItem {
    Method(ObjectMethod),
    Property(ObjectProperty),
});


node_kind!(pub enum MethodKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental
    Get,
    Set,
});
impl default::Default for MethodKind {
    fn default() -> MethodKind {
        MethodKind::Normal
    }
}
impl NodeDisplay for MethodKind {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match *self {
            MethodKind::Normal => {}
            MethodKind::Generator => f.punctuator(Punctuator::Star),
            MethodKind::Async => f.keyword(Keyword::Async),
            MethodKind::AsyncGenerator => {
                f.keyword(Keyword::Async);
                f.punctuator(Punctuator::Star);
            }
            MethodKind::Get => f.keyword(Keyword::Get),
            MethodKind::Set => f.keyword(Keyword::Set),
        }

        Ok(())
    }
}


node!(pub struct ObjectMethod {
    pub kind: MethodKind,
    pub id: PropertyName, // TODO: Rename to "name" to match property?
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for ObjectMethod {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_object_method {
    use super::*;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectMethod {
            kind: Default::default(),
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "fooMethod(){}");
    }

    #[test]
    fn it_prints_async() {
        assert_serialize!(ObjectMethod {
            kind: MethodKind::Async,
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "async fooMethod(){}");
    }

    #[test]
    fn it_prints_generator() {
        assert_serialize!(ObjectMethod {
            kind: MethodKind::Generator,
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "*fooMethod(){}");
    }

    #[test]
    fn it_prints_async_generator() {
        assert_serialize!(ObjectMethod {
            kind: MethodKind::AsyncGenerator,
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "async*fooMethod(){}");
    }

    #[test]
    fn it_prints_getter() {
        assert_serialize!(ObjectMethod {
            kind: MethodKind::Get,
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "get fooMethod(){}");
    }

    #[test]
    fn it_prints_setter() {
        assert_serialize!(ObjectMethod {
            kind: MethodKind::Set,
            id: PropertyIdentifier::from("fooMethod").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "set fooMethod(){}");
    }
}


node!(pub struct ObjectProperty {
    pub name: PropertyName,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for ObjectProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {

        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);

        f.require_precedence(Precedence::Assignment).node(
            &self.value,
        )?;

        Ok(())
    }
}
#[cfg(test)]
mod tests_object_property {
    use super::*;
    use ast::literal;
    use ast::expression::SequenceExpression;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectProperty {
            name: PropertyIdentifier::from("fooProp").into(),
            value: literal::Boolean::from(false).into(),
            position: None,
        }, "fooProp:false");
    }

    #[test]
    fn it_prints_with_precedence() {
        assert_serialize!(ObjectProperty {
            name: PropertyIdentifier::from("fooProp").into(),
            value: SequenceExpression {
                left: literal::Boolean::from(false).into(),
                right: literal::Boolean::from(true).into(),
                position: None,
            }.into(),
            position: None,
        }, "fooProp:(false,true)");
    }
}


// [1, 2, 3, ...4]
node!(#[derive(Default)] pub struct ArrayExpression {
    pub elements: Vec<Option<Box<alias::Expression>>>,
    pub spread: Option<Box<alias::Expression>>,
});
impl NodeDisplay for ArrayExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Primary);
        let mut f = f.wrap_square();

        let mut f = f.require_precedence(Precedence::Assignment);
        f.comma_list(&self.elements)?;

        // TODO: This is not handling comma elision property, it loses an item.

        if let Some(ref expr) = self.spread {
            if !self.elements.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);
            f.node(expr)?;
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests_array_expression {
    use super::*;
    use ast::literal;

    #[test]
    fn it_prints() {
        assert_serialize!(ArrayExpression::default(), "[]");
    }

    #[test]
    fn it_prints_with_elision() {
        assert_serialize!(ArrayExpression {
            elements: vec![
                None,
                None,
            ],
            spread: Default::default(),
            position: None,
        }, "[,]");
    }

    #[test]
    fn it_prints_with_items() {
        assert_serialize!(ArrayExpression {
            elements: vec![
                literal::Boolean::from(false).into(),
                literal::Boolean::from(true).into(),
            ],
            spread: Default::default(),
            position: None,
        }, "[false,true]");
    }

    #[test]
    fn it_prints_with_items_and_elision() {
        assert_serialize!(ArrayExpression {
            elements: vec![
                None,
                literal::Boolean::from(true).into(),
            ],
            spread: Default::default(),
            position: None,
        }, "[,true]");
    }

    #[test]
    fn it_prints_with_spread() {
        assert_serialize!(ArrayExpression {
            elements: vec![],
            spread: literal::Boolean::from(true).into(),
            position: None,
        }, "[...true]");
    }

    #[test]
    fn it_prints_with_items_and_spread() {
        assert_serialize!(ArrayExpression {
            elements: vec![
                literal::Boolean::from(false).into(),
                literal::Boolean::from(true).into(),
            ],
            spread: literal::Boolean::from(true).into(),
            position: None,
        }, "[false,true,...true]");
    }

    #[test]
    fn it_prints_with_items_elision_and_spread() {
        assert_serialize!(ArrayExpression {
            elements: vec![
                None,
                literal::Boolean::from(true).into(),
            ],
            spread: literal::Boolean::from(true).into(),
            position: None,
        }, "[,true,...true]");
    }
}
