use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence};

use ast::alias;

node!(pub struct Element {
    pub opening: ElementName,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
    pub closing: Option<ElementName>,
});
impl NodeDisplay for Element {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::AngleL);
        f.node(&self.opening)?;

        f.node_list(&self.attributes)?;

        if self.children.len() > 0 {
            f.punctuator(Punctuator::AngleR);

            f.node_list(&self.children)?;

            f.punctuator(Punctuator::AngleSlash);
            if let Some(ref close) = self.closing {
                f.node(close)?;
            } else {
                f.node(&self.opening)?;
            }
            f.punctuator(Punctuator::AngleR);
        } else {
            if let Some(ref close) = self.closing {
                f.punctuator(Punctuator::AngleR);
                f.punctuator(Punctuator::AngleSlash);
                f.node(close)?;
                f.punctuator(Punctuator::AngleR);
            } else {
                f.punctuator(Punctuator::SlashAngle);
            }
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests_element {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            Element {
                opening: Identifier::from("foo-bar").into(),
                attributes: Default::default(),
                children: Default::default(),
                closing: Default::default(),
                position: None,
            },
            "<foo-bar/>"
        );
    }

    #[test]
    fn it_prints_with_member_name() {
        assert_serialize!(
            Element {
                opening: MemberExpression {
                    object: Identifier::from("foo-bar").into(),
                    property: Identifier::from("baz-bat").into(),
                    position: None,
                }.into(),
                attributes: Default::default(),
                children: Default::default(),
                closing: Default::default(),
                position: None,
            },
            "<foo-bar.baz-bat/>"
        );
    }

    #[test]
    fn it_prints_with_namespace() {
        assert_serialize!(
            Element {
                opening: NamespacedName {
                    namespace: Identifier::from("foo-bar").into(),
                    name: Identifier::from("baz-bat").into(),
                    position: None,
                }.into(),
                attributes: Default::default(),
                children: Default::default(),
                closing: Default::default(),
                position: None,
            },
            "<foo-bar:baz-bat/>"
        );
    }

    #[test]
    fn it_prints_with_closing() {
        assert_serialize!(
            Element {
                opening: Identifier::from("foo-bar").into(),
                attributes: Default::default(),
                children: Default::default(),
                closing: Identifier::from("foo-bar").into(),
                position: None,
            },
            "<foo-bar></foo-bar>"
        );
    }

    #[test]
    fn it_prints_with_attributes() {
        assert_serialize!(
            Element {
                opening: Identifier::from("foo-bar").into(),
                attributes: vec![
                    SpreadAttribute {
                        expression: ReferenceIdentifier::from("someVar").into(),
                        position: None,
                    }.into(),
                ],
                children: Default::default(),
                closing: Default::default(),
                position: None,
            },
            "<foo-bar{...someVar}/>"
        );
    }

    #[test]
    fn it_prints_with_children() {
        assert_serialize!(
            Element {
                opening: Identifier::from("foo-bar").into(),
                attributes: Default::default(),
                children: vec![Text::from("some text").into()],
                closing: Default::default(),
                position: None,
            },
            "<foo-bar>some text</foo-bar>"
        );
    }

    #[test]
    fn it_prints_with_children_and_closing() {
        assert_serialize!(
            Element {
                opening: Identifier::from("foo-bar").into(),
                attributes: Default::default(),
                children: vec![Text::from("some text").into()],
                closing: Identifier::from("foo-bar").into(),
                position: None,
            },
            "<foo-bar>some text</foo-bar>"
        );
    }
}


node!(pub struct Identifier {
    // Same as a JS identifier, but allows "-"
    pub raw: Option<string::String>,
    pub value: string::String,
});
impl NodeDisplay for Identifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_identifier(&self.value, self.raw.as_ref().map(String::as_str))
    }
}
impl<T: Into<string::String>> From<T> for Identifier {
    fn from(v: T) -> Identifier {
        Identifier {
            raw: None,
            value: v.into(),
            position: None,
        }
    }
}


node_enum!(@node_display pub enum ElementName {
    Identifier(Identifier),
    Member(MemberExpression),
    Namespaced(NamespacedName),
});


node!(pub struct MemberExpression {
    pub object: Box<MemberObject>,
    pub property: Identifier,
});
impl NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.object)?;
        f.punctuator(Punctuator::Period);
        f.node(&self.property)
    }
}

node_enum!(@node_display pub enum MemberObject {
    Identifier(Identifier),
    Member(MemberExpression),
});


node!(pub struct NamespacedName {
    pub namespace: Identifier,
    pub name: Identifier,
});
impl NodeDisplay for NamespacedName {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.namespace)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.name)
    }
}


node_enum!(@node_display pub enum Attribute {
    Spread(SpreadAttribute),
    Pair(PairAttribute),
});
#[cfg(test)]
mod tests_attribute {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_spread() {
        assert_serialize!(
            SpreadAttribute {
                expression: ReferenceIdentifier::from("attrName").into(),
                position: None,
            },
            "{...attrName}"
        );
    }

    #[test]
    fn it_prints_pair() {
        assert_serialize!(
            PairAttribute {
                name: Identifier::from("attrName").into(),
                value: StringAttribute::from("omg").into(),
                position: None,
            },
            "attrName='omg'"
        );
    }

    #[test]
    fn it_prints_pair_namespace() {
        assert_serialize!(
            PairAttribute {
                name: NamespacedName {
                    namespace: Identifier::from("attrName").into(),
                    name: Identifier::from("prop").into(),
                    position: None,
                }.into(),
                value: StringAttribute::from("omg").into(),
                position: None,
            },
            "attrName:prop='omg'"
        );
    }

