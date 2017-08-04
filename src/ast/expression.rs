use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasInOperator, FirstSpecialToken, SpecialToken};

// use super::misc;
use ast::alias;
// use super::display;

use ast::patterns::{LeftHandSimpleAssign, LeftHandComplexAssign};
use ast::statement::BlockStatement;
use ast::general::{PropertyIdentifier};


// this
node!(pub struct ThisExpression {});
impl NodeDisplay for ThisExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::This);
        Ok(())
    }
}
impl FirstSpecialToken for ThisExpression {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::None
    }
}
impl HasInOperator for ThisExpression {}


node!(pub struct ParenthesizedExpression {
    pub expr: Box<alias::Expression>,
});
impl NodeDisplay for ParenthesizedExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_parens().node(&self.expr)
    }
}
impl FirstSpecialToken for ParenthesizedExpression {}
impl HasInOperator for ParenthesizedExpression {}


// fn`content`
node!(pub struct TaggedTemplateLiteral {
    pub tag: Box<alias::Expression>,
    pub template: TemplateLiteral,
});
impl NodeDisplay for TaggedTemplateLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::Member).node(
            &self.tag,
        )?;
        f.node(&self.template)
    }
}
impl FirstSpecialToken for TaggedTemplateLiteral {
    fn first_special_token(&self) -> SpecialToken {
        self.tag.first_special_token()
    }
}
impl HasInOperator for TaggedTemplateLiteral {}


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
impl FirstSpecialToken for TemplateLiteral {}
impl HasInOperator for TemplateLiteral {}


// TODO: Enum fix?
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


node!(pub struct CallArguments {
    pub args: Vec<Box<alias::Expression>>,
    pub spread: Option<Box<alias::Expression>>,
});
impl NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::ParenL);

        f.comma_list(&self.args)?;

        if let Some(ref spread) = self.spread {
            f.punctuator(Punctuator::Comma);
            f.require_precedence(Precedence::Assignment).node(
                spread,
            )?;
        }

        f.punctuator(Punctuator::ParenR);
        Ok(())
    }
}

// foo()
node!(pub struct CallExpression {
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
    pub optional: bool,
});
impl NodeDisplay for CallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::New).node(
            &self.callee,
        )?;
        if self.optional {
            f.punctuator(Punctuator::Question);
        }
        f.node(&self.arguments)
    }
}
impl FirstSpecialToken for CallExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.callee.first_special_token()
    }
}
impl HasInOperator for CallExpression {}


// new foo()
node!(pub struct NewExpression {
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
});
impl NodeDisplay for NewExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::New);
        f.require_precedence(Precedence::New).node(
            &self.callee,
        )?;
        f.node(&self.arguments)
    }
}
impl FirstSpecialToken for NewExpression {}
impl HasInOperator for NewExpression {}


// experimental
// import(foo)
node!(pub struct ImportCallExpression {
    pub argument: Box<alias::Expression>,
});
impl NodeDisplay for ImportCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.punctuator(Punctuator::ParenL);
        f.require_precedence(Precedence::Assignment).node(
            &self.argument,
        )?;
        f.punctuator(Punctuator::ParenR);
        Ok(())
    }
}
impl FirstSpecialToken for ImportCallExpression {}
impl HasInOperator for ImportCallExpression {}


node!(pub struct SuperCallExpression {
    pub arguments: CallArguments,
});
impl NodeDisplay for SuperCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Super);
        f.node(&self.arguments)
    }
}
impl FirstSpecialToken for SuperCallExpression {}
impl HasInOperator for SuperCallExpression {}

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
        f.require_precedence(Precedence::Member).node(
            &self.object,
        )?;
        if self.optional {
            f.punctuator(Punctuator::Question);
        }
        f.punctuator(Punctuator::Period);
        f.node(&self.property)?;
        Ok(())
    }
}
impl FirstSpecialToken for MemberExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.object.first_special_token()
    }
}
impl HasInOperator for MemberExpression {}


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
        f.punctuator(Punctuator::SquareL);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        f.punctuator(Punctuator::SquareR);
        Ok(())
    }
}


