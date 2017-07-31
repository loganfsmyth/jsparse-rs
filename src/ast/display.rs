
pub enum Punctuator {
    Eq,
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

    AngleR,
    AngleL,

    Semicolon,

    Ellipsis,
    Period,

    At,
    Comma,
    Question,

    Colon,
    ColonColon,

    Slash,

    Star,
    StarStar,

    Add,
    Plus,
    PlusPlus,

    Subtract,
    Minus,
    MinusMinus,

    Arrow,

    Caret,
    BitwiseXor,

    LAngle,
    LAngleEq,
    LAngleAngle,

    RAngle,
    RAngleEq,
    RAngleAngle,
    RAngleAngleEq,
    RAngleAngleAngle,

    Mod,

    Amp,
    AmpAmp,

    Bar,
    BarBar,
    Bind,

    Exclam,
    Tilde,

    Hash,

    TemplateOpen,
    TemplateClose,
    TemplateTick,

    SlashAngle,
    AngleSlash,
}

pub enum Keyword {
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
    Any,
    Mixed,
    True,
    False,
    Return,
    Case,
    Await,
    For,
    Throw,
    Try,
    Of,
    If,
    Continue,
    Break,
    Async,
    Null,
    Delete,
    Yield,
    Instanceof,
    Module,
    Interface,
    Void,
    Get,
    Set,
    Number,
    String,
    Boolean,
    Static,
    Exports,
    As,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub type NodeDisplayResult = Result<(), NodeDisplayError>;

pub struct NodeFormatter {
    current_depth: u32,

    prec: Precedence,
    in_operator: bool,

    ends_with_integer: bool,

    pub output: String,
}
impl NodeFormatter {
    pub fn new() -> NodeFormatter {
        NodeFormatter {
            current_depth: 0,
            prec: Precedence::Normal,
            in_operator: true,
            ends_with_integer: false,
            output: String::with_capacity(512 * 1024),
        }
    }

    pub fn precedence(&mut self, p: Precedence) -> WrapParens {
        let skip = (p as u32) <= (self.prec as u32);
        WrapParens::new(self, skip)
    }

    pub fn require_precedence(&mut self, p: Precedence) -> CachePrecedence {
        let mut lock = CachePrecedence::new(self);
        lock.prec = p;
        lock
    }

    pub fn allow_in(&mut self) -> CacheIn {
        let mut lock = CacheIn::new(self);
        lock.in_operator = true;
        lock
    }
    pub fn disallow_in(&mut self) -> CacheIn {
        let mut lock = CacheIn::new(self);
        lock.in_operator = false;
        lock
    }

    pub fn wrap_parens(&mut self) -> WrapParens {
        let mut lock = WrapParens::new(self, false);
        lock.prec = Precedence::Normal;
        lock.in_operator = true;
        lock
    }

    pub fn wrap_block(&mut self) -> WrapBlock {
        WrapBlock::new(self)
    }

    pub fn wrap_square(&mut self) -> WrapSquare {
        WrapSquare::new(self)
    }

    pub fn comma_list<T: NodeDisplay>(&mut self, list: &[T]) -> NodeDisplayResult {
        for (i, item) in list.iter().enumerate() {
            if i != 0 {
                self.punctuator(Token::Comma)?;
            }
            self.require_precedence(Precedence::Assignment).node(item)?;
        }

        Ok(())
    }

    pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
        s.fmt(self)
    }

    pub fn keyword(&mut self, t: Keyword) -> NodeDisplayResult {

    }

    pub fn operator(&mut self, t: Operator) -> NodeDisplayResult {

    }

    pub fn identifier(&mut self, _name: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(_raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "name"
        }
        Ok(())
    }
    pub fn string(&mut self, _value: &str, raw: Option<&str>) -> NodeDisplayResult {
        //write!(self, "\'")?;
        if let Some(ref _raw) = raw {
            // Ensure that single-quotes are escaped
        } else {
            // Serialize "value", escaping anything that _must_ be escaped, like newlines and slashes
        }
        Ok(()) //write!(self, "\'")?;
    }
    pub fn number(&mut self, _value: &f64, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is, possibly setting flag
            self.ends_with_integer = true;
        } else {
            // Serialize number
            self.ends_with_integer = true;
        }

