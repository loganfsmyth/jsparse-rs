use std::string;

use super::misc;
use super::alias;
use super::display;

nodes!(pub struct Element {
    opening: ElementName,
    attributes: Vec<Attribute>,
    children: Vec<Child>,
    closing: Option<ElementName>,
});
impl display::NodeDisplay for Element {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::AngleL)?;
        f.node(&self.opening)?;

        for attr in self.attributes.iter() {
            f.node(attr)?;
        }

        if self.children.len() > 0 {
            f.punctuator(display::Punctuator::AngleR)?;

            for child in self.children.iter() {
                f.node(child)?;
            }

            f.punctuator(display::Punctuator::AngleSlash)?;
            if let Some(ref close) = self.closing {
                f.node(close)?;
            } else {
                f.node(&self.opening)?;
            }
            f.punctuator(display::Punctuator::AngleR)?;
        } else {
            if let Some(ref close) = self.closing {
                f.punctuator(display::Punctuator::AngleR)?;
                f.punctuator(display::Punctuator::AngleSlash)?;
                f.node(close)?;
                f.punctuator(display::Punctuator::AngleR)?;
            } else {
                f.punctuator(display::Punctuator::SlashAngle)?;
            }
        }

            Ok(())
    }
}
impl misc::HasInOperator for Element {
    fn has_in_operator(&self) -> bool {
            false
    }
}
impl misc::FirstSpecialToken for Element {}


nodes!(pub struct Identifier {
    // Same as a JS identifier, but allows "-"
    raw: string::String,
    value: string::String,
});
impl display::NodeDisplay for Identifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.jsx_identifier(&self.value, Some(&self.raw))
    }
}


pub enum ElementName {
    Identifier(Identifier),
    Member(MemberExpression),
    Namespaced(NamespacedName),
}
impl display::NodeDisplay for ElementName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ElementName::Identifier(ref n) => f.node(n),
            ElementName::Member(ref n) => f.node(n),
            ElementName::Namespaced(ref n) => f.node(n),
        }
    }
}


nodes!(pub struct MemberExpression {
    object: Box<MemberObject>,
    property: Identifier,
});
impl display::NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.object)?;
        f.punctuator(display::Punctuator::Period)?;
        f.node(&self.property)
    }
}

pub enum MemberObject {
    Identifier(Identifier),
    Member(MemberExpression),
}
impl display::NodeDisplay for MemberObject {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            MemberObject::Identifier(ref n) => f.node(n),
            MemberObject::Member(ref n) => f.node(n),
        }
    }
}


nodes!(pub struct NamespacedName {
    namespace: Identifier,
    name: Identifier,
});
impl display::NodeDisplay for NamespacedName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.namespace)?;
        f.punctuator(display::Punctuator::Colon)?;
        f.node(&self.name)
    }
}


pub enum Attribute {
    Spread(SpreadAttribute),
    Pair(PairAttribute),
}
impl display::NodeDisplay for Attribute {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Attribute::Spread(ref n) => f.node(n),
            Attribute::Pair(ref n) => f.node(n),
        }
    }
}


pub enum AttributeName {
    Identifier(Identifier),
    Namespaced(NamespacedName),
}
impl display::NodeDisplay for AttributeName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            AttributeName::Identifier(ref n) => f.node(n),
            AttributeName::Namespaced(ref n) => f.node(n),
        }
    }
}


nodes!(pub struct SpreadAttribute {
    argument: alias::Expression,
});
impl display::NodeDisplay for SpreadAttribute {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::Ellipsis)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.argument)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}


nodes!(pub struct PairAttribute {
    name: AttributeName,
    value: Option<AttributeValue>,
});
impl display::NodeDisplay for PairAttribute {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.name)?;
        if let Some(ref value) = self.value {
            f.punctuator(display::Punctuator::Eq)?;
            f.node(value)?;
        }
        Ok(())
    }
}


pub enum AttributeValue {
    String(StringLiteral),
    Expression(Box<alias::Expression>),
    Element(Element),
}
impl display::NodeDisplay for AttributeValue {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            AttributeValue::String(ref n) => f.node(n),
            AttributeValue::Expression(ref n) => f.require_precedence(display::Precedence::Assignment).node(n),
            AttributeValue::Element(ref n) => f.node(n),
        }
    }
}


nodes!(pub struct StringLiteral {
    // String literal that allows _all_ chars, except closing quote
    raw: string::String,
    value: string::String,
});
impl display::NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.jsx_string(&self.value, Some(&self.raw))
    }
}


pub enum Child {
    Empty(Empty),
    Text(Text),
    Element(Element),
    Expression(Expression),
    Spread(ExpressionSpread),
}
impl display::NodeDisplay for Child {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Child::Empty(ref n) => f.node(n),
            Child::Text(ref n) => f.node(n),
            Child::Element(ref n) => f.node(n),
            Child::Expression(ref n) => f.node(n),
            Child::Spread(ref n) => f.node(n),
        }
    }
}

nodes!(pub struct Expression {
    expression: Box<alias::Expression>,
});
impl display::NodeDisplay for Expression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

// experimental?
nodes!(pub struct ExpressionSpread {
    expression: Box<alias::Expression>,
});
impl display::NodeDisplay for ExpressionSpread {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::Ellipsis)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

nodes!(pub struct Empty {});
impl display::NodeDisplay for Empty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

nodes!(pub struct Text {
    // Serialized string should contain HTML entities since it, allows all chars except {, }, <, and >
    value: string::String,
});
impl display::NodeDisplay for Text {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.jsx_text(&self.value, None)
    }
}
