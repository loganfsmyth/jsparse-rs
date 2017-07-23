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

	pub struct Identifier {
		// Same as a JS identifier, but allows "-"
		value: string::String,
	}

	pub enum ElementName {
		Identifier(Identifier),
		Member(MemberExpression),
		Namespaced(NamespacedName),
	}

	pub struct MemberExpression {
		object: Box<MemberObject>,
		property: Identifier,
	}

	pub enum MemberObject {
		Identifier(Identifier),
		Member(MemberExpression),
	}

	pub struct NamespacedName {
		namespace: Identifier,
		name: Identifier,
	}

	pub enum Attribute {
		Spread(SpreadAttribute),
		Pair(PairAttribute),
	}

	pub enum AttributeName {
		Identifier(Identifier),
		Namespaced(NamespacedName),
	}

	pub struct SpreadAttribute {
		argument: alias::Expression,
	}

	pub struct PairAttribute {
		name: AttributeName,
		value: AttributeValue,
	}

	pub enum AttributeValue {
		String(StringLiteral),
		Expression(Box<alias::Expression>),
		Element(Element),
	}

	pub struct StringLiteral {
		// String literal that allows _all_ chars, except closing quote
		value: string::String,
	}

	pub enum Child {
		Empty,
		Text(Text),
		Element(Element),
		Expression(Box<alias::Expression>),
		Spread(Box<alias::Expression>), // experimental?
	}
	pub struct Text {
		// Serialized string should contain HTML entities since it, allows all chars except {, }, <, and >
		value: string::String,
	}
}
