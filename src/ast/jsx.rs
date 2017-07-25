use std::string;
use super::misc;
use super::alias;

nodes!{
	pub struct Element {
		opening: ElementName,
		attributes: Vec<Attribute>,
		children: Vec<Child>,
		closing: Option<ElementName>,
	}
  impl misc::NodeDisplay for Element {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::AngleL)?;
    	f.node(&self.opening)?;

    	for attr in self.attributes.iter() {
    		f.space()?;
    		f.node(attr)?;
    	}

    	if self.children.len() > 0 {
    		f.token(misc::Token::AngleR)?;

	    	for child in self.children.iter() {
	    		f.node(child)?;
	    	}

    		f.token(misc::Token::AngleSlash)?;
    		if let Some(close) = self.closing {
    			f.node(close)?;
    		} else {
    			f.node(self.opening)?;
    		}
    		f.token(misc::Token::AngleR)?;
    	} else {
    		if let Some(close) = self.closing {
    			f.token(misc::Token::AngleR)?;
    			f.token(misc::Token::AngleSlash)?;
    			f.node(close)?;
    			f.token(misc::Token::AngleR)?;
    		} else {
    			f.space();
    			f.token(misc::token::SlashAngle)?;
    		}
    	}
    }
  }

	pub struct Identifier {
		// Same as a JS identifier, but allows "-"
		raw: string::String,
		value: string::String,
	}
  impl misc::NodeDisplay for ElementName {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.jsx_identifier(&self.value, &self.raw)
    }
  }

	pub enum ElementName {
		Identifier(Identifier),
		Member(MemberExpression),
		Namespaced(NamespacedName),
	}
  impl misc::NodeDisplay for ElementName {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self {
    		ElementName::Identifier(ref id) => f.node(id),
    		ElementName::Member(ref id) => f.node(id),
    		ElementName::Namespaced(ref id) => f.node(id),
    	}
    }
  }

	pub struct MemberExpression {
		object: Box<MemberObject>,
		property: Identifier,
	}
  impl misc::NodeDisplay for ElementName {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self.object {
    		MemberObject::Identifier(ref id) => f.node(id),
    		MemberObject::Member(ref id) => f.node(id),
    	}
    	f.token(misc::Token::Period)?;
    	f.node(self.property)
    }
  }
	pub enum MemberObject {
		Identifier(Identifier),
		Member(MemberExpression),
	}

	pub struct NamespacedName {
		namespace: Identifier,
		name: Identifier,
	}
  impl misc::NodeDisplay for NamespacedName {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	misc::NodeDisplay::fmt(self.namespace)?;
    	f.token(misc::Token::Colon)?;
    	misc::NodeDisplay::fmt(self.name)
    }
  }

	pub enum Attribute {
		Spread(SpreadAttribute),
		Pair(PairAttribute),
	}
  impl misc::NodeDisplay for Attribute {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self {
    		Attribute::Spread(ref attr) => f.node(attr),
    		Attribute::Pair(ref attr) => f.node(attr),
    	}
    }
  }

	pub enum AttributeName {
		Identifier(Identifier),
		Namespaced(NamespacedName),
	}
  impl misc::NodeDisplay for AttributeName {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self {
    		AttributeName::Identifier(ref id) => f.node(id),
    		AttributeName::Namespaced(ref name) => f.node(name),
    	}
    }
  }

	pub struct SpreadAttribute {
		argument: alias::Expression,
	}
  impl misc::NodeDisplay for SpreadAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.token(misc::Token::CurlyL)?;
    	f.token(misc::Token::Ellipsis)?;
    	f.node(self.argument)?;
    	f.token(misc::Token::CurlyR)?;
    }
  }

	pub struct PairAttribute {
		name: AttributeName,
		value: Option<AttributeValue>,
	}
  impl misc::NodeDisplay for PairAttribute {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.node(self.name)?;
    	if let Some(value) = self.value {
	    	f.token(misc::Token::Eq)?;
	    	f.node(value)
	    }
    }
  }

	pub enum AttributeValue {
		String(StringLiteral),
		Expression(Box<alias::Expression>),
		Element(Element),
	}
  impl misc::NodeDisplay for AttributeValue {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self {
    		AttributeValue::String(ref s) => f.node(s),
    		AttributeValue::Expression(ref expr) => f.node(expr),
    		AttributeValue::Element(ref elem) => f.node(elem),
    	}
    }
  }

	pub struct StringLiteral {
		// String literal that allows _all_ chars, except closing quote
		raw: string::String,
		value: string::String,
	}
  impl misc::NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.jsx_string(&self.value, &self.raw)
    }
  }

	pub enum Child {
		Empty,
		Text(Text),
		Element(Element),
		Expression(Box<alias::Expression>),
		Spread(Box<alias::Expression>), // experimental?
	}
  impl misc::NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	match self {
    		Child::Empty => {
    			f.node(misc::Token::CurlyL)?;
    			f.node(misc::Token::CurlyR)
    		}
    		Child::Text(ref t) => f.node(t),
    		Child::Element(ref t) => f.node(t),
    		Child::Expression(ref t) => {
    			f.token(misc::Token::CurlyL)?;
    			f.node(t)
    			f.token(misc::Token::CurlyR)
    		}
    		Child::Spread(ref t) => {
    			f.token(misc::Token::CurlyL)?;
    			f.token(misc::Token::Ellipsis)?;
    			f.node(t)
    			f.token(misc::Token::CurlyR)
    		}
    	}
    }
  }
	pub struct Text {
		// Serialized string should contain HTML entities since it, allows all chars except {, }, <, and >
		value: string::String,
	}
  impl misc::NodeDisplay for StringLiteral {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
    	f.jsx_text(&self.value, &self.raw)
    }
  }
}
