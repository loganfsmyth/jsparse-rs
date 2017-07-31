use std::string;
use std::fmt;

use super::misc;
use super::alias;
use super::display;

use super::statement::{BlockStatement};
use super::misc::FirstSpecialToken;

// TODO: None of these do "super()" do they?

// this
nodes!(pub struct ThisExpression {});
impl display::NodeDisplay for ThisExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::This)
    }
}
impl misc::FirstSpecialToken for ThisExpression {
    fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::None }
}
impl misc::HasInOperator for ThisExpression {}


nodes!(pub struct ParenthesizedExpression {
    expr: Box<alias::Expression>,
});
impl display::NodeDisplay for ParenthesizedExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.wrap_parens().node(&self.expr)
    }
}
impl misc::FirstSpecialToken for ParenthesizedExpression {}
impl misc::HasInOperator for ParenthesizedExpression {}


// [1, 2, 3, ...4]
nodes!(pub struct ArrayExpression {
    elements: Vec<Option<Box<alias::Expression>>>,
    spread: Option<Box<alias::Expression>>,
});
impl display::NodeDisplay for ArrayExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.precedence(display::Precedence::Primary);
        let mut f = f.allow_in();

        f.punctuator(display::Punctuator::SquareL)?;

        for (i, el) in self.elements.iter().enumerate() {
            if i != 0 {
                f.punctuator(display::Punctuator::Comma)?;
            }

            if let Some(ref expr) = *el {
                let mut f = f.allow_in();
                f.require_precedence(display::Precedence::Assignment).node(expr)?;
            }
        }

        if let Some(ref expr) = self.spread {
            f.require_precedence(display::Precedence::Assignment).node(expr)?;
        }

        Ok(())
    }
}
impl misc::FirstSpecialToken for ArrayExpression {}
impl misc::HasInOperator for ArrayExpression {}


// {a: 1, ...b}
nodes!(pub struct ObjectExpression {
    properties: Vec<ObjectProperty>,
    spread: Option<Box<alias::Expression>>, // experimental
});
impl display::NodeDisplay for ObjectExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::SquareL)?;

        f.comma_list(&self.properties)?;

        if let Some(ref expr) = self.spread {
            if !self.properties.is_empty() {
                f.punctuator(display::Punctuator::Comma)?;
            }

            f.require_precedence(display::Precedence::Assignment).node(expr)?;
        }

        Ok(())
    }
}
impl misc::FirstSpecialToken for ObjectExpression {
    fn first_special_token(&self) -> misc::SpecialToken {
        misc::SpecialToken::Object
    }
}
impl misc::HasInOperator for ObjectExpression {}


pub enum ObjectProperty {
    Method(ObjectMethod),
    Value(misc::PropertyName, Box<alias::Expression>),
}
impl display::NodeDisplay for ObjectProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ObjectProperty::Method(ref method) => f.node(method),
            ObjectProperty::Value(ref id, ref expr) => {
                f.node(id)?;
                f.punctuator(display::Punctuator::Colon)?;

                let mut f = f.allow_in();
                f.require_precedence(display::Precedence::Assignment).node(expr)?;

                Ok(())
            }
        }
    }
}

nodes!(pub struct ObjectMethod {
    kind: misc::MethodKind,
    id: misc::PropertyName,
    params: misc::FunctionParams,
    body: misc::FunctionBody,
    fn_kind: misc::FunctionKind,
});
impl display::NodeDisplay for ObjectMethod {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}


// (function(){})
nodes!(pub struct FunctionExpression {
    id: Option<misc::BindingIdentifier>,
    params: misc::FunctionParams,
    body: misc::FunctionBody,
    fn_kind: misc::FunctionKind,
});
impl display::NodeDisplay for FunctionExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Function)?;

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
impl misc::FirstSpecialToken for FunctionExpression {
    fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Declaration }
}
impl misc::HasInOperator for FunctionExpression {}


