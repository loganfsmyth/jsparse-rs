use std::string;
use std::default;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadRestriction, LookaheadSequence};

use ast::general::BindingIdentifier;
use ast::alias;
use ast::patterns::Pattern;

use ast::decorators::DecoratorValue;


node!(pub struct Directive {
    pub value: string::String,
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
    pub decorators: Vec<FunctionParamDecorator>, // experimental
    pub id: Pattern,
    pub init: Option<Box<alias::Expression>>,
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
    pub id: Pattern,
});
impl NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
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

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        f.node(&self.params)?;
        f.node(&self.body)
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
        f.keyword(Keyword::Function);
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
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


// (foo) => bar
node!(#[derive(Default)] pub struct ArrowFunctionExpression {
    // TODO: Needs to handle single-param Ident output as type of params
    pub kind: ArrowFunctionKind,
    pub params: FunctionParams,
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
