use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence,
                   LookaheadSequence};

use ast::alias;
use ast::general::{BindingIdentifier, PropertyName};

// TODO: Should we split member expression into an member access and member assign?
use ast::expression::MemberExpression;

node_enum!(@node_display pub enum LeftHandSimpleAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
});


node_enum!(@node_display pub enum LeftHandComplexAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
    Object(ObjectPattern),
    Array(ArrayPattern),
});


node_enum!(@node_display pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
});


// ({     } =
node!(#[derive(Default)] pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
    pub rest: Option<Box<LeftHandComplexAssign>>,
});
impl NodeDisplay for ObjectPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(LookaheadSequence::Curly);
        let mut f = f.wrap_curly();

        f.comma_list(&self.properties)?;

        if let Some(ref p) = self.rest {
            if !self.properties.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);

            f.node(p)?;
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests_object_pattern {
    use super::*;
    use ast::literal;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ObjectPattern::default(), "{}");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectPattern {
            properties: vec![
                ObjectPatternIdentifierProperty::from(BindingIdentifier::from("foo")).into(),
                ObjectPatternIdentifierProperty {
                    id: BindingIdentifier::from("foo2"),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                ObjectPatternPatternProperty {
                    name: PropertyIdentifier::from("foo3").into(),
                    pattern: BindingIdentifier::from("foo4").into(),
                    init: literal::Boolean::from(false).into(),
                    position: None,
                }.into(),
            ],
            rest: Default::default(),
            position: None,
        }, "{foo,foo2=true,foo3:foo4=false}");
    }
}


node!(pub struct ObjectPatternIdentifierProperty {
    pub id: BindingIdentifier,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectPatternIdentifierProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}
impl<T: Into<BindingIdentifier>> From<T> for ObjectPatternIdentifierProperty {
    fn from(val: T) -> ObjectPatternIdentifierProperty {
        ObjectPatternIdentifierProperty {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}


node!(pub struct ObjectPatternPatternProperty {
    pub name: PropertyName,
    pub pattern: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectPatternPatternProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.pattern)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}

node_enum!(@node_display pub enum ObjectPatternProperty {
    Identifier(ObjectPatternIdentifierProperty),
    Pattern(ObjectPatternPatternProperty),
});


// ([     ] =
node!(#[derive(Default)] pub struct ArrayPattern {
    pub items: Vec<Option<ArrayPatternElement>>,
    pub rest: Option<Box<Pattern>>,
});
impl NodeDisplay for ArrayPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();

        f.comma_list(&self.items)?;

        if let Some(ref p) = self.rest {
            if !self.items.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);

            f.node(p)?;
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests_array_pattern {
    use super::*;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ObjectPattern::default(), "{}");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ArrayPattern {
            items: vec![
                ArrayPatternElement::from(BindingIdentifier::from("foo")).into(),
                ArrayPatternElement {
                    id: BindingIdentifier::from("foo2").into(),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
            ],
            rest: Default::default(),
            position: None,
        }, "[foo,foo2=true]");
    }
}


node!(pub struct ArrayPatternElement {
    pub id: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ArrayPatternElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;

        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}
impl<T: Into<LeftHandComplexAssign>> From<T> for ArrayPatternElement {
    fn from(val: T) -> ArrayPatternElement {
        ArrayPatternElement {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}