node!(pub struct IdentifierPropertyAccess {
    pub id: PropertyIdentifier,
});
impl NodeDisplay for IdentifierPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Period);
        f.node(&self.id)
    }
}


node!(pub struct PrivatePropertyAccess {
    pub property: PropertyIdentifier,
});
impl NodeDisplay for PrivatePropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
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
                f.require_precedence(Precedence::Unary).node(
                    &self.value,
                )
            }
            UpdateOperator::PreDecrement => {
                f.punctuator(Punctuator::MinusMinus);
                f.require_precedence(Precedence::Unary).node(
                    &self.value,
                )
            }
            UpdateOperator::PostIncrement => {
                f.require_precedence(Precedence::LeftHand).node(
                    &self.value,
                )?;
                f.punctuator(Punctuator::PlusPlus);
                Ok(())
            }
            UpdateOperator::PostDecrement => {
                f.require_precedence(Precedence::LeftHand).node(
                    &self.value,
                )?;
                f.punctuator(Punctuator::MinusMinus);
                Ok(())
            }
        }
    }
}
impl FirstSpecialToken for UpdateExpression {
    fn first_special_token(&self) -> SpecialToken {
        match self.operator {
            UpdateOperator::PreIncrement => SpecialToken::None,
            UpdateOperator::PreDecrement => SpecialToken::None,
            UpdateOperator::PostIncrement => self.value.first_special_token(),
            UpdateOperator::PostDecrement => self.value.first_special_token(),
        }
    }
}
impl HasInOperator for UpdateExpression {}


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

        f.require_precedence(Precedence::Unary).node(
            &self.value,
        )
    }
}
impl FirstSpecialToken for UnaryExpression {}
impl HasInOperator for UnaryExpression {}


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
                f.require_precedence(Precedence::Multiplicative)
                    .node(&self.right)?;
            }
            BinaryOperator::Subtract => {
                let mut f = f.precedence(Precedence::Additive);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Minus);
                f.require_precedence(Precedence::Multiplicative)
                    .node(&self.right)?;
            }
            BinaryOperator::LeftShift => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::LAngleAngle);
                f.require_precedence(Precedence::Additive).node(
                    &self.right,
                )?;
            }
            BinaryOperator::RightShift => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleAngle);
                f.require_precedence(Precedence::Additive).node(
                    &self.right,
                )?;
            }
            BinaryOperator::RightShiftSigned => {
                let mut f = f.precedence(Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleAngleAngle);
                f.require_precedence(Precedence::Additive).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Divide => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Slash);
                f.require_precedence(Precedence::Exponential)
                    .node(&self.right)?;
            }
            BinaryOperator::Multiply => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Star);
                f.require_precedence(Precedence::Exponential)
                    .node(&self.right)?;
            }
            BinaryOperator::Modulus => {
                let mut f = f.precedence(Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Mod);
                f.require_precedence(Precedence::Exponential)
                    .node(&self.right)?;
            }
            BinaryOperator::BitAnd => {
                let mut f = f.precedence(Precedence::BitwiseAnd);
                f.node(&self.left)?;
                f.punctuator(Punctuator::Amp);
                f.require_precedence(Precedence::Equality).node(
                    &self.right,
                )?;
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
                f.require_precedence(Precedence::Exponential)
                    .node(&self.right)?;
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
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
            }
            BinaryOperator::LessThanEq => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::LAngleEq);
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
            }
            BinaryOperator::GreaterThan => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngle);
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
            }
            BinaryOperator::GreaterThanEq => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(Punctuator::RAngleEq);
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
            }
            BinaryOperator::In => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.keyword(Keyword::In);
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
            }
            BinaryOperator::Instanceof => {
                let mut f = f.precedence(Precedence::Relational);
                f.node(&self.left)?;
                f.keyword(Keyword::Instanceof);
                f.require_precedence(Precedence::Shift).node(
                    &self.right,
                )?;
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
impl FirstSpecialToken for BinaryExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.left.first_special_token()
    }
}
impl HasInOperator for BinaryExpression {
    fn has_in_operator(&self) -> bool {
        match self.operator {
            BinaryOperator::Add => false,
            BinaryOperator::Subtract => false,
            BinaryOperator::LeftShift => false,
            BinaryOperator::RightShift => false,
            BinaryOperator::RightShiftSigned => false,
            BinaryOperator::Divide => false,
            BinaryOperator::Multiply => false,
            BinaryOperator::Modulus => false,
            BinaryOperator::BitAnd => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::BitOr => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::BitXor => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::Power => false,
            BinaryOperator::Compare => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::StrictCompare => {
                self.left.has_in_operator() || self.right.has_in_operator()
            }
            BinaryOperator::NegateCompare => {
                self.left.has_in_operator() || self.right.has_in_operator()
            }
            BinaryOperator::NegateStrictCompare => {
                self.left.has_in_operator() || self.right.has_in_operator()
            }
            BinaryOperator::LessThan => self.left.has_in_operator(),
            BinaryOperator::LessThanEq => self.left.has_in_operator(),
            BinaryOperator::GreaterThan => self.left.has_in_operator(),
            BinaryOperator::GreaterThanEq => self.left.has_in_operator(),
            BinaryOperator::In => true,
            BinaryOperator::Instanceof => self.left.has_in_operator(),
            BinaryOperator::And => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::Or => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::Bind => false,
        }
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

        f.require_precedence(Precedence::LogicalOr).node(
            &self.test,
        )?;
        f.punctuator(Punctuator::Question);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Assignment).node(
                &self.consequent,
            )?;
        }
        f.punctuator(Punctuator::Colon);
        f.require_precedence(Precedence::Assignment).node(
            &self.alternate,
        )?;

        Ok(())
    }
}
impl FirstSpecialToken for ConditionalExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.test.first_special_token()
    }
}
impl HasInOperator for ConditionalExpression {
    fn has_in_operator(&self) -> bool {
        self.test.has_in_operator()
    }
}


