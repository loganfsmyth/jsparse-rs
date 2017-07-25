pub enum Token {
  // Tokens
  Eq
  EqEq,
  EqEqEq,
  Neq,
  NeqEq,
  NeqEqEq,
  CurlyR,
  CurlyL,
  ParenR,
  ParenL,
  SquareR,
  SquareL,
  Semicolon,
  Ellipsis,
  Period,
  At,
  Comma,
  Colon,

  // Keywords
  Export,
  Default,
  Function,
  Class,
  Import,
  From,
  This,
  Extends,
  Implements,
  New,
  Target,
  Meta,
  Sent,
  Arguments,
  Super,
  Typeof,
  Type,
  Declare,
  Var,
  Let,
  Const,
  In,
  While,
  Do,
  Switch,
  With,
  Finally,
  Debugger,
  Catch,
}

pub enum Precedence {
  Normal = 1,
  Assignment,
  Conditional,
  LogicalOr,
  LogicalAnd,
  BitwiseOr,
  BitwiseXOr,
  BitwiseAnd,
  Equality,
  Relational,
  Shift,
  Additive,
  Multiplicative,
  Exponential,
  Unary,
  Update,
  LeftHand,
  New,
  Member,
  Primary,
}

pub struct NodeFormatter {
  prec: Precedence,

}
pub type NodeDisplayResult = Result<(), NodeDisplayError>;
impl NodeFormatter {
  pub fn with_precedence<T>(&mut self, p: Precedence, cb: T) -> NodeDisplayResult
  where
    T: FnOnce(&mut self) -> NodeDisplayResult
  {
    if p as u8 < self.prec as u8 {
      self.with_parens(cb)
    } else {
      self.track_prec(|&mut self| {
        self.prec = p;

        cb(&mut self)
      })
    }
  }

  pub fn with_parens(&mut self, cb: T) -> NodeDisplayResult
  where
    T: FnOnce(&mut self) -> NodeDisplayResult
  {
    self.track_prec(|&mut self| {
      self.prec = Precedence::Normal;

      self.token(Token::ParenL)?;
      let result = cb(&mut self)?;
      self.token(Token::ParenR)?;

      Ok(result)
    })
  }

  fn track_prec<T>(&mut self, cb: T) -> NodeDisplayResult
  where
    T: FnOnce(&mut self) -> NodeDisplayResult
  {
    let prec = self.prec;
    let result = cb(&mut self)?;
    self.prec = prec;

    Ok(result)
  }

  pub fn node_with_precedence<T: NodeDisplay>(&mut self, p: Precedence, s: &T) -> NodeDisplayResult {
    self.with_precedence(s, |&mut self| self.node(*s))
  }

  pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
    s.fmt(&mut self)
  }
  pub fn token(&mut self, t: Token) {
    // TODO: If writing a period or ellipsis, ensure there is a whitespace if the previous token was an integer
    match t {
    }
  }

  pub fn identifier(&mut self, name: &str, raw: &str) {

  }
  pub fn string(&mut self, value: &str, raw: &str) {

  }
  pub fn number(&mut self, value: &f64, raw: &str) {

  }

  pub fn template_part(&mut self) {

  }

  pub fn regexp(&mut self, value: &str, flags: &[char]) -> NodeDisplayResult {
    self.token(misc::Token::Slash)?;
    self.template_part(value)?;
    self.token(misc::Token::Slash)?;
    self.template_part(flags)
  }

  pub fn jsx_identifier(&mut self, value: &str, raw: &str) {}
  pub fn jsx_string(&mut self, value: &str, raw: &str) {}
  pub fn jsx_text(&mut self, value: &str, raw: &str) {}
}

pub enum NodeDisplayError {

}

pub impl fmt::Write for NodeFormatter {

}

pub trait NodeDisplay {
  fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult;
}
