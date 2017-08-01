use std::string;

use super::alias;
use super::literal::{String, Numeric};
use super::display;
use super::expression::{MemberExpression};

macro_rules! assert_serialize {
    ($id:ident, { $($key:ident: $val:expr),* $(,)* }, $s:expr) => {
        {
            let o = $id {
                position: None,
                $($key: $val),*
            };

            assert_eq!(format!("{}", o), $crate::std::string::String::from($s));
        }


    };
}

macro_rules! node_position {
    ($id:ident) => {
        impl<T> $crate::ast::misc::WithPosition<T> for $id
        where
            T: Into<Option<Box<$crate::ast::misc::NodePosition>>>
        {
            fn set_position(&mut self, pos: T) {
                self.position = pos.into();
            }
        }
    };
}
macro_rules! node_display {
    ($id:ident) => {
        impl ::std::fmt::Display for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut node_fmt = display::NodeFormatter::new();

                display::NodeDisplay::fmt(self, &mut node_fmt);

                write!(f, "{}", node_fmt.output)
            }
        }
    };
}


macro_rules! nodes {
    (pub struct $id:ident { $($field_id:ident: $field_type:ty ,)* }) => {
        pub struct $id {
            $($field_id: $field_type,)*
            position: Option<Box<$crate::ast::misc::NodePosition>>,
        }
        node_position!($id);
        node_display!($id);
    };
}

pub struct NodePosition {
    start: usize,
    end: usize,
    range: PositionRange,
}
pub struct PositionRange {
    start: (usize, usize),
    end: (usize, usize),
}

pub trait WithPosition<T: Into<Option<Box<NodePosition>>>> {
    fn set_position(&mut self, pos: T);
}

pub trait HasOrphanIf {
    fn orphan_if(&self) -> bool {
        false
    }
}
// impl<T: HasOrphanIf> HasOrphanIf for Box<T> {
//     fn orphan_if(&self) -> bool {
//         (*self).orphan_if()
//     }
// }

pub trait HasInOperator {
    fn has_in_operator(&self) -> bool {
        false
    }
}

pub enum SpecialToken {
    None,
    Declaration,
    Object,

    // TODO: Lookahead needed for :: operator
    // New,
}
pub trait FirstSpecialToken {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::None
    }
}


pub enum Ast {
    Script(Box<Script>),
    Module(Box<Module>),
}
impl display::NodeDisplay for Ast {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Ast::Script(ref n) => f.node(n),
            Ast::Module(ref n) => f.node(n),
        }
    }
}

nodes!(pub struct Script {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for Script {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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

nodes!(pub struct Module {
    directives: Vec<Directive>,
    body: Vec<alias::ModuleStatementItem>,
});
impl display::NodeDisplay for Module {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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

nodes!(pub struct Directive {
    value: string::String,
});
impl display::NodeDisplay for Directive {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.string(&self.value, Some(&self.value))
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum Pattern {
        Identifier(BindingIdentifier),
        Object(ObjectPattern),
        Array(ArrayPattern),
    }
}
impl display::NodeDisplay for Pattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Pattern::Identifier(ref n) => f.node(n),
            Pattern::Object(ref n) => f.node(n),
            Pattern::Array(ref n) => f.node(n),
        }
    }
}

// identifiers used as labels
nodes!(pub struct LabelIdentifier {
    value: string::String,
    raw: string::String,
});
impl display::NodeDisplay for LabelIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.identifier(&self.value, Some(&self.raw))
    }
}

// identifiers used as variables
nodes!(pub struct BindingIdentifier {
    value: string::String,
    raw: Option<string::String>,
});
impl display::NodeDisplay for BindingIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl HasInOperator for BindingIdentifier {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for BindingIdentifier {}

// identifiers used as properties
nodes!(pub struct PropertyIdentifier {
    value: string::String,
    raw: string::String,
});
impl display::NodeDisplay for PropertyIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.identifier(&self.value, Some(&self.raw))
    }
}


