use std::string;
use super::alias;
use super::flow;
use super::literal;
use super::display;
use super::expression;

macro_rules! node_position {
	($id:ident) => {
    impl<T> $crate::ast::WithPosition<T> for $id
    where
    	T: Into<Option<Box<$crate::ast::NodePosition>>>
    {
      fn set_position(&mut self, pos: T) {
        self.position = pos.into();
      }
    }
	};
}

macro_rules! nodes {
  () => {};
  ($item:item $($items:item)+) => {
    nodes!($item);
    $(
    	nodes!($items);
    )+
  };
  (pub struct $id:ident { $($field_id:ident : $field_type:ty ,)* }) => {
    pub struct $id {
      $($field_id: $field_type,)*
      position: Option<Box<$crate::ast::NodePosition>>,
    }
    node_position!($id);
  };
  ($item:item) => {
    $item
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
// 	fn orphan_if(&self) -> bool {
// 		(*self).orphan_if()
// 	}
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
}
pub trait FirstSpecialToken {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::None
    }
}

nodes!{
  pub enum Ast {
    Script(Box<Script>),
    Module(Box<Module>),
  }
  impl display::NodeDisplay for Ast {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&Ast::Script(ref s) => f.node(s),
    		&Ast::Module(ref m) => f.node(m),
    	}
    }
  }

  pub struct Script {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }
  impl display::NodeDisplay for Script {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for d in self.directives.iter() {
    		f.node(d)?;
    	}
    	for item in self.body.iter() {
    		f.node(item)?;
    	}

    	Ok(())
    }
  }

  pub struct Module {
    directives: Vec<Directive>,
    body: Vec<alias::ModuleStatementItem>,
  }
  impl display::NodeDisplay for Module {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for d in self.directives.iter() {
    		f.node(d)?;
    	}
    	for item in self.body.iter() {
    		f.node(item)?;
    	}

    	Ok(())
    }
  }

  pub struct Directive {
    value: string::String,
  }
  impl display::NodeDisplay for Directive {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.string(&self.value, Some(&self.value))
    }
  }

  pub enum Pattern {
    Identifier(BindingIdentifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
  }
  impl display::NodeDisplay for Pattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&Pattern::Identifier(ref id) => f.node(id),
    		&Pattern::Object(ref obj) => f.node(obj),
    		&Pattern::Array(ref obj) => f.node(obj),
    	}
    }
  }

  // identifiers used as labels
  pub struct LabelIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl display::NodeDisplay for LabelIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.identifier(&self.value, Some(&self.raw))
    }
  }

  // identifiers used as variables
  pub struct BindingIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl display::NodeDisplay for BindingIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.identifier(&self.value, Some(&self.raw))
    }
  }
  impl HasInOperator for BindingIdentifier {
    fn has_in_operator(&self) -> bool {
    	false
    }
  }
  impl FirstSpecialToken for BindingIdentifier {}

  // identifiers used as properties
  pub struct PropertyIdentifier {
    value: string::String,
    raw: string::String,
  }
  impl display::NodeDisplay for PropertyIdentifier {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.identifier(&self.value, Some(&self.raw))
    }
  }


	pub enum LeftHandSimpleAssign {
		// TODO: Parenthesized ident and member
		Identifier(BindingIdentifier),
		Member(expression::MemberExpression),
	}
  impl display::NodeDisplay for LeftHandSimpleAssign {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&LeftHandSimpleAssign::Identifier(ref s) => f.node(s),
    		&LeftHandSimpleAssign::Member(ref m) => f.node(m),
    	}
    }
  }
	impl FirstSpecialToken for LeftHandSimpleAssign {
	  fn first_special_token(&self) -> SpecialToken {
    	match self {
    		&LeftHandSimpleAssign::Identifier(ref s) => s.first_special_token(),
    		&LeftHandSimpleAssign::Member(ref m) => m.first_special_token(),
    	}
	  }
	}
	pub enum LeftHandComplexAssign {
		// TODO: Parenthesized ident and member
		Identifier(BindingIdentifier),
		Member(expression::MemberExpression),
		Object(ObjectPattern),
		Array(ArrayPattern),
	}
  impl display::NodeDisplay for LeftHandComplexAssign {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&LeftHandComplexAssign::Identifier(ref s) => f.node(s),
    		&LeftHandComplexAssign::Member(ref m) => f.node(m),
    		&LeftHandComplexAssign::Object(ref m) => f.node(m),
    		&LeftHandComplexAssign::Array(ref m) => f.node(m),
    	}
    }
  }
	impl FirstSpecialToken for LeftHandComplexAssign {
	  fn first_special_token(&self) -> SpecialToken {
    	match self {
    		&LeftHandComplexAssign::Identifier(ref s) => s.first_special_token(),
    		&LeftHandComplexAssign::Member(ref m) => m.first_special_token(),
    		&LeftHandComplexAssign::Object(ref m) => m.first_special_token(),
    		&LeftHandComplexAssign::Array(ref m) => m.first_special_token(),
    	}
	  }
	}


  // ({   } =
	pub struct ObjectPattern {
		properties: Vec<ObjectPatternProperty>,
    rest: Option<Box<LeftHandComplexAssign>>,
	}
  impl display::NodeDisplay for ObjectPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::CurlyL)?;
    	for prop in self.properties.iter() {
    		f.node(prop)?;

    		f.token(display::Token::Comma)?;
    	}

    	if let Some(ref p) = self.rest {
    		f.token(display::Token::Ellipsis)?;

    		f.node(p)?;
    	}

    	f.token(display::Token::CurlyR)
    }
  }
  impl FirstSpecialToken for ObjectPattern {
  	fn first_special_token(&self) -> SpecialToken {
  		SpecialToken::Object
  	}
  }
	pub enum ObjectPatternProperty {
		Identifier(BindingIdentifier, Option<alias::Expression>),
		Pattern(PropertyId, LeftHandComplexAssign, Option<alias::Expression>),
	}
  impl display::NodeDisplay for ObjectPatternProperty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ObjectPatternProperty::Identifier(ref id, ref expr) => {
    			f.node(id)?;
    			if let &Some(ref expr) = expr {
    				f.token(display::Token::Eq)?;
    				f.node(expr)?;
    			}

    			Ok(())
    		}
    		&ObjectPatternProperty::Pattern(ref prop, ref pattern, ref expr) => {
    			f.node(prop)?;
    			f.token(display::Token::Colon)?;
    			f.node(pattern)?;
    			if let &Some(ref expr) = expr {
    				f.token(display::Token::Eq)?;
    				f.node(expr)?;
    			}

    			Ok(())
    		}
    	}
    }
  }


  // ([   ] =
	pub struct ArrayPattern {
		items: Vec<Option<ArrayPatternElement>>,
    rest: Option<Box<Pattern>>,
	}
  impl display::NodeDisplay for ArrayPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::SquareL)?;
    	for prop in self.items.iter() {
    		if let &Some(ref prop) = prop {
	    		f.node(prop)?;
	    	}

    		f.token(display::Token::Comma)?;
    	}

    	if let Some(ref p) = self.rest {
    		f.token(display::Token::Ellipsis)?;

    		f.node(p)?;
    	}

    	f.token(display::Token::SquareR)
    }
  }
  impl FirstSpecialToken for ArrayPattern {}

	pub struct ArrayPatternElement {
		id: LeftHandComplexAssign,
		init: Option<alias::Expression>,
	}
  impl display::NodeDisplay for ArrayPatternElement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.id)?;

    	if let Some(ref init) = self.init {
    		f.token(display::Token::Eq)?;
    		f.node(init)?;
    	}

    	Ok(())
    }
  }



  pub struct FunctionBody {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
  }
  impl display::NodeDisplay for FunctionBody {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
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
  pub enum Decorator {
    Property(DecoratorMemberExpression),
    Call(DecoratorCallExpression),

    // Backward-compat for older decorator spec
    Expression(alias::Expression),
  }
  impl display::NodeDisplay for Decorator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::At)?;

    	match self {
    		&Decorator::Property(ref expr) => f.node(expr),
    		&Decorator::Call(ref expr) => f.node(expr),
    		&Decorator::Expression(ref expr) => f.node(expr),
    	}
    }
  }

  // experimental
  pub enum DecoratorMemberExpression {
    Identifier(BindingIdentifier),
    Member(Box<DecoratorMemberExpression>, PropertyIdentifier),
  }
  impl display::NodeDisplay for DecoratorMemberExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&DecoratorMemberExpression::Identifier(ref id) => f.node(id),
    		&DecoratorMemberExpression::Member(ref member, ref id) => {
    			f.node(member)?;
    			f.token(display::Token::Period)?;
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
  impl display::NodeDisplay for DecoratorCallExpression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.callee)?;
    	f.node(&self.arguments)
    }
  }

  pub struct CallArguments {
    args: Vec<Box<alias::Expression>>,
    spread: Option<Box<alias::Expression>>,
  }
  impl display::NodeDisplay for CallArguments {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::ParenL)?;
    	for arg in self.args.iter() {
    		f.node(arg)?;

    		f.token(display::Token::Comma)?;
    	}
    	f.token(display::Token::ParenR)
    }
  }


  pub struct ClassBody {
    items: Vec<ClassItem>,
  }
  impl display::NodeDisplay for ClassBody {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::CurlyL)?;

    	for item in self.items.iter() {
    		f.node(item)?;
    	}

    	f.token(display::Token::CurlyR)
    }
  }
  pub struct ClassEmpty {}
  impl display::NodeDisplay for ClassEmpty {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.token(display::Token::Semicolon)
    }
  }
  pub enum ClassItem {
    Method(ClassMethod),
    Field(ClassField),
    Empty(ClassEmpty),
  }
  impl display::NodeDisplay for ClassItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ClassItem::Method(ref item) => f.node(item),
    		&ClassItem::Field(ref item) => f.node(item),
    		&ClassItem::Empty(ref item) => f.node(item),
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
  impl display::NodeDisplay for PropertyId {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&PropertyId::Literal(ref id) => {
    			f.token(display::Token::Period)?;
    			f.node(id)
    		}
    		&PropertyId::String(ref id) => {
    			f.token(display::Token::Period)?;
    			f.node(id)
    		}
    		&PropertyId::Number(ref id) => {
    			f.token(display::Token::Period)?;
    			f.node(id)
    		}
    		&PropertyId::Computed(ref expr) => {
    			f.token(display::Token::SquareL)?;
    			f.node(expr)?;
    			f.token(display::Token::SquareR)
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
  impl display::NodeDisplay for ClassMethod {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	if let FieldPosition::Static = self.pos {
    		f.token(display::Token::Static)?;
    	}

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
    Public(PropertyId),
    Private(PropertyIdentifier),
  }
  impl display::NodeDisplay for ClassFieldId {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	match self {
    		&ClassFieldId::Public(ref id) => f.node(id),
    		&ClassFieldId::Private(ref id) => f.node(id),
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
  impl display::NodeDisplay for ClassField {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	if let FieldPosition::Static = self.pos {
    		f.token(display::Token::Static)?;
    	}

    	if let Some(ref var) = self.type_variance {
    		f.node(var)?;
    	}
    	f.node(&self.id)?;

    	if let Some(ref val) = self.value {
    		f.token(display::Token::Eq)?;
    		f.node(val)?;
    	}

    	Ok(())
    }
  }

  pub struct FunctionParams {
    params: Vec<FunctionParam>,
    rest: Option<FunctionRestParam>,
  }
  impl display::NodeDisplay for FunctionParams {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for param in self.params.iter() {
    		f.node(param)?;

    		f.token(display::Token::Comma)?;
    	}

    	if let Some(ref param) = self.rest {
    		f.token(display::Token::Ellipsis)?;
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
  impl display::NodeDisplay for FunctionParam {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	for dec in self.decorators.iter() {
    		f.node(dec)?;
    	}

    	f.node(&self.id)?;
    	if self.optional {
    		f.token(display::Token::Question)?;
    	}
    	if let Some(ref anno) = self.type_annotation {
    		f.node(anno)?;
    	}

    	if let Some(ref init) = self.init {
    		f.node(init)?;
    	}
    	Ok(())
    }
  }
  pub struct FunctionRestParam {
    id: Pattern,

    // Flow extensionF
    type_annotation: Option<Box<flow::Annotation>>,
  }
  impl display::NodeDisplay for FunctionRestParam {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
    	f.node(&self.id)?;

    	if let Some(ref anno) = self.type_annotation {
    		f.node(anno)?;
    	}
    	Ok(())
    }
  }
}
