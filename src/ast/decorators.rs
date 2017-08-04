use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasInOperator, FirstSpecialToken, SpecialToken};

use ast::general::{BindingIdentifier, PropertyIdentifier};
use ast::alias;

use ast::expression::{CallArguments};

// experimental
// TODO: Enum fix
node_enum!(pub enum DecoratorValue {
    Property(DecoratorValueExpression),
    Call(DecoratorCallExpression),

    // Backward-compat for older decorator spec
    Expression(alias::Expression),
});
impl NodeDisplay for DecoratorValue {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match *self {
            DecoratorValue::Property(ref n) => f.node(n),
            DecoratorValue::Call(ref n) => f.node(n),
            DecoratorValue::Expression(ref expr) => {
                f.require_precedence(Precedence::Normal).node(expr)
            }
        }
    }
}

node!(pub struct DecoratorMemberAccess {
    object: Box<DecoratorValueExpression>,
    property: PropertyIdentifier,
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
    Identifier(BindingIdentifier),
    Member(DecoratorMemberAccess),
});

// experimental
node!(pub struct DecoratorCallExpression {
    callee: DecoratorValueExpression,
    arguments: CallArguments,
});
impl NodeDisplay for DecoratorCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.callee)?;
        f.node(&self.arguments)
    }
}
