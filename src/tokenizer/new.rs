use std::borrow::{Cow, Borrow};
use std::result;
use tokenizer::tokens::{Token, PunctuatorToken, NumericLiteralToken,
    StringLiteralToken, TemplateToken, TemplateFormat, CommentToken, CommentFormat};

// use ucd::Codepoint;

#[derive(Clone, Debug)]
pub struct Tokenizer<'a, T: 'a> {
  code: &'a T,

  offset: usize,
  line: usize,
  column: usize,

  template_stack: Vec<bool>,
}

pub struct Hint {
  expression: bool,
  template: bool,
  strict: bool
}

pub enum TokenizerError { }

pub type Result<T> = result::Result<T, TokenizerError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub lines: usize,
  pub width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
  pub start: (usize, Position),
  pub end: (usize, Position),
}


impl<'a, T> Tokenizer<'a, T>
where
  T: Borrow<str>
{
  pub fn new(code: &T) -> Tokenizer<T> {
    Tokenizer {
      code,
      offset: 0,
      line: 1,
      column: 0,
      template_stack: Default::default(),
    }
  }

  // pub fn next(&mut self, hint: &Hint) -> Result<Token> {
  //   unimplemented!()
  // //   let token = self.readNext(hint)?;

  // //   self.offset = token.offset.end;
  // //   self.line = token.position.end.line;
  // //   self.line = token.offset.end;

  // //   Ok(token);
  // }
}

pub struct TokenResult<'a>(Token<'a>, TokenSize);

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

fn punc(tok: PunctuatorToken, size: usize) -> TokenResult<'static> {
  TokenResult(Token::Punctuator(tok), single_size(size))
}

fn number<'a>(tok: f64, raw: Cow<'a, str>) -> TokenResult<'a> {
    let len = raw.len();
  TokenResult(
    Token::NumericLiteral(
      NumericLiteralToken {
        raw,
        value: tok
      }
    ),
    single_size(len),
  )
}
fn string<'a>(tok: Cow<'a, str>, raw: Cow<'a, str>) -> TokenResult<'a> {
    let len = raw.len();
  TokenResult(
    Token::StringLiteral(
      StringLiteralToken {
        raw,
        value: tok
      }
    ),
    single_size(len),
  )
}

fn template<'a>(tok: Cow<'a, str>, raw: Cow<'a, str>, format: TemplateFormat) -> TokenResult<'a> {
    let len = raw.len();
  TokenResult(
    Token::Template(
      TemplateToken {
        format,
        raw,
        cooked: tok
      }
    ),
    single_size(len),
  )
}

fn comment<'a>(tok: Cow<'a, str>, format: CommentFormat) -> TokenResult<'a> {
    let len = tok.len() + match format {
        CommentFormat::Line => 2,
        CommentFormat::Block => 4,
        _ => unimplemented!("unsupported comment format"),
    };

  TokenResult(
    Token::Comment(
      CommentToken {
        format,
        value: tok
      }
    ),
    single_size(len),
  )
}




pub fn read_next<'a>(code: &'a str, hint: &Hint) -> TokenResult<'a> {
  // loop, eating whitespace chars?

  let bytes = code.as_bytes();
  let len = bytes.len();
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

    b'.' => {
      if index + 2 < len && bytes[index + 1] == b'.' && bytes[index + 2] == b'.' {
        punc(PunctuatorToken::Ellipsis, 3)
      } else if index + 1 < len && bytes[index + 1] >= b'0' && bytes[index + 1] <= b'9' {
        unimplemented!("numeric fractional")
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
        punc(PunctuatorToken::LAngleEq, 3)
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
                _ => {}
            }
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

        comment(Cow::from(&code[index + 2..end]), CommentFormat::Line)
      } else if hint.expression {
        unimplemented!("regex")
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
            val = val.powi(exp);
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

        if offset < len && bytes[offset] == b'.' {
          let (frac, num) = parse_decimal_digits(&bytes[offset + 1..]);
          if num != 0 {
            val += frac;
            offset += num + 1;
          }
        }

        let (exp, num) = parse_exponent(&bytes[offset..]);
        if num != 0 {
          val = val.powi(exp);
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

    _ => {
      // walk for identifiers
      // walk for U+FEFF


      unimplemented!("identifier")
    }
  }
}


fn parse_int_literal(bytes: &[u8]) -> (f64, usize) {
  let mut value = 0f64;
  let mut factor = 1f64;

  let mut i = 0;
  while i < bytes.len() {
    match bytes[i] {
      b'0' => {
        i = 1;
        break;
      }
      v @ b'1' ... b'9' => {
        value += factor * (v - b'0') as f64;
        factor *= 10f64;
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


