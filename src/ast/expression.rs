use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadSequence};

// use super::misc;
use ast::alias;
// use super::display;

use ast::patterns::{LeftHandSimpleAssign, LeftHandComplexAssign};
use ast::statement::BlockStatement;
use ast::general::{BindingIdentifier, PropertyIdentifier};


// this
node!(#[derive(Default)] pub struct ThisExpression {});
impl NodeDisplay for ThisExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::This);
        Ok(())
    }
}

#[cfg(test)]
mod tests_this {
    use super::*;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ThisExpression::default(), "this");
    }
}


node!(pub struct ParenthesizedExpression {
    pub expr: Box<alias::Expression>,
});
impl NodeDisplay for ParenthesizedExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_parens().node(&self.expr)
    }
}
impl<T: Into<alias::Expression>> From<T> for ParenthesizedExpression {
    fn from(expr: T) -> ParenthesizedExpression {
        ParenthesizedExpression {
            expr: Box::new(expr.into()),
            position: None,
        }
    }
}

#[cfg(test)]
mod tests_paren_expr {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ParenthesizedExpression::from(ThisExpression::default()),
            "(this)"
        );
    }
}

// fn`content`
node!(pub struct TaggedTemplateLiteral {
    pub tag: Box<alias::Expression>,
    pub template: TemplateLiteral,
});
impl NodeDisplay for TaggedTemplateLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::Member).node(&self.tag)?;
        f.node(&self.template)
    }
}


// `content`
node!(pub struct TemplateLiteral {
    pub piece: TemplateLiteralPiece,
});
impl NodeDisplay for TemplateLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::TemplateTick);
        f.node(&self.piece)
    }
}


// TODO: Enum fix?
#[derive(Debug)]
pub enum TemplateLiteralPiece {
    Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteralPiece>),
    End(TemplatePart),
}
impl NodeDisplay for TemplateLiteralPiece {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        match *self {
            TemplateLiteralPiece::Piece(ref part, ref expr, ref next_literal) => {
                f.node(part)?;
                f.punctuator(Punctuator::TemplateClose);
                f.require_precedence(Precedence::Normal).node(expr)?;
                f.punctuator(Punctuator::TemplateOpen);
                f.node(next_literal)
            }
            TemplateLiteralPiece::End(ref part) => {
                f.node(part)?;
                f.punctuator(Punctuator::TemplateTick);
                Ok(())
            }
        }
    }
}


node!(pub struct TemplatePart {
    pub value: string::String,
    pub raw_value: Option<string::String>,
});
impl NodeDisplay for TemplatePart {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.template_part(&self.value, self.raw_value.as_ref().map(String::as_str))
    }
}


node!(#[derive(Default)] pub struct CallArguments {
    pub args: Vec<alias::Expression>,
    pub spread: Option<Box<alias::Expression>>,
});
impl NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_parens();

        f.comma_list(&self.args)?;

        if let Some(ref spread) = self.spread {
            if !self.args.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);
            f.require_precedence(Precedence::Assignment).node(spread)?;
        }

        Ok(())
    }
}
impl From<Vec<alias::Expression>> for CallArguments {
    fn from(v: Vec<alias::Expression>) -> CallArguments {
        CallArguments {
            args: v,
            spread: Default::default(),
            position: None,
        }
    }
}

#[cfg(test)]
mod tests_call_args {
    use super::*;
    use ast::general::BindingIdentifier;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(CallArguments::default(), "()");
    }

    #[test]
    fn it_prints_args() {
        assert_serialize!(
            CallArguments::from(vec![
                ThisExpression::default().into(),
                BindingIdentifier::from("arg").into(),
                literal::Boolean::from(true).into(),
            ]),
            "(this,arg,true)"
        );
    }

    #[test]
    fn it_prints_args_with_precedence() {
        assert_serialize!(
            CallArguments::from(vec![
                BindingIdentifier::from("arg1").into(),
                SequenceExpression {
                    left: ThisExpression::default().into(),
                    right: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                BindingIdentifier::from("arg3").into(),
            ]),
            "(arg1,(this,true),arg3)"
        );
    }

    #[test]
    fn it_prints_args_with_spread() {
        assert_serialize!(
            CallArguments {
                args: vec![
                    ThisExpression::default().into(),
                    BindingIdentifier::from("arg").into(),
                    literal::Boolean::from(true).into(),
                ],
                spread: literal::Numeric::from(3.6).into(),
                position: None,
            },
            "(this,arg,true,...3.6)"
        );
    }

    #[test]
    fn it_prints_args_with_spread_and_precedence() {
        assert_serialize!(
            CallArguments {
                args: vec![BindingIdentifier::from("arg").into()],
                spread: SequenceExpression {
                    left: ThisExpression::default().into(),
                    right: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                position: None,
            },
            "(arg,...(this,true))"
        );
    }

}

// foo()
// foo?()
node!(pub struct CallExpression {
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
    pub optional: bool,
});
impl NodeDisplay for CallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::New).node(&self.callee)?;
        if self.optional {
            f.punctuator(Punctuator::Question);
        }
        f.node(&self.arguments)
    }
}


