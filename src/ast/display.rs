use std::fmt;
use std::fmt::Write;

use ast::{MaybeTokenPosition, KeywordData};
use ast;

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

    Plus,
    PlusPlus,

    Subtract,
    Minus,
    MinusMinus,

    Arrow,
    ArrowStar,

    Caret,
    LAngle,
    LAngleEq,
    LAngleAngle,

    RAngle,
    RAngleEq,
    RAngleAngle,
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

    QuestionPeriod,
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
    Else,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LookaheadRestriction {
    // No function/class declarations
    ExportDefault,
    // No function/class declarations, opencurlies, or let[
    ExpressionStatement,

    // No let[
    ForInit,

    // No let
    ForOfInit,

    // No {
    ConciseBody,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LookaheadSequence {
    None,
    Declaration,
    Curly,
    LetSquare,
    Let,
}

pub type NodeDisplayResult = Result<(), NodeDisplayError>;

pub struct NodeFormatter {
    prec: Precedence,
    in_operator: bool,
    wrap_standalone_if: bool,

    lookahead_restriction: Option<LookaheadRestriction>,
    ends_with_integer: bool,
    ends_with_keyword: bool,

    pub output: String,
}
impl NodeFormatter {
    pub fn new() -> NodeFormatter {
        NodeFormatter {
            prec: Precedence::Normal,
            in_operator: true,
            wrap_standalone_if: false,

            lookahead_restriction: None,
            ends_with_integer: false,
            ends_with_keyword: false,
            output: String::with_capacity(512 * 1024),
        }
    }

    /// Set the active precedence
    pub fn precedence<'a>(&'a mut self, p: Precedence) -> FormatterLock<'a> {
        let wrap = (p as u32) < (self.prec as u32);

        // TODO: does this need to set and restore the precedence too?

        // println!("{:?} to {:?}: {}", self.prec, p, wrap);

        self.wrap_parens_inner(wrap)
    }

    pub fn require_precedence<'a>(&'a mut self, p: Precedence) -> FormatterLock<'a> {
        let prec = self.prec;
        self.prec = p;

        FormatterLock::new(self, Box::new(move |fmt| { fmt.prec = prec; }))
    }

    /// Creates a formatter lock that will wrap the next expression in parentheses
    /// if the expression begins with a sequence of tokens disallowed by the
    /// given lookahead restriction.
    pub fn restrict_lookahead<'a>(
        &'a mut self,
        lookahead: LookaheadRestriction,
    ) -> FormatterLock<'a> {
        let lookahead_restriction = self.lookahead_restriction;
        self.lookahead_restriction = Some(lookahead);

        FormatterLock::new(
            self,
            Box::new(move |fmt| {
                if let Some(_) = fmt.lookahead_restriction {
                    // If the previous lookahead got cleared, we don't want to restore
                    // any existing lookahead restrictions either.
                    fmt.lookahead_restriction = lookahead_restriction;
                }
            }),
        )
    }

    /// Creates a formatter lock that wraps the output in parens if the given sequence
    /// matches a previously assigned lookahead restriction.
    pub fn lookahead_wrap_parens<'a>(
        &'a mut self,
        sequence: LookaheadSequence,
    ) -> FormatterLock<'a> {
        self.wrap_parens_inner(match self.lookahead_restriction {
            // No function/class declarations allowed
            Some(LookaheadRestriction::ExportDefault) => sequence == LookaheadSequence::Declaration,

            // No function/class declarations, opencurlies, or let[ allowed
            Some(LookaheadRestriction::ExpressionStatement) => sequence != LookaheadSequence::None,

            // No let[ allowed
            Some(LookaheadRestriction::ForInit) => sequence == LookaheadSequence::LetSquare,

            // No let[ allowed
            Some(LookaheadRestriction::ForOfInit) => {
                sequence == LookaheadSequence::Let || sequence == LookaheadSequence::LetSquare
            }

            // No { allowed
            Some(LookaheadRestriction::ConciseBody) => sequence == LookaheadSequence::Curly,

            None => false,
        })
    }

    /// Creates a formatter lock that allows unparenthesized usage of the "in" operator.
    pub fn allow_in<'a>(&'a mut self) -> FormatterLock<'a> {
        let in_operator = self.in_operator;
        self.in_operator = true;

        FormatterLock::new(
            self,
            Box::new(move |fmt| { fmt.in_operator = in_operator; }),
        )
    }

    /// Creates a formatter lock that disallows unparenthesized usage of the "in" operator.
    pub fn disallow_in<'a>(&'a mut self) -> FormatterLock<'a> {
        let in_operator = self.in_operator;
        self.in_operator = false;

        FormatterLock::new(
            self,
            Box::new(move |fmt| { fmt.in_operator = in_operator; }),
        )
    }

    /// Creates a formatter lock that will wrap the next items in parens if the in operator
    /// is currently disallowed.
    pub fn in_wrap_parens<'a>(&'a mut self) -> FormatterLock<'a> {
        let in_operator = self.in_operator;
        self.wrap_parens_inner(in_operator)
    }

    fn wrap_parens_inner<'a>(&'a mut self, wrap: bool) -> FormatterLock<'a> {
        if !wrap {
            return FormatterLock::new(self, Box::new(move |_fmt| {}));
        }

        let prec = self.prec;
        let in_operator = self.in_operator;
        let wrap_standalone_if = self.wrap_standalone_if;

        self.prec = Precedence::Normal;
        self.in_operator = true;
        self.wrap_standalone_if = false;
        self.punctuator(Punctuator::ParenL);

        FormatterLock::new(
            self,
            Box::new(move |fmt| {
                fmt.prec = prec;
                fmt.in_operator = in_operator;
                fmt.wrap_standalone_if = wrap_standalone_if;
                fmt.punctuator(Punctuator::ParenR);
            }),
        )
    }

    /// Creates a formatter lock that wraps the output in parens.
    pub fn wrap_parens<'a>(&'a mut self) -> FormatterLock<'a> {
        self.wrap_parens_inner(true)
    }

    /// Creates a formatter lock that wraps the output in curly brackets.
    pub fn wrap_curly<'a>(&'a mut self) -> FormatterLock<'a> {
        let wrap_standalone_if = self.wrap_standalone_if;
        let in_operator = self.in_operator;

        self.wrap_standalone_if = false;
        self.in_operator = true;
        self.punctuator(Punctuator::CurlyL);

        FormatterLock::new(
            self,
            Box::new(move |fmt| {
                fmt.wrap_standalone_if = wrap_standalone_if;
                fmt.in_operator = in_operator;
                fmt.punctuator(Punctuator::CurlyR);
            }),
        )
    }

    /// Creates a formatter lock that wraps the output in square brackets.
    pub fn wrap_square<'a>(&'a mut self) -> FormatterLock<'a> {
        let wrap_standalone_if = self.wrap_standalone_if;
        let in_operator = self.in_operator;

        self.wrap_standalone_if = false;
        self.in_operator = true;
        self.punctuator(Punctuator::SquareL);

        FormatterLock::new(
            self,
            Box::new(move |fmt| {
                fmt.wrap_standalone_if = wrap_standalone_if;
                fmt.in_operator = in_operator;
                fmt.punctuator(Punctuator::SquareR);
            }),
        )
    }

    /// Creates a formatter lock that disallows orphan "if" blocks
    /// (without else blocks) in the current scope.
    pub fn disallow_orphan_if<'a>(&'a mut self) -> FormatterLock<'a> {
        let wrap_standalone_if = self.wrap_standalone_if;

        self.wrap_standalone_if = true;
        FormatterLock::new(
            self,
            Box::new(move |fmt| { fmt.wrap_standalone_if = wrap_standalone_if; }),
        )
    }

    /// Creates a formatter lock that will wrap the output in curly brackets
    /// if orphan if blocks are currently disallowed.
    pub fn wrap_orphan_if<'a>(&'a mut self) -> FormatterLock<'a> {
        if !self.wrap_standalone_if {
            return FormatterLock::new(self, Box::new(move |_fmt| {}));
        }

        self.wrap_curly()
    }

    /// Prints a list of items with commas between them.
    pub fn comma_list<T: NodeDisplay>(&mut self, list: &[(T, KeywordData)]) -> NodeDisplayResult {
        let mut f = self.require_precedence(Precedence::Assignment);

        for &(ref item, ref dat) in list {
            f.node(item)?;
            f.punctuator(Punctuator::Comma, dat);
        }

        Ok(())
    }

    pub fn node_list<T: NodeDisplay>(&mut self, list: &[T]) -> NodeDisplayResult {
        for item in list.iter() {
            self.node(item)?;
        }

        Ok(())
    }

    /// Prints a given node.
    pub fn node<T: NodeDisplay>(&mut self, s: &T) -> NodeDisplayResult {
        s.fmt(self)
    }

    // pub fn separators<T: Separators>(&mut self, s: &T) {
    //     unimplemented!();
    // }

    /// Prints a given keyword.
    pub fn keyword<T: KeywordData>(&mut self, t: Keyword, _pos: &T) {
        // println!("{:?}", t);

        if self.ends_with_keyword {
            write!(self, " ").unwrap();
        }
        self.ends_with_keyword = true;
        self.ends_with_integer = false;
        self.lookahead_restriction = None;

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
            Keyword::If => write!(self, "if"),
            Keyword::Else => write!(self, "else"),
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

    /// Prints a given punctuator.
    pub fn punctuator(&mut self, p: Punctuator, _pos: &KeywordData) {
        self.ends_with_keyword = false;
        self.ends_with_integer = false;
        self.lookahead_restriction = None;

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
            Punctuator::Plus => write!(self, "+"),
            Punctuator::PlusPlus => write!(self, "++"),
            Punctuator::Subtract => write!(self, "-"),
            Punctuator::Minus => write!(self, "-"),
            Punctuator::MinusMinus => write!(self, "--"),
            Punctuator::Arrow => write!(self, "=>"),
            Punctuator::ArrowStar => write!(self, "=*>"),
            Punctuator::Caret => write!(self, "^"),
            Punctuator::LAngle => write!(self, "<"),
            Punctuator::LAngleEq => write!(self, "<="),
            Punctuator::LAngleAngle => write!(self, "<<"),
            Punctuator::RAngle => write!(self, ">"),
            Punctuator::RAngleEq => write!(self, ">="),
            Punctuator::RAngleAngle => write!(self, ">>"),
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
            Punctuator::QuestionPeriod => write!(self, "?."),
        }.unwrap()
    }

    /// Prints a given identifier.
    pub fn identifier(&mut self, name: &str, raw: Option<&str>) -> NodeDisplayResult {
        if self.ends_with_keyword {
            write!(self, " ").unwrap();
        }
        self.ends_with_keyword = true;
        self.ends_with_integer = false;
        self.lookahead_restriction = None;

        if let Some(raw) = raw {
            // Write raw value as-is
            write!(self, "{}", raw)?;
        } else {
            // Serialize "name"
            write!(self, "{}", name)?;
        }
        Ok(())
    }
    pub fn string(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        self.lookahead_restriction = None;

        self.punctuator(Punctuator::SQuote);
        if let Some(ref raw) = raw {
            // Ensure that single-quotes are escaped
            write!(self, "{}", raw)?;
        } else {
            write!(self, "{}", value)?;
            // Serialize "value", escaping anything that _must_ be escaped,
            // like newlines and slashes
        }
        self.punctuator(Punctuator::SQuote);

        Ok(())
    }
    pub fn number(&mut self, value: &f64, _raw: Option<&str>) -> NodeDisplayResult {
        if self.ends_with_keyword {
            write!(self, " ").unwrap();
            self.ends_with_integer = false;
        }
        self.lookahead_restriction = None;

        let s = format!("{}", value);
        write!(self, "{}", s)?;

        // Serialize number
        self.ends_with_integer = !s.chars().any(|c| c == '.');
        self.ends_with_keyword = false;

        Ok(())
    }

    pub fn template_part(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref raw) = raw {
            // Write raw value as-is
            write!(self, "{}", raw)?;
        } else {
            // Serialize "value"
            write!(self, "{}", value)?;
        }
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

    pub fn jsx_identifier(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref raw) = raw {
            // Write raw value as-is
            write!(self, "{}", raw)?;
        } else {
            // Serialize "value"
            write!(self, "{}", value)?;
        }
        Ok(())
    }
    pub fn jsx_string(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        self.punctuator(Punctuator::SQuote);
        if let Some(ref raw) = raw {
            // Write raw value as-is
            write!(self, "{}", raw)?;
        } else {
            // Serialize "value", encoding all entities like {}<>
            write!(self, "{}", value)?;
        }
        self.punctuator(Punctuator::SQuote);
        Ok(())
    }
    pub fn jsx_text(&mut self, value: &str, raw: Option<&str>) -> NodeDisplayResult {
        if let Some(ref raw) = raw {
            // Write raw value as-is
            write!(self, "{}", raw)?;
        } else {
            // Serialize "value", encoding all entities like {}<>
            write!(self, "{}", value)?;
        }
        Ok(())
    }
}
impl fmt::Write for NodeFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output += s;

        Ok(())
    }
}


