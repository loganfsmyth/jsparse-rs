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

pub trait HasOrphanIf {
	fn orphan_if(&self) -> bool {
		false
	}
}
pub trait HasInOperator {
	fn has_in_operator(&self) -> bool {
		false
	}
}

pub enum SpecialToken {
	None,
	Class,
	Function,
	Curly,
}
pub trait FirstSpecialToken {
	  fn first_special_token(&self) -> SpecialToken {
	  	SpecialToken::None
	  }
}

nodes!{
  pub enum Ast {
    Script(Script),
    Module(Module),
  }
  impl NodeDisplay for TryStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		Ast::Script(ref s) => f.node(s),
    		Ast::Module(ref m) => f.node(m),
    	}
    }
  }

  pub struct Script {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }
  impl NodeDisplay for Script {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for d in self.directives.iter() {
    		f.node(d)?;
    	}
    	for item in self.body.iter() {
    		f.node(item)?;
    	}
    }
  }

  pub struct Module {
    directives: Vec<Directive>,
    body: Vec<alias::ModuleStatementItem>,
  }
  impl NodeDisplay for Module {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for d in self.directives.iter() {
    		f.node(d)?;
    	}
    	for item in self.body.iter() {
    		f.node(item)?;
    	}
    }
  }

  pub struct Directive {
    value: string::String,
  }
  impl NodeDisplay for Directive {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.string(&self.value, &self.value)
    }
  }

  pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
  }
  impl NodeDisplay for Pattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		Pattern::Identifier(ref id) => f.node(id),
    		Pattern::Object(ref obj) => f.node(obj),
    		Pattern::Array(ref obj) => f.node(obj),
    	}
    }
  }

  // identifiers used as labels
  pub struct LabelIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl NodeDisplay for LabelIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.identifier(&self.value, &self.raw)
    }
  }

  // identifiers used as variables
  pub struct BindingIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl NodeDisplay for BindingIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.identifier(&self.value, &self.raw)
    }
  }

  // identifiers used as properties
  pub struct PropertyIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl NodeDisplay for PropertyIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.identifier(&self.value, &self.raw)
    }
  }

  // ({   } =
  pub struct ObjectPattern {
    properties: Vec<ObjectPatternProperty>,

    // TODO: Pattern here is wrong, probably any lefthand should work?
    rest: Option<Box<Pattern>>,
  }
  impl NodeDisplay for ObjectPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::CurlyL)?;
    	for prop in self.properties.iter() {
    		f.node(prop)?;

    		f.token(misc::Token::Comma)?;
    	}

    	if let Some(p) = self.rest {
    		f.token(misc::Token::Ellipsis)?;

    		f.node(p)?;
    	}

    	f.token(misc::Token::CurlyR)?;
    }
  }
  pub struct ObjectPatternProperty {
    // foo (= expr)?
    // prop: foo (= expr)?
    // prop: {a} (= expr)?
    name: Option<PropertyIdentifier>,
    id: Pattern,
    init: Option<alias::Expression>,
  }
  impl NodeDisplay for ObjectPatternProperty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	if let Some(name) = self.name {
    		f.node(name)?;

    		f.token(misc::Token::Colon)?;
    	}
    	f.node(self.id)?;

    	if let Some(init) = self.init {
    		f.token(misc::Token::Eq)?;
    		f.node(init)?;
    	}
    }
  }

  pub struct ArrayPattern {
    elements: Vec<Option<ArrayPatternElement>>,
    rest: Option<Box<Pattern>>,
  }
  impl NodeDisplay for ArrayPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::SquareL)?;
    	for prop in self.properties.iter() {
    		f.node(prop)?;

    		f.token(misc::Token::Comma)?;
    	}

    	if let Some(p) = self.rest {
    		f.token(misc::Token::Ellipsis)?;

    		f.node(p)?;
    	}

    	f.token(misc::Token::SquareR)?;
    }
  }
  pub struct ArrayPatternElement {
    // foo (= expr)?
    // {a} (= expr)?
    id: Pattern,
    init: Option<alias::Expression>,
  }
  impl NodeDisplay for ArrayPatternElement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.node(self.id)?;

    	if let Some(init) = self.init {
    		f.token(misc::Token::Eq)?;
    		f.node(init)?;
    	}
    }
  }



  pub struct FunctionBody {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }
  impl NodeDisplay for FunctionBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for d in self.directives.iter() {
    		f.node(d)?;
    	}

    	for item in self.body.iter() {
    		f.node(item)?;
    	}
    }
  }

  // experimental
  pub enum Decorator {
    Property(DecoratorMemberExpression),
    Call(DecoratorCallExpression),

    // Backward-compat for older decorator spec
    Expression(alias::Expression),
  }
  impl NodeDisplay for Decorator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::At)?;

    	match self {
    		Decorator::Property(ref expr) => f.node(expr)?,
    		Decorator::Call(ref expr) => f.node(expr)?,
    		Decorator::Expression(ref expr) => f.node(expr)?,
    	}
    }
  }

  // experimental
  pub enum DecoratorMemberExpression {
    Identifier(BindingIdentifier),
    Member(Box<DecoratorMemberExpression>, PropertyIdentifier),
  }
  impl NodeDisplay for DecoratorMemberExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		DecoratorMemberExpression::Identifier(ref id) => f.node(id)?,
    		DecoratorMemberExpression::Member(ref member, ref id) => {
    			f.node(member)?;
    			f.token(misc::Token::Period)?;
    			f.node(id)
    		},
    	}
    }
  }
  // experimental
  pub struct DecoratorCallExpression {
    callee: DecoratorMemberExpression,
    arguments: CallArguments,
  }
  impl NodeDisplay for DecoratorCallExpression {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.node(self.callee)?;
    	f.node(self.arguments)
    }
  }

  pub struct CallArguments {
    args: Vec<Box<alias::Expression>>,
    spread: Option<Box<alias::Expression>>,
  }
  impl NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::ParenL)?;
    	for arg in self.args.iter() {
    		f.node(arg)?;

    		f.token(misc::Token::Comma)?;
    	}
    	f.token(misc::Token::ParenR)
    }
  }


  pub struct ClassBody {
    items: Vec<ClassItem>,
  }
  impl NodeDisplay for ClassBody {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::CurlyL)?;

    	for item in self.items.iter() {
    		f.node(item)?;
    	}

    	f.token(misc::Token::CurlyR)
    }
  }
  pub struct ClassEmpty {}
  impl NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.token(misc::Token::Semicolon)
    }
  }
  pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
  }
  impl NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		ClassItem::Method(ref item) => f.node(item),
    		ClassItem::Field(ref item) => f.node(item),
    		ClassItem::Empty(ref item) => f.node(item),
    	}
    }
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
  impl NodeDisplay for PropertyId {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		PropertyId::Literal(ref id) => {
    			f.token(misc::Token::Period)?;
    			f.node(id)
    		}
    		PropertyId::String(ref id) => {
    			f.token(misc::Token::Period)?;
    			f.node(id)
    		}
    		PropertyId::Number(ref id) => {
    			f.token(misc::Token::Period)?;
    			f.node(id)
    		}
    		PropertyId::Computed(ref expr) => {
    			f.token(misc::Token::SquareL)?;
    			f.node(id)?;
    			f.token(misc::Token::SquareR)
    		}
    	}
    }
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
  impl NodeDisplay for ClassMethod {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	if let FieldPosition::Static = self.pos {
    		f.token(misc::Token::Static)?;
    	}

    	f.node(self.id)?;
    	f.node(self.params)?;
    	f.node(self.body)?;

    }
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
  impl NodeDisplay for ClassFieldId {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	match self {
    		ClassFieldId::Public(ref id) => f.node(id),
    		ClassFieldId::Private(ref id) => f.node(id),
    	}
    }
  }

  // experimental
  pub struct ClassField {
    pos: FieldPosition,
    decorators: Vec<Decorator>,

    id: ClassFieldId,
    value: Option<alias::Expression>,

    // Flow extension
    type_variance: Option<flow::Variance>,
  }
  impl NodeDisplay for ClassField {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	if let FieldPosition::Static = self.pos {
    		f.token(misc::Token::Static)?;
    	}

    	if let Some(var) = self.type_variance {
    		f.node(var)?;
    	}
    	f.node(self.id)?;

    	if let Some(val) = self.value {
    		f.token(misc::Token::Eq)?;
    		f.node(val)?;
    	}

    	Ok(())
    }
  }

  pub struct FunctionParams {
    params: Vec<FunctionParam>,
    rest: Option<FunctionRestParam>,
  }
  impl NodeDisplay for FunctionParams {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for param in self.params.iter() {
    		f.node(param)?;

    		f.token(misc::Token::Comma)?;
    	}

    	if let Some(param) = self.rest {
    		f.token(misc::Token::Ellipsis)?;
    		f.node(param)?;
    	}
    	Ok(())
    }
  }
  pub struct FunctionParam {
    decorators: Vec<Decorator>, // experimental
    id: Pattern,
    init: Option<Box<alias::Expression>>,

    // Flow extension
    type_annotation: Option<Box<flow::Annotation>>,
    optional: bool,
  }
  impl NodeDisplay for FunctionParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	f.node(self.id)?;
    	if self.optional {
    		f.token(misc::Token::Question)?;
    	}
    	if let Some(anno) = self.type_annotation {
    		f.node(anno)?;
    	}

    	if let Some(init) = self.init {
    		f.node(init)?;
    	}
    }
  }
  pub struct FunctionRestParam {
    id: Pattern,

    // Flow extensionF
    type_annotation: Option<Box<flow::Annotation>>,
  }
  impl NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
    	f.node(self.id)?;

    	if let Some(anno) = self.type_annotation {
    		f.node(anno)?;
    	}
    	Ok(())
    }
  }
}
