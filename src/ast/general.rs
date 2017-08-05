use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Precedence, FirstSpecialToken};
use ast::alias;
use ast::literal;

// identifiers used as variables
// TODO: Split this into a binding-declaration type and a binding-reference type
node!(pub struct BindingIdentifier {
    pub value: string::String,
    pub raw: Option<string::String>,
});
impl BindingIdentifier {
    pub fn new<T: Into<String>>(s: T) -> BindingIdentifier {
        BindingIdentifier {
            value: s.into(),
            raw: None,
            position: None,
        }
    }
}
impl NodeDisplay for BindingIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl FirstSpecialToken for BindingIdentifier {}

impl<T: Into<string::String>> From<T> for BindingIdentifier {
    fn from(value: T) -> BindingIdentifier {
        BindingIdentifier {
            value: value.into(),
            raw: None,
            position: None,
        }
    }
}



// identifiers used as properties
node!(pub struct PropertyIdentifier {
    pub value: string::String,
    pub raw: Option<string::String>,
});
impl PropertyIdentifier {
    pub fn new<T: Into<String>>(s: T) -> PropertyIdentifier {
        PropertyIdentifier {
            value: s.into(),
            raw: None,
            position: None,
        }
    }
}
impl NodeDisplay for PropertyIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(String::as_str))
    }
}
impl From<string::String> for PropertyIdentifier {
    fn from(value: string::String) -> PropertyIdentifier {
        PropertyIdentifier {
            value,
            raw: None,
            position: None,
        }
    }
}


node_enum!(@node_display pub enum PropertyName {
    Identifier(PropertyIdentifier),
    String(literal::String),
    Number(literal::Numeric),
    Computed(ComputedPropertyName),
});

node!(pub struct ComputedPropertyName {
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for ComputedPropertyName {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();
        let mut f = f.allow_in();

        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;

        Ok(())
    }
}