pub struct FormatterLock<'a> {
    fmt: &'a mut NodeFormatter,
    drop: Box<Fn(&mut NodeFormatter) + 'static>,
}
impl<'a> FormatterLock<'a> {
    fn new(
        fmt: &'a mut NodeFormatter,
        drop: Box<Fn(&mut NodeFormatter) + 'static>,
    ) -> FormatterLock<'a> {
        FormatterLock { fmt, drop }
    }
}
impl<'a> ::std::ops::Drop for FormatterLock<'a> {
    fn drop(&mut self) {
        (self.drop)(self.fmt);
    }
}
impl<'a> ::std::ops::Deref for FormatterLock<'a> {
    type Target = NodeFormatter;

    fn deref(&self) -> &NodeFormatter {
        self.fmt
    }
}
impl<'a> ::std::ops::DerefMut for FormatterLock<'a> {
    fn deref_mut(&mut self) -> &mut NodeFormatter {
        self.fmt
    }
}


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
        let ref n = **self;
        n.fmt(f)
    }
}

impl<T: NodeDisplay> NodeDisplay for Option<T> {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        if let Some(ref n) = *self {
            n.fmt(f)?;
        }
        Ok(())
    }
}

trait KeywordData {
  fn leading(&self) {

  }

  fn keyword(&self) {

  }

  fn trailing(&self) {

  }
}

impl KeywordData for ast::KeywordData {

}