pub enum LeftHandSimpleAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
}
impl display::NodeDisplay for LeftHandSimpleAssign {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            LeftHandSimpleAssign::Identifier(ref n) => f.node(n),
            LeftHandSimpleAssign::Member(ref n) => f.node(n),
        }
    }
}
impl FirstSpecialToken for LeftHandSimpleAssign {
    fn first_special_token(&self) -> SpecialToken {
        match *self {
            LeftHandSimpleAssign::Identifier(ref s) => s.first_special_token(),
            LeftHandSimpleAssign::Member(ref m) => m.first_special_token(),
        }
    }
}
pub enum LeftHandComplexAssign {
    // TODO: Parenthesized ident and member?
    Identifier(BindingIdentifier),
    Member(MemberExpression),
    Object(ObjectPattern),
    Array(ArrayPattern),
}
impl display::NodeDisplay for LeftHandComplexAssign {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            LeftHandComplexAssign::Identifier(ref n) => f.node(n),
            LeftHandComplexAssign::Member(ref n) => f.node(n),
            LeftHandComplexAssign::Object(ref n) => f.node(n),
            LeftHandComplexAssign::Array(ref n) => f.node(n),
        }
    }
}
impl FirstSpecialToken for LeftHandComplexAssign {
    fn first_special_token(&self) -> SpecialToken {
        match *self {
            LeftHandComplexAssign::Identifier(ref s) => s.first_special_token(),
            LeftHandComplexAssign::Member(ref m) => m.first_special_token(),
            LeftHandComplexAssign::Object(ref m) => m.first_special_token(),
            LeftHandComplexAssign::Array(ref m) => m.first_special_token(),
        }
    }
}


// ({     } =
nodes!(pub struct ObjectPattern {
    properties: Vec<ObjectPatternProperty>,
    rest: Option<Box<LeftHandComplexAssign>>,
});
impl display::NodeDisplay for ObjectPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;

        f.comma_list(&self.properties)?;

        if let Some(ref p) = self.rest {
            if !self.properties.is_empty() {
                f.punctuator(display::Punctuator::Comma)?;
            }

            f.punctuator(display::Punctuator::Ellipsis)?;

            f.node(p)?;
        }

        f.punctuator(display::Punctuator::CurlyR)
    }
}
impl FirstSpecialToken for ObjectPattern {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::Object
    }
}


nodes!(pub struct ObjectPatternIdentifierProperty {
    id: BindingIdentifier,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for ObjectPatternIdentifierProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(display::Punctuator::Eq)?;
            f.require_precedence(display::Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}
nodes!(pub struct ObjectPatternPatternProperty {
    name: PropertyName,
    pattern: LeftHandComplexAssign,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for ObjectPatternPatternProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.name)?;
        f.punctuator(display::Punctuator::Colon)?;
        f.node(&self.pattern)?;
        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(display::Punctuator::Eq)?;
            f.require_precedence(display::Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}

pub enum ObjectPatternProperty {
    Identifier(ObjectPatternIdentifierProperty),
    Pattern(ObjectPatternPatternProperty),
}
impl display::NodeDisplay for ObjectPatternProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ObjectPatternProperty::Identifier(ref n) => f.node(n),
            ObjectPatternProperty::Pattern(ref n) => f.node(n),
        }
    }
}


// ([     ] =
nodes!(pub struct ArrayPattern {
    items: Vec<Option<ArrayPatternElement>>,
    rest: Option<Box<Pattern>>,
});
impl display::NodeDisplay for ArrayPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::SquareL)?;

        for (i, prop) in self.items.iter().enumerate() {
            if i != 0 {
                f.punctuator(display::Punctuator::Comma)?;
            }

            if let Some(ref prop) = *prop {
                f.node(prop)?;
            }
        }

        if let Some(ref p) = self.rest {
            if !self.items.is_empty() {
                f.punctuator(display::Punctuator::Comma)?;
            }

            f.punctuator(display::Punctuator::Ellipsis)?;

            f.node(p)?;
        }

        f.punctuator(display::Punctuator::SquareR)
    }
}
impl FirstSpecialToken for ArrayPattern {}