// (class {})
nodes!(pub struct ClassExpression {
    decorators: Vec<misc::Decorator>, // experimental
    id: Option<misc::BindingIdentifier>,
    extends: Option<Box<alias::Expression>>,
    body: misc::ClassBody,
});
impl display::NodeDisplay for ClassExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.keyword(display::Keyword::Class)?;

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        if let Some(ref expr) = self.extends {
            f.keyword(display::Keyword::Extends)?;
            f.require_precedence(display::Precedence::LeftHand).node(expr)?;
        }

        f.node(&self.body)
    }
}
impl misc::FirstSpecialToken for ClassExpression {
    fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Declaration }
}
impl misc::HasInOperator for ClassExpression {}


// fn`content`
nodes!(pub struct TaggedTemplateLiteral {
    tag: Box<alias::Expression>,
    template: TemplateLiteral,
});
impl display::NodeDisplay for TaggedTemplateLiteral {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::Member).node(&self.tag)?;
        f.node(&self.template)
    }
}
impl misc::FirstSpecialToken for TaggedTemplateLiteral {
    fn first_special_token(&self) -> misc::SpecialToken { self.tag.first_special_token() }
}
impl misc::HasInOperator for TaggedTemplateLiteral {}


// `content`
nodes!(pub struct TemplateLiteral {
    piece: TemplateLiteralPiece,
});
impl display::NodeDisplay for TemplateLiteral {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::TemplateTick)?;
        f.node(&self.piece)
    }
}
impl misc::FirstSpecialToken for TemplateLiteral {}
impl misc::HasInOperator for TemplateLiteral {}


pub enum TemplateLiteralPiece {
    Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteral>),
    End(TemplatePart),
}
impl display::NodeDisplay for TemplateLiteralPiece {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        match *self {
            TemplateLiteralPiece::Piece(ref part, ref expr, ref next_literal) => {
                f.node(part)?;
                f.punctuator(display::Punctuator::TemplateClose)?;
                f.require_precedence(display::Precedence::Normal).node(expr)?;
                f.punctuator(display::Punctuator::TemplateOpen)?;
                f.node(next_literal)
            }
            TemplateLiteralPiece::End(ref part) => {
                f.node(part)?;
                f.punctuator(display::Punctuator::TemplateTick)
            }
        }
    }
}


nodes!(pub struct TemplatePart {
    value: string::String,
    rawValue: Option<string::String>,
});
impl display::NodeDisplay for TemplatePart {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.template_part(&self.value, self.rawValue.as_ref().map(String::as_str))
    }
}


// foo()
nodes!(pub struct CallExpression {
    callee: Box<alias::Expression>,
    arguments: misc::CallArguments,
    optional: bool,
});
impl display::NodeDisplay for CallExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::New).node(&self.callee)?;
        if self.optional {
            f.punctuator(display::Punctuator::Question)?;
        }
        f.node(&self.arguments)
    }
}
impl misc::FirstSpecialToken for CallExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.callee.first_special_token() }
}
impl misc::HasInOperator for CallExpression {}


// new foo()
nodes!(pub struct NewExpression {
    callee: Box<alias::Expression>,
    arguments: misc::CallArguments,
});
impl display::NodeDisplay for NewExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::New)?;
        f.require_precedence(display::Precedence::New).node(&self.callee)?;
        f.node(&self.arguments)
    }
}
impl misc::FirstSpecialToken for NewExpression {}
impl misc::HasInOperator for NewExpression {}


// experimental
// import(foo)
nodes!(pub struct ImportExpression {
    argument: Box<alias::Expression>,
});
impl display::NodeDisplay for ImportExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Import)?;
        f.punctuator(display::Punctuator::ParenL)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.argument)?;
        f.punctuator(display::Punctuator::ParenR)?;
        Ok(())
    }
}
impl misc::FirstSpecialToken for ImportExpression {}
impl misc::HasInOperator for ImportExpression {}


// foo.bar
// foo?.bar
// foo.#bar
// foo?.#bar
nodes!(pub struct MemberExpression {
    object: Box<alias::Expression>,
    property: MemberProperty,
    optional: bool,
});
impl display::NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::Member).node(&self.object)?;
        if self.optional {
            f.punctuator(display::Punctuator::Question)?;
        }
        f.punctuator(display::Punctuator::Period)?;
        f.node(&self.property)?;
        Ok(())
    }
}
impl misc::FirstSpecialToken for MemberExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.object.first_special_token() }
}
impl misc::HasInOperator for MemberExpression {}


