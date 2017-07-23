use std::string;

nodes!{
  // null
  pub struct Null {}

  // true/false
  pub struct Boolean {
  	value: bool,
  }

  // 12
  pub struct Numeric {
  	value: f64,
  }

  // "foo"
  pub struct String {
  	value: string::String,
  }
}