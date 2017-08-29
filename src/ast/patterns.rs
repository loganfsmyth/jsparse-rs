use ast::{MaybeTokenPosition, KeywordData, KeywordSuffixData};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence};

use ast::alias;
use ast::general;
use ast::general::{BindingIdentifier, ReferenceIdentifier, PropertyName};

// TODO: Should we split member expression into an member access and member assign?
use ast::expression::MemberExpression;

// Used for update and update-assignment
node_enum!(@node_display pub enum LeftHandSimpleAssign {
    Identifier(ReferenceIdentifier),
    Member(MemberExpression),
    Parenthesized(ParenthesizedAssignmentPattern),
});

// Used for standard assignment, and for..in and for..of LHS
node_enum!(@node_display pub enum LeftHandComplexAssign {
    Parenthesized(ParenthesizedAssignmentPattern),
    Identifier(ReferenceIdentifier),
    Member(MemberExpression),
    Object(ObjectAssignmentPattern),
    Array(ArrayAssignmentPattern),
});


// (i) = 4; and (obj.foo) = 4; are valid assignments in JS.
node!(pub struct ParenthesizedAssignmentPattern {
    pub token_paren_l: KeywordData,
    pub pattern: Box<LeftHandSimpleAssign>,
    pub token_paren_r: KeywordData,
});
impl NodeDisplay for ParenthesizedAssignmentPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_parens().node(&self.pattern)
    }
}


// {     } =
node!(#[derive(Default)] pub struct ObjectAssignmentPattern {
    pub token_curly_l: MaybeTokenPosition,
    pub properties: Vec<(ObjectAssignmentPatternProperty, KeywordData)>,
    pub last_property: Option<ObjectAssignmentPatternLastProperty>,
    pub token_curly_r: KeywordData,
});
impl NodeDisplay for ObjectAssignmentPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.comma_list(&self.properties)?;
        f.node(&self.last_property)
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
        assert_serialize!(
            ObjectAssignmentPattern {
                properties: vec![
                    ObjectAssignmentPatternIdentifierProperty::from(
                        ReferenceIdentifier::from("foo")
                    ).into(),
                    ObjectAssignmentPatternIdentifierProperty {
                        id: ReferenceIdentifier::from("foo2"),
                        init: literal::Boolean::from(true).into(),
                        position: None,
                    }.into(),
                    ObjectAssignmentPatternPatternProperty {
                        name: PropertyIdentifier::from("foo3").into(),
                        pattern: ReferenceIdentifier::from("foo4").into(),
                        init: literal::Boolean::from(false).into(),
                        position: None,
                    }.into(),
                ],
                rest: Default::default(),
                position: None,
            },
            "{foo,foo2=true,foo3:foo4=false}"
        );
    }
}

node_enum!(@node_display pub enum ObjectAssignmentPatternProperty {
    Identifier(ObjectAssignmentPatternIdentifierProperty),
    Pattern(ObjectAssignmentPatternPatternProperty),
});
node_enum!(@node_display pub enum ObjectAssignmentPatternLastProperty {
    Identifier(ObjectAssignmentPatternIdentifierProperty),
    Pattern(ObjectAssignmentPatternPatternProperty),
    Rest(ObjectAssignmentPatternRestProperty),
});

node!(pub struct ObjectAssignmentPatternRestProperty {
    pub token_ellipsis: KeywordSuffixData,
    // Object rest patterns exclude object and arrays since they wouldn't really be useful.
    pub pattern: Box<LeftHandSimpleAssign>,
});
impl NodeDisplay for ObjectAssignmentPatternRestProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.pattern)
    }
}

node!(pub struct ObjectAssignmentPatternIdentifierProperty {
    pub id: ReferenceIdentifier,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ObjectAssignmentPatternIdentifierProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)?;

        Ok(())
    }
}
impl<T: Into<ReferenceIdentifier>> From<T> for ObjectAssignmentPatternIdentifierProperty {
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
    pub pattern: Box<LeftHandComplexAssign>,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ObjectAssignmentPatternPatternProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.pattern)?;
        f.node(&self.init)?;

        Ok(())
    }
}





// [     ] =
node!(#[derive(Default)] pub struct ArrayAssignmentPattern {
    pub items: Vec<(Option<ArrayAssignmentPatternElement>, KeywordData)>,
    pub last_item: Option<ArrayAssignmentPatternLastElement>,
});
impl NodeDisplay for ArrayAssignmentPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();

        f.comma_list(&self.items)?;
        f.node(&self.last_item)
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
        assert_serialize!(
            ArrayAssignmentPattern {
                items: vec![
                    ArrayAssignmentPatternElement::from(ReferenceIdentifier::from("foo"))
                        .into(),
                    ArrayAssignmentPatternElement {
                        id: ReferenceIdentifier::from("foo2").into(),
                        init: literal::Boolean::from(true).into(),
                        position: None,
                    }.into(),
                ],
                rest: Default::default(),
                position: None,
            },
            "[foo,foo2=true]"
        );
    }
}