// foo = bar
node!(pub struct AssignmentExpression {
    pub left: Box<LeftHandComplexAssign>,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AssignmentExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(
            &self.left,
        )?;
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
impl FirstSpecialToken for AssignmentExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.left.first_special_token()
    }
}
impl HasInOperator for AssignmentExpression {
    fn has_in_operator(&self) -> bool {
        self.right.has_in_operator()
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
        f.require_precedence(Precedence::LeftHand).node(
            &self.left,
        )?;
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
impl FirstSpecialToken for AssignmentUpdateExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.left.first_special_token()
    }
}
impl HasInOperator for AssignmentUpdateExpression {
    fn has_in_operator(&self) -> bool {
        self.right.has_in_operator()
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
impl FirstSpecialToken for SequenceExpression {
    fn first_special_token(&self) -> SpecialToken {
        self.left.first_special_token()
    }
}
impl HasInOperator for SequenceExpression {
    fn has_in_operator(&self) -> bool {
        self.left.has_in_operator() || self.right.has_in_operator()
    }
}

// do { foo; }
node!(pub struct DoExpression {
    pub body: BlockStatement,
});
impl NodeDisplay for DoExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Do);
        f.node(&self.body)
    }
}
impl FirstSpecialToken for DoExpression {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::Declaration
    }
}
impl HasInOperator for DoExpression {}


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
impl FirstSpecialToken for MetaProperty {}
impl HasInOperator for MetaProperty {}


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
impl FirstSpecialToken for SuperMemberExpression {}
impl HasInOperator for SuperMemberExpression {}


node_enum!(@node_display pub enum SuperMemberAccess {
    Identifier(IdentifierPropertyAccess),
    Computed(ComputedPropertyAccess),
});
