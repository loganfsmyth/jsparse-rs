use std::string;

use ast::{MaybeTokenPosition, KeywordData, KeywordSuffixData, KeywordWrappedData, SeparatorTokens, SeparatorTokensSingleLine};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadSequence};

// use super::misc;
use ast::alias;
// use super::display;

use ast::patterns::{LeftHandSimpleAssign, LeftHandComplexAssign};
use ast::statement::BlockStatement;
use ast::general::{ReferenceIdentifier, PropertyIdentifier};


// this
node!(#[derive(Default)] pub struct ThisExpression {
    pub token_this: MaybeTokenPosition,
});
impl NodeDisplay for ThisExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::This, &self.token_this);
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
    pub token_paren_l: KeywordSuffixData,
    pub expr: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
});
impl NodeDisplay for ParenthesizedExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.wrap_parens().node(&self.expr)
    }
}
impl<T: Into<alias::Expression>> From<T> for ParenthesizedExpression {
    fn from(expr: T) -> ParenthesizedExpression {
        ParenthesizedExpression {
            token_paren_l: Default::default(),
            expr: Box::new(expr.into()),
            token_paren_r: Default::default(),
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
    pub tokens_sep: SeparatorTokens,
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
    pub token_tick_open: MaybeTokenPosition,
    pub parts: Vec<(TemplatePart, KeywordSuffixData, alias::Expression, KeywordData)>,
    pub last_part: TemplatePart,
    pub token_tick_close: MaybeTokenPosition,
});
impl NodeDisplay for TemplateLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::TemplateTick, &self.token_tick_open);
        for &(ref part, ref close, ref expr, ref open) in self.parts.iter() {
            f.node(part)?;
            f.punctuator(Punctuator::TemplateClose, close);
            f.allow_in().node(expr)?;
            f.punctuator(Punctuator::TemplateOpen, open);
        }
        f.node(&self.last_part)?;
        f.punctuator(Punctuator::TemplateTick, &self.token_tick_close);
        Ok(())
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


node_enum!(@node_display pub enum CallArgument {
    Expression(CallArgumentExpression),
    Spread(CallArgumentSpread),
});
impl From<alias::Expression> for CallArgument {
    fn from(e: alias::Expression) -> CallArgument {
        CallArgumentExpression {
            tokens_prefix: Default::default(),
            expression: e.into(),
            position: None,
        }.into()
    }
}

node!(pub struct CallArgumentExpression {
    pub tokens_prefix: SeparatorTokens,
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for CallArgumentExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.expression)
    }
}

node!(pub struct CallArgumentSpread {
    pub token_ellipsis: KeywordData,
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for CallArgumentSpread {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.expression)
    }
}

node!(#[derive(Default)] pub struct CallArguments {
    pub token_paren_l: KeywordSuffixData,

    pub args: Vec<(CallArgument, KeywordData)>,
    pub last_arg: Option<CallArgument>,

    pub token_paren_r: KeywordData,
});
impl CallArguments {
    pub fn is_empty(&self) -> bool {
        self.args.is_empty() && self.last_arg.is_none()
    }
}
impl NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_parens();

        f.comma_list(&self.args)?;
        f.node(&self.last_arg)
    }
}
impl From<Vec<alias::Expression>> for CallArguments {
    fn from(v: Vec<alias::Expression>) -> CallArguments {
        CallArguments {
            token_paren_l: Default::default(),
            args: v.into_iter().map(|e| (e.into(), Default::default())).collect(),
            last_arg: Default::default(),
            token_paren_r: Default::default(),
            position: None,
        }
    }
}