// new foo()
node!(pub struct NewExpression {
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
});
impl NodeDisplay for NewExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::New);
        f.require_precedence(Precedence::New).node(&self.callee)?;
        f.node(&self.arguments)
    }
}


// experimental
// import(foo)
node!(pub struct ImportCallExpression {
    pub argument: Box<alias::Expression>,
});
impl NodeDisplay for ImportCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);

        let mut f = f.wrap_parens();
        f.require_precedence(Precedence::Assignment).node(
            &self.argument,
        )?;

        Ok(())
    }
}


node!(pub struct SuperCallExpression {
    pub arguments: CallArguments,
});
impl NodeDisplay for SuperCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Super);
        f.node(&self.arguments)
    }
}


// foo.bar
// foo?.bar
// foo.#bar
// foo?.#bar
node!(pub struct MemberExpression {
    pub object: Box<alias::Expression>,
    pub property: PropertyAccess,
    pub optional: bool,
});
impl NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(get_sequence(self));
        f.require_precedence(Precedence::Member).node(&self.object)?;
        if self.optional {
            f.punctuator(Punctuator::Question);
        }
        f.punctuator(Punctuator::Period);
        f.node(&self.property)?;
        Ok(())
    }
}

/// Figure out which, if any, lookahead sequence matches this expression.
fn get_sequence(expr: &MemberExpression) -> LookaheadSequence {
    use ast::alias::Expression::Binding;

    match *expr.object {
        Binding(BindingIdentifier { ref value, .. }) if value == "let" && !expr.optional => {
            if let PropertyAccess::Computed(_) = expr.property {
                LookaheadSequence::LetSquare
            } else {
                LookaheadSequence::Let
            }
        }
        _ => LookaheadSequence::None,
    }
}


node_enum!(@node_display pub enum PropertyAccess {
    Identifier(IdentifierPropertyAccess),
    Computed(ComputedPropertyAccess),
    Private(PrivatePropertyAccess),
});


node!(pub struct ComputedPropertyAccess {
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for ComputedPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        Ok(())
    }
}

// .foo
node!(pub struct IdentifierPropertyAccess {
    pub id: PropertyIdentifier,
});
impl NodeDisplay for IdentifierPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Period);
        f.node(&self.id)
    }
}

// .#foo
node!(pub struct PrivatePropertyAccess {
    pub property: PropertyIdentifier,
});
impl NodeDisplay for PrivatePropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Period);
        f.punctuator(Punctuator::Hash);
        f.node(&self.property)
    }
}


// i++
// i--
// ++i
// --i
node!(pub struct UpdateExpression {
    pub value: LeftHandSimpleAssign,
    pub operator: UpdateOperator,
});
node_kind!(pub enum UpdateOperator {
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
});
impl NodeDisplay for UpdateExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match self.operator {
            UpdateOperator::PreIncrement => {
                f.punctuator(Punctuator::PlusPlus);
                f.node(&self.value)?;
                f.require_precedence(Precedence::Unary).node(&self.value)
            }
            UpdateOperator::PreDecrement => {
                f.punctuator(Punctuator::MinusMinus);
                f.require_precedence(Precedence::Unary).node(&self.value)
            }
            UpdateOperator::PostIncrement => {
                f.require_precedence(Precedence::LeftHand).node(&self.value)?;
                f.punctuator(Punctuator::PlusPlus);
                Ok(())
            }
            UpdateOperator::PostDecrement => {
                f.require_precedence(Precedence::LeftHand).node(&self.value)?;
                f.punctuator(Punctuator::MinusMinus);
                Ok(())
            }
        }
    }
}

// void foo
node!(pub struct UnaryExpression {
    pub value: Box<alias::Expression>,
    pub operator: UnaryOperator,
});
node_kind!(pub enum UnaryOperator {
    Delete,
    Void,
    Typeof,
    Positive,
    Negative,
    BitNegate,
    Negate,
    Await,
    Yield,
    YieldDelegate,
    Bind,
});
impl NodeDisplay for UnaryExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match self.operator {
            UnaryOperator::Delete => f.keyword(Keyword::Delete),
            UnaryOperator::Void => f.keyword(Keyword::Void),
            UnaryOperator::Typeof => f.keyword(Keyword::Typeof),
            UnaryOperator::Positive => f.punctuator(Punctuator::Plus),
            UnaryOperator::Negative => f.punctuator(Punctuator::Minus),
            UnaryOperator::BitNegate => f.punctuator(Punctuator::Tilde),
            UnaryOperator::Negate => f.punctuator(Punctuator::Exclam),
            UnaryOperator::Await => f.keyword(Keyword::Await),
            UnaryOperator::Yield => f.keyword(Keyword::Yield),
            UnaryOperator::YieldDelegate => {
                f.keyword(Keyword::Yield);
                f.punctuator(Punctuator::Star)
            }