pub enum MemberProperty {
    Normal(misc::PropertyAccess),
    Private(PrivateProperty),
}
impl display::NodeDisplay for MemberProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            MemberProperty::Normal(ref id) => f.node(id),
            MemberProperty::Private(ref prop) => f.node(prop),
        }
    }
}


nodes!(pub struct PrivateProperty {
    property: misc::PropertyIdentifier,
});
impl display::NodeDisplay for PrivateProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Hash)?;
        f.node(&self.property)
    }
}


// #bar
nodes!(pub struct PrivateExpression {
    property: misc::PropertyIdentifier,
});
impl display::NodeDisplay for PrivateExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Hash)?;
        f.node(&self.property)
    }
}
impl misc::FirstSpecialToken for PrivateExpression {}
impl misc::HasInOperator for PrivateExpression {}


// i++
// i--
// ++i
// --i
nodes!(pub struct UpdateExpression {
    value: misc::LeftHandSimpleAssign,
    operator: UpdateOperator,
});
pub enum UpdateOperator {
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}
impl display::NodeDisplay for UpdateExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match self.operator {
            UpdateOperator::PreIncrement => {
                f.punctuator(display::Punctuator::PlusPlus)?;
                f.node(&self.value)?;
                f.require_precedence(display::Precedence::Unary).node(&self.value)
            }
            UpdateOperator::PreDecrement => {
                f.punctuator(display::Punctuator::MinusMinus)?;
                f.require_precedence(display::Precedence::Unary).node(&self.value)
            }
            UpdateOperator::PostIncrement => {
                f.require_precedence(display::Precedence::LeftHand).node(&self.value)?;
                f.punctuator(display::Punctuator::PlusPlus)
            }
            UpdateOperator::PostDecrement => {
                f.require_precedence(display::Precedence::LeftHand).node(&self.value)?;
                f.punctuator(display::Punctuator::MinusMinus)
            }
        }
    }
}
impl misc::FirstSpecialToken for UpdateExpression {
    fn first_special_token(&self) -> misc::SpecialToken {
        match self.operator {
            UpdateOperator::PreIncrement => misc::SpecialToken::None,
            UpdateOperator::PreDecrement => misc::SpecialToken::None,
            UpdateOperator::PostIncrement => self.value.first_special_token(),
            UpdateOperator::PostDecrement => self.value.first_special_token(),
        }
    }
}
impl misc::HasInOperator for UpdateExpression {}


// void foo
nodes!(pub struct UnaryExpression {
    value: Box<alias::Expression>,
    operator: UnaryOperator,
});
pub enum UnaryOperator {
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
}
impl display::NodeDisplay for UnaryExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match self.operator {
            UnaryOperator::Delete => f.keyword(display::Keyword::Delete)?,
            UnaryOperator::Void => f.keyword(display::Keyword::Void)?,
            UnaryOperator::Typeof => f.keyword(display::Keyword::Typeof)?,
            UnaryOperator::Positive => f.punctuator(display::Punctuator::Plus)?,
            UnaryOperator::Negative => f.punctuator(display::Punctuator::Minus)?,
            UnaryOperator::BitNegate => f.punctuator(display::Punctuator::Tilde)?,
            UnaryOperator::Negate => f.punctuator(display::Punctuator::Exclam)?,
            UnaryOperator::Await => f.keyword(display::Keyword::Await)?,
            UnaryOperator::Yield => f.keyword(display::Keyword::Yield)?,
            UnaryOperator::YieldDelegate => {
                f.keyword(display::Keyword::Yield)?;
                f.punctuator(display::Punctuator::Star)?
            }

            // TODO: Precedence on this is hard
            UnaryOperator::Bind => f.punctuator(display::Punctuator::Bind)?,
        }

        f.require_precedence(display::Precedence::Unary).node(&self.value)
    }
}
impl misc::FirstSpecialToken for UnaryExpression {}
impl misc::HasInOperator for UnaryExpression {}


// foo OP bar
nodes!(pub struct BinaryExpression {
    left: Box<alias::Expression>,
    operator: BinaryOperator,
    right: Box<alias::Expression>,
});
pub enum BinaryOperator {
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
}
impl display::NodeDisplay for BinaryExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {

        match self.operator {
            BinaryOperator::Add => {
                let mut f = f.precedence(display::Precedence::Additive);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Plus)?;
                f.require_precedence(display::Precedence::Multiplicative).node(&self.right)?;
            }
            BinaryOperator::Subtract => {
                let mut f = f.precedence(display::Precedence::Additive);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Minus)?;
                f.require_precedence(display::Precedence::Multiplicative).node(&self.right)?;
            }
            BinaryOperator::LeftShift => {
                let mut f = f.precedence(display::Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::LAngleAngle)?;
                f.require_precedence(display::Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::RightShift => {
                let mut f = f.precedence(display::Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::RAngleAngle)?;
                f.require_precedence(display::Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::RightShiftSigned => {
                let mut f = f.precedence(display::Precedence::Shift);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::RAngleAngleAngle)?;
                f.require_precedence(display::Precedence::Additive).node(&self.right)?;
            }
            BinaryOperator::Divide => {
                let mut f = f.precedence(display::Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Slash)?;
                f.require_precedence(display::Precedence::Exponential).node(&self.right)?;
            }
            BinaryOperator::Multiply => {
                let mut f = f.precedence(display::Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Star)?;
                f.require_precedence(display::Precedence::Exponential).node(&self.right)?;
            }
            BinaryOperator::Modulus => {
                let mut f = f.precedence(display::Precedence::Multiplicative);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Mod)?;
                f.require_precedence(display::Precedence::Exponential).node(&self.right)?;
            }
            BinaryOperator::BitAnd => {
                let mut f = f.precedence(display::Precedence::BitwiseAnd);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Amp)?;
                f.require_precedence(display::Precedence::Equality).node(&self.right)?;
            }
            BinaryOperator::BitOr => {
                let mut f = f.precedence(display::Precedence::BitwiseOr);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Bar)?;
                f.require_precedence(display::Precedence::BitwiseXOr).node(&self.right)?;
            }
            BinaryOperator::BitXor => {
                let mut f = f.precedence(display::Precedence::BitwiseXOr);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Caret)?;
                f.require_precedence(display::Precedence::BitwiseAnd).node(&self.right)?;
            }
            BinaryOperator::Power => {
                let mut f = f.precedence(display::Precedence::Update);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::StarStar)?;
                f.require_precedence(display::Precedence::Exponential).node(&self.right)?;
            }
            BinaryOperator::Compare => {
                let mut f = f.precedence(display::Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::EqEq)?;
                f.require_precedence(display::Precedence::Relational).node(&self.right)?;
            }
            BinaryOperator::StrictCompare => {
                let mut f = f.precedence(display::Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::EqEqEq)?;
                f.require_precedence(display::Precedence::Relational).node(&self.right)?;
            }
            BinaryOperator::NegateCompare => {
                let mut f = f.precedence(display::Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::Neq)?;
                f.require_precedence(display::Precedence::Relational).node(&self.right)?;
            }
            BinaryOperator::NegateStrictCompare => {
                let mut f = f.precedence(display::Precedence::Equality);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::NeqEq)?;
                f.require_precedence(display::Precedence::Relational).node(&self.right)?;
            }
            BinaryOperator::LessThan => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::LAngle)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::LessThanEq => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::LAngleEq)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::GreaterThan => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::RAngle)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::GreaterThanEq => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::RAngleEq)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::In => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.keyword(display::Keyword::In)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::Instanceof => {
                let mut f = f.precedence(display::Precedence::Relational);
                f.node(&self.left)?;
                f.keyword(display::Keyword::Instanceof)?;
                f.require_precedence(display::Precedence::Shift).node(&self.right)?;
            }
            BinaryOperator::And => {
                let mut f = f.precedence(display::Precedence::LogicalAnd);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::AmpAmp)?;
                f.require_precedence(display::Precedence::BitwiseOr).node(&self.right)?;
            }
            BinaryOperator::Or => {
                let mut f = f.precedence(display::Precedence::LogicalOr);
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::BarBar)?;
                f.require_precedence(display::Precedence::LogicalAnd).node(&self.right)?;
            }
            BinaryOperator::Bind => {
                // TODO: Precedence
                f.node(&self.left)?;
                f.punctuator(display::Punctuator::ColonColon)?;
                f.node(&self.right)?;
            }
        }

        Ok(())
    }
}
impl misc::FirstSpecialToken for BinaryExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
}
impl misc::HasInOperator for BinaryExpression {
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
            BinaryOperator::StrictCompare => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::NegateCompare => self.left.has_in_operator() || self.right.has_in_operator(),
            BinaryOperator::NegateStrictCompare => {
                self.left.has_in_operator() || self.right.has_in_operator()
            },
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
nodes!(pub struct ConditionalExpression {
    test: Box<alias::Expression>,
    consequent: Box<alias::Expression>,
    alternate: Box<alias::Expression>,
});
impl display::NodeDisplay for ConditionalExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::LogicalOr).node(&self.test)?;
        f.punctuator(display::Punctuator::Question)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Assignment).node(&self.consequent)?;
        }
        f.punctuator(display::Punctuator::Colon)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.alternate)
    }
}
impl misc::FirstSpecialToken for ConditionalExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.test.first_special_token() }
}
impl misc::HasInOperator for ConditionalExpression {
    fn has_in_operator(&self) -> bool {
        self.test.has_in_operator()
    }
}