#[cfg(test)]
mod tests_call_args {
    use super::*;
    use ast::general::ReferenceIdentifier;
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
                ReferenceIdentifier::from("arg").into(),
                literal::Boolean::from(true).into(),
            ]),
            "(this,arg,true)"
        );
    }

    #[test]
    fn it_prints_args_with_precedence() {
        assert_serialize!(
            CallArguments::from(vec![
                ReferenceIdentifier::from("arg1").into(),
                SequenceExpression {
                    left: ThisExpression::default().into(),
                    right: literal::Boolean::from(true).into(),
                    position: None,
                }.into(),
                ReferenceIdentifier::from("arg3").into(),
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
                    ReferenceIdentifier::from("arg").into(),
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
                args: vec![ReferenceIdentifier::from("arg").into()],
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
node!(pub struct CallExpression {
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
});
impl NodeDisplay for CallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO: I think this sometimes adds parens when not needed?
        f.require_precedence(Precedence::New).node(&self.callee)?;
        f.node(&self.arguments)
    }
}

// foo?.()
node!(pub struct OptionalCallExpression {
    pub callee: Box<alias::Expression>,
    pub token_question: KeywordData,
    pub arguments: CallArguments,
});
impl NodeDisplay for OptionalCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO: I think this sometimes adds parens when not needed?
        f.require_precedence(Precedence::New).node(&self.callee)?;
        f.punctuator(Punctuator::QuestionPeriod);
        f.node(&self.arguments)
    }
}

node_enum!(pub enum NewExpression {
    Normal(NormalNewExpression),
    Empty(EmptyNewExpression),
});

// new foo()
node!(pub struct NormalNewExpression {
    pub token_new: KeywordSuffixData,
    pub callee: Box<alias::Expression>,
    pub arguments: CallArguments,
});
impl NodeDisplay for NormalNewExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Member);
        f.keyword(Keyword::New);
        f.require_precedence(Precedence::Member).node(&self.callee)?;
        f.node(&self.arguments)?;
        Ok(())
    }
}

// new foo
node!(pub struct EmptyNewExpression {
    pub token_new: KeywordSuffixData,
    pub callee: Box<alias::Expression>,
});
impl NodeDisplay for EmptyNewExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::New);
        f.keyword(Keyword::New);
        f.require_precedence(Precedence::New).node(&self.callee)?;
        Ok(())
    }
}


// experimental
// import(foo)
node!(pub struct ImportCallExpression {
    pub token_import: MaybeTokenPosition,
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
    pub token_super: MaybeTokenPosition,
    pub arguments: CallArguments,
});
impl NodeDisplay for SuperCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Super);
        f.node(&self.arguments)
    }
}


// foo.bar
// foo[bar]
// foo.#bar
node!(pub struct MemberExpression {
    pub object: Box<alias::Expression>,
    pub property: PropertyAccess,
});
impl NodeDisplay for MemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO: I think this sometimes adds parens when not needed?
        // Member expressions can either be part of the MemberExpression grammar or
        // the CallExpression grammar

        let mut f = f.lookahead_wrap_parens(get_sequence(self));
        f.require_precedence(Precedence::Member).node(&self.object)?;
        f.node(&self.property)?;
        Ok(())
    }
}

/// Figure out which, if any, lookahead sequence matches this expression.
fn get_sequence(expr: &MemberExpression) -> LookaheadSequence {
    use ast::alias::Expression::Binding;

    match *expr.object {
        Binding(ReferenceIdentifier { ref value, .. }) if value == "let" => {
            match expr.property {
                PropertyAccess::Computed(_) => LookaheadSequence::LetSquare,
                _ => LookaheadSequence::Let,
            }
        }
        _ => LookaheadSequence::None,
    }
}


// foo?.bar
// foo?.[bar]
// foo?.#bar
node!(pub struct OptionalMemberExpression {
    pub object: Box<alias::Expression>,
    pub property: OptionalPropertyAccess,
});
impl NodeDisplay for OptionalMemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::Member).node(&self.object)?;
        f.punctuator(Punctuator::Question);
        f.node(&self.property)?;
        Ok(())
    }
}


node_enum!(@node_display pub enum PropertyAccess {
    Identifier(IdentifierPropertyAccess),
    Computed(ComputedPropertyAccess),
    Private(PrivatePropertyAccess),
});

// .foo
node!(pub struct IdentifierPropertyAccess {
    pub token_period: KeywordData,
    pub tokens_separator: SeparatorTokens,
    pub id: PropertyIdentifier,
});
impl NodeDisplay for IdentifierPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Period);
        f.node(&self.id)
    }
}

