use std::string;
use std::default;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadRestriction, LookaheadSequence};

use ast::general::BindingIdentifier;
use ast::alias;
use ast::patterns::BindingPattern;

use ast::decorators::DecoratorValue;


node!(pub struct Directive {
    pub value: DirectiveLiteral,
});
impl NodeDisplay for Directive {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.value)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl<T: Into<DirectiveLiteral>> From<T> for Directive {
    fn from(v: T) -> Directive {
        Directive {
            value: v.into(),
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_directive {
    use super::*;

    #[test]
    fn it_prints_default() {
        assert_serialize!(Directive::from("use strict"), "'use strict';");
    }
}

node!(pub struct DirectiveLiteral {
    pub value: string::String,
});
impl NodeDisplay for DirectiveLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO: This needs its own primitive rendering to preserve
        // the codepoint sequence exactly.
        f.string(&self.value, Some(&self.value))
    }
}
impl<T: Into<string::String>> From<T> for DirectiveLiteral {
    fn from(v: T) -> DirectiveLiteral {
        DirectiveLiteral {
            value: v.into(),
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_directive_literal {
    use super::*;

    #[test]
    fn it_prints_default() {
        assert_serialize!(DirectiveLiteral::from("use strict"), "'use strict'");
    }
}


node_kind!(pub enum FunctionKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental
});
impl default::Default for FunctionKind {
    fn default() -> FunctionKind {
        FunctionKind::Normal
    }
}
impl NodeDisplay for FunctionKind {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match *self {
            FunctionKind::Normal => {
                f.keyword(Keyword::Function);
            }
            FunctionKind::Generator => {
                f.keyword(Keyword::Function);
                f.punctuator(Punctuator::Star);
            }
            FunctionKind::Async => {
                f.keyword(Keyword::Async);
                f.keyword(Keyword::Function);
            }
            FunctionKind::AsyncGenerator => {
                f.keyword(Keyword::Async);
                f.keyword(Keyword::Function);
                f.punctuator(Punctuator::Star);
            }
        }
        Ok(())
    }
}


node!(#[derive(Default)] pub struct FunctionParams {
    pub params: Vec<FunctionParam>,
    pub rest: Option<FunctionRestParam>,
});
impl NodeDisplay for FunctionParams {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_parens();

        f.comma_list(&self.params)?;

        if let Some(ref param) = self.rest {
            if !self.params.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.node(param)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests_function_params {
    use super::*;
    use ast::literal;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(FunctionParams::default(), "()");
    }

    #[test]
    fn it_prints_params() {
        assert_serialize!(FunctionParams {
            params: vec![
                FunctionParam {
                    decorators: Default::default(),
                    id: BindingIdentifier::from("arg").into(),
                    init: Default::default(),
                    position: None,
                },
                FunctionParam {
                    decorators: Default::default(),
                    id: BindingIdentifier::from("arg2").into(),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                },
            ],
            rest: Default::default(),
            position: None,
        }, "(arg,arg2=true)");
    }

    #[test]
    fn it_prints_rest() {
        assert_serialize!(FunctionParams {
            params: Default::default(),
            rest: FunctionRestParam {
                id: BindingIdentifier::from("arg").into(),
                position: None,
            }.into(),
            position: None,
        }, "(...arg)");
    }

    #[test]
    fn it_prints_params_and_rest() {
        assert_serialize!(FunctionParams {
            params: vec![
                FunctionParam {
                    decorators: Default::default(),
                    id: BindingIdentifier::from("arg").into(),
                    init: Default::default(),
                    position: None,
                },
                FunctionParam {
                    decorators: Default::default(),
                    id: BindingIdentifier::from("arg2").into(),
                    init: literal::Boolean::from(true).into(),
                    position: None,
                },
            ],
            rest: FunctionRestParam {
                id: BindingIdentifier::from("arg3").into(),
                position: None,
            }.into(),
            position: None,
        }, "(arg,arg2=true,...arg3)");
    }
}


node!(pub struct FunctionParam {
    pub decorators: Vec<FunctionParamDecorator>, // experimental
    pub id: BindingPattern,
    pub init: Option<Box<alias::Expression>>,
});
impl NodeDisplay for FunctionParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node_list(&self.decorators)?;

