use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasInOperator, FirstSpecialToken, SpecialToken};

use ast::general::{BindingIdentifier};
use ast::alias;
use ast::patterns::Pattern;

use ast::decorators::DecoratorValue;


node!(pub struct Directive {
    value: string::String,
});
impl NodeDisplay for Directive {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.string(&self.value, Some(&self.value))
    }
}


node_kind!(pub enum FunctionKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental
});
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


node!(pub struct FunctionParams {
    params: Vec<FunctionParam>,
    rest: Option<FunctionRestParam>,
});
impl NodeDisplay for FunctionParams {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        f.comma_list(&self.params)?;

        if let Some(ref param) = self.rest {
            if !self.params.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);
            f.node(param)?;
        }
        Ok(())
    }
}


node!(pub struct FunctionParam {
    decorators: Vec<FunctionParamDecorator>, // experimental
    id: Pattern,
    init: Option<Box<alias::Expression>>,
});
impl NodeDisplay for FunctionParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.node(&self.id)?;

        if let Some(ref init) = self.init {
            f.node(init)?;
        }
        Ok(())
    }
}


node!(pub struct FunctionRestParam {
    id: Pattern,
});
impl NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;

        Ok(())
    }
}


node!(pub struct FunctionBody {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
});
impl NodeDisplay for FunctionBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        for d in self.directives.iter() {
            f.node(d)?;
        }

        for item in self.body.iter() {
            f.node(item)?;
        }

        Ok(())
    }
}
impl HasInOperator for FunctionBody {
    fn has_in_operator(&self) -> bool {
        false
    }
}

node!(pub struct FunctionParamDecorator {
    value: DecoratorValue,
});
impl NodeDisplay for FunctionParamDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}


// export default function name() {}
node!(pub struct ExportDefaultFunctionDeclaration {
    kind: FunctionKind,
    id: Option<BindingIdentifier>,
    params: FunctionParams,
    body: FunctionBody,
});
impl NodeDisplay for ExportDefaultFunctionDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.keyword(Keyword::Default);
        f.node(&self.kind)?;

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        f.node(&self.params)?;
        f.node(&self.body)
    }
}

// function name() {}
node!(pub struct FunctionDeclaration {
    kind: FunctionKind,
    id: BindingIdentifier,
    params: FunctionParams,
    body: FunctionBody,
});
impl NodeDisplay for FunctionDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.kind)?;
        f.keyword(Keyword::Function);
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}

// (function(){})
node!(pub struct FunctionExpression {
    kind: FunctionKind,
    id: Option<BindingIdentifier>,
    params: FunctionParams,
    body: FunctionBody,
});
impl NodeDisplay for FunctionExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.kind)?;

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        f.node(&self.params)?;
        f.node(&self.body)
    }
}
impl FirstSpecialToken for FunctionExpression {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::Declaration
    }
}
impl HasInOperator for FunctionExpression {}


// (foo) => bar
node!(pub struct ArrowFunctionExpression {
    // TODO: Needs to handle single-param Ident output as type of params
    kind: ArrowFunctionKind,
    params: FunctionParams,
    body: ArrowFunctionBody,
});
node_kind!(pub enum ArrowFunctionKind {
    Normal,
    Async,
    Generator, // experimental
    AsyncGenerator, // experimental
});
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
impl FirstSpecialToken for ArrowFunctionExpression {}
impl HasInOperator for ArrowFunctionExpression {
    fn has_in_operator(&self) -> bool {
        self.body.has_in_operator()
    }
}


node!(pub struct ArrowFunctionExpressionBody {
    expression: Box<alias::Expression>,
});
impl NodeDisplay for ArrowFunctionExpressionBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        if let SpecialToken::Object = self.expression.first_special_token() {
            f.wrap_parens().node(&self.expression)
        } else {
            f.require_precedence(Precedence::Assignment).node(
                &self.expression,
            )
        }
    }
}
impl HasInOperator for ArrowFunctionExpressionBody {
    fn has_in_operator(&self) -> bool {
        self.expression.has_in_operator()
    }
}


node_enum!(@node_display @has_in_operator pub enum ArrowFunctionBody {
    Expression(ArrowFunctionExpressionBody),
    // TODO: Do we need an async arrow body for fn return val
    Block(FunctionBody),
});
