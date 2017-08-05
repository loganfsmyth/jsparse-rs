use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   FirstSpecialToken, SpecialToken};

// TODO: Should we have a MethodBody?
use ast::functions::{FunctionParams, FunctionBody};
use ast::decorators::DecoratorValue;
use ast::objects::MethodKind;
use ast::general::{BindingIdentifier, PropertyIdentifier, PropertyName};

use ast::alias;

// export default class name {}
node!(#[derive(Default)] pub struct ExportDefaultClassDeclaration {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub id: Option<BindingIdentifier>,
    pub extends: Option<Box<alias::Expression>>,
    pub body: ClassBody,
});

// display_dsl!(ExportDefaultClassDeclaration:
//   export default @[decorators] class @?id @?extends[extends @] @body);

impl NodeDisplay for ExportDefaultClassDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Export);
        f.keyword(Keyword::Default);

        for dec in self.decorators.iter() {
            f.node(dec)?;
        }
        f.keyword(Keyword::Class);
        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        if let Some(ref extends) = self.extends {
            f.keyword(Keyword::Extends);
            f.require_precedence(Precedence::LeftHand).node(extends)?;
        }
        f.node(&self.body)
    }
}


// class name {}
node!(pub struct ClassDeclaration {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub id: BindingIdentifier,
    pub extends: Option<Box<alias::Expression>>,
    pub body: ClassBody,
});
// display_dsl!(ClassDeclaration:
//   export default @[decorators] class @id @?extends[extends @] @body);

impl NodeDisplay for ClassDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.keyword(Keyword::Class);

        f.node(&self.id)?;

        if let Some(ref expr) = self.extends {
            f.keyword(Keyword::Extends);
            f.require_precedence(Precedence::LeftHand).node(expr)?;
        }

        f.node(&self.body)
    }
}


// (class {})
node!(#[derive(Default)] pub struct ClassExpression {
    pub decorators: Vec<ClassDecorator>, // experimental
    pub id: Option<BindingIdentifier>,
    pub extends: Option<Box<alias::Expression>>,
    pub body: ClassBody,
});
// display_dsl!(ClassExpression: @[decorators] class @?id @?extends[extends @] @body);

impl NodeDisplay for ClassExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.keyword(Keyword::Class);

        if let Some(ref id) = self.id {
            f.node(id)?;
        }
        if let Some(ref expr) = self.extends {
            f.keyword(Keyword::Extends);
            f.require_precedence(Precedence::LeftHand).node(expr)?;
        }

        f.node(&self.body)
    }
}
impl FirstSpecialToken for ClassExpression {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::Declaration
    }
}


node!(#[derive(Default)] pub struct ClassBody {
    pub items: Vec<ClassItem>,
});
// display_dsl!(ClassBody: { @[items] });
impl NodeDisplay for ClassBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        for item in self.items.iter() {
            f.node(item)?;
        }

        Ok(())
    }
}

node!(#[derive(Default)] pub struct ClassEmpty {});
// display_dsl!(ClassEmpty: ;);
impl NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}


// TODO: Should the class constructor be it's own item type to make "super()" checks easier?
node_enum!(@node_display pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
});

// experimental
node_enum!(@node_display pub enum ClassFieldId {
    Public(PropertyName),
    Private(PropertyIdentifier),
});

// experimental
node!(pub struct ClassField {
    pub pos: FieldPosition,
    pub decorators: Vec<ClassItemDecorator>,

    pub id: ClassFieldId,
    pub value: Option<alias::Expression>,
});
// display_dsl!(ClassField: @[decorators] @pos @id @?value[= @]);

impl NodeDisplay for ClassField {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.node(&self.pos)?;

        f.node(&self.id)?;

        if let Some(ref val) = self.value {
            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(val)?;
        }

        Ok(())
    }
}

node!(pub struct ClassMethod {
    pub decorators: Vec<ClassItemDecorator>,
    pub pos: FieldPosition,
    pub kind: MethodKind,
    pub id: ClassFieldId,
    pub params: FunctionParams,
    pub body: FunctionBody,
});
// display_dsl!(ClassMethod: @[decorators] @pos @kind @id @params @body);

impl NodeDisplay for ClassMethod {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.node(&self.pos)?;
        f.node(&self.kind)?;

        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)?;

        Ok(())
    }
}
node_kind!(pub enum FieldPosition {
    Instance,
    Static,
});
impl NodeDisplay for FieldPosition {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        if let FieldPosition::Static = *self {
            f.keyword(Keyword::Static);
        }
        Ok(())
    }
}

node!(pub struct ClassDecorator {
    pub value: DecoratorValue,
});
// display_dsl!(ClassDecorator: @@ @value);
impl NodeDisplay for ClassDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}

node!(pub struct ClassItemDecorator {
    pub value: DecoratorValue,
});
// display_dsl!(ClassItemDecorator: @@ @value);
impl NodeDisplay for ClassItemDecorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::At);
        f.node(&self.value)
    }
}
