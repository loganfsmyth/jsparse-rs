use std::borrow::{Cow, Borrow};
use tokenizer::tokens;
use tokenizer::tokens::{PunctuatorToken,
    TemplateFormat, CommentToken, CommentFormat};

use tokenizer::{Hint, IntoTokenizer, Tokenizer, Position, TokenRange};

#[derive(Debug)]
pub struct SliceTokenizer<'code> {
    code: &'code str,
    position: Position,

    template_stack: Vec<bool>,
}

impl<'code> Clone for SliceTokenizer<'code> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl<'code> Tokenizer<'code> for SliceTokenizer<'code> {
    fn next_token(&mut self, hint: &Hint) -> (tokens::Token<'code>, TokenRange) {
        let start = self.position;

        let code_s: &'code str = self.code.borrow();

        let s = &code_s[self.position.offset..];

        let result: TokenResult<'code> = read_next(s, hint);
        let TokenResult(token, size) = result;

        if let Some((byte_step, _)) = s.char_indices().skip(size.chars).next() {
            self.position.offset += byte_step;
        } else {
            self.position.offset = code_s.len();
        }

        if size.lines == 0 {
            self.position.column += size.width;
        } else {
            self.position.line += size.lines;
            self.position.column = size.width;
        }

        let range = TokenRange {
            start,
            end: self.position,
        };

        (token, range)
    }
}

impl<'code> IntoTokenizer<'code> for &'code str {
    type Item = SliceTokenizer<'code>;

    fn into_tokenizer(self) -> Self::Item {
        SliceTokenizer {
            code: self,
            position: Default::default(),
            template_stack: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenResult<'code>(tokens::Token<'code>, TokenSize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSize {
  chars: usize,
  lines: usize,
  width: usize,
}

fn single_size(size: usize) -> TokenSize {
  TokenSize {
    chars: size,
    lines: 0,
    width: size,
  }
}

fn punc(tok: tokens::PunctuatorToken, size: usize) -> TokenResult<'static> {
  TokenResult(tokens::Token::Punctuator(tok), single_size(size))
}

fn number<'a>(tok: f64, raw: Cow<'a, str>) -> TokenResult<'a> {
    let len = raw.len();
    TokenResult(
        tokens::NumericLiteralToken {
            value: tok
        }.into(),
        single_size(len),
    )
}
fn string<'a>(tok: Cow<'a, str>, raw: Cow<'a, str>) -> TokenResult<'a> {
    let len = raw.len();
    TokenResult(
        tokens::StringLiteralToken {
            value: tok
        }.into(),
        single_size(len),
    )
}

fn template<'a>(tok: Cow<'a, str>, raw: Cow<'a, str>, format: TemplateFormat) -> TokenResult<'a> {
    let len = tok.chars().count() + match format {
        TemplateFormat::NoSubstitution => 2,
        TemplateFormat::Head => 3,
        TemplateFormat::Middle => 3,
        TemplateFormat::Tail => 2,
    };

    TokenResult(
        tokens::TemplateToken {
            format,
            raw,
            cooked: tok
        }.into(),
        single_size(len),
    )
}

fn comment<'a>(tok: Cow<'a, str>, format: CommentFormat) -> TokenResult<'a> {
    let mut chars = tok.chars().count();
    let mut lines = 0;
    let mut width = 0;
    match format {
        CommentFormat::Line => {
            chars += 2;
            width = chars
        }
        CommentFormat::Block => {
            chars += 4;
            width = 4;

            let mut saw_cr = false;
            for c in tok.chars() {
                match c {
                    '\r' => {
                        saw_cr = true;
                        lines += 1;
                        width = 2;
                    }
                    '\n' => {
                        if !saw_cr {
                            lines += 1;
                            width = 2;
                            saw_cr = false;
                        }
                    }
                    '\u{2028}' | '\u{2029}' => {
                        lines += 1;
                        width = 2;
                        saw_cr = false;
                    }
                    _ => {
                        width += 1;
                        saw_cr = false;
                    }
                }
            }
        }
        _ => unimplemented!("unsupported comment format"),
    }

    TokenResult(
        CommentToken {
            format,
            value: tok
        }.into(),
        TokenSize {
            chars,
            lines,
            width,
        }
    )
}

