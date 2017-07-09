use ucd::Codepoint;
use std::iter::Peekable;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //tokenize("one;");
        let mut s = TokenStream::new("one;'foo';`foo`;0.3;08.2;".chars());

        for token in s {
            println!("Token: {:?}", token);
        }
    }
}

impl Default for TState {
    fn default() -> TState {
        TState::Start
    }
}

struct TokenStream<T: Iterator<Item = char>> {
    it: Peekable<T>,
    state: TState,
    flags: TokenFlags,
}

impl<T: Iterator<Item = char>> TokenStream<T> {
    fn new(it: T) -> TokenStream<T> {
        TokenStream {
            it: it.peekable(),
            state: TState::new(),
            flags: TokenFlags::new(true),
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for TokenStream<T> {
    type Item = TState;

    fn next(&mut self) -> Option<TState> {
        loop {
            {
                let state = self.state;
                let c = self.it.peek();

                match c {
                    Some(c) => {
                        // println!("Step: {}", *c);
                        self.state = self.state.step(*c, self.flags);

                        // println!("From {:?} to {:?}", state, self.state);

                        match self.state {
                            TState::Start => {
                                return Some(state);
                            }
                            _ => {}
                        }
                    }
                    None => {
                        match self.state {
                            TState::Start => {
                                return None;
                            }
                            _ => {
                                self.state = self.state.step('_', self.flags);
                                match self.state {
                                    TState::Start => {
                                        return Some(state);
                                    }
                                    _ => {
                                        panic!(
                                            "Unexpected end of content {:?} to {:?}",
                                            state,
                                            self.state
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match self.state {
                TState::Start => {}
                TState::Unknown => {
                    // TODO: Figure out a good storing for recoverable tokenization?
                    self.it.next();
                    return Some(TState::Unknown);
                }
                _ => {
                    self.it.next();
                }
            }
        }
    }
}


fn tokenize(c: &str) {
    let mut state = TState::new();
    let mut chars: Vec<char> = c.chars().collect();
    // let mut chars = c;

    let flags: TokenFlags = Default::default();

    let mut start = 0;
    let mut i = 0;
    while i < chars.len() {
        println!("=== {}", i);
        println!("char: {}", chars[i]);
        let next = state.step(chars[i], flags);

        match next {
            TState::Start => {
                println!("All done, {:?}", state);
                // Use last state to create token
            }
            TState::Unknown => panic!("syntax error"),
            _ => {
                println!("next, {:?}", next);
                i += 1;
            }
        }

        state = next;
    }

    // Push a last character in to see if we can flush out any last tokens.
    match state {
        TState::Start => {}
        _ => {
            let state = state.step('_', flags);
            match state {
                TState::Start => {}
                _ => panic!("Unexpected end of content"),
            }
        }
    }
}



fn is_ident_start(c: char) -> bool {
    c.is_id_start() || c == '$' || c == '_' || c == '\\'
}

fn is_ident_continue(c: char) -> bool {
    c.is_id_continue() || c == '$' || c == '_' || c == '\\' || c == '\u{200C}' || c == '\u{200D}'
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TState {
    Start,
    Unknown,

    MiscLineText,

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

    OperatorSlash,
    ExpressionSlash,
    SlashEq,

    Period,
    Dot,

    // Comments
    MultiLineComment,
    MultiLineCommentStar,
    MultiLineCommentStarSlash,
    SingleLineComment,

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

    // annex B
    // top-level
    // 0[0-7]+

    // in fraction
    // 0      [8-9]?[0-9]+
    // 0[0-7]+[8-9][0-9]*
    Integer,
    Decimal,
    Exponent,
    ExponentSign,
    ExponentDigit,
    LegacyOctal,

    // String parsing
    DChars,
    DCharEnd,
    SChars,
    SCharEnd,

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
    TemplateCharEnd,

    EscapeSequenceOrContinuation(EscapeReturnState),
    EscapeSequenceLegacyOctal1(EscapeReturnState),
    EscapeSequenceLegacyOctal2(EscapeReturnState),
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EscapeReturnState {
    IdentifierPart,
    SingleString,
    DoubleString,
    TemplateChar,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct TokenFlags {
    operator: bool,
    template: bool,
    annexb: bool,

    read_line: bool
}

impl TokenFlags {
    fn new(annexb: bool) -> TokenFlags {
        TokenFlags {
            operator: false,
            template: false,
            annexb: annexb,
            read_line: false,
        }
    }
}

impl Default for TokenFlags {
    fn default() -> TokenFlags {
        TokenFlags::new(false)
    }
}

impl TState {
    pub fn new() -> TState {
        TState::Start
    }

    pub fn step(&self, c: char, flags: TokenFlags) -> TState {
        match *self {
            TState::Unknown => TState::Start,
            TState::Start => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::LineTerminator,
                    _ if flags.read_line => TState::MiscLineText,

                    // Spec explicit whitespace whitelist.
                    '\u{9}' | '\u{B}' | '\u{C}' | '\u{20}' | '\u{A0}' | '\u{FEFF}' => {
                        TState::Whitespace
                    }
                    // Unicode "Space_Separator" characters
                    '\u{1680}' |
                    '\u{2000}'...'\u{200A}' |
                    '\u{202F}' |
                    '\u{205F}' |
                    '\u{3000}' => TState::Whitespace,
                    '(' => TState::LParen,
                    ')' => TState::RParen,
                    '{' => TState::LCurly,
                    '}' => {
                        if flags.template {
                            TState::TemplateChars
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
                    '`' => TState::TemplateChars,
                    '.' => TState::Period,
                    '0' => TState::Zero,
                    '1'...'9' => TState::Integer,
                    '"' => TState::DChars,
                    '\'' => TState::SChars,

                    '\\' => TState::EscapeSequence(EscapeReturnState::IdentifierPart),
                    c if is_ident_start(c) => TState::Ident,
                    _ => TState::MiscLineText,
                }
            }

            TState::MiscLineText => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Start,
                    _ => TState::MiscLineText,
                }
            }

            TState::LParen => TState::Start,
            TState::RParen => TState::Start,
            TState::LCurly => TState::Start,
            TState::LSquare => TState::Start,
            TState::RSquare => TState::Start,
            TState::Semicolon => TState::Start,
            TState::Comma => TState::Start,
            TState::Tilde => TState::Start,
            TState::Quest => TState::Start,
            TState::Colon => TState::Start,
            TState::Whitespace => TState::Start,
            TState::LineTerminator => TState::Start,
            TState::RCurly => TState::Start,

            TState::LAngle => {
                match c {
                    '<' => TState::LAngleAngle,
                    '=' => TState::LAngleEq,
                    _ => TState::Start,
                }
            }
            TState::LAngleEq => TState::Start,
            TState::LAngleAngle => {
                match c {
                    '=' => TState::LAngleAngleEq,
                    _ => TState::Start,
                }
            }
            TState::LAngleAngleEq => TState::Start,
            TState::RAngle => {
                match c {
                    '>' => TState::RAngleAngle,
                    '=' => TState::RAngleEq,
                    _ => TState::Start,
                }
            }
            TState::RAngleEq => TState::Start,
            TState::RAngleAngle => {
                match c {
                    '>' => TState::RAngleAngleAngle,
                    '=' => TState::RAngleAngleEq,
                    _ => TState::Start,
                }
            }
            TState::RAngleAngleEq => TState::Start,
            TState::RAngleAngleAngle => {
                match c {
                    '=' => TState::RAngleAngleEq,
                    _ => TState::Start,
                }
            }
            TState::RAngleAngleAngleEq => TState::Start,
            TState::Exclam => {
                match c {
                    '=' => TState::ExclamEq,
                    _ => TState::Start,
                }
            }
            TState::ExclamEq => {
                match c {
                    '=' => TState::ExclamEqEq,
                    _ => TState::Start,
                }
            }
            TState::ExclamEqEq => TState::Start,
            TState::Eq => {
                match c {
                    '=' => TState::EqEq,
                    _ => TState::Start,
                }
            }
            TState::EqEq => {
                match c {
                    '=' => TState::EqEqEq,
                    _ => TState::Start,
                }
            }
            TState::EqEqEq => TState::Start,
            TState::Plus => {
                match c {
                    '+' => TState::PlusPlus,
                    '=' => TState::PlusEq,
                    _ => TState::Start,
                }
            }
            TState::PlusPlus => TState::Start,
            TState::PlusEq => TState::Start,
            TState::Minus => {
                match c {
                    '-' => TState::MinusMinus,
                    '=' => TState::MinusEq,
                    _ => TState::Start,
                }
            }
            TState::MinusMinus => TState::Start,
            TState::MinusEq => TState::Start,
            TState::Mod => {
                match c {
                    '=' => TState::ModEq,
                    _ => TState::Start,
                }
            }
            TState::ModEq => TState::Start,
            TState::Star => {
                match c {
                    '*' => TState::StarStar,
                    '=' => TState::StarEq,
                    _ => TState::Start,
                }
            }
            TState::StarEq => TState::Start,
            TState::StarStar => {
                match c {
                    '=' => TState::StarEq,
                    _ => TState::Start,
                }
            }
            TState::StarStarEq => TState::Start,
            TState::Amp => {
                match c {
                    '=' => TState::AmpEq,
                    '&' => TState::AmpAmp,
                    _ => TState::Start,
                }
            }
            TState::AmpEq => TState::Start,
            TState::AmpAmp => TState::Start,
            TState::Bar => {
                match c {
                    '=' => TState::BarEq,
                    '|' => TState::BarBar,
                    _ => TState::Start,
                }
            }
            TState::BarEq => TState::Start,
            TState::BarBar => TState::Start,
            TState::Caret => {
                match c {
                    '=' => TState::CaretEq,
                    _ => TState::Start,
                }
            }
            TState::CaretEq => TState::Start,
            TState::OperatorSlash => {
                match c {
                    '/' => TState::SingleLineComment,
                    '*' => TState::MultiLineComment,
                    '=' => TState::SlashEq,
                    _ => TState::Start,
                }
            }
            TState::SlashEq => TState::Start,
            TState::MultiLineComment => {
                match c {
                    '*' => TState::MultiLineCommentStar,
                    _ => TState::MultiLineComment,
                }
            }
            TState::MultiLineCommentStar => {
                match c {
                    '*' => TState::MultiLineCommentStar,
                    '/' => TState::MultiLineCommentStarSlash,
                    _ => TState::MultiLineComment,
                }
            }
            TState::MultiLineCommentStarSlash => TState::Start,

            TState::SingleLineComment => {
                match c {
                    // Line terminator chars
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Start,
                    _ => TState::SingleLineComment,
                }
            }
            TState::Ident => {
                match c {
                    '\\' => TState::EscapeSequence(EscapeReturnState::IdentifierPart),
                    c if is_ident_continue(c) => TState::Ident,
                    _ => TState::Start,
                }
            }

            TState::Period => {
                match c {
                    '0'...'9' => TState::Integer,
                    _ => TState::Dot,
                }
            }
            TState::Dot => TState::Start,

            TState::Zero => {
                match c {
                    '.' => TState::Decimal,

                    'h' | 'H' => TState::Hex,
                    'o' | 'O' => TState::Octal,
                    'b' | 'B' => TState::Binary,

                    'e' | 'E' => TState::ExponentSign,
                    '0'...'7' if flags.annexb => TState::LegacyOctal,
                    '8' | '9' if flags.annexb => TState::Integer,
                    _ => {
                        TState::Start
                    }
                }
            }
            TState::Hex => {
                match c {
                    '0'...'9' => TState::Hex,
                    'a'...'f' => TState::Hex,
                    'A'...'F' => TState::Hex,
                    _ => TState::Start,
                }
            }
            TState::Octal => {
                match c {
                    '0'...'7' => TState::Octal,
                    _ => TState::Start,
                }
            }
            TState::Binary => {
                match c {
                    '0' | '1' => TState::Binary,
                    _ => TState::Start,
                }
            }

            TState::LegacyOctal => {
                match c {
                    '0'...'7' => TState::LegacyOctal,
                    '8' | '9' => TState::Integer,
                    _ => TState::Start,
                }
            }

            TState::Integer => {
                match c {
                    '0'...'9' => TState::Integer,
                    '.' => TState::Decimal,
                    'e' | 'E' => TState::ExponentSign,
                    _ => TState::Start,
                }
            }
            TState::Decimal => {
                match c {
                    '0'...'9' => TState::Decimal,
                    'e' | 'E' => TState::ExponentSign,
                    _ => TState::Start,
                }
            }
            TState::ExponentSign => {
                match c {
                    '+' | '-' => TState::Exponent,
                    '0'...'9' => TState::ExponentDigit,
                    _ => TState::Unknown,
                }
            }
            TState::Exponent => {
                match c {
                    '0'...'9' => TState::ExponentDigit,
                    _ => TState::Unknown,
                }
            }
            TState::ExponentDigit => {
                match c {
                    '0'...'9' => TState::ExponentDigit,
                    _ => TState::Start,
                }
            }
            TState::DChars => {
                match c {
                    '"' => TState::DCharEnd,
                    '\\' => TState::EscapeSequenceOrContinuation(EscapeReturnState::DoubleString),
                    _ => TState::DChars,
                }
            }
            TState::DCharEnd => TState::Start,

            TState::SChars => {
                match c {
                    '\'' => TState::SCharEnd,
                    '\\' => TState::EscapeSequenceOrContinuation(EscapeReturnState::SingleString),
                    _ => TState::SChars,
                }
            }
            TState::SCharEnd => TState::Start,

            TState::ExpressionSlash => {
                match c {
                    '/' => TState::SingleLineComment,
                    '*' => TState::MultiLineComment,

                    '[' => TState::RegexClassChars,
                    '\\' => TState::RegexEscapedChars,

                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::RegexChars,
                }
            }
            TState::RegexChars => {
                match c {
                    '/' => TState::RegexFlags,
                    '[' => TState::RegexClassChars,
                    '\\' => TState::RegexEscapedChars,

                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::RegexChars,
                }
            }
            TState::RegexClassChars => {
                match c {
                    '/' => TState::RegexFlags,
                    ']' => TState::RegexChars,
                    '\\' => TState::RegexClassEscapedChars,
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::RegexClassChars,
                }
            }

            TState::RegexClassEscapedChars => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::RegexClassChars,
                }
            }
            TState::RegexEscapedChars => {
                match c {
                    '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::RegexChars,
                }
            }
            TState::RegexFlags => {
                if is_ident_continue(c) {
                    TState::RegexFlags
                } else {
                    TState::Start
                }
            }

            TState::TemplateChars => {
                match c {
                    '`' => TState::TemplateCharEnd,
                    '$' => TState::TemplateDollarChar,
                    '\\' => TState::EscapeSequenceOrContinuation(EscapeReturnState::TemplateChar),
                    '\r' => TState::TemplateCharLineTerminator,
                    _ => TState::TemplateChars,
                }
            }
            TState::TemplateDollarChar => {
                match c {
                    '`' => TState::TemplateCharEnd,
                    '{' => TState::TemplateCharEnd,
                    '$' => TState::TemplateDollarChar,
                    '\\' => TState::EscapeSequenceOrContinuation(EscapeReturnState::TemplateChar),
                    '\r' => TState::TemplateCharLineTerminator,
                    _ => TState::TemplateChars,
                }
            }

            TState::TemplateCharLineTerminator => {
                match c {
                    '\n' => TState::TemplateChars,
                    '$' => TState::TemplateDollarChar,
                    '\\' => TState::EscapeSequenceOrContinuation(EscapeReturnState::TemplateChar),
                    '\r' => TState::TemplateCharLineTerminator,
                    _ => TState::TemplateChars,
                }
            }

            TState::TemplateCharEnd => TState::Start,

            // TODO: Escape sequences aren't working properly
            TState::EscapeSequenceOrContinuation(ref next) => {
                match c {
                    '\n' | '\u{2028}' | '\u{2029}' => TState::EscapeSequenceDone(*next),
                    '\r' => TState::EscapeSequenceMaybeContinuationSequence(*next),

                    'u' => TState::EscapeSequenceUnicode(*next),
                    'x' => TState::EscapeSequenceHex1(*next),
                    '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                        TState::EscapeSequenceDone(*next)
                    }
                    '1'...'9' => TState::Unknown,
                    _ => TState::EscapeSequenceDone(*next),
                }
            }
            TState::EscapeSequenceMaybeContinuationSequence(ref next) => {
                match c {
                    '\n' => TState::EscapeSequenceDone(*next),
                    _ => {
                        // TODO: Make escaped items their own tokens to make decoding easier
                        // _ => TState::EscapeSequenceDone(*next), // no-consume
                        match *next {  
                            EscapeReturnState::IdentifierPart => TState::Ident,
                            EscapeReturnState::SingleString => TState::SChars,
                            EscapeReturnState::DoubleString => TState::DChars,
                            EscapeReturnState::TemplateChar => TState::TemplateChars,
                        }
                    }
                }
            }

            TState::EscapeSequence(ref next) => {
                match c {
                    'u' => TState::EscapeSequenceUnicode(*next),
                    'x' => TState::EscapeSequenceHex1(*next),
                    '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                        TState::EscapeSequenceDone(*next)
                    }

                    '0'...'3' if flags.annexb => TState::EscapeSequenceLegacyOctal1(*next),
                    '4'...'7' if flags.annexb => TState::EscapeSequenceLegacyOctal2(*next),

                    '1'...'9' => TState::Unknown,
                    '\r' | '\n' | '\u{2028}' | '\u{2029}' => TState::Unknown,
                    _ => TState::EscapeSequenceDone(*next),
                }
            }

            TState::EscapeSequenceLegacyOctal1(ref next) => {
                match c {
                    '0'...'7' => TState::EscapeSequenceLegacyOctal2(*next),
                    _ => TState::Start, // no-consume
                }
            }
            TState::EscapeSequenceLegacyOctal2(ref next) => {
                match c {
                    '0'...'7' => TState::EscapeSequenceDone(*next),
                    _ => TState::Start, // no-consume
                }
            }
            TState::EscapeSequenceUnicode(ref next) => {
                match c {
                    '{' => TState::EscapeSequenceUnicodeHex(*next),
                    _ => TState::EscapeSequenceUnicodeHex1(*next), // no-consume
                }
            }
            TState::EscapeSequenceUnicodeHex(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceUnicodeHex(*next),
                    '}' => TState::EscapeSequenceDone(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceUnicodeHex1(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceUnicodeHex2(*next),
                    _ => TState::Unknown,
                }
            }

            TState::EscapeSequenceUnicodeHex2(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceUnicodeHex3(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceUnicodeHex3(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceUnicodeHex4(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceUnicodeHex4(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceDone(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceHex1(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceHex2(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceHex2(ref next) => {
                match c {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => TState::EscapeSequenceDone(*next),
                    _ => TState::Unknown,
                }
            }
            TState::EscapeSequenceDone(ref next) => {
                match *next {
                    EscapeReturnState::IdentifierPart => TState::Ident,
                    EscapeReturnState::SingleString => TState::SChars,
                    EscapeReturnState::DoubleString => TState::DChars,
                    EscapeReturnState::TemplateChar => TState::TemplateChars,
                }
            }
        }
    }
}
