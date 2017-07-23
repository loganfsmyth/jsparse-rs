use std::string;
use super::alias;
use super::flow;
use super::literal;

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

macro_rules! nodes {
  () => {};
  (pub struct $id:ident {
    $(
        $field_id:ident : $field_type: ty
    ),*
    $(,)*
  } $($rest:tt)*) => {
    pub struct $id {
      $(
        $field_id: $field_type ,
      )*
      position: Option<Box<$crate::ast::misc::NodePosition>>,
    }
    node_position!($id);

    nodes!($($rest)*);
  };
  (pub struct $id:ident {
  } $($rest:tt)*) => {
    pub struct $id {
      position: Option<Box<$crate::ast::misc::NodePosition>>,
    }
    node_position!($id);

    nodes!($($rest)*);
  };
  ($item:item $($rest:tt)*) => {
    $item
    nodes!($($rest)*);
  };
  ($item:item, $($items: item),+) => {
    nodes!($item);
    nodes!($($items),+);
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

nodes!{
  pub enum AST {
    Script(Script),
    Module(Module),
  }

  pub struct Script {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }

  pub struct Module {
    directives: Vec<Directive>,
    body: Vec<alias::ModuleStatementItem>,
  }

  pub struct Directive {
    value: string::String,
  }

  pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
  }

  // identifiers used as labels
  pub struct LabelIdentifier {
    value: string::String,
  }

  // identifiers used as variables
  pub struct BindingIdentifier {
    id: string::String,
  }

  // identifiers used as properties
  pub struct PropertyIdentifier {
    id: string::String,
  }

  // ({   } =
  pub struct ObjectPattern {
    properties: Vec<ObjectPatternProperty>,
    rest: Option<Box<Pattern>>,
  }
  pub struct ObjectPatternProperty {
    // foo (= expr)?
    // prop: foo (= expr)?
    // prop: {a} (= expr)?
    name: Option<PropertyIdentifier>,
    id: Pattern,
    init: Option<alias::Expression>,
  }

  pub struct ArrayPattern {
    elements: Vec<Option<ArrayPatternElement>>,
    rest: Option<Box<Pattern>>,
  }
  pub struct ArrayPatternElement {
    // foo (= expr)?
    // {a} (= expr)?
    id: Pattern,
    init: Option<alias::Expression>,
  }



  pub struct FunctionBody {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }

  // experimental
  pub enum Decorator {
    Property(DecoratorMemberExpression),
    Call(DecoratorCallExpression),

    // Backward-compat for older decorator spec
    Expression(alias::Expression),
  }

  // experimental
  pub enum DecoratorMemberExpression {
    Identifier(BindingIdentifier),
    Member(Box<DecoratorMemberExpression>, PropertyIdentifier),
  }
  // experimental
  pub struct DecoratorCallExpression {
    callee: DecoratorMemberExpression,
    arguments: CallArguments,
  }

  pub struct CallArguments {
    args: Vec<Box<alias::Expression>>,
    spread: Option<Box<alias::Expression>>,
  }


  pub struct ClassBody {
    items: Vec<ClassItem>,
  }
  pub struct ClassEmpty {}
  pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
  }

  pub enum FunctionKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator, // experimental
  }

  pub enum MethodKind {
    Normal,
    Get,
    Set,
  }
  pub enum PropertyId {
    Literal(PropertyIdentifier),
    String(literal::String),
    Number(literal::Numeric),
    Computed(Box<alias::Expression>),
  }

  pub struct ClassMethod {
    pos: FieldPosition,
    kind: MethodKind,
    id: ClassFieldId,
    params: FunctionParams,
    body: FunctionBody,
    fn_kind: FunctionKind,
    decorators: Vec<Decorator>,

    return_type: Option<Box<flow::Annotation>>,
  }
  pub enum FieldPosition {
    Instance,
    Static,
  }

  // experimental
  pub enum ClassFieldId {
    Public(PropertyId),
    Private(PropertyIdentifier),
  }

  // experimental
  pub struct ClassField {
    pos: FieldPosition,
    decorators: Vec<Decorator>,

    // This is limited to >= 1 item
    items: Vec<ClassFieldPair>,
  }
  pub struct ClassFieldPair {
    id: ClassFieldId,
    value: alias::Expression,

    // Flow extension
    type_variance: flow::Variance,
  }


  pub struct FunctionParams {
    params: Vec<FunctionParam>,
    rest: Option<FunctionRestParam>,
  }
  pub struct FunctionParam {
    decorators: Vec<Decorator>, // experimental
    id: Pattern,
    init: Option<Box<alias::Expression>>,

    // Flow extension
    type_annotation: Option<Box<flow::Annotation>>,
    optional: bool,
  }
  pub struct FunctionRestParam {
    decorators: Vec<Decorator>, // experimental
    id: Pattern,

    // Flow extensionF
    type_annotation: Option<Box<flow::Annotation>>,
  }
}
