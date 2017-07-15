use std::string;
use super::misc;
use super::alias;

pub struct Element {
	opening: ElementName,
	attributes: Vec<Attribute>,
	children: Vec<Child>,
	closing: Option<ElementName>,

	position: misc::MaybePosition,
}

struct Identifier {
	// Same as a JS identifier, but allows "-"
	value: string::String,
	position: MaybePosition,
}

enum ElementName {
	Identifier(Identifier),
	Member(MemberExpression),
	Namespaced(NamespacedName),
}

struct MemberExpression {
	object: Box<MemberObject>,
	property: Identifier,
	position: MaybePosition,
}

enum MemberObject {
	Identifier(Identifier),
	Member(MemberExpression),
}

struct NamespacedName {
	namespace: Identifier,
	name: Identifier,
	position: MaybePosition,
}

enum Attribute {
	Spread(SpreadAttribute),
	Pair(PairAttribute),
}

enum AttributeName {
	Identifier(Identifier),
	Namespaced(NamespacedName),
}

struct SpreadAttribute {
	argument: alias::Expression,
	position: misc::MaybePosition,
}

struct PairAttribute {
	name: AttributeName,
	value: AttributeValue,
	position: misc::MaybePosition,
}

enum AttributeValue {
	String(StringLiteral),
	Expression(Box<alias::Expression>),
	Element(Element),
}

struct StringLiteral {
	// String literal that allows _all_ chars, except closing quote
	value: string::String,
	position: MaybePosition,
}

enum Child {
	Empty,
	Text(Text),
	Element(Element),
	Expression(Box<alias::Expression>),
	Spread(Box<alias::Expression>), // experimental?
}
struct Text {
	// Serialized string should contain HTML entities since it, allows all chars except {, }, <, and >
	value: string::String,
	position: MaybePosition,
}

