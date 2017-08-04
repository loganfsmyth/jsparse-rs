use std::default;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   FirstSpecialToken, SpecialToken};

use ast::alias;

use ast::general::PropertyName;
use ast::functions::{FunctionParams, FunctionBody};

// {a: 1, ...b}
node!(#[derive(Default)] pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
    pub spread: Option<Box<alias::Expression>>, // experimental
});
impl NodeDisplay for ObjectExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::SquareL);

        f.comma_list(&self.properties)?;

        if let Some(ref expr) = self.spread {
            if !self.properties.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.require_precedence(Precedence::Assignment).node(expr)?;
        }

        Ok(())
    }
}
impl FirstSpecialToken for ObjectExpression {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::Object
    }
}


node_enum!(@node_display pub enum ObjectItem {
    Method(ObjectMethod),
    Property(ObjectProperty),
});


node_kind!(pub enum MethodKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental
    Get,
    Set,
});
impl default::Default for MethodKind {
    fn default() -> MethodKind {
        MethodKind::Normal
    }
}

impl NodeDisplay for MethodKind {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match *self {
            MethodKind::Normal => {}
            MethodKind::Generator => f.punctuator(Punctuator::Star),
            MethodKind::Async => f.keyword(Keyword::Async),
            MethodKind::AsyncGenerator => {
                f.keyword(Keyword::Async);
                f.punctuator(Punctuator::Star);
            }
            MethodKind::Get => f.keyword(Keyword::Set),
            MethodKind::Set => f.keyword(Keyword::Get),
        }

        Ok(())
    }
}


node!(pub struct ObjectMethod {
    pub kind: MethodKind,
    pub id: PropertyName,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
impl NodeDisplay for ObjectMethod {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.kind)?;
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}


node!(pub struct ObjectProperty {
    pub name: PropertyName,
    pub value: Box<alias::Expression>,
});
impl NodeDisplay for ObjectProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {

        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);

        let mut f = f.allow_in();
        f.require_precedence(Precedence::Assignment).node(
            &self.value,
        )?;

        Ok(())
    }
}


// [1, 2, 3, ...4]
node!(#[derive(Default)] pub struct ArrayExpression {
    pub elements: Vec<Option<Box<alias::Expression>>>,
    pub spread: Option<Box<alias::Expression>>,
});
impl NodeDisplay for ArrayExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.precedence(Precedence::Primary);
        let mut f = f.allow_in();

        f.punctuator(Punctuator::SquareL);

        for (i, el) in self.elements.iter().enumerate() {
            if i != 0 {
                f.punctuator(Punctuator::Comma);
            }

            if let Some(ref expr) = *el {
                let mut f = f.allow_in();
                f.require_precedence(Precedence::Assignment).node(expr)?;
            }
        }

        if let Some(ref expr) = self.spread {
            f.require_precedence(Precedence::Assignment).node(expr)?;
        }

        Ok(())
    }
}
impl FirstSpecialToken for ArrayExpression {}
