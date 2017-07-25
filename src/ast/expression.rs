use std::string;
use std::fmt;

use super::misc;
use super::alias;
use super::flow;
use super::literal;
use super::statement;

nodes!{
	// this
	pub struct ThisExpression {}
	impl misc::NodeDisplay for ThisExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::This)
		}
	}
	impl misc::FirstSpecialToken for ThisExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken:: None }
	}
	impl misc::HasInOperator for ThisExpression {}

	struct ParenthesizedExpression {
		expr: Box<alias::Expression>,
	}
	impl misc::NodeDisplay for ParenthesizedExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.with_parens(|&mut f| f.node(self.expr))
		}
	}
	impl misc::FirstSpecialToken for ParenthesizedExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// [1, 2, 3, ...4]
	pub struct ArrayExpression {
		elements: Vec<Option<Box<alias::Expression>>>,
		spread: Option<Box<alias::Expression>>,
	}
	impl misc::NodeDisplay for ArrayExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::SquareL)?;

			for el in self.elements.iter() {
				if let Some(expr) = el {
					f.with_precedence(Precedence::Assignment, |&mut f| f.node(expr))?;
				}
				f.token(misc::Token::Comma)?;
			}

			if let Some(expr) = self.spread {
				f.with_precedence(Precedence::Assignment, |&mut f| f.node(expr))?;
			}

			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ArrayExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// {a: 1, ...b}
	pub struct ObjectExpression {
		properties: Vec<ObjectProperty>,
		spread: Option<Box<alias::Expression>>, // experimental
	}
	impl misc::NodeDisplay for ObjectExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::SquareL)?;

			for prop in self.properties.iter() {
				if let Some(p) = prop {
					f.node(p)?;
				}
				f.token(misc::Token::Comma)?;
			}

			if let Some(expr) = self.spread {
				f.with_precedence(Precedence::Assignment, |&mut f| f.node(expr))?;
			}

			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ObjectExpression {
		fn first_special_token(&self) -> misc::SpecialToken {
			misc::SpecialToken::Curly
		}
	}
	impl misc::HasInOperator for ThisExpression {}

	pub enum ObjectProperty {
		Method(ObjectMethod),
		Value(misc::PropertyId, Box<alias::Expression>),
	}
	impl misc::NodeDisplay for ObjectProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				ObjectProperty::Method(ref method) => f.node(method),
				ObjectProperty::Value(ref id, ref expr) => {
					f.node(id)?;
					f.token(misc::Token::Colon)?;
					f.with_precedence(Precedence::Assignment, |&mut f| f.node(expr))
				}
			}
		}
	}
	pub struct ObjectMethod {
		kind: misc::MethodKind,
		id: misc::PropertyId,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		return_type: Option<Box<flow::Annotation>>,
	}
	impl misc::NodeDisplay for ObjectMethod {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.kind {
				misc::MethodKind::Normal => {}
				misc::MethodKind::Get => {
					f.token(misc::Token::Get);
					f.space()
				}
				misc::MethodKind::Set => {
					f.token(misc::Token::Set);
					f.space()
				}
			}

			// TODO:
			// f.node(self.fn_kind)?;

			f.node(self.id)?;
			f.node(self.params)?;
			f.node(self.return_type)?;
			f.node(self.body)
		}
	}

	// (function(){})
	pub struct FunctionExpression {
		decorators: Vec<misc::Decorator>, // experimental
		id: Option<misc::BindingIdentifier>,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	impl misc::NodeDisplay for FunctionExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Function)?;
			f.space();

			if let Some(id) = self.id {
				f.node(id)?;
			}
			f.node(self.type_parameters)?;
			f.node(self.params)?;
			f.node(self.return_type)?;
			f.node(self.body)
		}
	}
	impl misc::FirstSpecialToken for FunctionExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Function }
	}
	impl misc::HasInOperator for ThisExpression {}

	// (class {})
	pub struct ClassExpression {
		decorators: Vec<misc::Decorator>, // experimental
		id: Option<misc::BindingIdentifier>,
		extends: Option<Box<alias::Expression>>,
		implements: Option<flow::BindingIdentifierAnnotationList>,
		body: misc::ClassBody,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
	}
	impl misc::NodeDisplay for ClassExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			for dec in self.decorators.iter() {
				f.node(dec)?;
			}

			f.token(misc::Token::Class)?;
			f.space();

			if let Some(id) = self.id {
				f.node(id)?;
				// f.node(self.type_parameters)?;
			}
			if let Some(expr) = self.extends {
				f.token(misc::Token::Extends);
				f.with_precedence(Precedence::LeftHand, |&mut f| f.node(expr))?;
			}
			if let Some(anno) = self.implements {
				f.token(misc::Token::Implements);
				f.node(anno)?;
			}

			f.node(self.body)
		}
	}
	impl misc::FirstSpecialToken for ClassExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Class }
	}
	impl misc::HasInOperator for ThisExpression {}

	// /foo/g
	pub struct RegularExpressionLiteral {
		value: string::String,
		flags: Vec<char>,
	}
	impl misc::NodeDisplay for RegularExpressionLiteral {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.regexp(&self.value, &self.flags)
		}
	}
	impl misc::FirstSpecialToken for RegularExpressionLiteral {}
	impl misc::HasInOperator for ThisExpression {}

	// fn`content`
	pub struct TaggedTemplateLiteral {
		tag: Box<alias::Expression>,
		template: TemplateLiteral,
	}
	impl misc::NodeDisplay for TaggedTemplateLiteral {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.with_precedence(Precedence::Member, |&mut f| f.node(self.tag))?;
			f.node(self.template)
		}
	}
	impl misc::FirstSpecialToken for TaggedTemplateLiteral {
	  fn first_special_token(&self) -> misc::SpecialToken { self.tag.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {}

	// `content`
	pub enum TemplateLiteral {
		Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteral>),
		End(TemplatePart),
	}
	impl misc::NodeDisplay for TemplateLiteral {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			// TODO: Handle initial backtick here
			match self {
				TemplateLiteral::Piece(ref part, ref expr, ref next_literal) => {
					f.node(part)?;
					f.token(misc::Token::TemplateClose)?;
					f.with_precedence(Precedence::Normal, |&mut f| f.node(expr))?;
					f.token(misc::Token::TemplateOpen)?;
					f.node(next_literal)
				}
				TemplateLiteral::End(ref part) => {
					f.node(part)?;
					f.token(misc::Token::TemplateTick)
				}
			}
		}
	}
	impl misc::FirstSpecialToken for TemplateLiteral {}
	impl misc::HasInOperator for ThisExpression {}


	pub struct TemplatePart {
		value: string::String,
		rawValue: string::String,
	}
	impl misc::NodeDisplay for TemplatePart {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.template_part(&self.value, &self.rawValue)
		}
	}

	// foo()
	pub struct CallExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
		optional: bool,
	}
	impl misc::NodeDisplay for CallExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.with_precedence(Precedence::New, |&mut f| f.node(&self.callee))?;
			if self.optional {
				f.token(misc::Token::Question)?;
			}
			f.node(self.arguments)
		}
	}
	impl misc::FirstSpecialToken for CallExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.callee.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {}

	// new foo()
	pub struct NewExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
	}
	impl misc::NodeDisplay for NewExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::New)?;
			f.with_precedence(Precedence::New, |&mut f| f.node(&self.callee))?;
			f.node(self.arguments)
		}
	}
	impl misc::FirstSpecialToken for NewExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// experimental
	// import(foo)
	pub struct ImportExpression {
		argument: Box<alias::Expression>,
	}
	impl misc::NodeDisplay for ImportExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Import)?;
			f.token(misc::Token::ParenL)?;
			f.with_precedence(Precedence::Assignment, |&mut f| f.node(&self.argument))?;
			f.token(misc::Token::ParenR)?;
			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ImportExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// foo.bar
	// foo?.bar
	// foo.#bar
	// foo?.#bar
	pub struct MemberExpression {
		object: Box<alias::Expression>,
		property: MemberProperty,
		optional: bool,
	}
	impl misc::NodeDisplay for MemberExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.with_precedence(Precedence::Member, |&mut f| f.node(&self.object))?;
			if self.optional {
				f.token(misc::Token::Question)?;
			}
			f.token(misc::Token::Period)?;
			f.node(self.property)?;
			Ok(())
		}
	}
	impl misc::FirstSpecialToken for MemberExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.object.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {}

	pub enum MemberProperty {
		Normal(misc::PropertyId),
		Private(PrivateProperty),
	}
	impl misc::NodeDisplay for MemberProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				MemberProperty::Normal(ref id) => f.node(id),
				MemberProperty::Private(ref prop) => f.node(prop),
			}
		}
	}
	pub struct PrivateProperty {
		property: misc::PropertyIdentifier,
	}
	impl misc::NodeDisplay for PrivateProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Hash)?;
			f.node(self.property)
		}
	}

	// #bar
	pub struct PrivateExpression {
		property: misc::PropertyIdentifier,
	}
	impl misc::NodeDisplay for PrivateExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Hash)?;
			f.node(self.property)
		}
	}
	impl misc::FirstSpecialToken for PrivateExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// i++
	// i--
	// ++i
	// --i
	pub struct UpdateExpression {
		value: LeftHandExpression,
		operator: UpdateOperator,
	}
	pub enum UpdateOperator {
		PreIncrement,
		PreDecrement,
		PostIncrement,
		PostDecrement,
	}
	impl misc::NodeDisplay for UpdateExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.operator {
				UpdateOperator::PreIncrement => {
					f.token(misc::Token::PlusPlus)?;
					misc::NodeDisplay::fmt(self.value)
					f.with_precedence(Precedence::Unary, |&mut f| f.node(&self.value))?;
				}
				UpdateOperator::PreDecrement => {
					f.token(misc::Token::MinusMinus)?;
					f.with_precedence(Precedence::Unary, |&mut f| f.node(&self.value))?;
				}
				UpdateOperator::PostIncrement => {
					f.with_precedence(Precedence::LeftHand, |&mut f| f.node(&self.value))?;
					f.token(misc::Token::PlusPlus)
				}
				UpdateOperator::PostDecrement => {
					f.with_precedence(Precedence::LeftHand, |&mut f| f.node(&self.value))?;
					f.token(misc::Token::MinusMinus)
				}
			}
		}
	}
	impl misc::FirstSpecialToken for UpdateExpression {
	  fn first_special_token(&self) -> misc::SpecialToken {
			match self.operator {
				UpdateOperator::PreIncrement => misc::SpecialToken::None,
				UpdateOperator::PreDecrement => misc::SpecialToken::None,
				UpdateOperator::PostIncrement => self.value.first_special_token(),
				UpdateOperator::PostDecrement => self.value.first_special_token(),
			}
	  }
	}
	impl misc::HasInOperator for ThisExpression {}


	// void foo
	pub struct UnaryExpression {
		value: Box<alias::Expression>,
		operator: UnaryOperator,
	}
	pub enum UnaryOperator {
		Delete,
		Void,
		Typeof,
		Positive,
		Negative,
		BitNegate,
		Negate,
		Await,
		Yield,
		YieldDelegate,
		Bind,
	}
	impl misc::NodeDisplay for UnaryOperator {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.operator {
				UnaryOperator::Delete => f.token(misc::Token::Delete),
				UnaryOperator::Void => f.token(misc::Token::Void),
				UnaryOperator::Typeof => f.token(misc::Token::Typeof),
				UnaryOperator::Positive => f.token(misc::Token::Plus),
				UnaryOperator::Negative => f.token(misc::Token::Minus),
				UnaryOperator::BitNegate => f.token(misc::Token::Tilde),
				UnaryOperator::Negate => f.token(misc::Token::Exclam),
				UnaryOperator::Await => f.token(misc::Token::Await),
				UnaryOperator::Yield => f.token(misc::Token::Yield),
				UnaryOperator::YieldDelegate => {
					f.token(misc::Token::Delete)?;
					f.token(misc::Token::Star)
				}
				UnaryOperator::Bind => f.token(misc::Token::Bind),
			}

			f.node_with_precedence(Precedence::Unary, &self.value)
		}
	}
	impl misc::FirstSpecialToken for UnaryExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// foo OP bar
	pub struct BinaryExpression {
		left: Box<alias::Expression>,
		operator: BinaryOperator,
		right: Box<alias::Expression>,
	}
	pub enum BinaryOperator {
		Add,
		Subtract,
		LeftShift,
		RightShift,
		RightShiftSigned,
		Divide,
		Multiply,
		Modulus,
		BitAnd,
		BitOr,
		BitXor,
		Power,

		Compare,
		StrictCompare,
		NegateCompare,
		NegateStrictCompare,
		LessThan,
		LessThanEq,
		GreaterThan,
		GreaterThanEq,
		In,
		Instanceof,

		And,
		Or,

		Bind, // experimental
	}

	// 4 + 5 * (3 + 2)
	impl misc::NodeDisplay for BinaryExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.operator {
				BinaryOperator::Add => {
					f.node_with_precedence(Precedence::Add, self.left)?;
					f.token(misc::Token::Plus)?;
					f.node_with_precedence(Precedence::Multiplicative, self.right)?;
				}
				BinaryOperator::Subtract => {
					f.node_with_precedence(Precedence::Add, self.left)?;
					f.token(misc::Token::Minus)?;
					f.node_with_precedence(Precedence::Multiplicative, self.right)?;
				}
				BinaryOperator::LeftShift => {
					f.node_with_precedence(Precedence::Shift, self.left)?;
					f.token(misc::Token::LAngleAngle)?;
					f.node_with_precedence(Precedence::Additive, self.right)?;
				}
				BinaryOperator::RightShift => {
					f.node_with_precedence(Precedence::Shift, self.left)?;
					f.token(misc::Token::RAngleAngle)?;
					f.node_with_precedence(Precedence::Additive, self.right)?;
				}
				BinaryOperator::RightShiftSigned => {
					f.node_with_precedence(Precedence::Shift, self.left)?;
					f.token(misc::Token::RAngleAngleAngle)?;
					f.node_with_precedence(Precedence::Additive, self.right)?;
				}
				BinaryOperator::Divide => {
					f.node_with_precedence(Precedence::Multiplicative, self.left)?;
					f.token(misc::Token::Slash)?;
					f.node_with_precedence(Precedence::Exponential, self.right)?;
				}
				BinaryOperator::Multiply => {
					f.node_with_precedence(Precedence::Multiplicative, self.left)?;
					f.token(misc::Token::Star)?;
					f.node_with_precedence(Precedence::Exponential, self.right)?;
				}
				BinaryOperator::Modulus => {
					f.node_with_precedence(Precedence::Multiplicative, self.left)?;
					f.token(misc::Token::Mod)?;
					f.node_with_precedence(Precedence::Exponential, self.right)?;
				}
				BinaryOperator::BitAnd => {
					f.node_with_precedence(Precedence::BitwiseAnd, self.left)?;
					f.token(misc::Token::Amp)?;
					f.node_with_precedence(Precedence::Equality, self.right)?;
				}
				BinaryOperator::BitOr => {
					f.node_with_precedence(Precedence::BitwiseOr, self.left)?;
					f.token(misc::Token::Bar)?;
					f.node_with_precedence(Precedence::BitwiseXor, self.right)?;
				}
				BinaryOperator::BitXor => {
					f.node_with_precedence(Precedence::BitwiseXor, self.left)?;
					f.token(misc::Token::Caret)?;
					f.node_with_precedence(Precedence::BitwiseAnd, self.right)?;
				}
				BinaryOperator::Power => {
					f.node_with_precedence(Precedence::Update, self.left)?;
					f.token(misc::Token::StarStar)?;
					f.node_with_precedence(Precedence::Exponential, self.right)?;
				}
				BinaryOperator::Compare => {
					f.node_with_precedence(Precedence::Equality, self.left)?;
					f.token(misc::Token::EqEq)?;
					f.node_with_precedence(Precedence::Relational, self.right)?;
				}
				BinaryOperator::StrictCompare => {
					f.node_with_precedence(Precedence::Equality, self.left)?;
					f.token(misc::Token::EqEqEq)?;
					f.node_with_precedence(Precedence::Relational, self.right)?;
				}
				BinaryOperator::NegateCompare => {
					f.node_with_precedence(Precedence::Equality, self.left)?;
					f.token(misc::Token::Neq)?;
					f.node_with_precedence(Precedence::Relational, self.right)?;
				}
				BinaryOperator::NegateStrictCompare => {
					f.node_with_precedence(Precedence::Equality, self.left)?;
					f.token(misc::Token::NeqEq)?;
					f.node_with_precedence(Precedence::Relational, self.right)?;
				}
				BinaryOperator::LessThan => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::LAngle)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::LessThanEq => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::LAngleEq)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::GreaterThan => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::RAngle)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::GreaterThanEq => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::RAngleEq)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::In => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::In)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::Instanceof => {
					f.node_with_precedence(Precedence::Relational, self.left)?;
					f.token(misc::Token::Instanceof)?;
					f.node_with_precedence(Precedence::Shift, self.right)?;
				}
				BinaryOperator::And => {
					f.node_with_precedence(Precedence::LogicalAnd, self.left)?;
					f.token(misc::Token::AmpAmp)?;
					f.node_with_precedence(Precedence::BitwiseOr, self.right)?;
				}
				BinaryOperator::Or => {
					f.node_with_precedence(Precedence::LogicalOr, self.left)?;
					f.token(misc::Token::BarBar)?;
					f.node_with_precedence(Precedence::LogicalAnd, self.right)?;
				}
				BinaryOperator::Bind => {
					f.node(self.left)?;
					f.token(misc::Token::ColonColon)?;
					f.node(self.right)?;
				}
			}
		}
	}
	impl misc::FirstSpecialToken for BinaryExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {
		fn has_in_operator(&self) -> bool {
			match self.operator {
				BinaryOperator::Add => false,
				BinaryOperator::Subtract => false,
				BinaryOperator::LeftShift => false,
				BinaryOperator::RightShift => false,
				BinaryOperator::RightShiftSigned => false,
				BinaryOperator::Divide => false,
				BinaryOperator::Multiply => false,
				BinaryOperator::Modulus => false,
				BinaryOperator::BitAnd => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::BitOr => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::BitXor => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::Power => false
				BinaryOperator::Compare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::StrictCompare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::NegateCompare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::NegateStrictCompare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::LessThan => self.left.has_in_operator(),
				BinaryOperator::LessThanEq => self.left.has_in_operator(),
				BinaryOperator::GreaterThan => self.left.has_in_operator(),
				BinaryOperator::GreaterThanEq => self.left.has_in_operator(),
				BinaryOperator::In => true,
				BinaryOperator::Instanceof => self.left.has_in_operator(),
				BinaryOperator::And => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::Or => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::Bind => false,
			}
		}
	}

	// foo ? bar : baz
	pub struct ConditionalExpression {
		test: Box<alias::Expression>,
		consequent: Box<alias::Expression>,
		alternate: Box<alias::Expression>,
	}
	impl misc::NodeDisplay for ConditionalExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node_with_precedence(Precedence::LogicalOr, self.test)?;
			f.token(misc::Token::Question)?;
			f.node_with_precedence(Precedence::Assignment, self.consequent)?;
			f.token(misc::Token::Colon)?;
			f.node_with_precedence(Precedence::Assignment, self.alternate)
		}
	}
	impl misc::FirstSpecialToken for ConditionalExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.test.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {
		fn has_in_operator(&self) -> bool {
			self.test.has_in_operator()
		}
	}

	// foo OP= bar
	pub struct AssignmentExpression {
		left: Box<LeftHandExpression>,
		operator: AssignmentOperator,
		right: Box<alias::Expression>,
	}
	pub enum AssignmentOperator {
		None,

		Add,
		Subtract,
		LeftShift,
		RightShift,
		RightShiftSigned,
		Divide,
		Multiply,
		Modulus,
		BitAnd,
		BitOr,
		BitXor,
		Power,
	}
	impl misc::NodeDisplay for AssignmentExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node_with_precedence(Precedence::LeftHand, self.left)?;
			match self.operator {
				AssignmentOperator::None => {},
				AssignmentOperator::Add => f.token(misc::Token::Plus),
				AssignmentOperator::Subtract => f.token(misc::Token::Subtract),
				AssignmentOperator::LeftShift => f.token(misc::Token::LAngleAngle),
				AssignmentOperator::RightShift => f.token(misc::Token::RAngleAngle),
				AssignmentOperator::RightShiftSigned => f.token(misc::Token::RAngleAngleAngle),
				AssignmentOperator::Divide => f.token(misc::Token::Slash),
				AssignmentOperator::Multiply => f.token(misc::Token::Star),
				AssignmentOperator::Modulus => f.token(misc::Token::Mod),
				AssignmentOperator::BitAnd => f.token(misc::Token::Amp),
				AssignmentOperator::BitOr => f.token(misc::Token::Bar),
				AssignmentOperator::BitXor => f.token(misc::Token::Caret),
				AssignmentOperator::Power => f.token(misc::Token::StarStar),
			}
			f.token(misc::Token::Eq)?;
			f.node_with_precedence(Precedence::Assignment, self.right)
		}
	}
	impl misc::FirstSpecialToken for AssignmentExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {
		fn has_in_operator(&self) -> bool {
			self.right.has_in_operator()
		}
	}

	pub enum LeftHandExpression {
		Pattern(misc::Pattern),
		Member(MemberExpression),
		SuperProperty(SuperMemberExpression),

		// "yield" is disallowed in strict mode
		// "await" is disallowed in module
		BindingIdentifier(misc::BindingIdentifier), // May not be "eval" or "arguments" in strict
	}
	impl misc::NodeDisplay for LeftHandExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				LeftHandExpression::Pattern(ref pat) => f.node(pat),
				LeftHandExpression::Member(ref pat) => f.node(pat),
				LeftHandExpression::SuperProperty(ref pat) => f.node(pat),
				LeftHandExpression::BindingIdentifier(ref pat) => f.node(pat),
			}
		}
	}
	impl misc::FirstSpecialToken for LeftHandExpression {
		// TODO
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::None }
	}
	impl misc::HasInOperator for ThisExpression {}

	// foo, bar
	pub struct SequenceExpression {
		left: Box<alias::Expression>,
		right: Box<alias::Expression>,
	}
	impl misc::NodeDisplay for SequenceExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.node_with_precedence(Precedence::Normal, self.left)?;
			f.token(misc::Token::Comma)?;
			// TODO: This precedence may not be strictly needed?
			f.node_with_precedence(Precedence::Assignment, self.right)?;
		}
	}
	impl misc::FirstSpecialToken for SequenceExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for ThisExpression {
		fn has_in_operator(&self) -> bool {
			self.left.has_in_operator() || self.right.has_in_operator()
		}
	}

	// (foo) => bar
	pub struct ArrowFunctionExpression {
		params: misc::FunctionParams,
		body: ArrowFunctionBody,
		fn_kind: ArrowFunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	pub enum ArrowFunctionKind {
		Normal,
		Async,

		Generator, // experimental
		AsyncGenerator, // experimental
	}
	impl misc::NodeDisplay for ArrowFunctionExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.fn_kind {
				ArrowFunctionKind::Normal => {},
				ArrowFunctionKind::Async => f.token(misc::Token::Async)?,
			}

			f.node(self.params)?;
			f.token(misc::Token::Arrow)?;
			f.node(self.body)?;
		}
	}
	impl misc::FirstSpecialToken for ArrowFunctionExpression {}
	impl misc::HasInOperator for ThisExpression {
		fn has_in_operator(&self) -> bool {
			match self.body {
				ArrowFunctionBody::Block(_) => false,
				ArrowFunctionBody::Expression(ref expr) => expr.has_in_operator(),
			}
		}
	}


	pub enum ArrowFunctionBody {
		Expression(Box<alias::Expression>),
		Block(misc::FunctionBody),
	}
	impl misc::NodeDisplay for ArrowFunctionBody {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self {
				ArrowFunctionBody::Expression(ref expr) => {
    			if let misc::SpecialToken::Curly = expr.first_special_token() {
						f.with_parens(|&mut f| f.node(expr)?)
					} else {
						f.node(expr)
					}
				}
				ArrowFunctionBody::Block(ref body) => f.node(body),
			}
		}
	}

	// do { foo; }
	pub struct DoExpression {
		body: statement::BlockStatement,
	}
	impl misc::NodeDisplay for DoExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Do)?;
			f.node(self.body)
		}
	}
	impl misc::FirstSpecialToken for DoExpression {}
	impl misc::HasInOperator for ThisExpression {}

	// new.target
	pub struct MetaProperty {
		kind: MetaPropertyKind,
	}
	pub enum MetaPropertyKind {
		NewTarget,
		ImportMeta, // experimental
		FunctionSent, // experimental
		FunctionArguments, // experimental
	}
	impl misc::NodeDisplay for MetaProperty {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			match self.kind {
				MetaPropertyKind::NewTarget => {
					f.token(misc::Token::New)?;
					f.token(misc::Token::Period)?;
					f.token(misc::Token::Target)
				}
				MetaPropertyKind::ImportMeta => {
					f.token(misc::Token::Import)?;
					f.token(misc::Token::Period)?;
					f.token(misc::Token::Meta)
				}
				MetaPropertyKind::FunctionSent => {
					f.token(misc::Token::Function)?;
					f.token(misc::Token::Period)?;
					f.token(misc::Token::Sent)
				}
				MetaPropertyKind::FunctionArguments => {
					f.token(misc::Token::Function)?;
					f.token(misc::Token::Period)?;
					f.token(misc::Token::Arguments)
				}
			}
		}
	}
	impl misc::FirstSpecialToken for MetaProperty {}
	impl misc::HasInOperator for ThisExpression {}

	// super.foo
	// super[foo]
	pub struct SuperMemberExpression {
		property: misc::PropertyId,
	}
	impl misc::NodeDisplay for SuperMemberExpression {
		fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
			f.token(misc::Token::Super)?;
			misc::NodeDisplay::fmt(self.property)
		}
	}
	impl misc::FirstSpecialToken for SuperMemberExpression {}
	impl misc::HasInOperator for ThisExpression {}
}