// [foo]
node!(pub struct ComputedPropertyAccess {
    pub tokens_prefix: SeparatorTokens,
    pub token_square_l: KeywordSuffixData,
    pub expression: Box<alias::Expression>,
    pub token_square_r: KeywordData,
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

// .#foo
node!(pub struct PrivatePropertyAccess {
    pub token_period: KeywordData,
    pub token_hash: KeywordData,
    pub tokens_separator: SeparatorTokens,
    pub property: PropertyIdentifier,
});
impl NodeDisplay for PrivatePropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Period);
        f.punctuator(Punctuator::Hash);
        f.node(&self.property)
    }
}




node_enum!(@node_display pub enum OptionalPropertyAccess {
    Identifier(OptionalIdentifierPropertyAccess),
    Computed(OptionalComputedPropertyAccess),
    Private(OptionalPrivatePropertyAccess),
});

// ?.[foo]
node!(pub struct OptionalComputedPropertyAccess {
    pub token_question_period: KeywordData,
    pub tokens_separator: SeparatorTokens,
    pub token_square_l: KeywordSuffixData,
    pub expression: Box<alias::Expression>,
    pub token_square_r: KeywordSuffixData,
});
impl NodeDisplay for OptionalComputedPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::QuestionPeriod);

        let mut f = f.wrap_square();
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        Ok(())
    }
}

// ?.foo
node!(pub struct OptionalIdentifierPropertyAccess {
    pub token_question_period: KeywordData,
    pub tokens_separator: SeparatorTokens,
    pub id: PropertyIdentifier,
});
impl NodeDisplay for OptionalIdentifierPropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::QuestionPeriod);
        f.node(&self.id)
    }
}

// ?.#foo
node!(pub struct OptionalPrivatePropertyAccess {
    pub token_question_period: KeywordData,
    pub token_hash: KeywordData,
    pub tokens_separator: SeparatorTokens,
    pub property: PropertyIdentifier,
});
impl NodeDisplay for OptionalPrivatePropertyAccess {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::QuestionPeriod);
        f.punctuator(Punctuator::Hash);
        f.node(&self.property)
    }
}

// i++
node!(pub struct PostIncrementExpression {
    pub value: LeftHandSimpleAssign,
    pub tokens_separator: SeparatorTokensSingleLine,
    pub token_plusplus: MaybeTokenPosition,
});
impl NodeDisplay for PostIncrementExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.value)?;
        f.punctuator(Punctuator::PlusPlus);
        Ok(())
    }
}

// i--
node!(pub struct PostDecrementExpression {
    pub value: LeftHandSimpleAssign,
    pub tokens_separator: SeparatorTokensSingleLine,
    pub token_minusminus: MaybeTokenPosition,
});
impl NodeDisplay for PostDecrementExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.value)?;
        f.punctuator(Punctuator::MinusMinus);
        Ok(())
    }
}

// ++i
node!(pub struct PreIncrementExpression {
    pub token_plusplus: KeywordSuffixData,
    pub value: LeftHandSimpleAssign,
});
impl NodeDisplay for PreIncrementExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::PlusPlus);
        f.require_precedence(Precedence::Unary).node(&self.value)
    }
}

// --i
node!(pub struct PreDecrementExpression {
    pub token_minusminus: KeywordSuffixData,
    pub value: LeftHandSimpleAssign,
});
impl NodeDisplay for PreDecrementExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::MinusMinus);
        f.require_precedence(Precedence::Unary).node(&self.value)
    }
}





