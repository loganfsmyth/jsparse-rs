use std::string;

nodes!{
  // null
  pub struct Null {}
  impl misc::NodeDisplay for Null {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
      f.token(misc::Token::Null)
    }
  }

  // true/false
  pub struct Boolean {
  	value: bool,
  }
  impl misc::NodeDisplay for Boolean {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
      if self.value {
        f.token(misc::Token::True)
      } else {
        f.token(misc::Token::False)
      }
    }
  }

  // 12
  pub struct Numeric {
    raw: string::String,
  	value: f64,
  }
  impl misc::NodeDisplay for Numeric {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
      f.number(&self.value, &self.raw)
    }
  }

  // "foo"
  pub struct String {
    raw: string::String,
  	value: string::String,
  }
  impl misc::NodeDisplay for String {
    fn fmt(&self, f: &mut NodeFormatter) -> misc::NodeDisplayResult {
      f.string(&self.value, &self.raw)
    }
  }
}