pub fn read_next<'a>(code: &'a str, hint: &Hint) -> TokenResult<'a> {
    // loop, eating whitespace chars?

    let bytes = code.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return TokenResult (
            tokens::EOFToken {}.into(),
            TokenSize {
                chars: 0,
                lines: 0,
                width: 0,
            }
        );
    }

    let index = 0;
    match bytes[index] {
        b'{' => punc(PunctuatorToken::CurlyOpen, 1),
        b'(' => punc(PunctuatorToken::ParenOpen, 1),
        b')' => punc(PunctuatorToken::ParenClose, 1),
        b'[' => punc(PunctuatorToken::SquareOpen, 1),
        b']' => punc(PunctuatorToken::SquareClose, 1),
        b';' => punc(PunctuatorToken::Semicolon, 1),
        b',' => punc(PunctuatorToken::Comma, 1),
        b'?' => punc(PunctuatorToken::Question, 1),
        b':' => punc(PunctuatorToken::Colon, 1),
        b'~' => punc(PunctuatorToken::Tilde, 1),

        b'.' => {
            if index + 2 < len && bytes[index + 1] == b'.' && bytes[index + 2] == b'.' {
                punc(PunctuatorToken::Ellipsis, 3)
            } else if index + 1 < len && bytes[index + 1] >= b'0' && bytes[index + 1] <= b'9' {

                let mut val = 0f64;
                let mut offset = 1;

                let (frac, num) = parse_decimal_digits(&bytes[offset..]);

                val += frac;
                offset += num;

                let (exp, num) = parse_exponent(&bytes[offset..]);

                if num != 0 {
                    val = val * 10f64.powi(exp);
                    offset += num;
                }

                return number(val, code[0..offset].into())
            } else {
                punc(PunctuatorToken::Period, 1)
            }
        }
        b'<' => {
            if index + 1 < len && bytes[index + 1] == b'<' {
                if index + 2 < len && bytes[index + 2] == b'=' {
                    punc(PunctuatorToken::LAngleAngleEq, 3)
                } else {
                    punc(PunctuatorToken::LAngleAngle, 2)
                }
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::LAngleEq, 2)
            } else {
                punc(PunctuatorToken::LAngle, 1)
            }
        }
        b'>' => {
            if index + 1 < len && bytes[index + 1] == b'>' {
                if index + 2 < len && bytes[index + 2] == b'>' {
                    if index + 3 < len && bytes[index + 3] == b'=' {
                        punc(PunctuatorToken::RAngleAngleAngleEq, 4)
                    } else {
                        punc(PunctuatorToken::RAngleAngleAngle, 3)
                    }
                } else if index + 2 < len && bytes[index + 2] == b'=' {
                    punc(PunctuatorToken::RAngleAngleEq, 3)
                } else {
                    punc(PunctuatorToken::RAngleAngle, 2)
                }
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::RAngleEq, 2)
            } else {
                punc(PunctuatorToken::RAngle, 1)
            }
        }
        b'=' => {
            if index + 1 < len && bytes[index + 1] == b'=' {
                if index + 2 < len && bytes[index + 2] == b'=' {
                    punc(PunctuatorToken::EqEqEq, 3)
                } else {
                    punc(PunctuatorToken::EqEq, 2)
                }
            } else if index + 1 < len && bytes[index + 1] == b'>' {
                punc(PunctuatorToken::Arrow, 2)
            } else {
                punc(PunctuatorToken::Eq, 1)
            }
        }
        b'!' => {
            if index + 1 < len && bytes[index + 1] == b'=' {
                if index + 2 < len && bytes[index + 2] == b'=' {
                    punc(PunctuatorToken::ExclamEqEq, 3)
                } else {
                    punc(PunctuatorToken::ExclamEq, 2)
                }
            } else {
                punc(PunctuatorToken::Exclam, 1)
            }
        }
        b'+' => {
            if index + 1 < len && bytes[index + 1] == b'+' {
                punc(PunctuatorToken::PlusPlus, 2)
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::PlusEq, 2)
            } else {
                punc(PunctuatorToken::Plus, 1)
            }
        }
        b'-' => {
            if index + 1 < len && bytes[index + 1] == b'-' {
                if index + 2 < len && bytes[index + 2] == b'>' {
                    punc(PunctuatorToken::MinusMinusAngle, 3)
                } else {
                    punc(PunctuatorToken::MinusMinus, 2)
                }
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::MinusEq, 2)
            } else {
                punc(PunctuatorToken::Minus, 1)
            }
        }
        b'&' => {
            if index + 1 < len && bytes[index + 1] == b'&' {
                punc(PunctuatorToken::AmpAmp, 2)
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::AmpEq, 2)
            } else {
                punc(PunctuatorToken::Amp, 1)
            }
        }
        b'|' => {
            if index + 1 < len && bytes[index + 1] == b'|' {
                punc(PunctuatorToken::BarBar, 2)
            } else  if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::BarEq, 2)
            } else {
                punc(PunctuatorToken::Bar, 1)
            }
        }
        b'^' => {
            if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::CaretEq, 2)
            } else {
                punc(PunctuatorToken::Caret, 1)
            }
        }
        b'%' => {
            if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::PercentEq, 2)
            } else {
                punc(PunctuatorToken::Percent, 1)
            }
        }
        b'*' => {
            if index + 1 < len && bytes[index + 1] == b'*' {
                if index + 2 < len && bytes[index + 2] == b'=' {
                    punc(PunctuatorToken::StarStarEq, 3)
                } else {
                    punc(PunctuatorToken::StarStar, 2)
                }
            } else if index + 1 < len && bytes[index + 1] == b'=' {
                punc(PunctuatorToken::StarEq, 2)
            } else {
                punc(PunctuatorToken::Star, 1)
            }
        }
        b'/' => {
            if index + 1 < len && bytes[index + 1] == b'/' {
                let mut end = index + 2;
                for (i, c) in code.char_indices().skip(2) {
                    match c {
                        '\r' | '\n' | '\u{2028}' | '\u{2029}' => {
                            end = i;
                            break;
                        }
                        _ => {
                        }
                    }
                }
                if end == index + 2 {
                    end = code.len();
                }

                comment(Cow::from(&code[index + 2..end]), CommentFormat::Line)
            } else if index + 1 < len && bytes[index + 1] == b'*' {
                let mut end = index + 2;

                let mut break_slash = false;
                for (i, c) in code.char_indices().skip(2) {
                    match c {
                        '/' if break_slash => {
                            end = i - 1;
                        }
                        '*' => {
                            break_slash = true;
                        }
                        _ => {
                            break_slash = false;
                        }
                    }
                }

                comment(Cow::from(&code[index + 2..end]), CommentFormat::Block)
            } else if hint.expression {
                let mut end = index + 1;

                let mut in_class = false;
                for (i, c) in code.char_indices().skip(2) {
                    match c {
                        '/' if !in_class => {
                            end = i;
                            break;
                        }
                        '[' if !in_class => {
                            in_class = true;
                        }
                        ']' if in_class => {
                            in_class = false;
                        }
                        _ => {},
                    }
                }
                if end == index + 1 {
                    unimplemented!("unterminated regex");
                }

                // println!("end: {}", end + 1);

                let mut flag_end = end + 1;
                for (i, c) in (&code[flag_end..]).char_indices() {
                    // println!("{:?}", (i, c));
                    match c {
                        'a'...'z' => {
                            flag_end = (end + 1) + (i + 1);
                            // println!("Set {}", flag_end);
                        }
                        _ => break,
                    }
                }

                // println!("{}, {}, {}", index, end, flag_end);

                return TokenResult(
                    tokens::RegularExpressionLiteralToken {
                        pattern: (&code[index + 1..end]).into(),
                        flags: (&code[end + 1..flag_end]).into()
                    }.into(),
                    single_size(flag_end - index),
                );
            } else {
                if index + 1 < len && bytes[index + 1] == b'=' {
                    punc(PunctuatorToken::SlashEq, 2)
                } else {
                    punc(PunctuatorToken::Slash, 1)
                }
            }
        }
        b'}' => {
            if hint.template {
                let mut break_curly = false;

                let mut val = String::new();

                for (_i, c) in code.char_indices().skip(1) {
                    match c {
                        '{' if  break_curly => {
                            return template(val.clone().into(), val.into(), TemplateFormat::Middle);
                        }
                        '`' => {
                            return template(val.clone().into(), val.into(), TemplateFormat::Tail);
                        }
                        '$' => {
                            break_curly = true;
                        }
                        _ => {
                            break_curly = false;

                            val.push(c);
                        }
                    }
                }

                unimplemented!("template tail")
            } else {
                punc(PunctuatorToken::CurlyClose, 1)
            }
        }

        t @ b'\'' | t @ b'\"' => {
            let mut pieces = vec![];

            let mut start = index;
            let mut end = start;

            let mut s: usize = index + 1;
            for (i, c) in code.char_indices().skip(1) {
                // TODO: Build state transition tables for parsing escapes

                match c {
                    '\\' => {
                        // pieces.push(Cow::from(&code[s..i]));
                        // start = i + 1;

                        unimplemented!("escape")
                    }
                    '\"' if t == b'\"' => {
                        pieces.push(Cow::from(&code[s..i]));
                        end = i + 1;
                        break;
                    },
                    '\'' if t == b'\'' => {
                        pieces.push(Cow::from(&code[s..i]));
                        end = i + 1;
                        break;
                    },
                    '\r' | '\n' | '\u{2028}' | '\u{2029}' => {
                        // Invalid string
                        unimplemented!("string with newlines")
                    }
                    _ => { }
                }
            }

            let raw = Cow::from(&code[start..end]);

            return if pieces.len() == 1 {
                string(pieces.pop().unwrap(), raw)
            } else {
                let decoded: String = pieces.into_iter().collect();
                string(decoded.into(), raw)
            }
        }

        b'0' => {
            let b = if index + 2 < len { bytes[index + 1] } else { 0 };
            match b {
                b'x' | b'X' => {
                    let mut val = (bytes[index + 2] - b'0') as f64;

                    let mut i = index + 3;
                    loop {
                        if i == len { break }

                        match bytes[i] {
                            v @ b'0'...b'9' => {
                                val *= 16f64;
                                val += (v - b'0') as f64;
                            }
                            v @ b'a'...b'f' => {
                                val *= 16f64;
                                val += (v - b'a') as f64;
                            }
                            v @ b'A'...b'F' => {
                                val *= 16f64;
                                val += (v - b'A') as f64;
                            }
                            _ => break,
                        }

                        i += 1;
                    }

                    number(val, code[index..i].into())
                }
                b'o' | b'O' => {
                    let mut val = (bytes[index + 2] - b'0') as f64;

                    let mut i = index + 3;
                    loop {
                        if i == len { break }

                        match bytes[i] {
                            v @ b'0'...b'7' => {
                                val *= 8f64;
                                val += (v - b'0') as f64;
                            }
                            b'8' | b'9' => {
                                unimplemented!("invalid number error")
                            }
                            _ => break,
                        }

                        i += 1;
                    }

                    number(val, code[index..i].into())
                }
                b'b' | b'B' => {
                    let mut val = (bytes[index + 2] - b'0') as f64;

                    let mut i = index + 3;
                    loop {
                        if i == len { break }

                        match bytes[i] {
                            v @ b'0'...b'1' => {
                                val *= 2f64;
                                val += (v - b'0') as f64;
                            }
                            b'2'...b'9' => {
                                unimplemented!("invalid number error")
                            }
                            _ => break,
                        }

                        i += 1;
                    }

                    number(val, code[index..i].into())
                }
                b'.' => {
                    // 0.455
                    // 0.456e5

                    let mut val = 0f64;
                    let mut offset = 1;

                    let (frac, num) = parse_decimal_digits(&bytes[offset..]);
                    if num != 0 {
                        val += frac;
                        offset += num;
                    }

                    let (exp, num) = parse_exponent(&bytes[offset..]);
                    if num != 0 {
                        val = val * 10f64.powi(exp);
                        offset += num;
                    }

                    number(val, code[..offset].into())
                }
                _ => {
                    let (_, num) = parse_exponent(bytes);

                    number(0f64, code[index..num].into())
                }
            }
        }
        b'1'...b'9' => {
            let (mut val, mut offset) = parse_int_literal(bytes);

            // println!("+{}+", val);

            if offset < len && bytes[offset] == b'.' {
                let (frac, num) = parse_decimal_digits(&bytes[offset + 1..]);
                if num != 0 {
                    val += frac;
                    offset += num + 1;
                }
            }

            let (exp, num) = parse_exponent(&bytes[offset..]);
            if num != 0 {
                val = val * 10f64.powi(exp);
                offset += num;
            }

            number(val, code[..offset].into())
        }

        b'`' => {
            let mut break_curly = false;

            let mut val = String::new();

            for (_i, c) in code.char_indices().skip(1) {
                match c {
                    '{' if  break_curly => {
                        return template(val.clone().into(), val.into(), TemplateFormat::Head);
                    }
                    '`' => {
                        return template(val.clone().into(), val.into(), TemplateFormat::NoSubstitution);
                    }
                    '$' => {
                        break_curly = true;
                    }
                    _ => {
                        break_curly = false;

                        val.push(c);
                    }
                }
            }

            unimplemented!("template string")
        }
        b'\x09' | b'\x0B' | b'\x0C' | b'\x20' => {
            return TokenResult(
                tokens::WhitespaceToken {}.into(),
                single_size(1),
            );
        }
        b'\x0A' | b'\x0D' => {
            return TokenResult(
                tokens::LineTerminatorToken {}.into(),
                TokenSize {
                    chars: 1,
                    lines: 1,
                    width: 0,
                },
            );
        }

        _ => {
            for (i, c) in code.char_indices() {
                match c {
                    '\u{FEFF}' | '\u{00A0}' if i == 0 => {
                        return TokenResult(
                            tokens::WhitespaceToken {}.into(),
                            single_size(1),
                        );
                    }
                    '\u{2028}' | '\u{2029}' if i == 0 => {
                        return TokenResult(
                            tokens::LineTerminatorToken {}.into(),
                            TokenSize {
                                chars: 1,
                                lines: 1,
                                width: 0,
                            },
                        );
                    }
                    _ => {}
                }
            }

            let mut end = index;

            for (i, c) in code.char_indices() {
                match c {
                    '$' | '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                        end = i + 1;
                    }
                    _ => {
                        break;
                    }
                }
            }

            return TokenResult(
                tokens::IdentifierNameToken {
                    name: (&code[..end]).into(),
                }.into(),
                single_size(end - index),
            );
        }
    }
}