        f.node(&self.id)?;

        if let Some(ref init) = self.init {
            f.punctuator(Punctuator::Eq);
            f.node(init)?;
        }
        Ok(())
    }
}
impl<T: Into<BindingPattern>> From<T> for FunctionParam {
    fn from(v: T) -> FunctionParam {
        FunctionParam {
            decorators: Default::default(),
            id: v.into(),
            init: Default::default(),
            position: None,
        }
    }
}


node!(pub struct FunctionRestParam {
    pub id: BindingPattern,
});
impl NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Ellipsis);
        f.node(&self.id)?;

        Ok(())
    }
}


node!(#[derive(Default)] pub struct FunctionBody {
    pub directives: Vec<Directive>,
    pub body: Vec<alias::StatementItem>,
});
impl NodeDisplay for FunctionBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.node_list(&self.directives)?;
        f.node_list(&self.body)?;

        Ok(())
    }
}

node!(pub struct FunctionParamDecorator {
    pub value: DecoratorValue,
});
impl NodeDisplay for FunctionParamDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}


// export default function name() {}
node!(#[derive(Default)] pub struct ExportDefaultFunctionDeclaration {
    pub kind: FunctionKind,
    pub id: Option<BindingIdentifier>,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for ExportDefaultFunctionDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.keyword(Keyword::Default);
        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_function_export_default {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(ExportDefaultFunctionDeclaration::default(), "export default function(){}");
    }

    #[test]
    fn it_prints_with_name() {
        assert_serialize!(ExportDefaultFunctionDeclaration {
            kind: Default::default(),
            id: BindingIdentifier::from("someName").into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "export default function someName(){}");
    }
}

// function name() {}
node!(pub struct FunctionDeclaration {
    pub kind: FunctionKind,
    pub id: BindingIdentifier,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for FunctionDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_function_declaration {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(FunctionDeclaration {
            kind: Default::default(),
            id: "someName".into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "function someName(){}");
    }
}

// (function(){})
node!(#[derive(Default)] pub struct FunctionExpression {
    pub kind: FunctionKind,
    pub id: Option<BindingIdentifier>,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for FunctionExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(LookaheadSequence::Declaration);

        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_function_expression {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(FunctionExpression::default(), "function(){}");
    }

    #[test]
    fn it_prints_with_name() {
        assert_serialize!(FunctionDeclaration {
            kind: Default::default(),
            id: "someName".into(),
            params: Default::default(),
            body: Default::default(),
            position: None,
        }, "function someName(){}");
    }
}


// (foo) => bar
node!(#[derive(Default)] pub struct ArrowFunctionExpression {
    // TODO: Needs to handle single-param Ident output as type of params
    pub kind: ArrowFunctionKind,
    pub params: ArrowFunctionParams,
    pub body: ArrowFunctionBody,
});
node_kind!(pub enum ArrowFunctionKind {
    Normal,
    Async,
    Generator, // experimental
    AsyncGenerator, // experimental
});
impl default::Default for ArrowFunctionKind {
    fn default() -> ArrowFunctionKind {
        ArrowFunctionKind::Normal
    }
}
impl NodeDisplay for ArrowFunctionExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match self.kind {
            ArrowFunctionKind::Normal => {
                f.node(&self.params)?;
                f.punctuator(Punctuator::Arrow);
            }
            ArrowFunctionKind::Async => {
                f.keyword(Keyword::Async);
                f.node(&self.params)?;
                f.punctuator(Punctuator::Arrow);
            }
            ArrowFunctionKind::Generator => {
                f.node(&self.params)?;
                f.punctuator(Punctuator::ArrowStar);
            }
            ArrowFunctionKind::AsyncGenerator => {
                f.keyword(Keyword::Async);
                f.node(&self.params)?;
                f.punctuator(Punctuator::ArrowStar);
            }
        }

        f.node(&self.body)
    }
}
#[cfg(test)]
mod tests_arrow_function_expression {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(ArrowFunctionExpression::default(), "()=>{}");
    }

    #[test]
    fn it_prints_single_ident() {
        assert_serialize!(ArrowFunctionExpression {
            kind: Default::default(),
            params: BindingIdentifier::from("arg").into(),
            body: Default::default(),
            position: None,
        }, "arg=>{}");
    }

    #[test]
    fn it_prints_single_ident_async() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::Async,
            params: BindingIdentifier::from("arg").into(),
            body: Default::default(),
            position: None,
        }, "async arg=>{}");
    }

    #[test]
    fn it_prints_single_ident_gen() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::Generator,
            params: BindingIdentifier::from("arg").into(),
            body: Default::default(),
            position: None,
        }, "arg=*>{}");
    }

    #[test]
    fn it_prints_single_ident_asyncgen() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::AsyncGenerator,
            params: BindingIdentifier::from("arg").into(),
            body: Default::default(),
            position: None,
        }, "async arg=*>{}");
    }

    #[test]
    fn it_prints_multi_param() {
        assert_serialize!(ArrowFunctionExpression {
            kind: Default::default(),
            params: FunctionParams {
                params: vec![
                    BindingIdentifier::from("arg1").into(),
                    BindingIdentifier::from("arg2").into(),
                ],
                rest: Default::default(),
                position: None,
            }.into(),
            body: Default::default(),
            position: None,
        }, "(arg1,arg2)=>{}");
    }

    #[test]
    fn it_prints_multi_param_async() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::Async,
            params: FunctionParams {
                params: vec![
                    BindingIdentifier::from("arg1").into(),
                    BindingIdentifier::from("arg2").into(),
                ],
                rest: Default::default(),
                position: None,
            }.into(),
            body: Default::default(),
            position: None,
        }, "async(arg1,arg2)=>{}");
    }

    #[test]
    fn it_prints_multi_param_gen() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::Generator,
            params: FunctionParams {
                params: vec![
                    BindingIdentifier::from("arg1").into(),
                    BindingIdentifier::from("arg2").into(),
                ],
                rest: Default::default(),
                position: None,
            }.into(),
            body: Default::default(),
            position: None,
        }, "(arg1,arg2)=*>{}");
    }

    #[test]
    fn it_prints_multi_param_asyncgen() {
        assert_serialize!(ArrowFunctionExpression {
            kind: ArrowFunctionKind::AsyncGenerator,
            params: FunctionParams {
                params: vec![
                    BindingIdentifier::from("arg1").into(),
                    BindingIdentifier::from("arg2").into(),
                ],
                rest: Default::default(),
                position: None,
            }.into(),
            body: Default::default(),
            position: None,
        }, "async(arg1,arg2)=*>{}");
    }
}