    #[test]
    fn it_prints_pair_expression() {
        assert_serialize!(
            PairAttribute {
                name: Identifier::from("attrName").into(),
                value: ExpressionAttribute {
                    expression: ReferenceIdentifier::from("omg").into(),
                    position: None,
                }.into(),
                position: None,
            },
            "attrName={omg}"
        );
    }

    #[test]
    fn it_prints_pair_element() {
        assert_serialize!(
            PairAttribute {
                name: Identifier::from("attrName").into(),
                value: Element {
                    opening: Identifier::from("div").into(),
                    attributes: Default::default(),
                    children: Default::default(),
                    closing: Default::default(),
                    position: None,
                }.into(),
                position: None,
            },
            "attrName=<div/>"
        );
    }

    #[test]
    fn it_prints_pair_novalue() {
        assert_serialize!(
            PairAttribute {
                name: Identifier::from("attrName").into(),
                value: None,
                position: None,
            },
            "attrName"
        );
    }
}


node_enum!(@node_display pub enum AttributeName {
    Identifier(Identifier),
    Namespaced(NamespacedName),
});


node!(pub struct SpreadAttribute {
    pub expression: alias::Expression,
});
impl NodeDisplay for SpreadAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.punctuator(Punctuator::Ellipsis);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        Ok(())
    }
}


node!(pub struct PairAttribute {
    pub name: AttributeName,
    pub value: Option<AttributeValue>,
});
impl NodeDisplay for PairAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.name)?;
        if let Some(ref value) = self.value {
            f.punctuator(Punctuator::Eq);
            f.node(value)?;
        }
        Ok(())
    }
}


node_enum!(@node_display pub enum AttributeValue {
    String(StringAttribute),
    Expression(ExpressionAttribute),
    Element(Element),
});


node!(pub struct ExpressionAttribute {
    // String literal that allows _all_ chars, except closing quote
    pub expression: alias::Expression,
});
impl NodeDisplay for ExpressionAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_curly().node(&self.expression)
    }
}


node!(#[derive(Default)] pub struct StringAttribute {
    // String literal that allows _all_ chars, except closing quote
    pub raw: Option<string::String>,
    pub value: string::String,
});
impl NodeDisplay for StringAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_string(&self.value, self.raw.as_ref().map(String::as_str))
    }
}
impl<T: Into<string::String>> From<T> for StringAttribute {
    fn from(v: T) -> StringAttribute {
        StringAttribute {
            raw: None,
            value: v.into(),
            position: None,
        }
    }
}


node_enum!(@node_display pub enum Child {
    Empty(Empty),
    Text(Text),
    Element(Element),
    Expression(Expression),
    Spread(ExpressionSpread),
});
#[cfg(test)]
mod tests_element_children {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints_empty() {
        assert_serialize!(
            Element {
                opening: Identifier::from("div").into(),
                attributes: Default::default(),
                children: vec![Empty::default().into()],
                closing: Default::default(),
                position: None,
            },
            "<div>{}</div>"
        );
    }

    #[test]
    fn it_prints_text() {
        assert_serialize!(
            Element {
                opening: Identifier::from("div").into(),
                attributes: Default::default(),
                children: vec![Text::from("content").into()],
                closing: Default::default(),
                position: None,
            },
            "<div>content</div>"
        );
    }

    #[test]
    fn it_prints_element() {
        assert_serialize!(
            Element {
                opening: Identifier::from("div").into(),
                attributes: Default::default(),
                children: vec![
                    Text::from("before").into(),
                    Element {
                        opening: Identifier::from("div").into(),
                        attributes: Default::default(),
                        children: Default::default(),
                        closing: Default::default(),
                        position: None,
                    }.into(),
                    Text::from("after").into(),
                ],
                closing: Default::default(),
                position: None,
            },
            "<div>before<div/>after</div>"
        );
    }

    #[test]
    fn it_prints_expression() {
        assert_serialize!(
            Element {
                opening: Identifier::from("div").into(),
                attributes: Default::default(),
                children: vec![
                    Expression {
                        expression: ReferenceIdentifier::from("someVar").into(),
                        position: None,
                    }.into(),
                ],
                closing: Default::default(),
                position: None,
            },
            "<div>{someVar}</div>"
        );
    }

    #[test]
    fn it_prints_expression_spread() {
        assert_serialize!(
            Element {
                opening: Identifier::from("div").into(),
                attributes: Default::default(),
                children: vec![
                    ExpressionSpread {
                        expression: ReferenceIdentifier::from("someVar").into(),
                        position: None,
                    }.into(),
                ],
                closing: Default::default(),
                position: None,
            },
            "<div>{...someVar}</div>"
        );
    }
}

node!(pub struct Expression {
    pub expression: alias::Expression,
});
impl NodeDisplay for Expression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        Ok(())
    }
}

// experimental?
node!(pub struct ExpressionSpread {
    pub expression: alias::Expression,
});
impl NodeDisplay for ExpressionSpread {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();
        f.punctuator(Punctuator::Ellipsis);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;

        Ok(())
    }
}

node!(#[derive(Default)] pub struct Empty {});
impl NodeDisplay for Empty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_curly();
        Ok(())
    }
}

node!(#[derive(Default)] pub struct Text {
    // Serialized string should contain HTML entities since it,
    // allows all chars except {, }, <, and >
    pub value: string::String,
    pub raw: Option<string::String>,
});
impl NodeDisplay for Text {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_text(&self.value, None)
    }
}
impl<T: Into<string::String>> From<T> for Text {
    fn from(v: T) -> Text {
        Text {
            raw: None,
            value: v.into(),
            position: None,
        }
    }
}