fn parse_int_literal(bytes: &[u8]) -> (f64, usize) {
    let mut value = 0f64;

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'0' => {
                i = 1;

                if value == 0f64 {
                    break;
                }
                value *= 10f64;
            }
            v @ b'1' ... b'9' => {
                value *= 10f64;
                value += (v - b'0') as f64;
            }
            _ => break,
        }
        i += 1;
    }

    (value, i)
}
fn parse_decimal_digits(bytes: &[u8]) -> (f64, usize) {
    let mut value = 0f64;
    let mut factor = 0.1;

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            v @ b'0' ... b'9' => {
                value += factor * (v - b'0') as f64;
                factor /= 10f64;
            }
            _ => break,
        }
        i += 1;
    }

    (value, i)
}

fn parse_exponent(bytes: &[u8]) -> (i32, usize) {
    let b = if bytes.len() >= 2 { bytes[0] } else { 0 };
    let (offset, sign) = match b {
        b'e' | b'E' => {
            match bytes[1] {
                b'+' => (2, 1),
                b'-' => (2, -1),
                _ => (1, 1),
            }
        }
        _ => return (0, 0),
    };

    let mut value = 0;

    let mut i = offset;
    while i < bytes.len() {
        match bytes[i] {
            v @ b'0' ... b'9' => {
                let val = (v - b'0') as i32;
                if val != 0 || value != 0 {
                    value *= 10;
                    value += val;
                }
            }
            _ => break,
        }

        i += 1;
    }

    (sign * value, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_line_comments() {
        assert_eq!(
            read_next("// this is some", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Line,
                    value: " this is some".into(),
                }.into(),
                TokenSize {
                    chars: 15,
                    lines: 0,
                    width: 15,
                },
            ),
        );
        assert_eq!(
            read_next("// this is some\rmore", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Line,
                    value: " this is some".into(),
                }.into(),
                TokenSize {
                    chars: 15,
                    lines: 0,
                    width: 15,
                },
            ),
        );
        assert_eq!(
            read_next("// this is some\nmore", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Line,
                    value: " this is some".into(),
                }.into(),
                TokenSize {
                    chars: 15,
                    lines: 0,
                    width: 15,
                },
            ),
        );
        assert_eq!(
            read_next("// this is some\u{2028}more", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Line,
                    value: " this is some".into(),
                }.into(),
                TokenSize {
                    chars: 15,
                    lines: 0,
                    width: 15,
                },
            ),
        );
        assert_eq!(
            read_next("// this is some\u{2029}more", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Line,
                    value: " this is some".into(),
                }.into(),
                TokenSize {
                    chars: 15,
                    lines: 0,
                    width: 15,
                },
            ),
        );
    }

    #[test]
    fn it_parses_block_comments() {
        assert_eq!(
            read_next("/* this *is some */", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Block,
                    value: " this *is some ".into(),
                }.into(),
                TokenSize {
                    chars: 19,
                    lines: 0,
                    width: 19,
                },
            ),
        );
        assert_eq!(
            read_next("/* this *\nis some */", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Block,
                    value: " this *\nis some ".into(),
                }.into(),
                TokenSize {
                    chars: 20,
                    lines: 1,
                    width: 10,
                },
            ),
        );
        assert_eq!(
            read_next("/* this *\r\nis some */", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Block,
                    value: " this *\r\nis some ".into(),
                }.into(),
                TokenSize {
                    chars: 21,
                    lines: 1,
                    width: 10,
                },
            ),
        );
        assert_eq!(
            read_next("/* this *\u{2028}is some */", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Block,
                    value: " this *\u{2028}is some ".into(),
                }.into(),
                TokenSize {
                    chars: 20,
                    lines: 1,
                    width: 10,
                },
            ),
        );
        assert_eq!(
            read_next("/* this *\u{2029}is some */", &Hint::default()),
            TokenResult(
                tokens::CommentToken {
                    format: CommentFormat::Block,
                    value: " this *\u{2029}is some ".into(),
                }.into(),
                TokenSize {
                    chars: 20,
                    lines: 1,
                    width: 10,
                },
            ),
        );
    }

    #[test]
    fn it_parses_whitespace() {
        fn assert_whitespace(code: &str) {
            let len = code.chars().count();
            let s: String = vec![code, "\n"].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::WhitespaceToken {}.into(),
                    TokenSize {
                        chars: 1,
                        lines: 0,
                        width: 1,
                    },
                ),
            );
        }

        assert_whitespace("\u{0009}");
        assert_whitespace("\u{000B}");
        assert_whitespace("\u{000C}");
        assert_whitespace("\u{0020}");
        assert_whitespace("\u{00A0}");
        assert_whitespace("\u{FEFF}");
    }

    #[test]
    fn it_parses_line_terminators() {
        fn assert_line_terminator(code: &str) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::LineTerminatorToken {}.into(),
                    TokenSize {
                        chars: 1,
                        lines: 1,
                        width: 0,
                    },
                ),
            );
        }

        assert_line_terminator("\u{000A}");
        assert_line_terminator("\u{000D}");
        assert_line_terminator("\u{2028}");
        assert_line_terminator("\u{2029}");
    }

    #[test]
    fn it_parses_strings() {
        fn assert_string(code: &str, value: &str) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::StringLiteralToken {
                        value: value.into(),
                    }.into(),
                    TokenSize {
                        chars: len,
                        lines: 0,
                        width: len,
                    },
                ),
            );
        }

        assert_string("'a real string'", "a real string");
        assert_string("'a real\"string'", "a real\"string");
        assert_string("\"a real string\"", "a real string");
        assert_string("\"a real'string\"", "a real'string");
    }

    #[test]
    fn it_parses_templates() {
        fn assert_template(code: &str, value: &str, raw_value: &str, format: TemplateFormat, lines: usize, width: usize) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default().template(true)),
                TokenResult(
                    tokens::TemplateToken {
                        format,
                        cooked: value.into(),
                        raw: raw_value.into(),
                    }.into(),
                    TokenSize {
                        chars: len,
                        lines,
                        width,
                    },
                ),
            );
        }

        assert_template("`foo`", "foo", "foo", TemplateFormat::NoSubstitution, 0, 5);
        assert_template("`foo${", "foo", "foo", TemplateFormat::Head, 0, 6);
        assert_template("}foo${", "foo", "foo", TemplateFormat::Middle, 0, 6);
        assert_template("}foo`", "foo", "foo", TemplateFormat::Tail, 0, 5);
    }

    #[test]
    fn it_parses_numbers() {
        fn assert_number(code: &str, value: f64) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::NumericLiteralToken {
                        value,
                    }.into(),
                    TokenSize {
                        chars: len,
                        lines: 0,
                        width: len,
                    },
                ),
            );
        }

        assert_number("0x4", 4f64);
        assert_number("0x40", 64f64);
        assert_number("0o4", 4f64);
        assert_number("0o40", 32f64);
        assert_number("0b0101", 5f64);
        assert_number("0b1100000", 96f64);
        assert_number("1", 1f64);
        assert_number("145", 145f64);
        assert_number("14.5", 14.5f64);
        assert_number("14.5e2", 1450f64);
        assert_number("14.5e-2", 0.145f64);
        assert_number("14.5e+2", 1450f64);
        assert_number("14e2", 1400f64);
        assert_number("14e-2", 0.14f64);
        assert_number("14e+2", 1400f64);
        assert_number(".14", 0.14f64);
        assert_number(".14e2", 14.000000000000002f64); // TODO: Wrong
        assert_number(".14e-2", 0.0014000000000000002f64); // TODO: Wrong
        assert_number(".14e+2", 14.000000000000002f64); // TODO: Wrong
    }

    #[test]
    fn it_parses_regexes() {
        fn assert_regex(code: &str, pattern: &str, flags: &str) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default().expression(true)),
                TokenResult(
                    tokens::RegularExpressionLiteralToken {
                        pattern: pattern.into(),
                        flags: flags.into(),
                    }.into(),
                    TokenSize {
                        chars: len,
                        lines: 0,
                        width: len,
                    },
                ),
            );
        }

        assert_regex("/omg/", "omg", "");
        assert_regex("/omg/g", "omg", "g");
        assert_regex("/omg/u", "omg", "u");
        assert_regex("/om[/]g/", "om[/]g", "");
        assert_regex("/om[/]g/u", "om[/]g", "u");
    }

    #[test]
    fn it_parses_identifiers() {
        fn assert_identifier(code: &str, name: &str) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::IdentifierNameToken {
                        name: name.into(),
                    }.into(),
                    single_size(len),
                ),
            );
        }

        assert_identifier("omg", "omg");
    }

    #[test]
    fn it_parses_punctuators() {
        fn assert_punc(code: &str, punc: tokens::PunctuatorToken) {
            let len = code.chars().count();
            let s: String = vec![code, " "].into_iter().collect();

            assert_eq!(
                read_next(&s, &Hint::default()),
                TokenResult(
                    tokens::Token::Punctuator(punc),
                    TokenSize {
                        chars: len,
                        lines: 0,
                        width: len,
                    },
                ),
            );
        }

        use super::tokens::PunctuatorToken::*;

        assert_punc("{", CurlyOpen);
        assert_punc("}", CurlyClose);
        assert_punc("(", ParenOpen);
        assert_punc(")", ParenClose);
        assert_punc("[", SquareOpen);
        assert_punc("]", SquareClose);
        assert_punc(".", Period);
        assert_punc("...", Ellipsis);
        assert_punc(";", Semicolon);
        assert_punc(",", Comma);
        assert_punc("?", Question);
        assert_punc(":", Colon);
        assert_punc("~", Tilde);
        assert_punc("<", LAngle);
        assert_punc("<=", LAngleEq);
        assert_punc("<<", LAngleAngle);
        assert_punc("<<=", LAngleAngleEq);
        assert_punc(">", RAngle);
        assert_punc(">=", RAngleEq);
        assert_punc(">>", RAngleAngle);
        assert_punc(">>=", RAngleAngleEq);
        assert_punc(">>>", RAngleAngleAngle);
        assert_punc(">>>=", RAngleAngleAngleEq);
        assert_punc("=", Eq);
        assert_punc("=>", Arrow);
        assert_punc("==", EqEq);
        assert_punc("===", EqEqEq);
        assert_punc("!", Exclam);
        assert_punc("!=", ExclamEq);
        assert_punc("!==", ExclamEqEq);
        assert_punc("+", Plus);
        assert_punc("+=", PlusEq);
        assert_punc("++", PlusPlus);
        assert_punc("-", Minus);
        assert_punc("-=", MinusEq);
        assert_punc("--", MinusMinus);
        assert_punc("/", Slash);
        assert_punc("/=", SlashEq);
        assert_punc("%", Percent);
        assert_punc("%=", PercentEq);
        assert_punc("*", Star);
        assert_punc("*=", StarEq);
        assert_punc("**", StarStar);
        assert_punc("**=", StarStarEq);
        assert_punc("&", Amp);
        assert_punc("&=", AmpEq);
        assert_punc("&&", AmpAmp);
        assert_punc("|", Bar);
        assert_punc("|=", BarEq);
        assert_punc("||", BarBar);
        assert_punc("^", Caret);
        assert_punc("^=", CaretEq);
    }
}