node!(pub struct DeleteExpression {
    pub token_delete: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for DeleteExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Delete);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct VoidExpression {
    pub token_void: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for VoidExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Void);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct TypeofExpression {
    pub token_typeof: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for TypeofExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Typeof);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct PositiveExpression {
    pub token_plus: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for PositiveExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.punctuator(Punctuator::Plus);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct NegativeExpression {
    pub token_minus: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for NegativeExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.punctuator(Punctuator::Exclam);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct BitNegateExpression {
    pub token_caret: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for BitNegateExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.punctuator(Punctuator::Tilde);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct NegateExpression {
    pub token_exclam: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for NegateExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.punctuator(Punctuator::Exclam);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct AwaitExpression {
    pub token_await: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for AwaitExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Await);
        f.require_precedence(Precedence::Unary).node(&self.value)?;
        Ok(())
    }
}


node!(pub struct YieldExpression {
    pub token_yield: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for YieldExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Yield);
        f.require_precedence(Precedence::Assignment).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct DelegateYieldExpression {
    pub token_yield: KeywordSuffixData,
    // TODO: No newlines allowed
    pub token_star: KeywordSuffixData,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for DelegateYieldExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.keyword(Keyword::Yield);
        f.punctuator(Punctuator::Star);
        f.require_precedence(Precedence::Assignment).node(&self.value)?;
        Ok(())
    }
}
node!(pub struct BindMemberExpression {
    pub token_coloncolon: KeywordSuffixData,
    // TODO: No newlines allowed
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for BindMemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Unary);
        f.punctuator(Punctuator::Bind);
        f.require_precedence(Precedence::Member).node(&self.value)?;
        Ok(())
    }
}


// foo + bar
node!(pub struct AddExpression {
    pub left: Box<alias::Expression>,
    pub token_plus: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AddExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Additive);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Plus);
        f.require_precedence(Precedence::Multiplicative).node(
            &self.right,
        );
        Ok(())
    }
}

// foo - bar
node!(pub struct SubtractExpression {
    pub left: Box<alias::Expression>,
    pub token_minus: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for SubtractExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Additive);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Minus);
        f.require_precedence(Precedence::Multiplicative).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo << bar
node!(pub struct LeftShiftExpression {
    pub left: Box<alias::Expression>,
    pub token_langleangle: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for LeftShiftExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Shift);
        f.node(&self.left)?;
        f.punctuator(Punctuator::LAngleAngle);
        f.require_precedence(Precedence::Additive).node(&self.right)?;
        Ok(())
    }
}

// foo >> bar
node!(pub struct RightShiftExpression {
    pub left: Box<alias::Expression>,
    pub token_rangleangle: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for RightShiftExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Shift);
        f.node(&self.left)?;
        f.punctuator(Punctuator::RAngleAngle);
        f.require_precedence(Precedence::Additive).node(&self.right)?;
        Ok(())
    }
}

// foo >>> bar
node!(pub struct RightShiftSignedExpression {
    pub left: Box<alias::Expression>,
    pub token_rangleangleangle: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for RightShiftSignedExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Shift);
        f.node(&self.left)?;
        f.punctuator(Punctuator::RAngleAngleAngle);
        f.require_precedence(Precedence::Additive).node(&self.right)?;
        Ok(())
    }
}
// foo / bar
node!(pub struct DivideExpression {
    pub left: Box<alias::Expression>,
    pub token_slash: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for DivideExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Multiplicative);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Slash);
        f.require_precedence(Precedence::Exponential).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo * bar
node!(pub struct MultiplyExpression {
    pub left: Box<alias::Expression>,
    pub token_star: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for MultiplyExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Multiplicative);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Star);
        f.require_precedence(Precedence::Exponential).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo % bar
node!(pub struct ModulusExpression {
    pub left: Box<alias::Expression>,
    pub token_percent: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for ModulusExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Multiplicative);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Mod);
        f.require_precedence(Precedence::Exponential).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo & bar
node!(pub struct BitAndExpression {
    pub left: Box<alias::Expression>,
    pub token_amp: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitAndExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::BitwiseAnd);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Amp);
        f.require_precedence(Precedence::Equality).node(&self.right)?;
        Ok(())
    }
}

// foo | bar
node!(pub struct BitOrExpression {
    pub left: Box<alias::Expression>,
    pub token_bar: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitOrExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::BitwiseOr);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Bar);
        f.require_precedence(Precedence::BitwiseXOr).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo ^ bar
node!(pub struct BitXorExpression {
    pub left: Box<alias::Expression>,
    pub token_caret: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitXorExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::BitwiseXOr);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Caret);
        f.require_precedence(Precedence::BitwiseAnd).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo ** bar
node!(pub struct PowerExpression {
    pub left: Box<alias::Expression>,
    pub token_starstar: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for PowerExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Update);
        f.node(&self.left)?;
        f.punctuator(Punctuator::StarStar);
        f.require_precedence(Precedence::Exponential).node(
            &self.right,
        )?;
        Ok(())
    }
}