// foo = bar
nodes!(pub struct AssignmentExpression {
    left: Box<misc::LeftHandComplexAssign>,
    right: Box<alias::Expression>,
});
impl display::NodeDisplay for AssignmentExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::LeftHand).node(&self.left)?;
        f.punctuator(display::Punctuator::Eq)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.right)
    }
}
impl misc::FirstSpecialToken for AssignmentExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
}
impl misc::HasInOperator for AssignmentExpression {
    fn has_in_operator(&self) -> bool {
        self.right.has_in_operator()
    }
}


// foo OP= bar
nodes!(pub struct AssignmentUpdateExpression {
    left: Box<misc::LeftHandSimpleAssign>,
    operator: AssignmentUpdateOperator,
    right: Box<alias::Expression>,
});
pub enum AssignmentUpdateOperator {
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
}
impl display::NodeDisplay for AssignmentUpdateExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.require_precedence(display::Precedence::LeftHand).node(&self.left)?;
        match self.operator {
            AssignmentUpdateOperator::Add => f.punctuator(display::Punctuator::Plus)?,
            AssignmentUpdateOperator::Subtract => f.punctuator(display::Punctuator::Subtract)?,
            AssignmentUpdateOperator::LeftShift => f.punctuator(display::Punctuator::LAngleAngle)?,
            AssignmentUpdateOperator::RightShift => f.punctuator(display::Punctuator::RAngleAngle)?,
            AssignmentUpdateOperator::RightShiftSigned => f.punctuator(display::Punctuator::RAngleAngleAngle)?,
            AssignmentUpdateOperator::Divide => f.punctuator(display::Punctuator::Slash)?,
            AssignmentUpdateOperator::Multiply => f.punctuator(display::Punctuator::Star)?,
            AssignmentUpdateOperator::Modulus => f.punctuator(display::Punctuator::Mod)?,
            AssignmentUpdateOperator::BitAnd => f.punctuator(display::Punctuator::Amp)?,
            AssignmentUpdateOperator::BitOr => f.punctuator(display::Punctuator::Bar)?,
            AssignmentUpdateOperator::BitXor => f.punctuator(display::Punctuator::Caret)?,
            AssignmentUpdateOperator::Power => f.punctuator(display::Punctuator::StarStar)?,
        }
        f.punctuator(display::Punctuator::Eq)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.right)
    }
}
impl misc::FirstSpecialToken for AssignmentUpdateExpression {
    fn first_special_token(&self) -> misc::SpecialToken {
        self.left.first_special_token()
    }
}
impl misc::HasInOperator for AssignmentUpdateExpression {
    fn has_in_operator(&self) -> bool {
        self.right.has_in_operator()
    }
}


