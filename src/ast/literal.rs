use std::string;
use super::misc;

// null
pub struct Null {
	position: misc::MaybePosition,
}

// true/false
pub struct Boolean {
	value: bool,
	position: misc::MaybePosition,
}

// 12
pub struct Numeric {
	value: f64,
	position: misc::MaybePosition,
}

// "foo"
pub struct String {
	value: string::String,
	position: misc::MaybePosition,
}