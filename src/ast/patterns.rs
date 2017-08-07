use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence};

use ast::alias;
use ast::general::{BindingIdentifier, PropertyName};

// TODO: Should we split member expression into an member access and member assign?
use ast::expression::MemberExpression;

// Used for update and update-assignment
node_enum!(@node_display pub enum LeftHandSimpleAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
});

// Used for standard assignment, and for..in and for..of LHS
node_enum!(@node_display pub enum LeftHandComplexAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
    Object(ObjectAssignmentPattern),
    Array(ArrayAssignmentPattern),
});


// ({     } =
node!(#[derive(Default)] pub struct ObjectAssignmentPattern {
    pub properties: Vec<ObjectAssignmentPatternProperty>,

    // Object rest patterns exclude object and arrays since they wouldn't really be useful.
    pub rest: Option<Box<LeftHandSimpleAssign>>,
});
impl NodeDisplay for ObjectAssignmentPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
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
mod tests_object_assignment_pattern {
    use super::*;
    use ast::literal;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ObjectAssignmentPattern::default(), "{}");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectAssignmentPattern {
            properties: vec![
                ObjectAssignmentPatternIdentifierProperty::from(BindingIdentifier::from("foo")).into(),
                ObjectAssignmentPatternIdentifierProperty {
                    id: BindingIdentifier::from("foo2"),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                ObjectAssignmentPatternPatternProperty {
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


node!(pub struct ObjectAssignmentPatternIdentifierProperty {
    pub id: BindingIdentifier,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectAssignmentPatternIdentifierProperty {
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
impl<T: Into<BindingIdentifier>> From<T> for ObjectAssignmentPatternIdentifierProperty {
    fn from(val: T) -> ObjectAssignmentPatternIdentifierProperty {
        ObjectAssignmentPatternIdentifierProperty {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}


node!(pub struct ObjectAssignmentPatternPatternProperty {
    pub name: PropertyName,
    pub pattern: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectAssignmentPatternPatternProperty {
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

node_enum!(@node_display pub enum ObjectAssignmentPatternProperty {
    Identifier(ObjectAssignmentPatternIdentifierProperty),
    Pattern(ObjectAssignmentPatternPatternProperty),
});


// ([     ] =
node!(#[derive(Default)] pub struct ArrayAssignmentPattern {
    pub items: Vec<Option<ArrayAssignmentPatternElement>>,
    pub rest: Option<Box<LeftHandComplexAssign>>,
});
impl NodeDisplay for ArrayAssignmentPattern {
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
mod tests_array_assignment_pattern {
    use super::*;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ArrayAssignmentPattern::default(), "[]");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ArrayAssignmentPattern {
            items: vec![
                ArrayAssignmentPatternElement::from(BindingIdentifier::from("foo")).into(),
                ArrayAssignmentPatternElement {
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


node!(pub struct ArrayAssignmentPatternElement {
    pub id: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ArrayAssignmentPatternElement {
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
impl<T: Into<LeftHandComplexAssign>> From<T> for ArrayAssignmentPatternElement {
    fn from(val: T) -> ArrayAssignmentPatternElement {
        ArrayAssignmentPatternElement {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}













// Used for binding declarations (var, let, const, and function params)
node_enum!(@node_display pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectBindingPattern),
    Array(ArrayBindingPattern),
});

// ({     } =
node!(#[derive(Default)] pub struct ObjectBindingPattern {
    pub properties: Vec<ObjectBindingPatternProperty>,

    // Object rest binding patterns are explicitly identifiers only since using
    // an array pattern wouldn't make sense, and an object pattern would be useless.
    pub rest: Option<Box<BindingIdentifier>>,
});
impl NodeDisplay for ObjectBindingPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
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
mod tests_object_binding_pattern {
    use super::*;
    use ast::literal;
    use ast::general::PropertyIdentifier;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ObjectBindingPattern::default(), "{}");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ObjectBindingPattern {
            properties: vec![
                ObjectBindingPatternIdentifierProperty::from(BindingIdentifier::from("foo")).into(),
                ObjectBindingPatternIdentifierProperty {
                    id: BindingIdentifier::from("foo2"),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                ObjectBindingPatternPatternProperty {
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


node!(pub struct ObjectBindingPatternIdentifierProperty {
    pub id: BindingIdentifier,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectBindingPatternIdentifierProperty {
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
impl<T: Into<BindingIdentifier>> From<T> for ObjectBindingPatternIdentifierProperty {
    fn from(val: T) -> ObjectBindingPatternIdentifierProperty {
        ObjectBindingPatternIdentifierProperty {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}


node!(pub struct ObjectBindingPatternPatternProperty {
    pub name: PropertyName,
    pub pattern: Pattern,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectBindingPatternPatternProperty {
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

node_enum!(@node_display pub enum ObjectBindingPatternProperty {
    Identifier(ObjectBindingPatternIdentifierProperty),
    Pattern(ObjectBindingPatternPatternProperty),
});


// ([     ] =
node!(#[derive(Default)] pub struct ArrayBindingPattern {
    pub items: Vec<Option<ArrayBindingPatternElement>>,
    pub rest: Option<Box<Pattern>>,
});
impl NodeDisplay for ArrayBindingPattern {
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
mod tests_array_binding_pattern {
    use super::*;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ArrayBindingPattern::default(), "[]");
    }

    #[test]
    fn it_prints() {
        assert_serialize!(ArrayBindingPattern {
            items: vec![
                ArrayBindingPatternElement::from(BindingIdentifier::from("foo")).into(),
                ArrayBindingPatternElement {
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


node!(pub struct ArrayBindingPatternElement {
    pub id: Pattern,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ArrayBindingPatternElement {
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
impl<T: Into<Pattern>> From<T> for ArrayBindingPatternElement {
    fn from(val: T) -> ArrayBindingPatternElement {
        ArrayBindingPatternElement {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}
