use ucd::Codepoint;

fn is_ident_start(c: char) -> bool {
    c.is_id_start() || c == '$' || c == '_' || c == '\\'
}

fn is_ident_continue(c: char) -> bool {
    c.is_id_continue() || c == '$' || c == '_' || c == '\\' || c == '\u{200C}' || c == '\u{200D}'
}

#[derive(Clone, Copy)]
enum TokenState {
    Start,
    Unknown,

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
    Whitespace,
    LineTerminator,

    // multi-char states
    LAngle,
    LAngleEq,
    LAngleAngle,
    LAngleAngleEq,
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
    Slash,
    SlashEq,


    Period,


    // Comments
    MultiLineComment,
    MultiLineCommentStar,
    SingleLineComment,

    IdentStart,
    Ident,

    Zero,

	// 0[xX][0-9a-fA-F]+
	// 0[oO][0-7]+
	// 0[bB][0-1]+
    Hex,
    Octal,
    Binary,

	// 0       (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
	// [0-9]+  (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
	//         (\.[0-9]+)      ([eE][+-]?[0-9]+)?
    Integer,
    Decimal,
    Exponent,
    ExponentSign,
    ExponentDigit,


    // String parsing
    DChars,
    SChars,

    // Regex parsing
    RegexChars,
    RegexClassChars,
    RegexClassEscapedChars,
    RegexEscapedChars,
    RegexFlags,

    // Template Literal parsing
    TemplateChars,
    TemplateDollarChar,
    TemplateCharLineTerminator,

    EscapeSequenceOrContinuation(EscapeReturnState),
    EscapeSequenceMaybeContinuationSequence(EscapeReturnState),
    EscapeSequence(EscapeReturnState),
    EscapeSequenceUnicode(EscapeReturnState),
    EscapeSequenceUnicodeHex(EscapeReturnState),
    EscapeSequenceUnicodeHex1(EscapeReturnState),
    EscapeSequenceUnicodeHex2(EscapeReturnState),
    EscapeSequenceUnicodeHex3(EscapeReturnState),
    EscapeSequenceUnicodeHex4(EscapeReturnState),
    EscapeSequenceHex1(EscapeReturnState),
    EscapeSequenceHex2(EscapeReturnState),
    EscapeSequenceDone(EscapeReturnState),
}

#[derive(Clone, Copy)]
enum EscapeReturnState {
    IdentifierPart,
    SingleString,
    DoubleString,
    TemplateChar,
}

enum Step {
    None,
    Discard,
    Append,
}

