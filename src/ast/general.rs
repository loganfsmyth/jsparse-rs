use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasInOperator, FirstSpecialToken, SpecialToken};
use ast::alias;
use ast::literal;

// identifiers used as variables
// TODO: Split this into a binding-declaration type and a binding-reference type
node!(pub struct BindingIdentifier {
    value: string::String,
    raw: Option<string::String>,
});
impl NodeDisplay for BindingIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl HasInOperator for BindingIdentifier {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for BindingIdentifier {}



// identifiers used as properties
node!(pub struct PropertyIdentifier {
    value: string::String,
    raw: string::String,
});
impl NodeDisplay for PropertyIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, Some(&self.raw))
    }
}


node_enum!(@node_display pub enum PropertyName {
    Identifier(PropertyIdentifier),
    String(literal::String),
    Number(literal::Numeric),
    Computed(ComputedPropertyName),
});

node!(pub struct ComputedPropertyName {
    expression: Box<alias::Expression>,
});
impl NodeDisplay for ComputedPropertyName {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        f.punctuator(Punctuator::SquareL);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(Punctuator::SquareR);
        Ok(())
    }
}