nodes!(pub struct ArrayPatternElement {
    id: LeftHandComplexAssign,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for ArrayPatternElement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;

        if let Some(ref init) = self.init {
            let mut f = f.allow_in();

            f.punctuator(display::Punctuator::Eq)?;
            f.require_precedence(display::Precedence::Assignment).node(init)?;
        }

        Ok(())
    }
}


nodes!(pub struct FunctionBody {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for FunctionBody {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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


// experimental
// TODO: Enum fix
pub enum Decorator {
    Property(DecoratorValueExpression),
    Call(DecoratorCallExpression),

    // Backward-compat for older decorator spec
    Expression(alias::Expression),
}
impl display::NodeDisplay for Decorator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::At)?;

        match *self {
            Decorator::Property(ref n) => f.node(n),
            Decorator::Call(ref n) => f.node(n),
            Decorator::Expression(ref expr) => f.require_precedence(display::Precedence::Normal).node(expr),
        }
    }
}

nodes!(pub struct DecoratorMemberAccess {
    object: Box<DecoratorValueExpression>,
    property: PropertyIdentifier,
});
impl display::NodeDisplay for DecoratorMemberAccess {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.object)?;
        f.punctuator(display::Punctuator::Period)?;
        f.node(&self.property)
    }
}

// experimental
pub enum DecoratorValueExpression {
    Identifier(BindingIdentifier),
    Member(DecoratorMemberAccess),
}
impl display::NodeDisplay for DecoratorValueExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            DecoratorValueExpression::Identifier(ref n) => f.node(n),
            DecoratorValueExpression::Member(ref n) => f.node(n),
        }
    }
}
// experimental
nodes!(pub struct DecoratorCallExpression {
    callee: DecoratorValueExpression,
    arguments: CallArguments,
});
impl display::NodeDisplay for DecoratorCallExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.callee)?;
        f.node(&self.arguments)
    }
}

nodes!(pub struct CallArguments {
    args: Vec<Box<alias::Expression>>,
    spread: Option<Box<alias::Expression>>,
});
impl display::NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::ParenL)?;

        f.comma_list(&self.args)?;

        if let Some(ref spread) = self.spread {
            f.punctuator(display::Punctuator::Comma)?;
            f.require_precedence(display::Precedence::Assignment).node(spread)?;
        }

        f.punctuator(display::Punctuator::ParenR)
    }
}


nodes!(pub struct ClassBody {
    items: Vec<ClassItem>,
});
impl display::NodeDisplay for ClassBody {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::CurlyL)?;

        for item in self.items.iter() {
            f.node(item)?;
        }

        f.punctuator(display::Punctuator::CurlyR)
    }
}


nodes!(pub struct ClassEmpty {});
impl display::NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Semicolon)
    }
}


pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
}
impl display::NodeDisplay for ClassItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ClassItem::Method(ref n) => f.node(n),
            ClassItem::Field(ref n) => f.node(n),
            ClassItem::Empty(ref n) => f.node(n),
        }
    }
}

pub enum FunctionKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental

    Get,
    Set,
}


pub enum MethodKind {
    Normal,
    Generator,
    Async,
    Get,
    Set,
}
impl display::NodeDisplay for MethodKind {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match *self {
    		MethodKind::Normal => Ok(()),
    		MethodKind::Generator => f.punctuator(display::Punctuator::Star),
    		MethodKind::Async => f.keyword(display::Keyword::Async),
    		MethodKind::Get => f.keyword(display::Keyword::Set),
    		MethodKind::Set => f.keyword(display::Keyword::Get),
    	}
    }
}