node_enum!(@node_display pub enum ArrayAssignmentPatternLastElement {
    Pattern(ArrayAssignmentPatternElement),
    Rest(ArrayAssignmentRestElement),
});


node!(pub struct ArrayAssignmentRestElement {
    pub token_ellipsis: KeywordData,
    pub pattern: Box<LeftHandComplexAssign>,
});
impl NodeDisplay for ArrayAssignmentRestElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.pattern)
    }
}


node!(pub struct ArrayAssignmentPatternElement {
    pub id: Box<LeftHandComplexAssign>,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ArrayAssignmentPatternElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)?;

        Ok(())
    }
}
impl<T: Into<LeftHandComplexAssign>> From<T> for ArrayAssignmentPatternElement {
    fn from(val: T) -> ArrayAssignmentPatternElement {
        ArrayAssignmentPatternElement {
            id: Box::new(val.into()),
            init: None,
            position: None,
        }
    }
}


// Used for binding declarations (var, let, const, and function params)
node_enum!(@node_display pub enum BindingPattern {
    Identifier(BindingIdentifier),
    Object(ObjectBindingPattern),
    Array(ArrayBindingPattern),
});


// {     }
node!(#[derive(Default)] pub struct ObjectBindingPattern {
    pub properties: Vec<(ObjectBindingPatternProperty, KeywordData)>,
    pub last_property: Option<ObjectBindingPatternLastProperty>,
});
impl NodeDisplay for ObjectBindingPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.comma_list(&self.properties)?;
        f.node(&self.last_property)
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
        assert_serialize!(
            ObjectBindingPattern {
                properties: vec![
                    ObjectBindingPatternIdentifierProperty::from(
                        BindingIdentifier::from("foo")
                    ).into(),
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
            },
            "{foo,foo2=true,foo3:foo4=false}"
        );
    }
}
node_enum!(@node_display pub enum ObjectBindingPatternProperty {
    Identifier(ObjectBindingPatternIdentifierProperty),
    Pattern(ObjectBindingPatternPatternProperty),
});
node_enum!(@node_display pub enum ObjectBindingPatternLastProperty {
    Identifier(ObjectBindingPatternIdentifierProperty),
    Pattern(ObjectBindingPatternPatternProperty),
    Rest(ObjectBindingPatternRestProperty),
});


node!(pub struct ObjectBindingPatternRestProperty {
    pub token_ellipsis: KeywordData,

    // Object rest binding patterns are explicitly identifiers only since using
    // an array pattern wouldn't make sense, and an object pattern would be useless.
    pub pattern: BindingIdentifier,
});
impl NodeDisplay for ObjectBindingPatternRestProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.pattern)
    }
}


node!(pub struct ObjectBindingPatternIdentifierProperty {
    pub id: BindingIdentifier,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ObjectBindingPatternIdentifierProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)?;

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
    pub pattern: Box<BindingPattern>,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ObjectBindingPatternPatternProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.pattern)?;
        f.node(&self.init)?;

        Ok(())
    }
}


// [     ]
node!(#[derive(Default)] pub struct ArrayBindingPattern {
    pub items: Vec<Option<(ArrayBindingPatternElement, KeywordData)>>,
    pub last_item: Option<ArrayBindingPatternLastElement>,
});
impl NodeDisplay for ArrayBindingPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();

        f.comma_list(&self.items)?;
        f.node(&self.last_item)
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
        assert_serialize!(
            ArrayBindingPattern {
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
            },
            "[foo,foo2=true]"
        );
    }
}


node_enum!(@node_display pub enum ArrayBindingPatternLastElement {
    Pattern(Box<ArrayBindingPatternElement>),
    Rest(ArrayBindingRestElement),
});


node!(pub struct ArrayBindingRestElement {
    pub token_ellipsis: KeywordData,
    pub pattern: Box<BindingPattern>,
});
impl NodeDisplay for ArrayBindingRestElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.pattern)
    }
}


node!(pub struct ArrayBindingPatternElement {
    pub id: BindingPattern,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ArrayBindingPatternElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)?;

        Ok(())
    }
}
impl<T: Into<BindingPattern>> From<T> for ArrayBindingPatternElement {
    fn from(val: T) -> ArrayBindingPatternElement {
        ArrayBindingPatternElement {
            id: val.into(),
            init: None,
            position: None,
        }
    }
}
