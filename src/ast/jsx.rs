use std::string;

use super::misc;
use super::alias;
use super::display;

nodes!{
    pub struct Element {
        opening: ElementName,
        attributes: Vec<Attribute>,
        children: Vec<Child>,
        closing: Option<ElementName>,
    }
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


    pub struct Identifier {
        // Same as a JS identifier, but allows "-"
        raw: string::String,
        value: string::String,
    }
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
                ElementName::Identifier(ref id) => f.node(id),
                ElementName::Member(ref id) => f.node(id),
                ElementName::Namespaced(ref id) => f.node(id),
            }
        }
    }


    pub struct MemberExpression {
        object: Box<MemberObject>,
        property: Identifier,
    }
    impl display::NodeDisplay for MemberExpression {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            match *self.object {
                MemberObject::Identifier(ref id) => f.node(id)?,
                MemberObject::Member(ref id) => f.node(id)?,
            }
            f.punctuator(display::Punctuator::Period)?;
            f.node(&self.property)
        }
    }

    pub enum MemberObject {
        Identifier(Identifier),
        Member(MemberExpression),
    }


    pub struct NamespacedName {
        namespace: Identifier,
        name: Identifier,
    }
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
                Attribute::Spread(ref attr) => f.node(attr),
                Attribute::Pair(ref attr) => f.node(attr),
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
                AttributeName::Identifier(ref id) => f.node(id),
                AttributeName::Namespaced(ref name) => f.node(name),
            }
        }
    }


    pub struct SpreadAttribute {
        argument: alias::Expression,
    }
    impl display::NodeDisplay for SpreadAttribute {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.punctuator(display::Punctuator::CurlyL)?;
            f.punctuator(display::Punctuator::Ellipsis)?;
            f.require_precedence(display::Precedence::Assignment).node(&self.argument)?;
            f.punctuator(display::Punctuator::CurlyR)
        }
    }


    pub struct PairAttribute {
        name: AttributeName,
        value: Option<AttributeValue>,
    }
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
                AttributeValue::String(ref s) => f.node(s),
                AttributeValue::Expression(ref expr) => f.require_precedence(display::Precedence::Assignment).node(expr),
                AttributeValue::Element(ref elem) => f.node(elem),
            }
        }
    }


    pub struct StringLiteral {
        // String literal that allows _all_ chars, except closing quote
        raw: string::String,
        value: string::String,
    }
    impl display::NodeDisplay for StringLiteral {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.jsx_string(&self.value, Some(&self.raw))
        }
    }


    pub enum Child {
        Empty,
        Text(Text),
        Element(Element),
        Expression(Box<alias::Expression>),
        Spread(Box<alias::Expression>), // experimental?
    }
    impl display::NodeDisplay for Child {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            match *self {
                Child::Empty => {
                    f.punctuator(display::Punctuator::CurlyL)?;
                    f.punctuator(display::Punctuator::CurlyR)
                }
                Child::Text(ref t) => f.node(t),
                Child::Element(ref t) => f.node(t),
                Child::Expression(ref t) => {
                    f.punctuator(display::Punctuator::CurlyL)?;
                    f.require_precedence(display::Precedence::Assignment).node(t)?;
                    f.punctuator(display::Punctuator::CurlyR)
                }
                Child::Spread(ref t) => {
                    f.punctuator(display::Punctuator::CurlyL)?;
                    f.punctuator(display::Punctuator::Ellipsis)?;
                    f.require_precedence(display::Precedence::Assignment).node(t)?;
                    f.punctuator(display::Punctuator::CurlyR)
                }
            }
        }
    }


    pub struct Text {
        // Serialized string should contain HTML entities since it, allows all chars except {, }, <, and >
        value: string::String,
    }
    impl display::NodeDisplay for Text {
        fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
            f.jsx_text(&self.value, None)
        }
    }
}