impl TokenState {
    fn step(&self, c: char) -> TokenState {
        match *self {
            TokenState::Unknown => {
                TokenState::Start
            }
            TokenState::Start => {
                match c {
                    // Spec explicit whitespace whitelist.
                    '\u{9}' | '\u{B}' | '\u{C}' | '\u{20}' | '\u{A0}' | '\u{FEFF}' => TokenState::Whitespace,
                    // Unicode "Space_Separator" characters
                    '\u{1680}' | '\u{2000}'...'\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => TokenState::Whitespace,
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::LineTerminator,
                    '(' => TokenState::LParen,
                    ')' => TokenState::RParen,
                    '{' => TokenState::LCurly,
                    '}' => TokenState::RCurly,
                    '[' => TokenState::LSquare,
                    ']' => TokenState::RSquare,
                    ';' => TokenState::Semicolon,
                    ',' => TokenState::Comma,
                    '~' => TokenState::Tilde,
                    '?' => TokenState::Quest,
                    ':' => TokenState::Colon,
                    '<' => TokenState::LAngle,
                    '>' => TokenState::RAngle,
                    '!' => TokenState::Exclam,
                    '=' => TokenState::Eq,
                    '+' => TokenState::Plus,
                    '-' => TokenState::Minus,
                    '%' => TokenState::Mod,
                    '*' => TokenState::Star,
                    '&' => TokenState::Amp,
                    '|' => TokenState::Bar,
                    '^' => TokenState::Caret,
                    '/' => TokenState::Slash,
                    '`' => TokenState::TemplateChars,
                    '.' => TokenState::Period,
                    '0'...'9' => TokenState::Integer,
                    '"' => TokenState::DChars,
                    '\'' => TokenState::SChars,
                    _ => TokenState::IdentStart,
                }
            }


            TokenState::LParen => TokenState::Start,
            TokenState::RParen => TokenState::Start,
            TokenState::LCurly => TokenState::Start,
            TokenState::LSquare => TokenState::Start,
            TokenState::RSquare => TokenState::Start,
            TokenState::Semicolon => TokenState::Start,
            TokenState::Comma => TokenState::Start,
            TokenState::Tilde => TokenState::Start,
            TokenState::Quest => TokenState::Start,
            TokenState::Colon => TokenState::Start,
            TokenState::Whitespace => TokenState::Start,
            TokenState::LineTerminator => TokenState::Start,

            TokenState::RCurly => {
                // TODO
                // if self.template {
                //     TokenState::TemplateChar
                // } else {
                    TokenState::Start
                // }
            }



            TokenState::LAngle => {
                match c {
                    '<' => TokenState::LAngleAngle,
                    '=' => TokenState::LAngleEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::LAngleEq => {
                TokenState::Start
            }
            TokenState::LAngleAngle => {
                match c {
                    '=' => TokenState::LAngleAngleEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::LAngleAngleEq => {
                TokenState::Start
            }
            TokenState::RAngle => {
                match c {
                    '>' => TokenState::RAngleAngle,
                    '=' => TokenState::RAngleEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::RAngleEq => {
                TokenState::Start
            }
            TokenState::RAngleAngle => {
                match c {
                    '>' => TokenState::RAngleAngleAngle,
                    '=' => TokenState::RAngleAngleEq,
                    _ => TokenState::Start,
                }

            }
            TokenState::RAngleAngleEq => {
                TokenState::Start
            }
            TokenState::RAngleAngleAngle => {
                match c {
                    '=' => TokenState::RAngleAngleEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::RAngleAngleAngleEq => {
                TokenState::Start
            }
            TokenState::Exclam => {
                match c {
                    '=' => TokenState::ExclamEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::ExclamEq => {
                match c {
                    '=' => TokenState::ExclamEqEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::ExclamEqEq => {
                TokenState::Start
            }
            TokenState::Eq => {
                match c {
                    '=' => TokenState::EqEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::EqEq => {
                match c {
                    '=' => TokenState::EqEqEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::EqEqEq => {
                TokenState::Start
            }
            TokenState::Plus => {
                match c {
                    '+' => TokenState::PlusPlus,
                    '=' => TokenState::PlusEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::PlusPlus => {
                TokenState::Start
            }
            TokenState::PlusEq => {
                TokenState::Start
            }
            TokenState::Minus => {
                match c {
                    '-' => TokenState::MinusMinus,
                    '=' => TokenState::MinusEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::MinusMinus => {
                TokenState::Start
            }
            TokenState::MinusEq => {
                TokenState::Start
            }
            TokenState::Mod => {
                match c {
                    '=' => TokenState::ModEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::ModEq => {
                TokenState::Start
            }
            TokenState::Star => {
                match c {
                    '*' => TokenState::StarStar,
                    '=' => TokenState::StarEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::StarEq => {
                TokenState::Start
            }
            TokenState::StarStar => {
                match c {
                    '=' => TokenState::StarEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::StarStarEq => {
                TokenState::Start
            }
            TokenState::Amp => {
                match c {
                    '=' => TokenState::AmpEq,
                    '&' => TokenState::AmpAmp,
                    _ => TokenState::Start,
                }
            }
            TokenState::AmpEq => {
                TokenState::Start
            }
            TokenState::AmpAmp => {
                TokenState::Start
            }
            TokenState::Bar => {
                match c {
                    '=' => TokenState::BarEq,
                    '|' => TokenState::BarBar,
                    _ => TokenState::Start,
                }
            }
            TokenState::BarEq => {
                TokenState::Start
            }
            TokenState::BarBar => {
                TokenState::Start
            }
            TokenState::Caret => {
                match c {
                    '=' => TokenState::CaretEq,
                    _ => TokenState::Start,
                }
            }
            TokenState::CaretEq => {
                TokenState::Start
            }
            TokenState::Slash => {
                match c {
                    '/' => TokenState::SingleLineComment,
                    '*' => TokenState::MultiLineComment,

                    // TODO
                    // '=' if !self.expression => TokenState::SlashEq,
                    // _ if self.expression => TokenState::RegexChars,
                    _ => TokenState::Start,
                }
            }
            TokenState::SlashEq => {
                TokenState::Start
            }
            TokenState::MultiLineComment => {
                match c {
                    '*' => TokenState::MultiLineCommentStar,
                    _ => TokenState::MultiLineComment,
                }
            }
            TokenState::MultiLineCommentStar => {
                match c {
                    '*' => TokenState::MultiLineCommentStar,
                    '/' => TokenState::Start,
                    _ => TokenState::MultiLineComment,
                }
            }
            TokenState::SingleLineComment => {
                match c {
                    // Line terminator chars
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::Start,
                    _ => TokenState::SingleLineComment,
                }
            }
            TokenState::IdentStart => {
                match c {
                    '\\' => TokenState::EscapeSequence(EscapeReturnState::IdentifierPart),
                    c if is_ident_start(c) => TokenState::Ident,
                    _ => TokenState::Start,
                }
            }
            TokenState::Ident => {
                match c {
                    '\\' => TokenState::EscapeSequence(EscapeReturnState::IdentifierPart),
                    c if is_ident_continue(c) => TokenState::Ident,
                    _ => TokenState::Start,
                }
            }

            TokenState::Period => {
                match c {
                    '0'...'9' => TokenState::Integer,
                    _ => TokenState::Start,
                }
            }

            TokenState::Zero => {
                match c {
                    '1'...'9' => TokenState::Integer,
                    '.' => TokenState::Decimal,

                    'h' | 'H' => TokenState::Hex,
                    'o' | 'O' => TokenState::Octal,
                    'b' | 'B' => TokenState::Binary,
                    _ => TokenState::Start,
                }
            }
            TokenState::Hex => {
                match c {
                    '0'...'9' => TokenState::Start,
                    'a'...'f' => TokenState::Start,
                    'A'...'F' => TokenState::Start,
                    _ => TokenState::Hex,
                }
            }
            TokenState::Octal => {
                match c {
                    '0'...'7' => TokenState::Start,
                    _ => TokenState::Octal,
                }
            }
            TokenState::Binary => {
                match c {
                    '0' | '1' => TokenState::Start,
                    _ => TokenState::Binary,
                }
            }

            TokenState::Integer => {
                match c {
                    '0'...'9' => TokenState::Integer,
                    '.' => TokenState::Decimal,
                    'e' | 'E' => TokenState::ExponentSign,
                    _ => TokenState::Start,
                }
            }
            TokenState::Decimal => {
                match c {
                    '0'...'9' => TokenState::Decimal,
                    'e' | 'E' => TokenState::ExponentSign,
                    _ => TokenState::Start,
                }
            }
            TokenState::ExponentSign => {
                match c {
                    '+' | '-' => TokenState::Exponent,
                    '0'...'9' => TokenState::Exponent, //no consume
                    _ => TokenState::Unknown,
                }

            }
            TokenState::Exponent => {
                match c {
                    '0'...'9' => TokenState::ExponentDigit,
                    _ => TokenState::Unknown,
                }
            }
            TokenState::ExponentDigit => {
                match c {
                    '0'...'9' => TokenState::ExponentDigit,
                    _ => TokenState::Start,
                }
            }
            TokenState::DChars => {
                match c {
                    '"' => TokenState::Start,
                    '\\' => TokenState::EscapeSequenceOrContinuation(EscapeReturnState::DoubleString),
                    _ => TokenState::DChars,
                }
            }

            TokenState::SChars => {
                match c {
                    '\'' => TokenState::Start,
                    '\\' => TokenState::EscapeSequenceOrContinuation(EscapeReturnState::SingleString),
                    _ => TokenState::DChars,
                }
            }

            TokenState::RegexChars => {
                match c {
                    '/' => TokenState::RegexFlags,
                    '[' => TokenState::RegexClassChars,
                    '\\' => TokenState::RegexEscapedChars,

                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::Unknown,
                    _ => TokenState::RegexChars,
                }
            }
            TokenState::RegexClassChars => {
                match c {
                    '/' => TokenState::RegexChars, // no-consume
                    ']' => TokenState::RegexChars,
                    '\\' => TokenState::RegexClassEscapedChars,
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::Unknown,
                    _ => TokenState::RegexClassChars,
                }
            }

            TokenState::RegexClassEscapedChars => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::Unknown,
                    _ => TokenState::RegexClassChars,
                }
            }
            TokenState::RegexEscapedChars => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TokenState::Unknown,
                    _ => TokenState::RegexChars,
                }
            }
            TokenState::RegexFlags => {
                if is_ident_continue(c) {
                    TokenState::RegexFlags
                } else {
                    TokenState::Start
                }
            }

            TokenState::TemplateChars => {
                match c {
                    '`' => TokenState::Start,
                    '$' => TokenState::TemplateDollarChar,
                    '\\' => TokenState::EscapeSequenceOrContinuation(EscapeReturnState::TemplateChar),
                    '\r' =>TokenState::TemplateCharLineTerminator,
                    _ => TokenState::TemplateChars,
                }
            }
            TokenState::TemplateDollarChar => {
                match c {
                    '{' => TokenState::Start,
                    _ => TokenState::TemplateChars, // no-consume
                }
            }

            TokenState::TemplateCharLineTerminator => {
                match c {
                    '\n' => TokenState::TemplateChars,
                    _ => TokenState::TemplateChars, // no-consume
                }
            }

            TokenState::EscapeSequenceOrContinuation(ref next) => {
                match c {
                    '\n' | '\u{2028}' | '\u{2029}' => TokenState::EscapeSequenceDone(*next),
                    '\r' => TokenState::EscapeSequenceMaybeContinuationSequence(*next),
                    _ => TokenState::EscapeSequence(*next), // no-consume
                }
            }
            TokenState::EscapeSequenceMaybeContinuationSequence(ref next) => {
                match c {
                    '\n' => TokenState::EscapeSequenceDone(*next),
                    _ => TokenState::EscapeSequenceDone(*next), // no-consume
                }
            }

            TokenState::EscapeSequence(ref next) => {
                match c {
                    'u' => TokenState::EscapeSequenceUnicode(*next),
                    'x' => TokenState::EscapeSequenceHex1(*next),
                    '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => TokenState::EscapeSequenceDone(*next),
                    '1'...'9' => TokenState::Unknown,
                    '\r' | '\n' | '\u{2028}' | '\u{2029}' => TokenState::Unknown,
                    _ => TokenState::EscapeSequenceDone(*next),
                }
            }

            TokenState::EscapeSequenceUnicode(ref next) => {
                match c {
                    '{' => TokenState::EscapeSequenceUnicodeHex(*next),
                    _ => TokenState::EscapeSequenceUnicodeHex1(*next),
                }
            }
            TokenState::EscapeSequenceUnicodeHex(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceUnicodeHex(*next),
                    '}' => TokenState::EscapeSequenceDone(*next),
                    _ => TokenState::Unknown,
                }
            }
            TokenState::EscapeSequenceUnicodeHex1(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceUnicodeHex2(*next),
                    _ => TokenState::Unknown,
                }
            }

            TokenState::EscapeSequenceUnicodeHex2(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceUnicodeHex3(*next),
                    _ => TokenState::Unknown,
                }

            }
            TokenState::EscapeSequenceUnicodeHex3(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceUnicodeHex4(*next),
                    _ => TokenState::Unknown,
                }

            }
            TokenState::EscapeSequenceUnicodeHex4(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceDone(*next),
                    _ => TokenState::Unknown,
                }
            }
            TokenState::EscapeSequenceHex1(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceHex2(*next),
                    _ => TokenState::Unknown,
                }

            }
            TokenState::EscapeSequenceHex2(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TokenState::EscapeSequenceDone(*next),
                    _ => TokenState::Unknown,
                }
            }
            TokenState::EscapeSequenceDone(ref next) => {
                match *next {
                    EscapeReturnState::IdentifierPart => TokenState::Ident,
                    EscapeReturnState::SingleString => TokenState::SChars,
                    EscapeReturnState::DoubleString => TokenState::DChars,
                    EscapeReturnState::TemplateChar => TokenState::TemplateChars,
                }
            }
        }
    }
}