// foo, bar
nodes!(pub struct SequenceExpression {
    left: Box<alias::Expression>,
    right: Box<alias::Expression>,
});
impl display::NodeDisplay for SequenceExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.precedence(display::Precedence::Normal);

        f.node(&self.left)?;
        f.punctuator(display::Punctuator::Comma)?;

        // Note: This precedence isn't needed to reproduce functionality, but it is to make the
        // AST reproduce properly from the serialized code. Parens can be avoided by reordering AST.
        f.require_precedence(display::Precedence::Assignment).node(&self.right)?;

        Ok(())
    }
}
impl misc::FirstSpecialToken for SequenceExpression {
    fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
}
impl misc::HasInOperator for SequenceExpression {
    fn has_in_operator(&self) -> bool {
        self.left.has_in_operator() || self.right.has_in_operator()
    }
}


// (foo) => bar
nodes!(pub struct ArrowFunctionExpression {
    // TODO: Needs to handle single-param Ident output as type of params
    params: misc::FunctionParams,
    body: ArrowFunctionBody,
    fn_kind: ArrowFunctionKind,
});
pub enum ArrowFunctionKind {
    Normal,
    Async,

    Generator, // experimental
    AsyncGenerator, // experimental
}
impl display::NodeDisplay for ArrowFunctionExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match self.fn_kind {
            ArrowFunctionKind::Normal => {},
            ArrowFunctionKind::Async => f.keyword(display::Keyword::Async)?,
            _ => {
                // TODO
            }
        }

        f.node(&self.params)?;
        f.punctuator(display::Punctuator::Arrow)?;
        f.node(&self.body)
    }
}
impl misc::FirstSpecialToken for ArrowFunctionExpression {}
impl misc::HasInOperator for ArrowFunctionExpression {
    fn has_in_operator(&self) -> bool {
        match self.body {
            ArrowFunctionBody::Block(_) => false,
            ArrowFunctionBody::Expression(ref expr) => expr.has_in_operator(),
        }
    }
}


pub enum ArrowFunctionBody {
    Expression(Box<alias::Expression>),

    // TODO: Do we need an async arrow body for fn return val
    Block(misc::FunctionBody),
}
impl display::NodeDisplay for ArrowFunctionBody {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ArrowFunctionBody::Expression(ref expr) => {
                if let misc::SpecialToken::Object = expr.first_special_token() {
                    f.wrap_parens().node(expr)
                } else {
                    f.require_precedence(display::Precedence::Assignment).node(expr)
                }
            }
            ArrowFunctionBody::Block(ref body) => f.node(body),
        }
    }
}


// do { foo; }
nodes!(pub struct DoExpression {
    body: BlockStatement,
});
impl display::NodeDisplay for DoExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Do)?;
        f.node(&self.body)
    }
}
impl misc::FirstSpecialToken for DoExpression {}
impl misc::HasInOperator for DoExpression {}


// new.target
nodes!(pub struct MetaProperty {
    kind: MetaPropertyKind,
});
pub enum MetaPropertyKind {
    NewTarget,
    ImportMeta, // experimental
    FunctionSent, // experimental
    FunctionArguments, // experimental
}
impl display::NodeDisplay for MetaProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match self.kind {
            MetaPropertyKind::NewTarget => {
                f.keyword(display::Keyword::New)?;
                f.punctuator(display::Punctuator::Period)?;
                f.keyword(display::Keyword::Target)
            }
            MetaPropertyKind::ImportMeta => {
                f.keyword(display::Keyword::Import)?;
                f.punctuator(display::Punctuator::Period)?;
                f.keyword(display::Keyword::Meta)
            }
            MetaPropertyKind::FunctionSent => {
                f.keyword(display::Keyword::Function)?;
                f.punctuator(display::Punctuator::Period)?;
                f.keyword(display::Keyword::Sent)
            }
            MetaPropertyKind::FunctionArguments => {
                f.keyword(display::Keyword::Function)?;
                f.punctuator(display::Punctuator::Period)?;
                f.keyword(display::Keyword::Arguments)
            }
        }
    }
}
impl misc::FirstSpecialToken for MetaProperty {}
impl misc::HasInOperator for MetaProperty {}


// super.foo
// super[foo]
nodes!(pub struct SuperMemberExpression {
    property: misc::PropertyAccess,
});
impl display::NodeDisplay for SuperMemberExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Super)?;
        f.node(&self.property)
    }
}
impl misc::FirstSpecialToken for SuperMemberExpression {}
impl misc::HasInOperator for SuperMemberExpression {}
