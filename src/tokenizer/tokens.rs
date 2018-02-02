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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitespaceToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineTerminatorToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegularExpressionLiteralToken<'a> {
  pub pattern: Cow<'a, str>,
  pub flags: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierNameToken<'a> {
  pub raw: Cow<'a, str>,
  pub name: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteralToken<'a> {
  pub raw: Cow<'a, str>,
  pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralToken<'a> {
  pub raw: Cow<'a, str>,
  pub value: Cow<'a, str>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct EOFToken { }

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Punctuator(PunctuatorToken),
    Comment(CommentToken<'a>),
    Whitespace(WhitespaceToken),
    LineTerminator(LineTerminatorToken),
    RegularExpressionLiteral(RegularExpressionLiteralToken<'a>),
    IdentifierName(IdentifierNameToken<'a>),
    NumericLiteral(NumericLiteralToken<'a>),
    StringLiteral(StringLiteralToken<'a>),
    Template(TemplateToken<'a>),
    EOF(EOFToken),

    // Boxed so we can jam in helpful info without making the overall token
    // structure larger than it needs to be.
    Invalid(Box<InvalidToken>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidToken {
  Codepoints(InvalidCodepoints),
  String(InvalidString),
  Template(InvalidTemplate),
  Numeric(InvalidNumeric),
  RegularExpression(InvalidRegularExpression),
}

// Has random unknown code points

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidCodepoints {}

// Has a newline or unknown escape?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidString {}

// Has unknown escape
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidTemplate {}

// Has unknown number format or trailing decimal
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidNumeric {}

// Has a newline or escape in flags
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidRegularExpression {}