// foo == bar
node!(pub struct EqualExpression {
    pub left: Box<alias::Expression>,
    pub token_eqeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for EqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Equality);
        f.node(&self.left)?;
        f.punctuator(Punctuator::EqEq);
        f.require_precedence(Precedence::Relational).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo === bar
node!(pub struct StrictEqualExpression {
    pub left: Box<alias::Expression>,
    pub token_eqeqeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for StrictEqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Equality);
        f.node(&self.left)?;
        f.punctuator(Punctuator::EqEqEq);
        f.require_precedence(Precedence::Relational).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo != bar
node!(pub struct NotEqualExpression {
    pub left: Box<alias::Expression>,
    pub token_neeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for NotEqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Equality);
        f.node(&self.left)?;
        f.punctuator(Punctuator::Neq);
        f.require_precedence(Precedence::Relational).node(
            &self.right,
        )
    }
}
// foo !== bar
node!(pub struct StrictNotEqualExpression {
    pub left: Box<alias::Expression>,
    pub token_neeqeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for StrictNotEqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Equality);
        f.node(&self.left)?;
        f.punctuator(Punctuator::NeqEq);
        f.require_precedence(Precedence::Relational).node(
            &self.right,
        )?;
        Ok(())
    }
}
// foo < bar
node!(pub struct LessThanExpression {
    pub left: Box<alias::Expression>,
    pub token_langle: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for LessThanExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        f.node(&self.left)?;
        f.punctuator(Punctuator::LAngle);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}

// foo <= bar
node!(pub struct LessThanEqualExpression {
    pub left: Box<alias::Expression>,
    pub token_langleeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for LessThanEqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        f.node(&self.left)?;
        f.punctuator(Punctuator::LAngleEq);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}

// foo > bar
node!(pub struct GreatherThanExpression {
    pub left: Box<alias::Expression>,
    pub token_rangle: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for GreatherThanExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        f.node(&self.left)?;
        f.punctuator(Punctuator::RAngle);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}

// foo >= bar
node!(pub struct GreaterThanEqualExpression {
    pub left: Box<alias::Expression>,
    pub token_rangleeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for GreaterThanEqualExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        f.node(&self.left)?;
        f.punctuator(Punctuator::RAngleEq);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}


// foo in bar
node!(pub struct InExpression {
    pub left: Box<alias::Expression>,
    pub token_in: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for InExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        let mut f = f.in_wrap_parens();
        f.node(&self.left)?;
        f.keyword(Keyword::In);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}

// foo instanceof bar
node!(pub struct InstanceofExpression {
    pub left: Box<alias::Expression>,
    pub token_instanceof: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for InstanceofExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Relational);
        f.node(&self.left)?;
        f.keyword(Keyword::Instanceof);
        f.require_precedence(Precedence::Shift).node(&self.right)?;
        Ok(())
    }
}

// foo && bar
node!(pub struct AndExpression {
    pub left: Box<alias::Expression>,
    pub token_ampamp: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AndExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::LogicalAnd);
        f.node(&self.left)?;
        f.punctuator(Punctuator::AmpAmp);
        f.require_precedence(Precedence::BitwiseOr).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo || bar
node!(pub struct OrExpression {
    pub left: Box<alias::Expression>,
    pub token_barbar: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for OrExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::LogicalOr);
        f.node(&self.left)?;
        f.punctuator(Punctuator::BarBar);
        f.require_precedence(Precedence::LogicalAnd).node(
            &self.right,
        )?;
        Ok(())
    }
}

// foo :: bar
node!(pub struct BindExpression {
    pub left: Box<alias::Expression>,
    pub token_coloncolon: KeywordWrappedData,
    // TODO: For bind, 'right' can be restructed to MemberExpression or SuperProperty,
    // so it should probably be its own node type.
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BindExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO: Parens might not be right when inside a callexpr?
        let mut f = f.precedence(Precedence::LeftHand);
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::ColonColon);
        f.require_precedence(Precedence::Member).node(&self.right)?;
        Ok(())
    }
}




// foo ? bar : baz
node!(pub struct ConditionalExpression {
    pub test: Box<alias::Expression>,
    pub token_question: KeywordWrappedData,
    pub consequent: Box<alias::Expression>,
    pub token_colon: KeywordWrappedData,
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
    pub token_eq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AssignmentExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let sequence = if let LeftHandComplexAssign::Object(_) = *self.left {
            LookaheadSequence::Curly
        } else {
            LookaheadSequence::None
        };

        let mut f = f.lookahead_wrap_parens(sequence);
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )?;
        Ok(())
    }
}


// foo += bar
node!(pub struct AddAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_pluseq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for AddAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Plus);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo -= bar
node!(pub struct SubtractAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_minuseq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for SubtractAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Subtract);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo <<= bar
node!(pub struct LeftShiftAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_langleangleeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for LeftShiftAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::LAngleAngle);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo >>= bar
node!(pub struct RightShiftAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_rangleangleeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for RightShiftAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::RAngleAngle);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo >>>= bar
node!(pub struct RightShiftSignedAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_rangleangleangleeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for RightShiftSignedAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::RAngleAngleAngle);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo /= bar
node!(pub struct DivideAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_slasheq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for DivideAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Slash);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo *= bar
node!(pub struct MultipleAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_stareq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for MultipleAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Star);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo %= bar
node!(pub struct ModulusAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_percenteq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for ModulusAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Mod);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo &= bar
node!(pub struct BitAndAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_ampeq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitAndAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Amp);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo |= bar
node!(pub struct BitOrAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_bareq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitOrAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Bar);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo ^= bar
node!(pub struct BitXorAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_careteq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for BitXorAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::Caret);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}
// foo **= bar
node!(pub struct PowerAssignExpression {
    pub left: Box<LeftHandSimpleAssign>,
    pub token_starstareq: KeywordWrappedData,
    pub right: Box<alias::Expression>,
});
impl NodeDisplay for PowerAssignExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.require_precedence(Precedence::LeftHand).node(&self.left)?;
        f.punctuator(Punctuator::StarStar);
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.right,
        )
    }
}