            // TODO: Precedence on this is hard
            UnaryOperator::Bind => f.punctuator(Punctuator::Bind),
        }

        f.require_precedence(Precedence::Unary).node(&self.value)
    }
}


// foo OP bar
node!(pub struct BinaryExpression {
    pub left: Box<alias::Expression>,
    pub operator: BinaryOperator,
    pub right: Box<alias::Expression>,
});
node_kind!(pub enum BinaryOperator {
    Add,
    Subtract,
    LeftShift,
    RightShift,
    RightShiftSigned,
    Divide,
    Multiply,
    Modulus,
    BitAnd,
    BitOr,
    BitXor,
    Power,

    Compare,
    StrictCompare,
    NegateCompare,
    NegateStrictCompare,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    In,
    Instanceof,

    And,
    Or,

    Bind, // experimental
});
impl NodeDisplay for BinaryExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match self.operator {
            BinaryOperator::Add => {
                let mut f = f.precedence(Precedence::Additive);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Plus);
                f.require_precedence(Precedence::Multiplicative).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Subtract => {
                let mut f = f.precedence(Precedence::Additive);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Minus);
                f.require_precedence(Precedence::Multiplicative).node(
                    &self.right,
                )?;
            }
            BinaryOperator::LeftShift => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::LAngleAngle);
                f.require_precedence(Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::RightShift => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleAngle);
                f.require_precedence(Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::RightShiftSigned => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleAngleAngle);
                f.require_precedence(Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::Divide => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Slash);
                f.require_precedence(Precedence::Exponential).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Multiply => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Star);
                f.require_precedence(Precedence::Exponential).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Modulus => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Mod);
                f.require_precedence(Precedence::Exponential).node(
                    &self.right,
                )?;
            }
            BinaryOperator::BitAnd => {
                let mut f = f.precedence(Precedence::BitwiseAnd);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Amp);
                f.require_precedence(Precedence::Equality).node(&self.right)?;
            }
            BinaryOperator::BitOr => {
                let mut f = f.precedence(Precedence::BitwiseOr);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Bar);
                f.require_precedence(Precedence::BitwiseXOr).node(
                    &self.right,
                )?;
            }
            BinaryOperator::BitXor => {
                let mut f = f.precedence(Precedence::BitwiseXOr);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Caret);
                f.require_precedence(Precedence::BitwiseAnd).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Power => {
                let mut f = f.precedence(Precedence::Update);
                f.node(&self.left)?;
                f.punctuator(Punctuator::StarStar);
                f.require_precedence(Precedence::Exponential).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Compare => {
                let mut f = f.precedence(Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(Punctuator::EqEq);
                f.require_precedence(Precedence::Relational).node(
                    &self.right,
                )?;
            }
            BinaryOperator::StrictCompare => {
                let mut f = f.precedence(Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(Punctuator::EqEqEq);
                f.require_precedence(Precedence::Relational).node(
                    &self.right,
                )?;
            }
            BinaryOperator::NegateCompare => {
                let mut f = f.precedence(Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Neq);
                f.require_precedence(Precedence::Relational).node(
                    &self.right,
                )?;
            }
            BinaryOperator::NegateStrictCompare => {
                let mut f = f.precedence(Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(Punctuator::NeqEq);
                f.require_precedence(Precedence::Relational).node(
                    &self.right,
                )?;
            }
            BinaryOperator::LessThan => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::LAngle);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::LessThanEq => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::LAngleEq);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::GreaterThan => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngle);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::GreaterThanEq => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleEq);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::In => {
                let mut f = f.precedence(Precedence::Relational);
                let mut f = f.in_wrap_parens();
                f.node(&self.left)?;
                f.keyword(Keyword::In);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::Instanceof => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.keyword(Keyword::Instanceof);
                f.require_precedence(Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::And => {
                let mut f = f.precedence(Precedence::LogicalAnd);
                f.node(&self.left)?;
                f.punctuator(Punctuator::AmpAmp);
                f.require_precedence(Precedence::BitwiseOr).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Or => {
                let mut f = f.precedence(Precedence::LogicalOr);
                f.node(&self.left)?;
                f.punctuator(Punctuator::BarBar);
                f.require_precedence(Precedence::LogicalAnd).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Bind => {
                // TODO: Precedence
                f.node(&self.left)?;
                f.punctuator(Punctuator::ColonColon);
                f.node(&self.right)?;
            }
        }

        Ok(())
    }
}


// foo ? bar : baz
node!(pub struct ConditionalExpression {
    pub test: Box<alias::Expression>,
    pub consequent: Box<alias::Expression>,
    pub alternate: Box<alias::Expression>,
});
impl NodeDisplay for ConditionalExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Conditional);

        f.require_precedence(Precedence::LogicalOr).node(&self.test)?;

        f.punctuator(Punctuator::Question);

        let mut f = f.require_precedence(Precedence::Assignment);
        f.allow_in().node(&self.consequent)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.alternate)?;

        Ok(())
    }
}


