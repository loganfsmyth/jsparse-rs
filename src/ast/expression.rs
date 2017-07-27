use std::string;
use std::fmt;

use super::misc;
use super::alias;
use super::flow;
use super::display;
use super::literal;
use super::statement;

use super::misc::FirstSpecialToken;

nodes!{
	// this
	pub struct ThisExpression {}
	impl display::NodeDisplay for ThisExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::This)
		}
	}
	impl misc::FirstSpecialToken for ThisExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken:: None }
	}
	impl misc::HasInOperator for ThisExpression {}

	pub struct ParenthesizedExpression {
		expr: Box<alias::Expression>,
	}
	impl display::NodeDisplay for ParenthesizedExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.with_parens(|f| f.node(&self.expr))
		}
	}
	impl misc::FirstSpecialToken for ParenthesizedExpression {}
	impl misc::HasInOperator for ParenthesizedExpression {}

	// [1, 2, 3, ...4]
	pub struct ArrayExpression {
		elements: Vec<Option<Box<alias::Expression>>>,
		spread: Option<Box<alias::Expression>>,
	}
	impl display::NodeDisplay for ArrayExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::SquareL)?;

			for el in self.elements.iter() {
				if let &Some(ref expr) = el {
					f.with_precedence(display::Precedence::Assignment, |f| f.node(expr))?;
				}
				f.token(display::Token::Comma)?;
			}

			if let Some(ref expr) = self.spread {
				f.with_precedence(display::Precedence::Assignment, |f| f.node(expr))?;
			}

			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ArrayExpression {}
	impl misc::HasInOperator for ArrayExpression {}

	// {a: 1, ...b}
	pub struct ObjectExpression {
		properties: Vec<ObjectProperty>,
		spread: Option<Box<alias::Expression>>, // experimental
	}
	impl display::NodeDisplay for ObjectExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::SquareL)?;

			for prop in self.properties.iter() {
				f.node(prop)?;
				f.token(display::Token::Comma)?;
			}

			if let Some(ref expr) = self.spread {
				f.with_precedence(display::Precedence::Assignment, |f| f.node(expr))?;
			}

			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ObjectExpression {
		fn first_special_token(&self) -> misc::SpecialToken {
			misc::SpecialToken::Curly
		}
	}
	impl misc::HasInOperator for ObjectExpression {}

	pub enum ObjectProperty {
		Method(ObjectMethod),
		Value(misc::PropertyId, Box<alias::Expression>),
	}
	impl display::NodeDisplay for ObjectProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self {
				&ObjectProperty::Method(ref method) => f.node(method),
				&ObjectProperty::Value(ref id, ref expr) => {
					f.node(id)?;
					f.token(display::Token::Colon)?;
					f.with_precedence(display::Precedence::Assignment, |f| f.node(expr))
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
	impl display::NodeDisplay for ObjectMethod {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.kind {
				misc::MethodKind::Normal => {}
				misc::MethodKind::Get => {
					f.token(display::Token::Get)?;
					f.space()?;
				}
				misc::MethodKind::Set => {
					f.token(display::Token::Set)?;
					f.space()?;
				}
			}

			f.node(&self.id)?;
			f.node(&self.params)?;
			if let Some(ref return_type) = self.return_type {
				f.node(return_type)?;
			}
			f.node(&self.body)
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
	impl display::NodeDisplay for FunctionExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Function)?;
			f.space();

			if let Some(ref id) = self.id {
				f.node(id)?;
			}
			if let Some(ref type_parameters) = self.type_parameters {
				f.node(type_parameters)?;
			}
			f.node(&self.params)?;
			if let Some(ref return_type) = self.return_type {
				f.node(return_type)?;
			}
			f.node(&self.body)
		}
	}
	impl misc::FirstSpecialToken for FunctionExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Function }
	}
	impl misc::HasInOperator for FunctionExpression {}

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
	impl display::NodeDisplay for ClassExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			for dec in self.decorators.iter() {
				f.node(dec)?;
			}

			f.token(display::Token::Class)?;
			f.space()?;

			if let Some(ref id) = self.id {
				f.node(id)?;
				// f.node(&self.type_parameters)?;
			}
			if let Some(ref expr) = self.extends {
				f.token(display::Token::Extends)?;
				f.with_precedence(display::Precedence::LeftHand, |f| f.node(expr))?;
			}
			if let Some(ref anno) = self.implements {
				f.token(display::Token::Implements)?;
				f.node(anno)?;
			}

			f.node(&self.body)
		}
	}
	impl misc::FirstSpecialToken for ClassExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { misc::SpecialToken::Class }
	}
	impl misc::HasInOperator for ClassExpression {}

	// /foo/g
	pub struct RegularExpressionLiteral {
		value: string::String,
		flags: Vec<char>,
	}
	impl display::NodeDisplay for RegularExpressionLiteral {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.regexp(&self.value, &self.flags)
		}
	}
	impl misc::FirstSpecialToken for RegularExpressionLiteral {}
	impl misc::HasInOperator for RegularExpressionLiteral {}

	// fn`content`
	pub struct TaggedTemplateLiteral {
		tag: Box<alias::Expression>,
		template: TemplateLiteral,
	}
	impl display::NodeDisplay for TaggedTemplateLiteral {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.with_precedence(display::Precedence::Member, |f| f.node(&self.tag))?;
			f.node(&self.template)
		}
	}
	impl misc::FirstSpecialToken for TaggedTemplateLiteral {
	  fn first_special_token(&self) -> misc::SpecialToken { self.tag.first_special_token() }
	}
	impl misc::HasInOperator for TaggedTemplateLiteral {}


