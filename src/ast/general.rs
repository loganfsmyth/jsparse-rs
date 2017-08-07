use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Precedence};
use ast::alias;
use ast::literal;

// identifiers used as binding names
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

impl<T: Into<string::String>> From<T> for BindingIdentifier {
    fn from(value: T) -> BindingIdentifier {
        BindingIdentifier {
            value: value.into(),
            raw: None,
            position: None,
        }
    }
}


// identifiers used as references to bindings
node!(pub struct ReferenceIdentifier {
    pub value: string::String,
    pub raw: Option<string::String>,
});
impl ReferenceIdentifier {
    pub fn new<T: Into<String>>(s: T) -> ReferenceIdentifier {
        ReferenceIdentifier {
            value: s.into(),
            raw: None,
            position: None,
        }
    }
}
impl NodeDisplay for ReferenceIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl<T: Into<string::String>> From<T> for ReferenceIdentifier {
    fn from(value: T) -> ReferenceIdentifier {
        ReferenceIdentifier {
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
impl<T: Into<string::String>> From<T> for PropertyIdentifier {
    fn from(value: T) -> PropertyIdentifier {
        PropertyIdentifier {
            value: value.into(),
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

        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;

        Ok(())
    }
}
