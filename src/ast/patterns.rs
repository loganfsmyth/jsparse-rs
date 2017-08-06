use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Punctuator, Precedence,
                   LookaheadSequence};

use ast::alias;
use ast::general::{BindingIdentifier, PropertyName};

// TODO: Should we split member expression into an member access and member assign?
use ast::expression::MemberExpression;

node_enum!(@node_display pub enum LeftHandSimpleAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
});


node_enum!(@node_display pub enum LeftHandComplexAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
    Object(ObjectPattern),
    Array(ArrayPattern),
});


node_enum!(@node_display pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
});


// ({     } =
node!(#[derive(Default)] pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
    pub rest: Option<Box<LeftHandComplexAssign>>,
});
// display_dsl!(ClassDecorator: {
//     @[properties,]

//     @?rest[
//         // TODO: Needs a comma here _sometimes_
//         ...@
//     ]
// });


impl NodeDisplay for ObjectPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.lookahead_wrap_parens(LookaheadSequence::Curly);
        let mut f = f.wrap_curly();

        f.comma_list(&self.properties)?;

        if let Some(ref p) = self.rest {
            if !self.properties.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);

            f.node(p)?;
        }

        Ok(())
    }
}


node!(pub struct ObjectPatternIdentifierProperty {
    pub id: BindingIdentifier,
    pub init: Option<alias::Expression>,
});
// display_dsl!(ObjectPatternIdentifierProperty: @id @?init[= @in {}]

impl NodeDisplay for ObjectPatternIdentifierProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}
node!(pub struct ObjectPatternPatternProperty {
    pub name: PropertyName,
    pub pattern: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ObjectPatternPatternProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.name)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.pattern)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}

node_enum!(@node_display pub enum ObjectPatternProperty {
    Identifier(ObjectPatternIdentifierProperty),
    Pattern(ObjectPatternPatternProperty),
});


// ([     ] =
node!(#[derive(Default)] pub struct ArrayPattern {
    pub items: Vec<Option<ArrayPatternElement>>,
    pub rest: Option<Box<Pattern>>,
});
impl NodeDisplay for ArrayPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_square();

        f.comma_list(&self.items)?;

        if let Some(ref p) = self.rest {
            if !self.items.is_empty() {
                f.punctuator(Punctuator::Comma);
            }

            f.punctuator(Punctuator::Ellipsis);

            f.node(p)?;
        }

        Ok(())
    }
}


node!(pub struct ArrayPatternElement {
    pub id: LeftHandComplexAssign,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ArrayPatternElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;

        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}
