use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence, HasInOperator, FirstSpecialToken};

use ast::alias;

node!(pub struct Element {
    opening: ElementName,
    attributes: Vec<Attribute>,
    children: Vec<Child>,
    closing: Option<ElementName>,
});
impl NodeDisplay for Element {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::AngleL);
        f.node(&self.opening)?;

        for attr in self.attributes.iter() {
            f.node(attr)?;
        }

        if self.children.len() > 0 {
            f.punctuator(Punctuator::AngleR);

            for child in self.children.iter() {
                f.node(child)?;
            }

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
impl HasInOperator for Element {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for Element {}


node!(pub struct Identifier {
    // Same as a JS identifier, but allows "-"
    raw: string::String,
    value: string::String,
});
impl NodeDisplay for Identifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_identifier(&self.value, Some(&self.raw))
    }
}


node_enum!(@node_display pub enum ElementName {
    Identifier(Identifier),
    Member(MemberExpression),
    Namespaced(NamespacedName),
});


node!(pub struct MemberExpression {
    object: Box<MemberObject>,
    property: Identifier,
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
    namespace: Identifier,
    name: Identifier,
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


node_enum!(@node_display pub enum AttributeName {
    Identifier(Identifier),
    Namespaced(NamespacedName),
});


node!(pub struct SpreadAttribute {
    argument: alias::Expression,
});
impl NodeDisplay for SpreadAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::CurlyL);
        f.punctuator(Punctuator::Ellipsis);
        f.require_precedence(Precedence::Assignment).node(
            &self.argument,
        )?;
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}


node!(pub struct PairAttribute {
    name: AttributeName,
    value: Option<AttributeValue>,
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
    String(StringLiteral),
    Expression(Box<alias::Expression>),
    Element(Element),
});


node!(pub struct StringLiteral {
    // String literal that allows _all_ chars, except closing quote
    raw: string::String,
    value: string::String,
});
impl NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_string(&self.value, Some(&self.raw))
    }
}


node_enum!(@node_display pub enum Child {
    Empty(Empty),
    Text(Text),
    Element(Element),
    Expression(Expression),
    Spread(ExpressionSpread),
});

node!(pub struct Expression {
    expression: Box<alias::Expression>,
});
impl NodeDisplay for Expression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::CurlyL);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}

// experimental?
node!(pub struct ExpressionSpread {
    expression: Box<alias::Expression>,
});
impl NodeDisplay for ExpressionSpread {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::CurlyL);
        f.punctuator(Punctuator::Ellipsis);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}

node!(pub struct Empty {});
impl NodeDisplay for Empty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::CurlyL);
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}

node!(pub struct Text {
    // Serialized string should contain HTML entities since it,
    // allows all chars except {, }, <, and >
    value: string::String,
});
impl NodeDisplay for Text {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.jsx_text(&self.value, None)
    }
}
