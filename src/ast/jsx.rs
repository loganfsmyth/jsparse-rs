use std::string;

use super::misc;
use super::alias;
use super::display;

node!(pub struct Element {
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


node!(pub struct Identifier {
    // Same as a JS identifier, but allows "-"
    raw: string::String,
    value: string::String,
});
impl display::NodeDisplay for Identifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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
impl display::NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.object)?;
        f.punctuator(display::Punctuator::Period)?;
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
impl display::NodeDisplay for NamespacedName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.namespace)?;
        f.punctuator(display::Punctuator::Colon)?;
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
impl display::NodeDisplay for SpreadAttribute {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::Ellipsis)?;
        f.require_precedence(display::Precedence::Assignment).node(
            &self.argument,
        )?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}


node!(pub struct PairAttribute {
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
impl display::NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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
impl display::NodeDisplay for Expression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.require_precedence(display::Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

// experimental?
node!(pub struct ExpressionSpread {
    expression: Box<alias::Expression>,
});
impl display::NodeDisplay for ExpressionSpread {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::Ellipsis)?;
        f.require_precedence(display::Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

node!(pub struct Empty {});
impl display::NodeDisplay for Empty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;
        f.punctuator(display::Punctuator::CurlyR)
    }
}

node!(pub struct Text {
    // Serialized string should contain HTML entities since it,
    // allows all chars except {, }, <, and >
    value: string::String,
});
impl display::NodeDisplay for Text {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.jsx_text(&self.value, None)
    }
}
