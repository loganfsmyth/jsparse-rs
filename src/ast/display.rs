use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Punctuator {
    Eq,
    EqEq,
    EqEqEq,

    Neq,
    NeqEq,

    CurlyR,
    CurlyL,

    ParenR,
    ParenL,

    SquareR,
    SquareL,

    AngleR,
    AngleL,

    Semicolon,
    SQuote,

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

    // Add,
    Plus,
    PlusPlus,

    Subtract,
    Minus,
    MinusMinus,

    Arrow,
    ArrowStar,

    Caret,
    // BitwiseXor,
    LAngle,
    LAngleEq,
    LAngleAngle,

    RAngle,
    RAngleEq,
    RAngleAngle,
    // RAngleAngleEq,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Export,
    Default,
    Function,
    Class,
    Import,
    From,
    This,
    Extends,
    New,
    Target,
    Meta,
    Sent,
    Arguments,
    Super,
    Typeof,
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
    Void,
    Get,
    Set,
    Static,
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

pub trait HasOrphanIf {
    fn orphan_if(&self) -> bool {
        false
    }
}

pub enum SpecialToken {
    None,
    Declaration,
    Object,

    // TODO: Lookahead needed for :: operator
    // New,
}
pub trait FirstSpecialToken {
    fn first_special_token(&self) -> SpecialToken {
        SpecialToken::None
    }
}

pub type NodeDisplayResult = Result<(), NodeDisplayError>;

pub struct NodeFormatter {
    prec: Precedence,
    in_operator: bool,

    ends_with_integer: bool,
    ends_with_keyword: bool,

    pub output: String,
}
impl NodeFormatter {
    pub fn new() -> NodeFormatter {
        NodeFormatter {
            prec: Precedence::Normal,
            in_operator: true,
            ends_with_integer: false,
            ends_with_keyword: false,
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

    pub fn in_allowed(&self) -> bool {
        self.in_operator
    }

    pub fn wrap_parens(&mut self) -> WrapParens {
        let mut lock = WrapParens::new(self, false);
        lock.prec = Precedence::Normal;
        lock.in_operator = true;
        lock
    }

    pub fn wrap_curly(&mut self) -> WrapCurly {
        WrapCurly::new(self)
    }

    pub fn wrap_square(&mut self) -> WrapSquare {
        WrapSquare::new(self)
    }

    pub fn comma_list<T: NodeDisplay>(&mut self, list: &[T]) -> NodeDisplayResult {
        for (i, item) in list.iter().enumerate() {
            if i != 0 {
                self.punctuator(Punctuator::Comma);
            }
            self.require_precedence(Precedence::Assignment).node(item)?;
        }

        Ok(())
    }

    pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
        s.fmt(self)
    }

    pub fn keyword(&mut self, t: Keyword) {
        // println!("{:?}", t);

        if self.ends_with_keyword {
            write!(self, " ").unwrap();
        }
        self.ends_with_keyword = true;
        self.ends_with_integer = false;

        match t {
            Keyword::Export => write!(self, "export"),
            Keyword::Default => write!(self, "default"),
            Keyword::Function => write!(self, "function"),
            Keyword::Class => write!(self, "class"),
            Keyword::Import => write!(self, "import"),
            Keyword::From => write!(self, "from"),
            Keyword::This => write!(self, "this"),
            Keyword::Extends => write!(self, "extends"),
            Keyword::New => write!(self, "new"),
            Keyword::Target => write!(self, "target"),
            Keyword::Meta => write!(self, "meta"),
            Keyword::Sent => write!(self, "sent"),
            Keyword::Arguments => write!(self, "arguments"),
            Keyword::Super => write!(self, "super"),
            Keyword::Typeof => write!(self, "typeof"),
            Keyword::Var => write!(self, "var"),
            Keyword::Let => write!(self, "let"),
            Keyword::Const => write!(self, "const"),
            Keyword::In => write!(self, "in"),
            Keyword::While => write!(self, "while"),
            Keyword::Do => write!(self, "do"),
            Keyword::Switch => write!(self, "switch"),
            Keyword::With => write!(self, "with"),
            Keyword::Finally => write!(self, "finally"),
            Keyword::Debugger => write!(self, "debugger"),
            Keyword::Catch => write!(self, "catch"),
            Keyword::True => write!(self, "true"),
            Keyword::False => write!(self, "false"),
            Keyword::Return => write!(self, "return"),
            Keyword::Case => write!(self, "case"),
            Keyword::Await => write!(self, "await"),
            Keyword::For => write!(self, "for"),
            Keyword::Throw => write!(self, "throw"),
            Keyword::Try => write!(self, "try"),
            Keyword::Of => write!(self, "of"),
            Keyword::If => write!(self, "in"),
            Keyword::Continue => write!(self, "continue"),
            Keyword::Break => write!(self, "break"),
            Keyword::Async => write!(self, "async"),
            Keyword::Null => write!(self, "null"),
            Keyword::Delete => write!(self, "delete"),
            Keyword::Yield => write!(self, "yield"),
            Keyword::Instanceof => write!(self, "instanceof"),
            Keyword::Void => write!(self, "void"),
            Keyword::Get => write!(self, "get"),
            Keyword::Set => write!(self, "set"),
            Keyword::Static => write!(self, "static"),
            Keyword::As => write!(self, "as"),
        }.unwrap()
    }

    pub fn punctuator(&mut self, p: Punctuator) {
        self.ends_with_keyword = false;
        self.ends_with_integer = false;

        match p {
            Punctuator::Eq => write!(self, "="),
            Punctuator::EqEq => write!(self, "=="),
            Punctuator::EqEqEq => write!(self, "==="),
            Punctuator::Neq => write!(self, "!="),
            Punctuator::NeqEq => write!(self, "!=="),
            Punctuator::CurlyR => write!(self, "}}"),
            Punctuator::CurlyL => write!(self, "{{"),
            Punctuator::ParenR => write!(self, ")"),
            Punctuator::ParenL => write!(self, "("),
            Punctuator::SquareR => write!(self, "]"),
            Punctuator::SquareL => write!(self, "["),
            Punctuator::AngleR => write!(self, ">"),
            Punctuator::AngleL => write!(self, "<"),
            Punctuator::Semicolon => write!(self, ";"),
            Punctuator::SQuote => write!(self, "'"),
            Punctuator::Ellipsis => write!(self, "..."),
            Punctuator::Period => write!(self, "."),
            Punctuator::At => write!(self, "@"),
            Punctuator::Comma => write!(self, ","),
            Punctuator::Question => write!(self, "?"),
            Punctuator::Colon => write!(self, ":"),
            Punctuator::ColonColon => write!(self, "::"),
            Punctuator::Slash => write!(self, "/"),
            Punctuator::Star => write!(self, "*"),
            Punctuator::StarStar => write!(self, "**"),
            // Punctuator::Add => write!(self, "+"),
            Punctuator::Plus => write!(self, "+"),
            Punctuator::PlusPlus => write!(self, "++"),
            Punctuator::Subtract => write!(self, "-"),
            Punctuator::Minus => write!(self, "-"),
            Punctuator::MinusMinus => write!(self, "--"),
            Punctuator::Arrow => write!(self, "=>"),
            Punctuator::ArrowStar => write!(self, "=*>"),
            Punctuator::Caret => write!(self, "^"),
            // Punctuator::BitwiseXor => write!(self, "^"),
            Punctuator::LAngle => write!(self, "<"),
            Punctuator::LAngleEq => write!(self, "<="),
            Punctuator::LAngleAngle => write!(self, "<<"),
            Punctuator::RAngle => write!(self, ">"),
            Punctuator::RAngleEq => write!(self, ">="),
            Punctuator::RAngleAngle => write!(self, ">>"),
            // Punctuator::RAngleAngleEq => write!(self, ">>="),
            Punctuator::RAngleAngleAngle => write!(self, ">>>"),
            Punctuator::Mod => write!(self, "%"),
            Punctuator::Amp => write!(self, "&"),
            Punctuator::AmpAmp => write!(self, "&&"),
            Punctuator::Bar => write!(self, "|"),
            Punctuator::BarBar => write!(self, "||"),
            Punctuator::Bind => write!(self, "::"),
            Punctuator::Exclam => write!(self, "!"),
            Punctuator::Tilde => write!(self, "~"),
            Punctuator::Hash => write!(self, "#"),
            Punctuator::TemplateOpen => write!(self, "${{"),
            Punctuator::TemplateClose => write!(self, "}}"),
            Punctuator::TemplateTick => write!(self, "`"),
            Punctuator::SlashAngle => write!(self, "/>"),
            Punctuator::AngleSlash => write!(self, "</"),
        }.unwrap()
    }

    pub fn identifier(&mut self, name: &str, raw: Option<&str>) -> NodeDisplayResult {
        if self.ends_with_keyword {
            write!(self, " ").unwrap();
        }
        self.ends_with_keyword = true;
        self.ends_with_integer = false;

        if let Some(_raw) = raw {
            // Write raw value as-is
        } else {
            // Serialize "name"
        }
        write!(self, "{}", name)?;
        Ok(())
    }
    pub fn string(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        self.punctuator(Punctuator::SQuote);
        if let Some(ref _raw) = raw {
            // Ensure that single-quotes are escaped
            write!(self, "{}", value)?;
        } else {
            write!(self, "{}", value)?;
            // Serialize "value", escaping anything that _must_ be escaped,
            // like newlines and slashes
        }
        self.punctuator(Punctuator::SQuote);

        Ok(())
    }
    pub fn number(&mut self, value: &f64, _raw: Option<&str>) -> NodeDisplayResult {
        // if let Some(ref _raw) = raw {
        // Write raw value as-is, possibly setting flag
        // self.ends_with_integer = true;
        // } else {
        let s = format!("{}", value);
        write!(self, "{}", s)?;

        // Serialize number
        self.ends_with_integer = !s.chars().any(|c| c == '.');;
        // }

        Ok(())
    }

    pub fn template_part(&mut self, _value: &str, _raw: Option<&str>) -> NodeDisplayResult {
        // if let Some(ref _raw) = raw {
        //     // Write raw value as-is
        // } else {
        //     // Serialize "value"
        // }
        Ok(())
    }

    pub fn regexp(&mut self, value: &str, flags: &[char]) -> NodeDisplayResult {
        self.punctuator(Punctuator::Slash);
        write!(self, "{}", value)?;
        self.punctuator(Punctuator::Slash);
        for f in flags.iter() {
            write!(self, "{}", f)?;
        }
        Ok(())
    }

    pub fn jsx_identifier(&mut self, _value: &str, _raw: Option<&str>) -> NodeDisplayResult {
        // if let Some(ref _raw) = raw {
        //     // Write raw value as-is
        // } else {
        //     // Serialize "name"
        // }
        Ok(())
    }
    pub fn jsx_string(&mut self, _value: &str, _raw: Option<&str>) -> NodeDisplayResult {
        // if let Some(ref _raw) = raw {
        //     // Write raw value as-is
        // } else {
        //     // Serialize "value", encoding all entities like {}<>
        // }
        Ok(())
    }
    pub fn jsx_text(&mut self, _value: &str, _raw: Option<&str>) -> NodeDisplayResult {
        // if let Some(ref _raw) = raw {
        //     // Write raw value as-is
        // } else {
        //     // Serialize "value", encoding all entities like {}<>
        // }
        Ok(())
    }
}
impl fmt::Write for NodeFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output += s;

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
        fmt.punctuator(Punctuator::ParenL);

        WrapParens { skip, fmt }
    }
}
impl<'a> ::std::ops::Drop for WrapParens<'a> {
    fn drop(&mut self) {
        if !self.skip {
            self.fmt.punctuator(Punctuator::ParenR);
        }
    }
}