// Other syntactic limits
// ++ / --
// => Only allowed member expressions and identifiers, all else early errors
// foo = bar;  or {a} = bar;
// => Ident, member, or patterns
// foo += bar;
// => Ident or member only



	// `content`
	pub enum TemplateLiteral {
		Piece(TemplatePart, Box<alias::Expression>, Box<TemplateLiteral>),
		End(TemplatePart),
	}
	impl display::NodeDisplay for TemplateLiteral {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO: Handle initial backtick here
			match self {
				&TemplateLiteral::Piece(ref part, ref expr, ref next_literal) => {
					f.node(part)?;
					f.token(display::Token::TemplateClose)?;
					f.with_precedence(display::Precedence::Normal, |f| f.node(expr))?;
					f.token(display::Token::TemplateOpen)?;
					f.node(next_literal)
				}
				&TemplateLiteral::End(ref part) => {
					f.node(part)?;
					f.token(display::Token::TemplateTick)
				}
			}
		}
	}
	impl misc::FirstSpecialToken for TemplateLiteral {}
	impl misc::HasInOperator for TemplateLiteral {}


	pub struct TemplatePart {
		value: string::String,
		rawValue: Option<string::String>,
	}
	impl display::NodeDisplay for TemplatePart {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.template_part(&self.value, self.rawValue.as_ref().map(String::as_str))
		}
	}

	// foo()
	pub struct CallExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
		optional: bool,
	}
	impl display::NodeDisplay for CallExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.with_precedence(display::Precedence::New, |f| f.node(&self.callee))?;
			if self.optional {
				f.token(display::Token::Question)?;
			}
			f.node(&self.arguments)
		}
	}
	impl misc::FirstSpecialToken for CallExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.callee.first_special_token() }
	}
	impl misc::HasInOperator for CallExpression {}

	// new foo()
	pub struct NewExpression {
		callee: Box<alias::Expression>,
		arguments: misc::CallArguments,
	}
	impl display::NodeDisplay for NewExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::New)?;
			f.with_precedence(display::Precedence::New, |f| f.node(&self.callee))?;
			f.node(&self.arguments)
		}
	}
	impl misc::FirstSpecialToken for NewExpression {}
	impl misc::HasInOperator for NewExpression {}

	// experimental
	// import(foo)
	pub struct ImportExpression {
		argument: Box<alias::Expression>,
	}
	impl display::NodeDisplay for ImportExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Import)?;
			f.token(display::Token::ParenL)?;
			f.with_precedence(display::Precedence::Assignment, |f| f.node(&self.argument))?;
			f.token(display::Token::ParenR)?;
			Ok(())
		}
	}
	impl misc::FirstSpecialToken for ImportExpression {}
	impl misc::HasInOperator for ImportExpression {}

	// foo.bar
	// foo?.bar
	// foo.#bar
	// foo?.#bar
	pub struct MemberExpression {
		object: Box<alias::Expression>,
		property: MemberProperty,
		optional: bool,
	}
	impl display::NodeDisplay for MemberExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.with_precedence(display::Precedence::Member, |f| f.node(&self.object))?;
			if self.optional {
				f.token(display::Token::Question)?;
			}
			f.token(display::Token::Period)?;
			f.node(&self.property)?;
			Ok(())
		}
	}
	impl misc::FirstSpecialToken for MemberExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.object.first_special_token() }
	}
	impl misc::HasInOperator for MemberExpression {}

	pub enum MemberProperty {
		Normal(misc::PropertyId),
		Private(PrivateProperty),
	}
	impl display::NodeDisplay for MemberProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self {
				&MemberProperty::Normal(ref id) => f.node(id),
				&MemberProperty::Private(ref prop) => f.node(prop),
			}
		}
	}
	pub struct PrivateProperty {
		property: misc::PropertyIdentifier,
	}
	impl display::NodeDisplay for PrivateProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Hash)?;
			f.node(&self.property)
		}
	}

	// #bar
	pub struct PrivateExpression {
		property: misc::PropertyIdentifier,
	}
	impl display::NodeDisplay for PrivateExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Hash)?;
			f.node(&self.property)
		}
	}
	impl misc::FirstSpecialToken for PrivateExpression {}
	impl misc::HasInOperator for PrivateExpression {}

	// i++
	// i--
	// ++i
	// --i
	pub struct UpdateExpression {
		value: misc::LeftHandSimpleAssign,
		operator: UpdateOperator,
	}
	pub enum UpdateOperator {
		PreIncrement,
		PreDecrement,
		PostIncrement,
		PostDecrement,
	}
	impl display::NodeDisplay for UpdateExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.operator {
				UpdateOperator::PreIncrement => {
					f.token(display::Token::PlusPlus)?;
					f.node(&self.value)?;
					f.with_precedence(display::Precedence::Unary, |f| f.node(&self.value))
				}
				UpdateOperator::PreDecrement => {
					f.token(display::Token::MinusMinus)?;
					f.with_precedence(display::Precedence::Unary, |f| f.node(&self.value))
				}
				UpdateOperator::PostIncrement => {
					f.with_precedence(display::Precedence::LeftHand, |f| f.node(&self.value))?;
					f.token(display::Token::PlusPlus)
				}
				UpdateOperator::PostDecrement => {
					f.with_precedence(display::Precedence::LeftHand, |f| f.node(&self.value))?;
					f.token(display::Token::MinusMinus)
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
	impl misc::HasInOperator for UpdateExpression {}


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
	impl display::NodeDisplay for UnaryExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.operator {
				UnaryOperator::Delete => f.token(display::Token::Delete)?,
				UnaryOperator::Void => f.token(display::Token::Void)?,
				UnaryOperator::Typeof => f.token(display::Token::Typeof)?,
				UnaryOperator::Positive => f.token(display::Token::Plus)?,
				UnaryOperator::Negative => f.token(display::Token::Minus)?,
				UnaryOperator::BitNegate => f.token(display::Token::Tilde)?,
				UnaryOperator::Negate => f.token(display::Token::Exclam)?,
				UnaryOperator::Await => f.token(display::Token::Await)?,
				UnaryOperator::Yield => f.token(display::Token::Yield)?,
				UnaryOperator::YieldDelegate => {
					f.token(display::Token::Delete)?;
					f.token(display::Token::Star)?
				}
				UnaryOperator::Bind => f.token(display::Token::Bind)?,
			}

			f.node_with_precedence(display::Precedence::Unary, &self.value)
		}
	}
	impl misc::FirstSpecialToken for UnaryExpression {}
	impl misc::HasInOperator for UnaryExpression {}

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
	impl display::NodeDisplay for BinaryExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.operator {
				BinaryOperator::Add => {
					f.node_with_precedence(display::Precedence::Additive, &self.left)?;
					f.token(display::Token::Plus)?;
					f.node_with_precedence(display::Precedence::Multiplicative, &self.right)?;
				}
				BinaryOperator::Subtract => {
					f.node_with_precedence(display::Precedence::Additive, &self.left)?;
					f.token(display::Token::Minus)?;
					f.node_with_precedence(display::Precedence::Multiplicative, &self.right)?;
				}
				BinaryOperator::LeftShift => {
					f.node_with_precedence(display::Precedence::Shift, &self.left)?;
					f.token(display::Token::LAngleAngle)?;
					f.node_with_precedence(display::Precedence::Additive, &self.right)?;
				}
				BinaryOperator::RightShift => {
					f.node_with_precedence(display::Precedence::Shift, &self.left)?;
					f.token(display::Token::RAngleAngle)?;
					f.node_with_precedence(display::Precedence::Additive, &self.right)?;
				}
				BinaryOperator::RightShiftSigned => {
					f.node_with_precedence(display::Precedence::Shift, &self.left)?;
					f.token(display::Token::RAngleAngleAngle)?;
					f.node_with_precedence(display::Precedence::Additive, &self.right)?;
				}
				BinaryOperator::Divide => {
					f.node_with_precedence(display::Precedence::Multiplicative, &self.left)?;
					f.token(display::Token::Slash)?;
					f.node_with_precedence(display::Precedence::Exponential, &self.right)?;
				}
				BinaryOperator::Multiply => {
					f.node_with_precedence(display::Precedence::Multiplicative, &self.left)?;
					f.token(display::Token::Star)?;
					f.node_with_precedence(display::Precedence::Exponential, &self.right)?;
				}
				BinaryOperator::Modulus => {
					f.node_with_precedence(display::Precedence::Multiplicative, &self.left)?;
					f.token(display::Token::Mod)?;
					f.node_with_precedence(display::Precedence::Exponential, &self.right)?;
				}
				BinaryOperator::BitAnd => {
					f.node_with_precedence(display::Precedence::BitwiseAnd, &self.left)?;
					f.token(display::Token::Amp)?;
					f.node_with_precedence(display::Precedence::Equality, &self.right)?;
				}
				BinaryOperator::BitOr => {
					f.node_with_precedence(display::Precedence::BitwiseOr, &self.left)?;
					f.token(display::Token::Bar)?;
					f.node_with_precedence(display::Precedence::BitwiseXOr, &self.right)?;
				}
				BinaryOperator::BitXor => {
					f.node_with_precedence(display::Precedence::BitwiseXOr, &self.left)?;
					f.token(display::Token::Caret)?;
					f.node_with_precedence(display::Precedence::BitwiseAnd, &self.right)?;
				}
				BinaryOperator::Power => {
					f.node_with_precedence(display::Precedence::Update, &self.left)?;
					f.token(display::Token::StarStar)?;
					f.node_with_precedence(display::Precedence::Exponential, &self.right)?;
				}
				BinaryOperator::Compare => {
					f.node_with_precedence(display::Precedence::Equality, &self.left)?;
					f.token(display::Token::EqEq)?;
					f.node_with_precedence(display::Precedence::Relational, &self.right)?;
				}
				BinaryOperator::StrictCompare => {
					f.node_with_precedence(display::Precedence::Equality, &self.left)?;
					f.token(display::Token::EqEqEq)?;
					f.node_with_precedence(display::Precedence::Relational, &self.right)?;
				}
				BinaryOperator::NegateCompare => {
					f.node_with_precedence(display::Precedence::Equality, &self.left)?;
					f.token(display::Token::Neq)?;
					f.node_with_precedence(display::Precedence::Relational, &self.right)?;
				}
				BinaryOperator::NegateStrictCompare => {
					f.node_with_precedence(display::Precedence::Equality, &self.left)?;
					f.token(display::Token::NeqEq)?;
					f.node_with_precedence(display::Precedence::Relational, &self.right)?;
				}
				BinaryOperator::LessThan => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::LAngle)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::LessThanEq => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::LAngleEq)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::GreaterThan => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::RAngle)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::GreaterThanEq => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::RAngleEq)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::In => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::In)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::Instanceof => {
					f.node_with_precedence(display::Precedence::Relational, &self.left)?;
					f.token(display::Token::Instanceof)?;
					f.node_with_precedence(display::Precedence::Shift, &self.right)?;
				}
				BinaryOperator::And => {
					f.node_with_precedence(display::Precedence::LogicalAnd, &self.left)?;
					f.token(display::Token::AmpAmp)?;
					f.node_with_precedence(display::Precedence::BitwiseOr, &self.right)?;
				}
				BinaryOperator::Or => {
					f.node_with_precedence(display::Precedence::LogicalOr, &self.left)?;
					f.token(display::Token::BarBar)?;
					f.node_with_precedence(display::Precedence::LogicalAnd, &self.right)?;
				}
				BinaryOperator::Bind => {
					f.node(&self.left)?;
					f.token(display::Token::ColonColon)?;
					f.node(&self.right)?;
				}
			}

			Ok(())
		}
	}
	impl misc::FirstSpecialToken for BinaryExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for BinaryExpression {
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
				BinaryOperator::Power => false,
				BinaryOperator::Compare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::StrictCompare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::NegateCompare => self.left.has_in_operator() || self.right.has_in_operator(),
				BinaryOperator::NegateStrictCompare => {
					self.left.has_in_operator() || self.right.has_in_operator()
				},
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
	impl display::NodeDisplay for ConditionalExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node_with_precedence(display::Precedence::LogicalOr, &self.test)?;
			f.token(display::Token::Question)?;
			f.node_with_precedence(display::Precedence::Assignment, &self.consequent)?;
			f.token(display::Token::Colon)?;
			f.node_with_precedence(display::Precedence::Assignment, &self.alternate)
		}
	}
	impl misc::FirstSpecialToken for ConditionalExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.test.first_special_token() }
	}
	impl misc::HasInOperator for ConditionalExpression {
		fn has_in_operator(&self) -> bool {
			self.test.has_in_operator()
		}
	}

	// foo = bar
	pub struct AssignmentExpression {
		left: Box<misc::LeftHandComplexAssign>,
		right: Box<alias::Expression>,
	}
	impl display::NodeDisplay for AssignmentExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node_with_precedence(display::Precedence::LeftHand, &self.left)?;
			f.token(display::Token::Eq)?;
			f.node_with_precedence(display::Precedence::Assignment, &self.right)
		}
	}
	impl misc::FirstSpecialToken for AssignmentExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for AssignmentExpression {
		fn has_in_operator(&self) -> bool {
			self.right.has_in_operator()
		}
	}

	// foo OP= bar
	pub struct AssignmentUpdateExpression {
		left: Box<misc::LeftHandSimpleAssign>,
		operator: AssignmentUpdateOperator,
		right: Box<alias::Expression>,
	}
	pub enum AssignmentUpdateOperator {
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
	impl display::NodeDisplay for AssignmentUpdateExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node_with_precedence(display::Precedence::LeftHand, &self.left)?;
			match self.operator {
				AssignmentUpdateOperator::Add => f.token(display::Token::Plus)?,
				AssignmentUpdateOperator::Subtract => f.token(display::Token::Subtract)?,
				AssignmentUpdateOperator::LeftShift => f.token(display::Token::LAngleAngle)?,
				AssignmentUpdateOperator::RightShift => f.token(display::Token::RAngleAngle)?,
				AssignmentUpdateOperator::RightShiftSigned => f.token(display::Token::RAngleAngleAngle)?,
				AssignmentUpdateOperator::Divide => f.token(display::Token::Slash)?,
				AssignmentUpdateOperator::Multiply => f.token(display::Token::Star)?,
				AssignmentUpdateOperator::Modulus => f.token(display::Token::Mod)?,
				AssignmentUpdateOperator::BitAnd => f.token(display::Token::Amp)?,
				AssignmentUpdateOperator::BitOr => f.token(display::Token::Bar)?,
				AssignmentUpdateOperator::BitXor => f.token(display::Token::Caret)?,
				AssignmentUpdateOperator::Power => f.token(display::Token::StarStar)?,
			}
			f.token(display::Token::Eq)?;
			f.node_with_precedence(display::Precedence::Assignment, &self.right)
		}
	}
	impl misc::FirstSpecialToken for AssignmentUpdateExpression {
	  fn first_special_token(&self) -> misc::SpecialToken {
	  	self.left.first_special_token()
	  }
	}
	impl misc::HasInOperator for AssignmentUpdateExpression {
		fn has_in_operator(&self) -> bool {
			self.right.has_in_operator()
		}
	}


	// foo, bar
	pub struct SequenceExpression {
		left: Box<alias::Expression>,
		right: Box<alias::Expression>,
	}
	impl display::NodeDisplay for SequenceExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node_with_precedence(display::Precedence::Normal, &self.left)?;
			f.token(display::Token::Comma)?;
			// TODO: This precedence may not be strictly needed?
			f.node_with_precedence(display::Precedence::Assignment, &self.right)
		}
	}
	impl misc::FirstSpecialToken for SequenceExpression {
	  fn first_special_token(&self) -> misc::SpecialToken { self.left.first_special_token() }
	}
	impl misc::HasInOperator for SequenceExpression {
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
	impl display::NodeDisplay for ArrowFunctionExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.fn_kind {
				ArrowFunctionKind::Normal => {},
				ArrowFunctionKind::Async => f.token(display::Token::Async)?,
				_ => {
					// TODO
				}
			}

			f.node(&self.params)?;
			f.token(display::Token::Arrow)?;
			f.node(&self.body)
		}
	}
	impl misc::FirstSpecialToken for ArrowFunctionExpression {}
	impl misc::HasInOperator for ArrowFunctionExpression {
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
	impl display::NodeDisplay for ArrowFunctionBody {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self {
				&ArrowFunctionBody::Expression(ref expr) => {
    			if let misc::SpecialToken::Curly = expr.first_special_token() {
						f.with_parens(|f| f.node(expr))
					} else {
						f.node(expr)
					}
				}
				&ArrowFunctionBody::Block(ref body) => f.node(body),
			}
		}
	}

	// do { foo; }
	pub struct DoExpression {
		body: statement::BlockStatement,
	}
	impl display::NodeDisplay for DoExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Do)?;
			f.node(&self.body)
		}
	}
	impl misc::FirstSpecialToken for DoExpression {}
	impl misc::HasInOperator for DoExpression {}

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
	impl display::NodeDisplay for MetaProperty {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			match self.kind {
				MetaPropertyKind::NewTarget => {
					f.token(display::Token::New)?;
					f.token(display::Token::Period)?;
					f.token(display::Token::Target)
				}
				MetaPropertyKind::ImportMeta => {
					f.token(display::Token::Import)?;
					f.token(display::Token::Period)?;
					f.token(display::Token::Meta)
				}
				MetaPropertyKind::FunctionSent => {
					f.token(display::Token::Function)?;
					f.token(display::Token::Period)?;
					f.token(display::Token::Sent)
				}
				MetaPropertyKind::FunctionArguments => {
					f.token(display::Token::Function)?;
					f.token(display::Token::Period)?;
					f.token(display::Token::Arguments)
				}
			}
		}
	}
	impl misc::FirstSpecialToken for MetaProperty {}
	impl misc::HasInOperator for MetaProperty {}

	// super.foo
	// super[foo]
	pub struct SuperMemberExpression {
		property: misc::PropertyId,
	}
	impl display::NodeDisplay for SuperMemberExpression {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Super)?;
			f.node(&self.property)
		}
	}
	impl misc::FirstSpecialToken for SuperMemberExpression {}
	impl misc::HasInOperator for SuperMemberExpression {}
}
