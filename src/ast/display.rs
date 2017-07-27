use std::fmt;

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
    prec: Precedence,
    ends_with_integer: bool,
    in_operator: bool,

    max_depth: u32,
    current_depth: u32,
}
impl NodeFormatter {
    pub fn space(&mut self) -> NodeDisplayResult {
        Ok(())
    }
    // fn guard(&mut self) -> FlagGuard {}

    pub fn allow_in<T>(&mut self, cb: T) -> NodeDisplayResult
    where
        T: FnOnce(&mut Self) -> NodeDisplayResult,
    {
        let in_operator = self.in_operator;
        self.in_operator = true;

        let result = cb(self);

        self.in_operator = in_operator;

        Ok(())
    }

    pub fn with_precedence<T>(&mut self, p: Precedence, cb: T) -> NodeDisplayResult
    where
        T: FnOnce(&mut Self) -> NodeDisplayResult,
    {
        if (p as u8) < (self.prec as u8) {
            self.with_parens(cb)
        } else {
            self.track_prec(|f| {
                f.prec = p;

                cb(f)
            })
        }
    }

    pub fn with_parens<T>(&mut self, cb: T) -> NodeDisplayResult
    where
        T: FnOnce(&mut Self) -> NodeDisplayResult,
    {
        self.track_prec(|f| {
            f.prec = Precedence::Normal;

            f.token(Token::ParenL)?;
            let result = cb(f)?;
            f.token(Token::ParenR)?;

            Ok(result)
        })
    }

    fn track_prec<T>(&mut self, cb: T) -> NodeDisplayResult
    where
        T: FnOnce(&mut Self) -> NodeDisplayResult,
    {
        let prec = self.prec;
        let result = cb(self)?;
        self.prec = prec;

        Ok(result)
    }

    pub fn node_with_precedence<T: NodeDisplay>(
        &mut self,
        p: Precedence,
        s: &T,
    ) -> NodeDisplayResult {
        self.with_precedence(p, |f| f.node(s))
    }

    pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
        if self.current_depth == self.max_depth {
            // Configurable-per-node-type?
            Ok(()) //write!(self, "[[ Object ]]")
        } else {
            s.fmt(self)
        }
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

    pub fn identifier(&mut self, name: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(_raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "name"
        }
        Ok(())
    }
    pub fn string(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        //write!(self, "\'")?;
        if let Some(ref _raw) = raw {
            // Ensure that single-quotes are escaped
        } else {
            // Serialize "value", escaping anything that _must_ be escaped, like newlines and slashes
        }
        Ok(()) //write!(self, "\'")?;
    }
    pub fn number(&mut self, value: &f64, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is, possibly setting flag
            self.ends_with_integer = true;
        } else {
            // Serialize number
            self.ends_with_integer = true;
        }

        Ok(())
    }

    pub fn template_part(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value"
        }
        Ok(())
    }

    pub fn regexp(&mut self, value: &str, flags: &[char]) -> NodeDisplayResult {
        self.token(Token::Slash)?;
        // self.template_part(value)?;
        self.token(Token::Slash)?;
        // self.template_part(flags)
        Ok(())
    }

    pub fn jsx_identifier(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "name"
        }
        Ok(())
    }
    pub fn jsx_string(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value", encoding all entities like {}<>
        }
        Ok(())
    }
    pub fn jsx_text(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref _raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "value", encoding all entities like {}<>
        }
        Ok(())
    }
}
// impl fmt::Write for NodeFormatter {
//   pub fn write_str(&mut self, s: &str) -> std::fmt::Result {
//     Ok(())
//   }
// }


pub enum NodeDisplayError {}


pub trait NodeDisplay {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult;
}

impl<T: NodeDisplay> NodeDisplay for Box<T> {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        NodeDisplay::fmt(&*self, f)
    }
}


// impl fmt::Display for NodeDisplay {
// // where
// //   T: NodeDisplay
// // {
//   fn fmt(disp: &T, f: &mut fmt::Formatter) -> fmt::Result {
//     let mut node_fmt = NodeFormatter {};

//     NodeDisplay::fmt(&disp, &mut node_fmt)?;

//     // write!(f, node_fmt.build)
//   }
// }
