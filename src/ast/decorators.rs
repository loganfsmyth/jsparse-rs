use ast::MaybeTokenPosition;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence};

use ast::general::{ReferenceIdentifier, PropertyIdentifier};
use ast::alias;

use ast::expression::CallArguments;

// experimental
// TODO: Enum fix
node_enum!(@node_display pub enum DecoratorValue {
    Property(DecoratorValueExpression),
    Call(DecoratorCallExpression),
    Expression(DecoratorExpression), // Backward-compat
});


// Backward-compat for older decorator spec
node!(pub struct DecoratorExpression {
    pub expression: alias::Expression,
});
impl NodeDisplay for DecoratorExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::Normal).node(&self.expression)
    }
}

node!(pub struct DecoratorMemberAccess {
    pub object: Box<DecoratorValueExpression>,
    pub property: PropertyIdentifier,
});
impl NodeDisplay for DecoratorMemberAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.object)?;
        f.punctuator(Punctuator::Period);
        f.node(&self.property)
    }
}

// experimental
node_enum!(@node_display pub enum DecoratorValueExpression {
    Identifier(ReferenceIdentifier),
    Member(DecoratorMemberAccess),
});

// experimental
node!(pub struct DecoratorCallExpression {
    pub callee: DecoratorValueExpression,
    pub arguments: CallArguments,
});
impl NodeDisplay for DecoratorCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.callee)?;
        f.node(&self.arguments)
    }
}
