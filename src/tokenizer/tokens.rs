use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctuatorToken {
  CurlyOpen, // {
  CurlyClose, // }
  ParenOpen, // (
  ParenClose, // )
  SquareOpen, // [
  SquareClose, // ]
  Semicolon, // ;
  Comma, // ,
  Tilde, // ~
  Question, // ?
  Colon, // :
  Period, // .
  Ellipsis, // ...
  LAngle, // <
  LAngleEq, // <=
  LAngleAngle, // <<
  LAngleAngleEq, // <<=
  LAngleExclamDashDash, // <!--
  RAngle, // >
  RAngleEq, // >=
  RAngleAngle, // >>
  RAngleAngleEq, // >>=
  RAngleAngleAngle, // >>>
  RAngleAngleAngleEq, // >>>=
  Exclam, // !
  ExclamEq, // !=
  ExclamEqEq, // !==
  Eq, // =
  Arrow, // =>
  EqEq, // ==
  EqEqEq, // ===
  Plus, // +
  PlusEq, // +=
  PlusPlus, // ++
  Minus, // -
  MinusEq, // -=
  MinusMinus, // --
  MinusMinusAngle, // -->
  Percent, // %
  PercentEq, // %=
  Star, // *
  StarEq, // *=
  StarStar, // **
  StarStarEq, // **=
  Slash, // /
  SlashEq, // /=
  Amp, // &
  AmpAmp, // &&
  AmpEq, // &=
  Bar, // |
  BarBar, // ||
  BarEq, // |=
  Caret, // ^
  CaretEq, // ^=
}
impl From<PunctuatorToken> for Token<'static> {
    fn from(t: PunctuatorToken) -> Self {
        Token::Punctuator(t)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentFormat {
  Line,
  Block,
  HTMLOpen,
  HTMLClose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentToken<'a> {
  pub format: CommentFormat,
  pub value: Cow<'a, str>,
}
impl<'a> From<CommentToken<'a>> for Token<'a> {
    fn from<'b>(t: CommentToken<'b>) -> Token<'b> {
        Token::Comment(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitespaceToken {}
impl From<WhitespaceToken> for Token<'static> {
    fn from(t: WhitespaceToken) -> Self {
        Token::Whitespace(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineTerminatorToken {}
impl From<LineTerminatorToken> for Token<'static> {
    fn from(t: LineTerminatorToken) -> Self {
        Token::LineTerminator(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegularExpressionLiteralToken<'a> {
  pub pattern: Cow<'a, str>,
  pub flags: Cow<'a, str>,
}
impl<'a> From<RegularExpressionLiteralToken<'a>> for Token<'a> {
    fn from<'b>(t: RegularExpressionLiteralToken<'b>) -> Token<'b> {
        Token::RegularExpressionLiteral(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierNameToken<'a> {
  // pub raw: Cow<'a, str>,
  pub name: Cow<'a, str>,
}
impl<'a> From<IdentifierNameToken<'a>> for Token<'a> {
    fn from<'b>(t: IdentifierNameToken<'b>) -> Token<'b> {
        Token::IdentifierName(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteralToken {
  pub value: f64,
}
impl From<NumericLiteralToken> for Token<'static> {
    fn from(t: NumericLiteralToken) -> Self {
        Token::NumericLiteral(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralToken<'a> {
  pub value: Cow<'a, str>,
}
impl<'a> From<StringLiteralToken<'a>> for Token<'a> {
    fn from<'b>(t: StringLiteralToken<'b>) -> Token<'b> {
        Token::StringLiteral(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub  enum TemplateFormat {
  // `foo`
  NoSubstitution,
  // `foo${
  Head,
  // }foo${
  Middle,
  // }foo`
  Tail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateToken<'a> {
  pub format: TemplateFormat,
  pub cooked: Cow<'a, str>,
  pub raw: Cow<'a, str>,
}
impl<'a> From<TemplateToken<'a>> for Token<'a> {
    fn from<'b>(t: TemplateToken<'b>) -> Token<'b> {
        Token::Template(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EOFToken { }
impl From<EOFToken> for Token<'static> {
    fn from(t: EOFToken) -> Self {
        Token::EOF(t)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Punctuator(PunctuatorToken),
    Comment(CommentToken<'a>),
    Whitespace(WhitespaceToken),
    LineTerminator(LineTerminatorToken),
    RegularExpressionLiteral(RegularExpressionLiteralToken<'a>),
    IdentifierName(IdentifierNameToken<'a>),
    NumericLiteral(NumericLiteralToken),
    StringLiteral(StringLiteralToken<'a>),
    Template(TemplateToken<'a>),
    EOF(EOFToken),

    // Boxed so we can jam in helpful info without making the overall token
    // structure larger than it needs to be.
    // Invalid(Box<InvalidToken>),
}

impl<'a> Default for Token<'a> {
  fn default() -> Token<'a> {
    EOFToken {}.into()
  }
}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum InvalidToken {
//   Codepoints(InvalidCodepoints),
//   String(InvalidString),
//   Template(InvalidTemplate),
//   Numeric(InvalidNumeric),
//   RegularExpression(InvalidRegularExpression),
// }

// // Has random unknown code points

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct InvalidCodepoints {}

// // Has a newline or unknown escape?
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct InvalidString {}

// // Has unknown escape
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct InvalidTemplate {}

// // Has unknown number format or trailing decimal
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct InvalidNumeric {}

// // Has a newline or escape in flags
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct InvalidRegularExpression {}