pub enum PropertyName {
    Literal(PropertyIdentifier),
    String(String),
    Number(Numeric),
    Computed(ComputedPropertyName),
}
impl display::NodeDisplay for PropertyName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            PropertyName::Literal(ref n) => f.node(n),
            PropertyName::String(ref n) => f.node(n),
            PropertyName::Number(ref n) => f.node(n),
            PropertyName::Computed(ref n) => f.node(n),
        }
    }
}
nodes!(pub struct ComputedPropertyName {
    expression: Box<alias::Expression>,
});
impl display::NodeDisplay for ComputedPropertyName {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        f.punctuator(display::Punctuator::SquareL)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
        f.punctuator(display::Punctuator::SquareR)
    }
}


pub enum PropertyAccess {
    Identifier(IdentifierPropertyAccess),
    Computed(ComputedPropertyAccess),
}
impl display::NodeDisplay for PropertyAccess {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            PropertyAccess::Identifier(ref n) => f.node(n),
            PropertyAccess::Computed(ref n) => f.node(n),
        }
    }
}

nodes!(pub struct ComputedPropertyAccess {
    expression: Box<alias::Expression>,
});
impl display::NodeDisplay for ComputedPropertyAccess {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::SquareL)?;
        f.require_precedence(display::Precedence::Assignment).node(&self.expression)?;
        f.punctuator(display::Punctuator::SquareR)
    }
}
nodes!(pub struct IdentifierPropertyAccess {
    id: PropertyIdentifier,
});
impl display::NodeDisplay for IdentifierPropertyAccess {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Period)?;
        f.node(&self.id)
    }
}


nodes!(pub struct ClassMethod {
    pos: FieldPosition,
    kind: MethodKind,
    id: ClassFieldId,
    params: FunctionParams,
    body: FunctionBody,
    decorators: Vec<Decorator>,
});
impl display::NodeDisplay for ClassMethod {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        if let FieldPosition::Static = self.pos {
            f.keyword(display::Keyword::Static)?;
        }

        f.node(&self.kind)?;

        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)?;

        Ok(())
    }
}
pub enum FieldPosition {
    Instance,
    Static,
}

// experimental
pub enum ClassFieldId {
    Public(PropertyName),
    Private(PropertyIdentifier),
}
impl display::NodeDisplay for ClassFieldId {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ClassFieldId::Public(ref n) => f.node(n),
            ClassFieldId::Private(ref n) => f.node(n),
        }
    }
}

// experimental
nodes!(pub struct ClassField {
    pos: FieldPosition,
    decorators: Vec<Decorator>,

    id: ClassFieldId,
    value: Option<alias::Expression>,
});
impl display::NodeDisplay for ClassField {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        if let FieldPosition::Static = self.pos {
            f.keyword(display::Keyword::Static)?;
        }

        f.node(&self.id)?;

        if let Some(ref val) = self.value {
            f.punctuator(display::Punctuator::Eq)?;
            f.require_precedence(display::Precedence::Assignment).node(val)?;
        }

        Ok(())
    }
}

nodes!(pub struct FunctionParams {
    params: Vec<FunctionParam>,
    rest: Option<FunctionRestParam>,
});
impl display::NodeDisplay for FunctionParams {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        f.comma_list(&self.params)?;

        if let Some(ref param) = self.rest {
            if !self.params.is_empty() {
                f.punctuator(display::Punctuator::Comma)?;
            }

            f.punctuator(display::Punctuator::Ellipsis)?;
            f.node(param)?;
        }
        Ok(())
    }
}
nodes!(pub struct FunctionParam {
    decorators: Vec<Decorator>, // experimental
    id: Pattern,
    init: Option<Box<alias::Expression>>,
});
impl display::NodeDisplay for FunctionParam {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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


nodes!(pub struct FunctionRestParam {
    id: Pattern,
});
impl display::NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;

        Ok(())
    }
}
