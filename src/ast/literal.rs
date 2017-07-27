use std::string;
use super::display;
use super::misc;

nodes!{
  // null
  pub struct Null {}
  impl display::NodeDisplay for Null {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.token(display::Token::Null)
    }
  }
  impl misc::HasInOperator for Null {
      fn has_in_operator(&self) -> bool {
          false
      }
  }
  impl misc::FirstSpecialToken for Null {}

  // true/false
  pub struct Boolean {
  	value: bool,
  }
  impl display::NodeDisplay for Boolean {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      if self.value {
        f.token(display::Token::True)
      } else {
        f.token(display::Token::False)
      }
    }
  }
  impl misc::HasInOperator for Boolean {
      fn has_in_operator(&self) -> bool {
          false
      }
  }
  impl misc::FirstSpecialToken for Boolean {}

  // 12
  pub struct Numeric {
    raw: Option<string::String>,
  	value: f64,
  }
  impl display::NodeDisplay for Numeric {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.number(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
  }
  impl misc::HasInOperator for Numeric {
      fn has_in_operator(&self) -> bool {
          false
      }
  }
  impl misc::FirstSpecialToken for Numeric {}

  // "foo"
  pub struct String {
    raw: Option<string::String>,
  	value: string::String,
  }
  impl display::NodeDisplay for String {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
      f.string(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
  }
  impl misc::HasInOperator for String {
      fn has_in_operator(&self) -> bool {
          false
      }
  }
  impl misc::FirstSpecialToken for String {}
}