// foo = bar
node!(pub struct AssignmentExpression {
    pub left: Box<LeftHandComplexAssign>,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AssignmentExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}


// foo OP= bar
node!(pub struct AssignmentUpdateExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub operator: AssignmentUpdateOperator,
    pub right: Box<alias::Expression>,
});
node_kind!(pub enum AssignmentUpdateOperator {
    Add,
    Subtract,
    LeftShift,
    RightShift,
    RightShiftSigned,
    Divide,
    Multiply,
    Modulus,
    BitAnd,
    BitOr,
    BitXor,
    Power,
});
impl NodeDisplay for AssignmentUpdateExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        match self.operator {
            AssignmentUpdateOperator::Add => f.punctuator(Punctuator::Plus),
            AssignmentUpdateOperator::Subtract => f.punctuator(Punctuator::Subtract),
            AssignmentUpdateOperator::LeftShift => f.punctuator(Punctuator::LAngleAngle),
            AssignmentUpdateOperator::RightShift => f.punctuator(Punctuator::RAngleAngle),
            AssignmentUpdateOperator::RightShiftSigned => {
                f.punctuator(Punctuator::RAngleAngleAngle)
            }
            AssignmentUpdateOperator::Divide => f.punctuator(Punctuator::Slash),
            AssignmentUpdateOperator::Multiply => f.punctuator(Punctuator::Star),
            AssignmentUpdateOperator::Modulus => f.punctuator(Punctuator::Mod),
            AssignmentUpdateOperator::BitAnd => f.punctuator(Punctuator::Amp),
            AssignmentUpdateOperator::BitOr => f.punctuator(Punctuator::Bar),
            AssignmentUpdateOperator::BitXor => f.punctuator(Punctuator::Caret),
            AssignmentUpdateOperator::Power => f.punctuator(Punctuator::StarStar),
        }
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}


// foo, bar
node!(pub struct SequenceExpression {
    pub left: Box<alias::Expression>,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for SequenceExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Normal);

        f.node(&self.left)?;
        f.punctuator(Punctuator::Comma);

        // Note: This precedence isn't needed to reproduce functionality, but it is to make the
        // AST reproduce properly from the serialized code. Parens can be avoided by reordering AST.
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )?;

        Ok(())
    }
}


// do { foo; }
node!(#[derive(Default)] pub struct DoExpression {
    pub body: BlockStatement,
});
impl NodeDisplay for DoExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Do);
        f.node(&self.body)
    }
}


// new.target
node!(pub struct MetaProperty {
    pub kind: MetaPropertyKind,
});
node_kind!(pub enum MetaPropertyKind {
    NewTarget,
    ImportMeta, // experimental
    FunctionSent, // experimental
    FunctionArguments, // experimental
});
impl NodeDisplay for MetaProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match self.kind {
            MetaPropertyKind::NewTarget => {
                f.keyword(Keyword::New);
                f.punctuator(Punctuator::Period);
                f.keyword(Keyword::Target);
            }
            MetaPropertyKind::ImportMeta => {
                f.keyword(Keyword::Import);
                f.punctuator(Punctuator::Period);
                f.keyword(Keyword::Meta);
            }
            MetaPropertyKind::FunctionSent => {
                f.keyword(Keyword::Function);
                f.punctuator(Punctuator::Period);
                f.keyword(Keyword::Sent);
            }
            MetaPropertyKind::FunctionArguments => {
                f.keyword(Keyword::Function);
                f.punctuator(Punctuator::Period);
                f.keyword(Keyword::Arguments);
            }
        }

        Ok(())
    }
}


// super.foo
// super[foo]
node!(pub struct SuperMemberExpression {
    pub property: SuperMemberAccess,
});
impl NodeDisplay for SuperMemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Super);
        f.node(&self.property)
    }
}


node_enum!(@node_display pub enum SuperMemberAccess {
    Identifier(IdentifierPropertyAccess),
    Computed(ComputedPropertyAccess),
});
