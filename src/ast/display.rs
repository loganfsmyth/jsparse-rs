
pub enum Token {
    // Tokens
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
    Semicolon,
    Ellipsis,
    Period,
    At,
    Comma,
    Colon,
    Star,
    Slash,

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
    Arrow,
    Continue,
    Break,
    Bar,
    Async,
    Caret,
    Amp,
    StarStar,
    Null,
    Question,
    LAngle,
    LAngleAngle,
    RAngle,
    RAngleAngle,
    RAngleAngleAngle,

    Plus,
    Subtract,
    ColonColon,
    AmpAmp,
    Mod,
    BarBar,
    BitwiseXor,
    LAngleEq,
    RAngleEq,
    RAngleAngleEq,
    Minus,
    Add,
    Bind,
    Delete,
    Yield,
    Exclam,
    Tilde,
    Instanceof,
    PlusPlus,
    MinusMinus,
    Hash,

    TemplateOpen,
    TemplateClose,
    TemplateTick,
    Module,
    Interface,
    Void,
    Get,
    Set,

    Number,
    String,
    Boolean,
    SlashAngle,
    AngleSlash,
    AngleR,
    Static,
    AngleL,
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

    state_stack: Vec<(Precedence, bool)>,
    ends_with_integer: bool,

    pub output: String,
}
impl NodeFormatter {
    pub fn new() -> NodeFormatter {
        NodeFormatter {
            current_depth: 0,
            prec: Precedence::Normal,
            in_operator: true,
            state_stack: Vec::with_capacity(50),
            ends_with_integer: false,
            output: String::with_capacity(512 * 1024),
        }
    }

    pub fn precedence(&mut self, p: Precedence) -> Option<WrapParens> {
        if (p as u32) > (self.prec as u32) {
            Some(WrapParens::new(self))
        } else {
            None
        }
    }
    pub fn require_precedence(&mut self, p: Precedence) -> CachePrecedence {
        let lock = CachePrecedence::new(self);
        self.prec = p;
        lock
    }

    pub fn allow_in(&mut self) -> CacheIn {
        let lock = CacheIn::new(self);
        self.in_operator = true;
        lock
    }
    pub fn disallow_in(&mut self) -> CacheIn {
        let lock = CacheIn::new(self);
        self.in_operator = false;
        lock
    }

    pub fn with_parens(&mut self) -> WrapParens {
        WrapParens::new(self)
    }

    pub fn with_block(&mut self) -> WrapBlock {
        WrapBlock::new(self)
    }

    pub fn with_square(&mut self) -> WrapSquare {
        WrapSquare::new(self)
    }

    pub fn space(&mut self) -> NodeDisplayResult {
        Ok(())
    }


    pub fn node_list<T: NodeDisplay>(&mut self, list: &[T]) -> NodeDisplayResult {
        for (i, item) in list.iter().enumerate() {
            if i != 0 {
                self.token(Token::Comma)?;
            }
            self.node(item)?;
        }

        Ok(())
    }

    pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
        self.state_stack.push((self.prec, self.in_operator));

        let result = s.fmt(self);

        self.state_stack.pop();

        result
    }
    pub fn token(&mut self, t: Token) -> NodeDisplayResult {
        match t {
            _ => {
                // TODO: If writing a period or ellipsis, ensure there is a
                // whitespace if the previous token was an integer
                Ok(())
            }
        }
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
        self.token(Token::Slash)?;
        // self.template_part(value)?;
        self.token(Token::Slash)?;
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
  fmt: &'a mut NodeFormatter,
}
impl<'a> WrapParens<'a> {
    fn new(fmt: &mut NodeFormatter) -> WrapParens {
        fmt.token(Token::ParenL);

        WrapParens {
            fmt,
        }
    }
}
impl<'a> ::std::ops::Drop for WrapParens<'a> {
  fn drop(&mut self) {
    self.fmt.token(Token::ParenR);
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