pub struct WrapSquare<'a> {
    fmt: &'a mut NodeFormatter,
}
impl<'a> WrapSquare<'a> {
    fn new(fmt: &mut NodeFormatter) -> WrapSquare {
        fmt.punctuator(Punctuator::SquareL);

        WrapSquare { fmt }
    }
}
impl<'a> ::std::ops::Drop for WrapSquare<'a> {
    fn drop(&mut self) {
        self.fmt.punctuator(Punctuator::SquareR);
    }
}

pub struct WrapCurly<'a> {
    fmt: &'a mut NodeFormatter,
}
impl<'a> WrapCurly<'a> {
    fn new(fmt: &mut NodeFormatter) -> WrapCurly {
        fmt.punctuator(Punctuator::CurlyL);

        WrapCurly { fmt }
    }
}
impl<'a> ::std::ops::Drop for WrapCurly<'a> {
    fn drop(&mut self) {
        self.fmt.punctuator(Punctuator::CurlyR);
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
impl_deref!(CachePrecedence, CacheIn, WrapParens, WrapSquare, WrapCurly);


#[derive(Debug)]
pub enum NodeDisplayError {
    Fmt(fmt::Error),
}
impl From<fmt::Error> for NodeDisplayError {
    fn from(s: fmt::Error) -> NodeDisplayError {
        NodeDisplayError::Fmt(s)
    }
}


pub trait NodeDisplay {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult;
}


impl<T: NodeDisplay> NodeDisplay for Box<T> {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        NodeDisplay::fmt(&**self, f)
    }
}
