// extern crate ucd;

use std::char;
use std::f64;

use ucd::Codepoint;


#[derive(Debug, Default, Clone)]
pub struct CharRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default, Clone)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Token {
    // pub range: CharRange,
    // pub position: PositionRange,
    pub tok: TokenType,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TokenFlags {
    operator: bool,
    template: bool,
    annexb: bool,
    read_line: bool,
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Unknown
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    LCurly, // {
    RCurly, // }
    LParen, // (
    RParen, // )
    LSquare, // [
    RSquare, // ]
    Semicolon, // ;
    Comma, // ,
    Tilde, // ~
    Quest, // ?
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
    Mod, // %
    ModEq, // %=
    Star, // *
    StarEq, // *=
    StarStar, // **
    StarStarEq, // **=
    Div, // /
    DivEq, // /=
    Amp, // &
    AmpAmp, // &&
    AmpEq, // &=
    Bar, // |
    BarBar, // ||
    BarEq, // |=
    Caret, // ^
    CaretEq, // ^=

    Whitespace {
        // Whichever whitespace char was encountered
        value: char,
    },
    LineTerminator {
        // The line terminator that was encountered, may be one char or two.
        // TODO: Do we care about this value?
        value: String,
    },
    LineComment {
        // The comment text, excluding the initial `//` and final newlines.
        value: String,
    },
    BlockComment {
        // The comment text, excluding the initial `/*` and final `*/`.
        value: String,
    },
    HTMLOpenComment {
        // The comment text, excluding the initial `<!--` and final newlines.
        value: String,
    },
    HTMLCloseComment {
        // The comment text, excluding the initial `-->` and final newlines.
        value: String,
    },
    RegularExpressionLiteral {
        // The regex pattern text.
        value: String,

        // The list of flags.
        flags: String,
    },
    IdentifierName {
        // The raw identifier including escape codes
        raw: Option<String>,

        // The decoded identifier string.
        value: String,
    },
    NumericLiteral {
        // The raw string literal text, as it is in the original file.
        raw: Option<String>,

        // The decoded numeric value.
        value: f64,
    },
    StringLiteral {
        // The raw string literal text, as it is in the original file,
        // excluding leading and trailing quotes.
        raw: Option<String>,

        // The decoded string value.
        value: String,
    },
    TemplatePart {
        // The raw string content except CR and CRLF is converted to LF, as it is in the
        // original file, excluding leading and trailing backticks/${/}.
        raw: Option<String>,

        // The decoded string literal text.
        value: String,
    },

    Unknown,
    EOF,

    // TODO: Other invalid tokens?
    InvalidRegularExpressionLiteral,
    InvalidNumericLiteral,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TState {
    Start,

    // Single-char states
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    Semicolon,
    Comma,
    Tilde,
    Quest,
    Colon,

    // multi-char states
    LAngle,
    LAngleEq,
    LAngleAngle,
    LAngleAngleEq,
    LAngleExclam,
    LAngleExclamDash,
    LAngleExclamDashDash,

    RAngle,
    RAngleEq,
    RAngleAngle,
    RAngleAngleEq,
    RAngleAngleAngle,
    RAngleAngleAngleEq,
    Exclam,
    ExclamEq,
    ExclamEqEq,
    Eq,
    EqEq,
    EqEqEq,
    Plus,
    PlusEq,
    PlusPlus,
    Minus,
    MinusEq,
    MinusMinus,
    MinusMinusAngle,
    Mod,
    ModEq,
    Star,
    StarEq,
    StarStar,
    StarStarEq,
    Amp,
    AmpEq,
    AmpAmp,
    Bar,
    BarEq,
    BarBar,
    Caret,
    CaretEq,

    OperatorSlash,
    ExpressionSlash,
    SlashEq,

    Period(String, String),
    Dot,
    Ellipsis,

    Whitespace(char),

    // Comments
    MultiLineComment(String),
    MultiLineCommentStar(String),
    MultiLineCommentStarSlash(String),
    SingleLineComment(String),

    MiscLineText(String),

    LineTerminator(String),
    Ident(String, String),

    Zero(String, String),

    // 0[xX][0-9a-fA-F]+
    // 0[oO][0-7]+
    // 0[bB][0-1]+
    Hex(String, String),
    Octal(String, String),
    Binary(String, String),

    // 0       (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
    // [0-9]+  (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
    //         (\.[0-9]+)      ([eE][+-]?[0-9]+)?

    // annex B
    // top-level
    // 0[0-7]+

    // in fraction
    // 0      [8-9]?[0-9]+
    // 0[0-7]+[8-9][0-9]*
    Integer(String, String),
    Decimal(String, String),
    Exponent(String, String),
    ExponentSign(String, String),
    ExponentDigit(String, String),
    LegacyOctal(String, String),

    // String parsing
    DChars(String, String),
    DCharEnd(String, String),
    SChars(String, String),
    SCharEnd(String, String),

    // Regex parsing
    RegexChars(String),
    RegexClassChars(String),
    RegexClassEscapedChars(String),
    RegexEscapedChars(String),
    RegexFlags(String, String),

    // Template Literal parsing
    TemplateChars(String, String),
    TemplateDollarChar(String, String),
    TemplateCharLineTerminator(String, String),
    TemplateCharEnd(String, String),

    // IdentEscape
    IdentEscapeSequence(String, String),
    IdentEscapeHex1(String, String),
    IdentEscapeHex2(String, String, String),
    IdentEscapeHex3(String, String, String),
    IdentEscapeHex4(String, String, String),
    IdentEscapeHexStart(String, String),
    IdentEscapeHex(String, String, String),

    // SingleEscape
    SingleEscapeSequenceOrContinuation(String, String),
    SingleEscapeSequenceMaybeContinuationSequence(String, String),
    SingleLegacyOctal1(String, String, String),
    SingleLegacyOctal2(String, String, String),
    SingleEscapeHexStart(String, String),
    SingleEscapeHex(String, String, String),
    SingleEscapeHex1(String, String),
    SingleEscapeHex2(String, String, String),
    SingleEscapeHex3(String, String, String),
    SingleEscapeHex4(String, String, String),
    SingleEscapeSequenceHex1(String, String),
    SingleEscapeSequenceHex2(String, String, String),

    // DoubleEscape
    DoubleEscapeSequenceOrContinuation,
    DoubleEscapeSequenceMaybeContinuationSequence,
    DoubleLegacyOctal1,
    DoubleLegacyOctal2,
    DoubleEscapeHexStart,
    DoubleEscapeHex,
    DoubleEscapeHex1,
    DoubleEscapeHex2,
    DoubleEscapeHex3,
    DoubleEscapeHex4,
    DoubleEscapeSequenceHex1,
    DoubleEscapeSequenceHex2,

    // Template Literal
    TemplateEscapeSequenceOrContinuation,
    TemplateEscapeSequenceMaybeContinuationSequence,
    TemplateEscapeHexStart,
    TemplateEscapeHex,
    TemplateEscapeHex1,
    TemplateEscapeHex2,
    TemplateEscapeHex3,
    TemplateEscapeHex4,
    TemplateEscapeSequenceHex1,
    TemplateEscapeSequenceHex2,

    EOF,
    Unknown,

    InvalidRegexpLiteral,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberType {
    Hex,
    Octal,
    Binary,
    Float,
}

macro_rules! single_chars {
    ($r: ident, $s: ident, $c: ident) => {
        match $c {
            '\'' => TState::SCharEnd($r, $s),
            '\\' => TState::SingleEscapeSequenceOrContinuation(append($r, $c), $s),
            _ => TState::SChars(append($r, $c), append($s, $c)),
        }
    }
}


pub struct Tokenizer {
    state: TState,
    flags: TokenFlags,
    tokens: Vec<Token>,
}

fn append(mut s: String, c: char) -> String {
    s.push(c);
    s
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer::with_flags(TokenFlags {
            operator: false,
            template: false,
            annexb: false,
            read_line: false,
        })
    }
    pub fn with_flags(flags: TokenFlags) -> Tokenizer {
        Tokenizer {
            state: TState::Start,
            flags,
            // tokens: vec![Default::default(); 50],
            tokens: vec![Default::default(); 5],
            // raw: Default::default(),
        }
    }

    pub fn parse(mut self, s: &str) -> Vec<Token> {
        let mut chars = s;
        let mut v = Vec::with_capacity(100);

        while chars.len() > 0 {
            let pair = self.write_chars(chars);

            v.extend_from_slice(pair.0);
            chars = pair.1;
        }

        v.extend_from_slice(self.write_end());

        v
    }

    pub fn write_end(&mut self) -> &[Token] {
        match self.state {
            TState::Start => {
                self.state = TState::EOF;
                &self.tokens[0..0]
            }
            _ => {
                if let Some(num) = Tokenizer::write_char(
                    &mut self.state,
                    &self.flags,
                    '\n',
                    &mut self.tokens,
                )
                {
                    if let TState::Start = self.state {
                        self.state = TState::EOF;
                    } else {
                        // ERROR
                    }
                    self.state = TState::EOF;
                    &self.tokens[..num]
                } else {
                    self.state = TState::EOF;
                    &self.tokens[0..0]
                }
            }
        }
    }

    pub fn write_chars<'a, 'b>(&'a mut self, chars: &'b str) -> (&'a mut [Token], &'b str) {
        let mut char_offset = 0;
        let mut token_offset = 0;

        for c in chars.chars() {
            if let Some(num) = Tokenizer::write_char(
                &mut self.state,
                &self.flags,
                c,
                &mut self.tokens[token_offset..],
            )
            {
                token_offset += num;
                char_offset += c.len_utf8();
            } else {
                break;
            }
        }

        (&mut self.tokens[..token_offset], &chars[char_offset..])
    }


    // None -> Char not consumed, not enough space in tokens
    // Some(n) -> Char consumed, n tokens written
    fn write_char(
        state: &mut TState,
        flags: &TokenFlags,
        c: char,
        tokens: &mut [Token],
    ) -> Option<usize> {
        let mut count = 0;

        loop {
            // println!("C: {:04X} => {} from {:?}", c as u32, c, state);

            *state = match state.to_owned() {
                TState::Start => {
                    match c {
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::LineTerminator(append(String::new(), c)),
                        _ if flags.read_line => TState::MiscLineText(append(String::new(), c)),

                        // Spec explicit whitespace whitelist.
                        '\u{9}' | '\u{B}' | '\u{C}' | '\u{20}' | '\u{A0}' | '\u{FEFF}' |
                        // Unicode "Space_Separator" characters
                        '\u{1680}' |
                        '\u{2000}'...'\u{200A}' |
                        '\u{202F}' |
                        '\u{205F}' |
                        '\u{3000}' => {
                            TState::Whitespace(c)
                        },
                        '(' => TState::LParen,
                        ')' => TState::RParen,
                        '{' => TState::LCurly,
                        '}' => {
                            if flags.template {
                                TState::TemplateChars(String::new(), String::new())
                            } else {
                                TState::RCurly
                            }
                        }
                        '[' => TState::LSquare,
                        ']' => TState::RSquare,
                        ';' => TState::Semicolon,
                        ',' => TState::Comma,
                        '~' => TState::Tilde,
                        '?' => TState::Quest,
                        ':' => TState::Colon,
                        '<' => TState::LAngle,
                        '>' => TState::RAngle,
                        '!' => TState::Exclam,
                        '=' => TState::Eq,
                        '+' => TState::Plus,
                        '-' => TState::Minus,
                        '%' => TState::Mod,
                        '*' => TState::Star,
                        '&' => TState::Amp,
                        '|' => TState::Bar,
                        '^' => TState::Caret,
                        '/' => {
                            if flags.operator {
                                TState::OperatorSlash
                            } else {
                                TState::ExpressionSlash
                            }
                        }
                        '`' => TState::TemplateChars(String::new(), String::new()),
                        '.' => TState::Period(append(String::new(), c), append(String::new(), c)),
                        '0' => TState::Zero(append(String::new(), c), append(String::new(), c)),
                        '1'...'9' => TState::Integer(append(String::new(), c), append(String::new(), c)),
                        '"' => TState::DChars(String::new(), String::new()),
                        '\'' => TState::SChars(String::new(), String::new()),

                        '\\' => TState::IdentEscapeSequence(append(String::new(), c), String::new()),
                        c if is_ident_start(c) => TState::Ident(append(String::new(), c), append(String::new(), c)),
                        _ => TState::MiscLineText(append(String::new(), c)),
                    }
                }

                TState::Whitespace(c) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Whitespace { value: c };
                    count += 1;

                    TState::Start
                }

                TState::MiscLineText(s) => {
                    match c {
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::HTMLOpenComment { value: s };
                            count += 1;

                            TState::Start
                        }
                        _ => TState::MiscLineText(s),
                    }
                }

                TState::LParen => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LParen;
                    count += 1;

                    TState::Start
                }
                TState::RParen => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RParen;
                    count += 1;

                    TState::Start
                }
                TState::LCurly => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LCurly;
                    count += 1;

                    TState::Start
                }
                TState::LSquare => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LSquare;
                    count += 1;