node_enum!(@node_display pub enum ArrowFunctionParams {
    Singular(BindingIdentifier),
    Normal(FunctionParams),
});
impl Default for ArrowFunctionParams {
    fn default() -> ArrowFunctionParams {
        ArrowFunctionParams::Normal(Default::default())
    }
}


node_enum!(@node_display pub enum ArrowFunctionBody {
    Expression(ArrowFunctionExpressionBody),
    // TODO: Do we need an async arrow body for fn return val
    Block(FunctionBody),
});
impl default::Default for ArrowFunctionBody {
    fn default() -> ArrowFunctionBody {
        ArrowFunctionBody::Block(Default::default())
    }
}


node!(pub struct ArrowFunctionExpressionBody {
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for ArrowFunctionExpressionBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.restrict_lookahead(LookaheadRestriction::ConciseBody);
        f.require_precedence(Precedence::Assignment).node(
            &self.expression,
        )?;
        Ok(())
    }
}
impl<T: Into<alias::Expression>> From<T> for ArrowFunctionExpressionBody {
    fn from(v: T) -> ArrowFunctionExpressionBody {
        ArrowFunctionExpressionBody {
            expression: Box::new(v.into()),
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_arrow_function_expression_body {
    use super::*;
    use ast::literal;
    use ast::objects;

    #[test]
    fn it_prints() {
        assert_serialize!(ArrowFunctionExpressionBody {
            expression: literal::Boolean::from(false).into(),
            position: None,
        }, "false");
    }

    #[test]
    fn it_prints_with_parens() {
        assert_serialize!(ArrowFunctionExpressionBody {
            expression: objects::ObjectExpression::default().into(),
            position: None,
        }, "({})");
    }
}