// foo, bar
node!(pub struct SequenceExpression {
    pub left: Box<alias::Expression>,
    pub token_comma: KeywordWrappedData,
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
    pub token_do: MaybeTokenPosition,
    pub body: BlockStatement,
});
impl NodeDisplay for DoExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Do);
        f.node(&self.body)
    }
}


// new.target
node!(pub struct NewTargetExpression {
    pub token_new: MaybeTokenPosition,
    pub token_period: KeywordWrappedData,
    pub token_target: MaybeTokenPosition,
});
impl NodeDisplay for NewTargetExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::New);
        f.punctuator(Punctuator::Period);
        f.keyword(Keyword::Target);
        Ok(())
    }
}

// import.meta
node!(pub struct ImportMetaExpression {
    pub token_import: MaybeTokenPosition,
    pub token_period: KeywordWrappedData,
    pub token_meta: MaybeTokenPosition,
});
impl NodeDisplay for ImportMetaExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Import);
        f.punctuator(Punctuator::Period);
        f.keyword(Keyword::Meta);
        Ok(())
    }
}

// function.sent
node!(pub struct FunctionSentExpression {
    pub token_function: MaybeTokenPosition,
    pub token_period: KeywordWrappedData,
    pub token_send: MaybeTokenPosition,
});
impl NodeDisplay for FunctionSentExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Function);
        f.punctuator(Punctuator::Period);
        f.keyword(Keyword::Sent);
        Ok(())
    }
}

// function.arguments
node!(pub struct FunctionArgumentsExpression {
    pub token_function: MaybeTokenPosition,
    pub token_period: KeywordWrappedData,
    pub token_arguments: MaybeTokenPosition,
});
impl NodeDisplay for FunctionArgumentsExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Function);
        f.punctuator(Punctuator::Period);
        f.keyword(Keyword::Arguments);
        Ok(())
    }
}


// super.foo
// super[foo]
node!(pub struct SuperMemberExpression {
    pub token_super: MaybeTokenPosition,
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