                    TState::Start
                }
                TState::RSquare => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RSquare;
                    count += 1;

                    TState::Start
                }
                TState::Semicolon => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Semicolon;
                    count += 1;

                    TState::Start
                }
                TState::Comma => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Comma;
                    count += 1;

                    TState::Start
                }
                TState::Tilde => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Tilde;
                    count += 1;

                    TState::Start
                }
                TState::Quest => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Quest;
                    count += 1;

                    TState::Start
                }
                TState::Colon => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Colon;
                    count += 1;

                    TState::Start
                }
                TState::LineTerminator(s) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LineTerminator { value: s };
                    count += 1;

                    TState::Start
                }
                TState::RCurly => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RCurly;
                    count += 1;

                    TState::Start
                }

                TState::LAngle => {
                    match c {
                        '<' => TState::LAngleAngle,
                        '=' => TState::LAngleEq,
                        '!' if flags.annexb => TState::LAngleExclam,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::LAngle;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::LAngleEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LAngleEq;
                    count += 1;

                    TState::Start
                }
                TState::LAngleAngle => {
                    match c {
                        '=' => TState::LAngleAngleEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::LAngleAngle;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::LAngleAngleEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LAngleAngleEq;
                    count += 1;

                    TState::Start
                }

                TState::LAngleExclam => {
                    match c {
                        '-' => TState::LAngleExclamDash,
                        _ => {
                            if tokens.len() < 2 {
                                return None;
                            }

                            tokens[0].tok = TokenType::LAngle;
                            tokens[1].tok = TokenType::Exclam;
                            count += 2;

                            TState::Start
                        }
                    }
                }
                TState::LAngleExclamDash => {
                    match c {
                        '-' => TState::LAngleExclamDashDash,
                        _ => {
                            if tokens.len() < 3 {
                                return None;
                            }
                            tokens[0].tok = TokenType::LAngle;
                            tokens[1].tok = TokenType::Exclam;
                            tokens[2].tok = TokenType::Minus;
                            count += 3;

                            TState::Start
                        }
                    }
                }
                TState::LAngleExclamDashDash => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::LAngleExclamDashDash;
                    count += 1;

                    TState::Start
                }

                TState::RAngle => {
                    match c {
                        '>' => TState::RAngleAngle,
                        '=' => TState::RAngleEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::RAngle;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::RAngleEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RAngleEq;
                    count += 1;

                    TState::Start
                }
                TState::RAngleAngle => {
                    match c {
                        '>' => TState::RAngleAngleAngle,
                        '=' => TState::RAngleAngleEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::RAngleAngle;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::RAngleAngleEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RAngleAngleEq;
                    count += 1;

                    TState::Start
                }
                TState::RAngleAngleAngle => {
                    match c {
                        '=' => TState::RAngleAngleAngleEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::RAngleAngleAngle;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::RAngleAngleAngleEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::RAngleAngleAngleEq;
                    count += 1;

                    TState::Start
                }
                TState::Exclam => {
                    match c {
                        '=' => TState::ExclamEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Exclam;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::ExclamEq => {
                    match c {
                        '=' => TState::ExclamEqEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::ExclamEq;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::ExclamEqEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::ExclamEqEq;
                    count += 1;

                    TState::Start
                }
                TState::Eq => {
                    match c {
                        '=' => TState::EqEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Eq;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::EqEq => {
                    match c {
                        '=' => TState::EqEqEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::EqEq;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::EqEqEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::EqEqEq;
                    count += 1;

                    TState::Start
                }
                TState::Plus => {
                    match c {
                        '+' => TState::PlusPlus,
                        '=' => TState::PlusEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Plus;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::PlusPlus => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::PlusPlus;
                    count += 1;

                    TState::Start
                }
                TState::PlusEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::PlusEq;
                    count += 1;

                    TState::Start
                }
                TState::Minus => {
                    match c {
                        '-' => TState::MinusMinus,
                        '=' => TState::MinusEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Minus;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::MinusMinus => {
                    match c {
                        // TODO: This should only be a token after a comment on a new line
                        '>' if flags.annexb => TState::MinusMinusAngle,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::MinusMinus;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::MinusEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::MinusEq;
                    count += 1;

                    TState::Start
                }
                TState::MinusMinusAngle => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::MinusMinusAngle;
                    count += 1;

                    TState::Start
                }

                TState::Mod => {
                    match c {
                        '=' => TState::ModEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Mod;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::ModEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::ModEq;
                    count += 1;

                    TState::Start
                }
                TState::Star => {
                    match c {
                        '*' => TState::StarStar,
                        '=' => TState::StarEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Star;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::StarEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::StarEq;
                    count += 1;

                    TState::Start
                }
                TState::StarStar => {
                    match c {
                        '=' => TState::StarStarEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::StarStar;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::StarStarEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::StarStarEq;
                    count += 1;

                    TState::Start
                }
                TState::Amp => {
                    match c {
                        '=' => TState::AmpEq,
                        '&' => TState::AmpAmp,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Amp;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::AmpEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::AmpEq;
                    count += 1;

                    TState::Start
                }
                TState::AmpAmp => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::AmpAmp;
                    count += 1;

                    TState::Start
                }
                TState::Bar => {
                    match c {
                        '=' => TState::BarEq,
                        '|' => TState::BarBar,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Bar;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::BarEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::BarEq;
                    count += 1;

                    TState::Start
                }
                TState::BarBar => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::BarBar;
                    count += 1;

                    TState::Start
                }
                TState::Caret => {
                    match c {
                        '=' => TState::CaretEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Caret;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::CaretEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::CaretEq;
                    count += 1;

                    TState::Start
                }
                TState::OperatorSlash => {
                    match c {
                        '/' => TState::SingleLineComment(String::new()),
                        '*' => TState::MultiLineComment(String::new()),
                        '=' => TState::SlashEq,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Div;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::SlashEq => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::DivEq;
                    count += 1;

                    TState::Start
                }
                TState::MultiLineComment(s) => {
                    match c {
                        '*' => TState::MultiLineCommentStar(s),
                        _ => TState::MultiLineComment(append(s, c)),
                    }
                }
                TState::MultiLineCommentStar(s) => {
                    match c {
                        '*' => TState::MultiLineCommentStar(append(s, '*')),
                        '/' => TState::MultiLineCommentStarSlash(s),
                        _ => TState::MultiLineComment(append(s, c)),
                    }
                }
                TState::MultiLineCommentStarSlash(s) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::BlockComment { value: s };
                    count += 1;

                    TState::Start
                }

                TState::SingleLineComment(s) => {
                    match c {
                        // Line terminator chars
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::LineComment { value: s };
                            count += 1;

                            TState::Start
                        }
                        _ => TState::SingleLineComment(append(s, c)),
                    }
                }

                TState::Ident(r, s) => {
                    match c {
                        '\\' => TState::IdentEscapeSequence(append(r, c), s),
                        c if is_ident_continue(c) => TState::Ident(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::IdentifierName {
                                raw: r.into(), // TODO
                                value: s,
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }

                // .01234
                TState::Period(r, s) => {
                    match c {
                        '0'...'9' => TState::Integer(append(r, c), append(s, c)),
                        '.' => TState::Dot,
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Period;
                            count += 1;

                            TState::Start
                        },
                    }
                }
                // foo..3
                TState::Dot => {
                    match c {
                        '0'...'9' => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Period;
                            count += 1;

                            TState::Integer(append(String::from("."), c), append(String::from("."), c))
                        }
                        '.' => TState::Ellipsis,
                        _ => {
                            if tokens.len() < 2 {
                                return None;
                            }
                            tokens[0].tok = TokenType::Period;
                            tokens[1].tok = TokenType::Period;
                            count += 2;

                            TState::Start
                        }
                    }
                }
                TState::Ellipsis => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::Ellipsis;
                    count += 1;

                    TState::Start
                }

                // 0.1234
                TState::Zero(r, s) => {
                    match c {
                        '.' => TState::Decimal(append(r, c), append(s, c)),

                        'x' | 'X' => TState::Hex(append(r, c), String::new()),
                        'o' | 'O' => TState::Octal(append(r, c), String::new()),
                        'b' | 'B' => TState::Binary(append(r, c), String::new()),

                        'e' | 'E' => TState::ExponentSign(append(r, c), append(s, c)),
                        '0'...'7' if flags.annexb => {
                            TState::LegacyOctal(append(r, c), append(s, c))
                        }
                        '8' | '9' if flags.annexb => TState::Integer(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: s.parse::<f64>().unwrap(), // TODO
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }

                TState::Hex(r, s) => {
                    match c {
                        '0'...'9' => TState::Hex(append(r, c), append(s, c)),
                        'a'...'f' => TState::Hex(append(r, c), append(s, c)),
                        'A'...'F' => TState::Hex(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: u64::from_str_radix(&s, 16).unwrap() as f64,
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::Octal(r, s) => {
                    match c {
                        '0'...'7' => TState::Octal(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: u64::from_str_radix(&s, 8).unwrap() as f64,
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::Binary(r, s) => {
                    match c {
                        '0' | '1' => TState::Binary(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: u64::from_str_radix(&s, 2).unwrap() as f64,
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }

                TState::LegacyOctal(r, s) => {
                    match c {
                        '0'...'7' => TState::LegacyOctal(append(r, c), append(s, c)),
                        '8' | '9' => TState::Integer(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: {
                                    match u64::from_str_radix(&s, 8) {
                                        Ok(n) => {
                                            n as f64
                                        }
                                        Err(_) => {
                                            f64::INFINITY
                                        }
                                    }
                                },
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }

                // TODO: There's a bug in here, the period logic makes no sense
                TState::Integer(r, s) => {
                    match c {
                        '0'...'9' => TState::Integer(append(r, c), append(s, c)),
                        '.' => TState::Decimal(append(r, c), append(s, c)), // This doesn't seem right
                        'e' | 'E' => TState::ExponentSign(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: s.parse::<f64>().unwrap(), // TODO
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::Decimal(r, s) => {
                    match c {
                        '0'...'9' => TState::Decimal(append(r, c), append(s, c)),
                        'e' | 'E' => TState::ExponentSign(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: s.parse::<f64>().unwrap(), // TODO
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::ExponentSign(r, s) => {
                    match c {
                        '+' | '-' => TState::Exponent(append(r, c), append(s, c)),
                        '0'...'9' => TState::ExponentDigit(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::InvalidNumericLiteral;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::Exponent(r, s) => {
                    match c {
                        '0'...'9' => TState::ExponentDigit(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::InvalidNumericLiteral;
                            count += 1;

                            TState::Start
                        }
                    }
                }
                TState::ExponentDigit(r, s) => {
                    match c {
                        '0'...'9' => TState::ExponentDigit(append(r, c), append(s, c)),
                        _ => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            tokens[0].tok = TokenType::NumericLiteral {
                                raw: r.into(),
                                value: s.parse::<f64>().unwrap(), // TODO
                            };
                            count += 1;

                            TState::Start
                        }
                    }
                }

                TState::DChars(r, s) => {
                    match c {
                        '"' => TState::DCharEnd(r, s),
                        '\\' => TState::DoubleEscapeSequenceOrContinuation,
                        _ => TState::DChars(append(r, c), append(s, c)),
                    }
                }
                TState::DCharEnd(r, s) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::StringLiteral {
                        raw: r.into(),
                        value: s,
                    };
                    count += 1;

                    TState::Start
                }

                TState::SChars(r, s) => {
                    single_chars!(r, s, c)
                }
                TState::SCharEnd(r, s) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::StringLiteral {
                        raw: r.into(),
                        value: s,
                    };
                    count += 1;

                    TState::Start
                }

                TState::ExpressionSlash => {
                    match c {
                        '/' => TState::SingleLineComment(String::new()),
                        '*' => TState::MultiLineComment(String::new()),

                        '[' => TState::RegexClassChars(append(String::new(), c)),
                        '\\' => TState::RegexEscapedChars(append(String::new(), c)),

                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::InvalidRegexpLiteral,
                        _ => TState::RegexChars(append(String::new(), c)),
                    }
                }
                TState::RegexChars(s) => {
                    match c {
                        '/' => TState::RegexFlags(s, String::new()),
                        '[' => TState::RegexClassChars(append(s, c)),
                        '\\' => TState::RegexEscapedChars(append(s, c)),

                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::InvalidRegexpLiteral,
                        _ => TState::RegexChars(append(s, c)),
                    }
                }
                TState::RegexClassChars(s) => {
                    match c {
                        ']' => TState::RegexChars(append(s, c)),
                        '\\' => TState::RegexClassEscapedChars(append(s, c)),
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::InvalidRegexpLiteral,
                        _ => TState::RegexClassChars(append(s, c)),
                    }
                }

                TState::RegexClassEscapedChars(s) => {
                    match c {
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::InvalidRegexpLiteral,
                        _ => TState::RegexClassChars(append(s, c)),
                    }
                }
                TState::RegexEscapedChars(s) => {
                    match c {
                        '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::InvalidRegexpLiteral,
                        _ => TState::RegexChars(append(s, c)),
                    }
                }
                TState::RegexFlags(s, flags) => {
                    if is_ident_continue(c) {
                        TState::RegexFlags(s, append(flags, c))
                    } else {
                        if tokens.len() == 0 {
                            return None;
                        }
                        tokens[0].tok = TokenType::RegularExpressionLiteral { value: s, flags };
                        count += 1;

                        TState::Start
                    }
                }
                TState::InvalidRegexpLiteral => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::InvalidRegularExpressionLiteral;
                    count += 1;

                    TState::Start
                }

                TState::TemplateChars(r, s) => {
                    match c {
                        '`' => TState::TemplateCharEnd(r, s),
                        '$' => TState::TemplateDollarChar(r, s),
                        '\\' => TState::TemplateEscapeSequenceOrContinuation,
                        '\r' => TState::TemplateCharLineTerminator(r, s),
                        _ => TState::TemplateChars(append(r, c), append(s, c)),
                    }
                }
                TState::TemplateDollarChar(r, s) => {
                    match c {
                        '`' => TState::TemplateCharEnd(r, s),
                        '{' => TState::TemplateCharEnd(r, s),
                        '$' => TState::TemplateDollarChar(append(r, '$'), append(s, '$')),
                        '\\' => TState::TemplateEscapeSequenceOrContinuation,
                        '\r' => TState::TemplateCharLineTerminator(append(r, c), append(s, c)),
                        _ => TState::TemplateChars(append(r, c), append(s, c)),
                    }
                }

                TState::TemplateCharLineTerminator(r, s) => {
                    match c {
                        '\n' => TState::TemplateChars(append(r, c), append(s, c)),
                        '$' => TState::TemplateDollarChar(append(r, '$'), append(s, '$')),
                        '\\' => TState::TemplateEscapeSequenceOrContinuation,
                        '\r' => TState::TemplateCharLineTerminator(append(r, c), append(s, c)),
                        _ => TState::TemplateChars(append(r, c), append(s, c)),
                    }
                }

                TState::TemplateCharEnd(r, s) => {
                    if tokens.len() == 0 {
                        return None;
                    }
                    tokens[0].tok = TokenType::TemplatePart {
                        raw: r.into(),
                        value: s,
                    };
                    count += 1;

                    TState::Start
                }



                // IdentEscape
                TState::IdentEscapeSequence(r, s) => {
                    match c {
                        'u' => TState::IdentEscapeHex1(append(r, c), s),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHex1(r, s) => {
                    match c {
                        '{' => TState::IdentEscapeHexStart(append(r, c), s),
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::IdentEscapeHex2(append(r, c), s, append(String::new(), c)),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHex2(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::IdentEscapeHex3(append(r, c), s, append(h, c)),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHex3(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::IdentEscapeHex4(append(r, c), s, append(h, c)),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHex4(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => {
                            let h = append(h, c);
                            match u32::from_str_radix(&h, 16) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::Ident(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        }
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHexStart(r, s) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::IdentEscapeHex(append(r, c), s, append(String::new(), c)),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }
                TState::IdentEscapeHex(r, s, h) => {
                    match c {
                        '}' => {
                            match u32::from_str_radix(&h, 16) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::Ident(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        },
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::IdentEscapeHex(append(r, c), s, append(h, c)),
                        // if valid ident char, continue parsing ident, else back to start
                        _ => TState::Unknown,
                    }
                }




                // SingleEscape
                TState::SingleEscapeSequenceOrContinuation(r, s) => {
                    match c {
                        '0'...'3' if flags.annexb => TState::SingleLegacyOctal1(append(r, c), s, append(String::new(), c)),
                        '4'...'7' if flags.annexb => TState::SingleLegacyOctal2(append(r, c), s, append(String::new(), c)),

                        // TODO: This needs to actually throw if it has decimals after it
                        '0' => TState::SChars(append(r, c), append(s, '\u{0}')),
                        '1'...'9' => TState::Unknown, // Continue parsing string
                        'u' => TState::SingleEscapeHex1(append(r, c), s),
                        'x' => TState::SingleEscapeSequenceHex1(append(r, c), s),
                        '\r' => TState::SingleEscapeSequenceMaybeContinuationSequence(append(r, c), s),
                        '\n' | '\u{2028}' | '\u{2029}' => TState::SChars(append(r, c), s),

                        _ => single_chars!(r, s, c),
                    }
                }
                TState::SingleEscapeSequenceMaybeContinuationSequence(r, s) => {
                    match c {
                        '\n' => TState::SChars(append(r, c), s),
                        _ => single_chars!(r, s, c),
                    }
                }
                TState::SingleLegacyOctal1(r, s, h) => {
                    match c {
                        '0'...'7' => TState::SingleLegacyOctal2(append(r, c), s, append(h, c)),
                        _ => {
                            match u32::from_str_radix(&h, 8) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            let s = append(s, decoded_c);

                                            single_chars!(r, s, c)
                                        },
                                        None => {
                                            panic!("Unexpected number");
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number");
                                }
                            }
                        }
                    }
                }
                TState::SingleLegacyOctal2(r, s, h) => {
                    match c {
                        '0'...'7' => {
                            let h = append(h, c);
                            match u32::from_str_radix(&h, 8) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::SChars(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        }
                        _ => {
                            match u32::from_str_radix(&h, 8) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            let s = append(s, decoded_c);

                                            single_chars!(r, s, c)
                                        },
                                        None => {
                                            panic!("Unexpected number");
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number");
                                }
                            }
                        }
                    }
                }

                TState::SingleEscapeHex1(r, s) => {
                    match c {
                        '{' => TState::SingleEscapeHexStart(append(r, c), s),
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeHex2(append(r, c), s, append(String::new(), c)),
                        _ => TState::Unknown, // Keep looking for '
                    }
                }
                TState::SingleEscapeHex2(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeHex3(append(r, c), s, append(h, c)),
                        _ => TState::Unknown, // Keep looking for '
                    }
                }
                TState::SingleEscapeHex3(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeHex4(append(r, c), s, append(h, c)),
                        _ => TState::Unknown, // Keep looking for '
                    }
                }
                TState::SingleEscapeHex4(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => {
                            let h = append(h, c);
                            match u32::from_str_radix(&h, 16) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::SChars(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        }
                        _ => TState::Unknown, // Keep looking for '
                    }
                }
                TState::SingleEscapeHexStart(r, s) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeHex(append(r, c), s, append(String::new(), c)),
                        _ => TState::Unknown, // Keep looking for } or '
                    }
                }
                TState::SingleEscapeHex(r, s, h) => {
                    match c {
                        '}' => {
                            match u32::from_str_radix(&h, 16) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::SChars(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        }
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeHex(append(r, c), s, append(h, c)),
                        _ => TState::Unknown, // Keep looking for } or '
                    }
                }
                TState::SingleEscapeSequenceHex1(r, s) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::SingleEscapeSequenceHex2(append(r, c), s, append(String::new(), c)),
                        _ => TState::Unknown, // Keep looking for '
                    }
                }
                TState::SingleEscapeSequenceHex2(r, s, h) => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => {
                            let h = append(h, c);
                            match u32::from_str_radix(&h, 16) {
                                Ok(n) => {
                                    match char::from_u32(n) {
                                        Some(decoded_c) => {
                                            TState::SChars(append(r, c), append(s, decoded_c))
                                        },
                                        None => {
                                            panic!("Unexpected number")
                                        }
                                    }
                                }
                                Err(_) => {
                                    panic!("Unexpected number")
                                }
                            }
                        }
                        _ => TState::Unknown, // Keep looking for '
                    }
                }

                // DoubleEscape
                TState::DoubleEscapeSequenceOrContinuation => {
                    match c {
                        '0'...'3' if flags.annexb => TState::DoubleLegacyOctal1,
                        '4'...'7' if flags.annexb => TState::DoubleLegacyOctal2,

                        '0' => TState::DChars(String::new(), String::new()),
                        '1'...'9' => TState::Unknown,
                        'u' => TState::DoubleEscapeHex1,
                        'x' => TState::DoubleEscapeSequenceHex1,
                        '\r' => TState::DoubleEscapeSequenceMaybeContinuationSequence,
                        '\n' | '\u{2028}' | '\u{2029}' => TState::DChars(String::new(), String::new()),
                        _ => TState::DChars(String::new(), String::new()),
                    }
                }
                TState::DoubleEscapeSequenceMaybeContinuationSequence => {
                    match c {
                        '\n' => TState::DChars(String::new(), String::new()),
                        _ => TState::DChars(String::new(), String::new()),
                    }
                }

                TState::DoubleLegacyOctal1 => {
                    match c {
                        '0'...'7' => TState::DoubleLegacyOctal2,
                        _ => TState::DChars(String::new(), String::new()),
                    }
                }
                TState::DoubleLegacyOctal2 => {
                    match c {
                        '0'...'7' => TState::DChars(String::new(), String::new()),
                        _ => TState::DChars(String::new(), String::new()),
                    }
                }

                TState::DoubleEscapeHex1 => {
                    match c {
                        '{' => TState::DoubleEscapeHexStart,
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeHex2,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeHex2 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeHex3,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeHex3 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeHex4,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeHex4 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DChars(String::new(), String::new()),
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeHexStart => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeHex,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeHex => {
                    match c {
                        '}' => TState::DChars(String::new(), String::new()),
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeHex,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeSequenceHex1 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DoubleEscapeSequenceHex2,
                        _ => TState::Unknown,
                    }
                }
                TState::DoubleEscapeSequenceHex2 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::DChars(String::new(), String::new()),
                        _ => TState::Unknown,
                    }
                }

                // TemplateEscape
                TState::TemplateEscapeSequenceOrContinuation => {
                    match c {
                        '0' => TState::TemplateChars(String::new(), String::new()),
                        '1'...'9' => TState::Unknown,
                        'u' => TState::TemplateEscapeHex1,
                        'x' => TState::TemplateEscapeSequenceHex1,
                        '\r' => TState::TemplateEscapeSequenceMaybeContinuationSequence,
                        '\n' | '\u{2028}' | '\u{2029}' => {
                            TState::TemplateChars(String::new(), String::new())
                        }
                        _ => TState::TemplateChars(String::new(), String::new()),
                    }
                }
                TState::TemplateEscapeSequenceMaybeContinuationSequence => {
                    match c {
                        '\n' => TState::TemplateChars(String::new(), String::new()),
                        _ => TState::TemplateChars(String::new(), String::new()),
                    }
                }
                TState::TemplateEscapeHex1 => {
                    match c {
                        '{' => TState::TemplateEscapeHexStart,
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeHex2,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeHex2 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeHex3,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeHex3 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeHex4,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeHex4 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => {
                            TState::TemplateChars(String::new(), String::new())
                        }
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeHexStart => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeHex,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeHex => {
                    match c {
                        '}' => TState::TemplateChars(String::new(), String::new()),
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeHex,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeSequenceHex1 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => TState::TemplateEscapeSequenceHex2,
                        _ => TState::Unknown,
                    }
                }
                TState::TemplateEscapeSequenceHex2 => {
                    match c {
                        '0'...'9' | 'a'...'f' | 'A'...'F' => {
                            TState::TemplateChars(String::new(), String::new())
                        }
                        _ => TState::Unknown,
                    }
                }

                // TODO:
                TState::Unknown => TState::Unknown,
                TState::EOF => panic!("Attempted state transition after EOF"),
            };

            // println!("DONE");

            if let &mut TState::Start = state {
                // If we switched to the start state, the character is not consumed, so we
                // continue the loop in the state machine.
                continue;
            } else {
                break;
            }
        }

        Some(count)
    }
}


fn is_ident_start(c: char) -> bool {
    c.is_id_start() || c == '$' || c == '_' || c == '\\'
}

fn is_ident_continue(c: char) -> bool {
    c.is_id_continue() || c == '$' || c == '_' || c == '\\' || c == '\u{200C}' || c == '\u{200D}'
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_operator {
        ($str: expr, $type: expr) => {
            assert_token_with_flags!(TokenFlags {
                operator: true,
                template: false,
                annexb: false,
                read_line: false,
            }, $str, $type);
        }
    }

    macro_rules! assert_token {
        ($str: expr, $type: expr) => {
            assert_token_with_flags!(TokenFlags {
                operator: false,
                template: false,
                annexb: false,
                read_line: false,
            }, $str, $type);
        }
    }

    macro_rules! assert_operator_annexb {
        ($str: expr, $type: expr) => {
            assert_token_with_flags!(TokenFlags {
                operator: true,
                template: false,
                annexb: true,
                read_line: false,
            }, $str, $type);
        }
    }

    macro_rules! assert_token_annexb {
        ($str: expr, $type: expr) => {
            assert_token_with_flags!(TokenFlags {
                operator: false,
                template: false,
                annexb: true,
                read_line: false,
            }, $str, $type);
        }
    }

    macro_rules! assert_template {
        ($str: expr, $type: expr) => {
            assert_token_with_flags!(TokenFlags {
                operator: false,
                template: true,
                annexb: false,
                read_line: false,
            }, $str, $type);
        }
    }

    macro_rules! assert_token_with_flags {
        ($flags: expr, $str: expr, $type: expr) => {
            // println!("Testing: {} {:?}", $str, $type);

            assert_eq!(Tokenizer::with_flags($flags).parse($str), vec![
                Token {
                    tok: $type,
                },
            ]);
        }
    }

    // #[test]
    // fn it_should_run() {
    //     // let tokens = Tokenizer::parse("one;'foo';`foo`;0.3;08.2;`a\\u{123}c`;");
    //     let tokens = Tokenizer::parse("08.2;1.2e4;`a\\u{123}c`;");

    //     println!("{:#?}", tokens);
    // }


    #[test]
    fn it_should_tokenize_operators() {
        assert_operator!("{", TokenType::LCurly);
        assert_operator!("}", TokenType::RCurly);
        assert_operator!("(", TokenType::LParen);
        assert_operator!(")", TokenType::RParen);
        assert_operator!("[", TokenType::LSquare);
        assert_operator!("]", TokenType::RSquare);
        assert_operator!(";", TokenType::Semicolon);
        assert_operator!(",", TokenType::Comma);
        assert_operator!("~", TokenType::Tilde);
        assert_operator!("?", TokenType::Quest);
        assert_operator!(":", TokenType::Colon);
        assert_operator!(".", TokenType::Period);
        assert_operator!("...", TokenType::Ellipsis);
        assert_operator!("<", TokenType::LAngle);
        assert_operator!("<=", TokenType::LAngleEq);
        assert_operator!("<<", TokenType::LAngleAngle);
        assert_operator!("<<=", TokenType::LAngleAngleEq);
        assert_operator_annexb!("<!--", TokenType::LAngleExclamDashDash);
        assert_operator!(">", TokenType::RAngle);
        assert_operator!(">=", TokenType::RAngleEq);
        assert_operator!(">>", TokenType::RAngleAngle);
        assert_operator!(">>=", TokenType::RAngleAngleEq);
        assert_operator!(">>>", TokenType::RAngleAngleAngle);
        assert_operator!(">>>=", TokenType::RAngleAngleAngleEq);
        assert_operator!("!", TokenType::Exclam);
        assert_operator!("!=", TokenType::ExclamEq);
        assert_operator!("!==", TokenType::ExclamEqEq);
        assert_operator!("=", TokenType::Eq);
        assert_operator!("==", TokenType::EqEq);
        assert_operator!("===", TokenType::EqEqEq);
        assert_operator!("+", TokenType::Plus);
        assert_operator!("+=", TokenType::PlusEq);
        assert_operator!("++", TokenType::PlusPlus);
        assert_operator!("-", TokenType::Minus);
        assert_operator!("-=", TokenType::MinusEq);
        assert_operator!("--", TokenType::MinusMinus);
        assert_operator_annexb!("-->", TokenType::MinusMinusAngle);
        assert_operator!("%", TokenType::Mod);
        assert_operator!("%=", TokenType::ModEq);
        assert_operator!("*", TokenType::Star);
        assert_operator!("*=", TokenType::StarEq);
        assert_operator!("**", TokenType::StarStar);
        assert_operator!("**=", TokenType::StarStarEq);
        assert_operator!("/", TokenType::Div);
        assert_operator!("/=", TokenType::DivEq);
        assert_operator!("&", TokenType::Amp);
        assert_operator!("&=", TokenType::AmpEq);
        assert_operator!("&&", TokenType::AmpAmp);
        assert_operator!("|", TokenType::Bar);
        assert_operator!("|=", TokenType::BarEq);
        assert_operator!("||", TokenType::BarBar);
        assert_operator!("^", TokenType::Caret);
        assert_operator!("^=", TokenType::CaretEq);
    }

    #[test]
    fn it_should_tokenize_whitespace() {
        // Explicit ES whitespace chars
        assert_token!("\u{0009}", TokenType::Whitespace { value: '\u{0009}' });
        assert_token!("\u{000B}", TokenType::Whitespace { value: '\u{000B}' });
        assert_token!("\u{000C}", TokenType::Whitespace { value: '\u{000C}' });
        assert_token!("\u{0020}", TokenType::Whitespace { value: '\u{0020}' });
        assert_token!("\u{00A0}", TokenType::Whitespace { value: '\u{00A0}' });
        assert_token!("\u{FEFF}", TokenType::Whitespace { value: '\u{FEFF}' });

        // Unicode "Space_Separator" characters
        assert_token!("\u{1680}", TokenType::Whitespace { value: '\u{1680}' });
        assert_token!("\u{2000}", TokenType::Whitespace { value: '\u{2000}' });
        assert_token!("\u{2001}", TokenType::Whitespace { value: '\u{2001}' });
        assert_token!("\u{2002}", TokenType::Whitespace { value: '\u{2002}' });
        assert_token!("\u{2003}", TokenType::Whitespace { value: '\u{2003}' });
        assert_token!("\u{2004}", TokenType::Whitespace { value: '\u{2004}' });
        assert_token!("\u{2005}", TokenType::Whitespace { value: '\u{2005}' });
        assert_token!("\u{2006}", TokenType::Whitespace { value: '\u{2006}' });
        assert_token!("\u{2007}", TokenType::Whitespace { value: '\u{2007}' });
        assert_token!("\u{2008}", TokenType::Whitespace { value: '\u{2008}' });
        assert_token!("\u{2009}", TokenType::Whitespace { value: '\u{2009}' });
        assert_token!("\u{200A}", TokenType::Whitespace { value: '\u{200A}' });
        assert_token!("\u{202F}", TokenType::Whitespace { value: '\u{202F}' });
        assert_token!("\u{205F}", TokenType::Whitespace { value: '\u{205F}' });
        assert_token!("\u{3000}", TokenType::Whitespace { value: '\u{3000}' });
    }

    #[test]
    fn it_should_tokenize_newlines() {
        assert_token!("\u{000A}", TokenType::LineTerminator { value: "\u{000A}".into() });
        assert_token!("\u{000D}", TokenType::LineTerminator { value: "\u{000D}".into() });
        assert_token!("\u{2028}", TokenType::LineTerminator { value: "\u{2028}".into() });
        assert_token!("\u{2029}", TokenType::LineTerminator { value: "\u{2029}".into() });
    }

    #[test]
    fn it_should_tokenize_comments() {
        assert_token!("// text ", TokenType::LineComment { value: " text ".into() });
        assert_token!("/* text */", TokenType::BlockComment { value: " text ".into() });
        assert_token!("/* text **/", TokenType::BlockComment { value: " text *".into() });

        // TODO:
        // HTMLOpenComment {
        //     // The comment text, excluding the initial `<!--` and final newlines.
        //     value: String,
        // },
        // HTMLCloseComment {
        //     // The comment text, excluding the initial `-->` and final newlines.
        //     value: String,
        // },
    }

    #[test]
    fn it_should_tokenize_numbers() {
        // Decimal
        assert_token!(".2", TokenType::NumericLiteral {
            raw: Some(".2".into()),
            value: 0.2,
        });
        assert_token!("8.2", TokenType::NumericLiteral {
            raw: Some("8.2".into()),
            value: 8.2,
        });
        assert_token!("8.2e1", TokenType::NumericLiteral {
            raw: Some("8.2e1".into()),
            value: 82.0,
        });
        assert_token!("8.2e2", TokenType::NumericLiteral {
            raw: Some("8.2e2".into()),
            value: 820.0,
        });
        assert_token!("8.2E2", TokenType::NumericLiteral {
            raw: Some("8.2E2".into()),
            value: 820.0,
        });
        assert_token!("8.2e+2", TokenType::NumericLiteral {
            raw: Some("8.2e+2".into()),
            value: 820.0,
        });
        assert_token!("8.2E+2", TokenType::NumericLiteral {
            raw: Some("8.2E+2".into()),
            value: 820.0,
        });
        assert_token!("8.2e-2", TokenType::NumericLiteral {
            raw: Some("8.2e-2".into()),
            value: 0.082,
        });
        assert_token!("8.2E-2", TokenType::NumericLiteral {
            raw: Some("8.2E-2".into()),
            value: 0.082,
        });
        assert_token!("0.e2", TokenType::NumericLiteral {
            raw: Some("0.e2".into()),
            value: 0.0,
        });
        assert_token!("0.E2", TokenType::NumericLiteral {
            raw: Some("0.E2".into()),
            value: 0.0,
        });
        assert_token!("0.e-2", TokenType::NumericLiteral {
            raw: Some("0.e-2".into()),
            value: 0.0,
        });
        assert_token!("0.E2", TokenType::NumericLiteral {
            raw: Some("0.E2".into()),
            value: 0.0,
        });

        // Hex
        assert_token!("0x43", TokenType::NumericLiteral {
            raw: Some("0x43".into()),
            value: 67.0,
        });
        assert_token!("0X43", TokenType::NumericLiteral {
            raw: Some("0X43".into()),
            value: 67.0,
        });

        // Octal
        assert_token!("0o010", TokenType::NumericLiteral {
            raw: Some("0o010".into()),
            value: 8.0,
        });
        assert_token!("0O010", TokenType::NumericLiteral {
            raw: Some("0O010".into()),
            value: 8.0,
        });

        // Binary
        assert_token!("0b010", TokenType::NumericLiteral {
            raw: Some("0b010".into()),
            value: 2.0,
        });
        assert_token!("0B010", TokenType::NumericLiteral {
            raw: Some("0B010".into()),
            value: 2.0,
        });
    }

    #[test]
    fn it_should_tokenize_numbers_legacy_octal() {
        assert_token_annexb!("0333333333", TokenType::NumericLiteral {
            raw: Some("0333333333".into()),
            value: 0x36DB6DB as f64,
        });
        assert_token_annexb!("0778", TokenType::NumericLiteral {
            raw: Some("0778".into()),
            value: 778.0,
        });
    }

    #[test]
    fn it_should_tokenize_regexp_literal() {
        assert_token!("/omg/g", TokenType::RegularExpressionLiteral {
            value: "omg".into(),
            flags: "g".into(),
        });
        assert_token!("/o[/]mg/g", TokenType::RegularExpressionLiteral {
            value: "o[/]mg".into(),
            flags: "g".into(),
        });
    }

    #[test]
    fn it_should_tokenize_string_literal_single() {
        assert_token!("'omg'", TokenType::StringLiteral {
            raw: Some("omg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\\nmg'", TokenType::StringLiteral {
            raw: Some("o\\\nmg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\\rmg'", TokenType::StringLiteral {
            raw: Some("o\\\rmg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\\u{2028}mg'", TokenType::StringLiteral {
            raw: Some("o\\\u{2028}mg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\\u{2029}mg'", TokenType::StringLiteral {
            raw: Some("o\\\u{2029}mg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\\r\nmg'", TokenType::StringLiteral {
            raw: Some("o\\\r\nmg".into()),
            value: "omg".into(),
        });
        assert_token!("'o\\0mg'", TokenType::StringLiteral {
            raw: Some("o\\0mg".into()),
            value: "o\u{0}mg".into(),
        });
        assert_token!("'o\\x65mg'", TokenType::StringLiteral {
            raw: Some("o\\x65mg".into()),
            value: "o\u{65}mg".into(),
        });
        assert_token!("'o\\u0065mg'", TokenType::StringLiteral {
            raw: Some("o\\u0065mg".into()),
            value: "o\u{65}mg".into(),
        });
        assert_token!("'o\\u{65}mg'", TokenType::StringLiteral {
            raw: Some("o\\u{65}mg".into()),
            value: "o\u{65}mg".into(),
        });
    }

    #[test]
    fn it_should_tokenize_string_literal_single_legacy_octal() {
        assert_token_annexb!("'o\\18mg'", TokenType::StringLiteral {
            raw: Some("o\\18mg".into()),
            value: "o\u{1}8mg".into(),
        });
        assert_token_annexb!("'o\\1mg'", TokenType::StringLiteral {
            raw: Some("o\\1mg".into()),
            value: "o\u{1}mg".into(),
        });
        assert_token_annexb!("'o\\7mg'", TokenType::StringLiteral {
            raw: Some("o\\7mg".into()),
            value: "o\u{7}mg".into(),
        });
        assert_token_annexb!("'o\\17mg'", TokenType::StringLiteral {
            raw: Some("o\\17mg".into()),
            value: "o\u{F}mg".into(),
        });
        assert_token_annexb!("'o\\178mg'", TokenType::StringLiteral {
            raw: Some("o\\178mg".into()),
            value: "o\u{F}8mg".into(),
        });
        assert_token_annexb!("'o\\77mg'", TokenType::StringLiteral {
            raw: Some("o\\77mg".into()),
            value: "o\u{3F}mg".into(),
        });
        assert_token_annexb!("'o\\377mg'", TokenType::StringLiteral {
            raw: Some("o\\377mg".into()),
            value: "o\u{FF}mg".into(),
        });
        assert_token_annexb!("'o\\344mg'", TokenType::StringLiteral {
            raw: Some("o\\344mg".into()),
            value: "o\u{E4}mg".into(),
        });
        assert_token_annexb!("'o\\477mg'", TokenType::StringLiteral {
            raw: Some("o\\477mg".into()),
            value: "o\u{27}7mg".into(),
        });
        assert_token_annexb!("'o\\777mg'", TokenType::StringLiteral {
            raw: Some("o\\777mg".into()),
            value: "o\u{3F}7mg".into(),
        });
    }

    #[test]
    fn it_should_tokenize_string_literal_double() {
        assert_token!("\"omg\"", TokenType::StringLiteral {
            raw: Some("omg".into()),
            value: "omg".into(),
        });

        // TODO: Escape codes
    }

    #[test]
    fn it_should_tokenize_identifier() {
        assert_token!("omg", TokenType::IdentifierName {
            raw: Some("omg".into()),
            value: "omg".into(),
        });
        assert_token!("o\\u{1234}mg", TokenType::IdentifierName {
            raw: Some("o\\u{1234}mg".into()),
            value: "o\u{1234}mg".into(),
        });
        assert_token!("o\\u{65}mg", TokenType::IdentifierName {
            raw: Some("o\\u{65}mg".into()),
            value: "o\u{65}mg".into(),
        });
        assert_token!("o\\u1234mg", TokenType::IdentifierName {
            raw: Some("o\\u1234mg".into()),
            value: "o\u{1234}mg".into(),
        });
    }

    #[test]
    fn it_should_tokenize_template_literal() {
        assert_token!("`omg`", TokenType::TemplatePart {
            raw: Some("omg".into()),
            value: "omg".into(),
        });
        assert_token!("`omg${", TokenType::TemplatePart {
            raw: Some("omg".into()),
            value: "omg".into(),
        });
        assert_template!("}omg${", TokenType::TemplatePart {
            raw: Some("omg".into()),
            value: "omg".into(),
        });
        assert_template!("}omg`", TokenType::TemplatePart {
            raw: Some("omg".into()),
            value: "omg".into(),
        });

        // TODO: Escape codes
    }
}