        Ok(())
    }

    pub fn template_part(&mut self, _value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value"
        }
        Ok(())
    }

    pub fn regexp(&mut self, _value: &str, flags: &[char]) -> NodeDisplayResult {
        self.punctuator(Punctuator::Slash)?;
        // self.template_part(value)?;
        self.punctuator(Punctuator::Slash)?;
        // self.template_part(flags)
        Ok(())
    }

    pub fn jsx_identifier(&mut self, _value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "name"
        }
        Ok(())
    }
    pub fn jsx_string(&mut self, _value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value", encoding all entities like {}<>
        }
        Ok(())
    }
    pub fn jsx_text(&mut self, _value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value", encoding all entities like {}<>
        }
        Ok(())
    }
}

pub struct CachePrecedence<'a> {
  prec: Precedence,
  fmt: &'a mut NodeFormatter,
}
impl<'a> CachePrecedence<'a> {
    fn new(fmt: &mut NodeFormatter) -> CachePrecedence {
        CachePrecedence {
            prec: fmt.prec,
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for CachePrecedence<'a> {
  fn drop(&mut self) {
    self.fmt.prec = self.prec;
  }
}
pub struct CacheIn<'a> {
  in_operator: bool,
  fmt: &'a mut NodeFormatter,
}
impl<'a> CacheIn<'a> {
    fn new(fmt: &mut NodeFormatter) -> CacheIn {
        CacheIn {
            in_operator: fmt.in_operator,
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for CacheIn<'a> {
  fn drop(&mut self) {
    self.fmt.in_operator = self.in_operator;
  }
}

pub struct WrapParens<'a> {
  skip: bool,
  fmt: &'a mut NodeFormatter,
}
impl<'a> WrapParens<'a> {
    fn new(fmt: &mut NodeFormatter, skip: bool) -> WrapParens {
        fmt.token(Token::ParenL);

        WrapParens {
            skip,
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for WrapParens<'a> {
  fn drop(&mut self) {
    if !self.skip { self.fmt.token(Token::ParenR); }
  }
}

pub struct WrapSquare<'a> {
  fmt: &'a mut NodeFormatter,
}
impl<'a> WrapSquare<'a> {
    fn new(fmt: &mut NodeFormatter) -> WrapSquare {
        fmt.token(Token::SquareL);

        WrapSquare {
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for WrapSquare<'a> {
  fn drop(&mut self) {
    self.fmt.token(Token::SquareR);
  }
}

pub struct WrapBlock<'a> {
  fmt: &'a mut NodeFormatter,
}
impl<'a> WrapBlock<'a> {
    fn new(fmt: &mut NodeFormatter) -> WrapBlock {
        fmt.token(Token::CurlyL);

        WrapBlock {
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for WrapBlock<'a> {
  fn drop(&mut self) {
    self.fmt.token(Token::CurlyR);
  }
}

macro_rules! impl_deref {
    ($id:ident) => {
        impl<'a> ::std::ops::Deref for $id<'a> {
          type Target = NodeFormatter;

          fn deref(&self) -> &NodeFormatter {
            self.fmt
          }
        }
        impl<'a> ::std::ops::DerefMut for $id<'a> {
          fn deref_mut(&mut self) -> &mut NodeFormatter {
            self.fmt
          }
        }
    };
    ($id:ident, $($ids:ident),+) => {
        impl_deref!($id);
        impl_deref!($($ids),+);
    };
}
impl_deref!(CachePrecedence, CacheIn, WrapParens, WrapSquare, WrapBlock);


pub enum NodeDisplayError {}


pub trait NodeDisplay {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult;
}


impl<T: NodeDisplay> NodeDisplay for Box<T> {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        NodeDisplay::fmt(&*self, f)
    }
}

