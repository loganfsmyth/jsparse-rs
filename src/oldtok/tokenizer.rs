use std::iter::{Peekable, Enumerate};
use ucd::Codepoint;

pub struct ReadTokensResult {
  // 
  InputEmpty,


  OutputFull,

  End
}

pub fn read_tokens(str: &str, &mut [Token])


enum Goal {
  Script,
  Module,
}

enum GrammarContext {
  // Default context
  // No regexes or template middle/end expected
  Div,

  // where a RegularExpressionLiteral is permitted but neither a TemplateMiddle, nor a TemplateTail is permitted
  // No division or template middles expected, regex expected
  // Expects regex instead of division, essentially
  RegExp,

  // where a TemplateMiddle or a TemplateTail is permitted but a RegularExpressionLiteral is not permitted
  // No Regex or right curlies
  // Expect template literal curly instead of right curly
  TemplateTail,

  // where a RegularExpressionLiteral, a TemplateMiddle, or a TemplateTail is permitted.
  // No division or right curlies
  // Expect regex _or_ template curly
  RegExpOrTemplateTail,
}


// Open curly => increment




enum Mode {
  Nonstrict,
  Strict,
}

enum Annex {
  None,
  B,
}


pub struct Tokenizer<T: Iterator<Item=char>> {
  chars: CharReader<T>,

  pub strict: bool,
  annexb: bool,

  expression: bool,
  template: bool,
}

macro_rules! set_and_restore {
    ($property: expr, $call: expr, $value: expr) => {
        {
            let value = $property;
            $property = $value;
            let result = $call;
            $property = value;
            result
        }
    }
}

impl<T: Iterator<Item=char>> Tokenizer<T> {
  pub fn new(chars: T) -> Tokenizer<T> {
    Tokenizer {
      chars: CharReader::new(chars),
      strict: false,
      annexb: false,
      expression: false,
      template: false,
    }
  }

  pub fn with_strict<F, R>(&mut self, f: F) -> R where
    F: FnOnce(&mut Self) -> R,
  {
    set_and_restore!(self.strict, f(self), true)
  }

  pub fn next(&mut self) -> Option<Token> {
    Some(match self.chars.entry() {
      (Some('('), _, _, _) => Token::LParen,
      (Some(')'), _, _, _) => Token::RParen,
      (Some('{'), _, _, _) => Token::LCurly,
      (Some('['), _, _, _) => Token::LSquare,
      (Some(']'), _, _, _) => Token::RSquare,
      (Some(';'), _, _, _) => Token::Semicolon,
      (Some(','), _, _, _) => Token::Comma,
      (Some('~'), _, _, _) => Token::Tilde,
      (Some('?'), _, _, _) => Token::Quest,
      (Some(':'), _, _, _) => Token::Colon,

      (Some('.'), Some('.'), Some('.'), _) => { // ...
        self.chars.next();
        self.chars.next();

        Token::Ellipsis
      },
      (Some('.'), b, _, _) if b.map_or(false, |c| (c < '0' || c > '9')) => Token::Period, // .

      (Some('<'), Some('<'), Some('='), _) => { // <<=
        self.chars.next();
        self.chars.next();
        Token::LAngleAngleEq
      },
      (Some('<'), Some('<'), _, _) => { // <<
        self.chars.next();
        Token::LAngleAngle
      },
      (Some('<'), Some('='), _, _) => { // <=
        self.chars.next();
        Token::LessEq
      },
      (Some('<'), _, _, _) => { // <
        Token::LAngle
      },

      (Some('>'), Some('>'), Some('>'), Some('=')) => { // >>>=
        self.chars.next();
        self.chars.next();
        self.chars.next();
        Token::RAngleAngleAngleEq
      },
      (Some('>'), Some('>'), Some('>'), _) => { // >>>
        self.chars.next();
        self.chars.next();
        Token::RAngleAngleAngle
      },
      (Some('>'), Some('>'), Some('='), _) => { // >>=
        self.chars.next();
        self.chars.next();
        Token::RAngleAngleEq
      },
      (Some('>'), Some('>'), _, _) => { // >>
        self.chars.next();
        Token::RAngleAngle
      },
      (Some('>'), Some('='), _, _) => { // >=
        self.chars.next();
        Token::GreaterEq
      },
      (Some('>'), _, _, _) => { // >
        Token::RAngle
      },

      (Some('!'), Some('='), Some('='), _) => { // !==
        self.chars.next();
        self.chars.next();
        Token::NEqEq
      },
      (Some('!'), Some('='), _, _) => { // !=
        self.chars.next();
        Token::NEq
      },
      (Some('!'), _, _, _) => { // !
        Token::Exclam
      },

      (Some('='), Some('='), Some('='), _) => { // ===
        self.chars.next();
        self.chars.next();
        Token::EqEqEq
      },
      (Some('='), Some('='), _, _) => { // ==
        self.chars.next();
        Token::EqEq
      },
      (Some('='), _, _, _) => { // =
        Token::Eq
      },


      (Some('+'), Some('+'), _, _) => { // ++
        self.chars.next();
        self.chars.next();
        Token::PlusPlus
      },
      (Some('+'), Some('='), _, _) => { // +=
        self.chars.next();
        Token::PlusEq
      },
      (Some('+'), _, _, _) => { // +
        Token::Plus
      },

      (Some('-'), Some('-'), _, _) => { // --
        self.chars.next();
        self.chars.next();
        Token::MinusMinus
      },
      (Some('-'), Some('='), _, _) => { // -=
        self.chars.next();
        Token::MinusEq
      },
      (Some('-'), _, _, _) => { // -
        Token::Minus
      },

      (Some('%'), Some('='), _, _) => { // %=
        self.chars.next();
        Token::ModEq
      },
      (Some('%'), _, _, _) => { // %
        Token::Mod
      },

      (Some('*'), Some('*'), Some('='), _) => { // **=
        self.chars.next();
        self.chars.next();
        Token::StarStarEq
      },
      (Some('*'), Some('='), _, _) => { // *=
        self.chars.next();
        Token::StarEq
      },
      (Some('*'), Some('*'), _, _) => { // **
        self.chars.next();
        Token::StarStar
      },
      (Some('*'), _, _, _) => { // *
        Token::Star
      },

      (Some('&'), Some('&'), _, _) => { // &&
        self.chars.next();
        Token::AmpAmp
      },
      (Some('&'), Some('='), _, _) => { // &=
        self.chars.next();
        Token::AmpEq
      },
      (Some('&'), _, _, _) => { // &
        Token::Amp
      },


      (Some('|'), Some('|'), _, _) => { // ||
        self.chars.next();
        Token::BarBar
      },
      (Some('|'), Some('='), _, _) => { // |=
        self.chars.next();
        Token::BarEq
      },
      (Some('|'), _, _, _) => { // |
        Token::Bar
      },

      (Some('^'), Some('='), _, _) => { // ^=
        self.chars.next();
        Token::CaretEq
      },
      (Some('^'), _, _, _) => { // ^
        Token::Caret
      },

      (Some('/'), Some(c @ '/'), _, _) => { // //
        self.chars.next();
        self.parse_comment(c)
      },
      (Some('/'), Some(c @ '*'), _, _) => { // //* 
        self.chars.next();
        self.parse_comment(c)
      },
      (Some('/'), Some('='), _, _) if !self.expression => {
        self.chars.next();
        Token::DivEq
      },
      (Some('/'), _, _, _) => {
        if self.expression {
          self.parse_regexp()
        } else {
          Token::Div
        }
      },
      (Some('`'), _, _, _) => {
        // parse template literal (or start)
        self.parse_template_start()
      },
      (Some('}'), _, _, _) => {
        if self.template {
          // template literal middle or end
          self.parse_template_middle_end()
        } else {
          Token::RCurly
        }
      },

      (Some(c), next, _, _) => {
        match c {
          // numbers
          '0'...'9' => self.parse_number(c, next),
          '.' => self.parse_number(c, next),
          '"' | '\'' => self.parse_string(c),
          '$' | '_' | '\\' => self.parse_identifier(c, next),

          // Spec explicit whitespace whitelist.
          '\u{9}' | '\u{B}' | '\u{C}' | '\u{20}' | '\u{A0}' | '\u{FEFF}' => Token::Whitespace,
          // Unicode "Space_Separator" characters
          '\u{1680}' | '\u{2000}'...'\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => Token::Whitespace,

          '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => Token::LineTerminator,
          
          c if c.is_id_start() => self.parse_identifier(c, next),

          _ => Token::Unknown,
        }

      }

      _ => return None,
    })
  }


  // 0[xX][0-9a-fA-F]+
  // 0[oO][0-7]+
  // 0[bB][0-1]+
  // 0       (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
  // [0-9]+  (\.[0-9]*)?     ([eE][+-]?[0-9]+)?
  //         (\.[0-9]+)      ([eE][+-]?[0-9]+)?
  fn parse_number(&mut self, c: char, next: Option<char>) -> Token {
    let mut s = String::with_capacity(5);
    s.push(c);

    if c == '0' {
      if let Some(c) = next {
        match c {
          'x' | 'X' => {
            s.push(c);
            loop {
              let (_, next, _, _) = self.chars.entry();

              if let Some(c) = next {
                match c {
                  '0'...'9' | 'A'...'F' | 'a'...'f' => {
                    s.push(c);
                  }
                  _ => break
                }
              } else {
                break;
              }
            }

            return Token::NumericLiteral(NumberType::Hex);
          }
          'o' | 'O' => {
            s.push(c);
            loop {
              let (_, next, _, _) = self.chars.entry();

              if let Some(c) = next {
                match c {
                  '0'...'7' => {
                    s.push(c);
                  }
                  _ => break
                }
              } else {
                break;
              }
            }

            return Token::NumericLiteral(NumberType::Octal);
          }
          'b' | 'B' => {
            s.push(c);
            loop {
              let (_, next, _, _) = self.chars.entry();

              if let Some(c) = next {
                match c {
                  '0' | '1' => {
                    s.push(c);
                  }
                  _ => break
                }
              } else {
                break;
              }
            }

            return Token::NumericLiteral(NumberType::Binary);
          }

          '.' => {
            s.push(c);
            while let (_, Some(c @ '0'...'9'), _, _) = self.chars.entry() {
              s.push(c);
            }
          }
          _ => ()
        }
      } else {
        return Token::NumericLiteral(NumberType::Float);
      }
    } else if c == '.' {
      // match next {
      //   '0'...'9' => {
      //     s.push(next);
      //     while let (_, Some(c @ '0'...'9'), _, _) = self.chars.entry() {
      //       s.push(c);
      //     }
      //   }
      //   _ => return Token::Period
      // }
    } else {
      // s.push(c);

      // match next {
      //   '0'...'9' => {


      //   }
      //   '.' => {
      //     s.push(c);
      //     while let (_, Some(c @ '0'...'9'), _, _) = self.chars.entry() {
      //       s.push(c);
      //     }
      //   }
      // }
      // s.push(c);
      // while let (_, Some(c @ '0'...'9'), _, _) = self.chars.entry() {
      //   s.push(c);
      // }
    }

    // TODO: Exponent parse_string


    Token::Unknown
  }

  fn parse_identifier(&mut self, c: char, next: Option<char>) -> Token {
    if let Some(c) = next {
      if c.is_id_continue() {
        loop {
          let (c, next, _, _) = self.chars.entry();

          if let Some(c) = next {
            if !c.is_id_continue() { break; }
          } else {
            break;
          }
        }
      }
    }

    Token::IdentifierName
  }

  fn parse_template_start(&mut self) -> Token {
    Token::Unknown
  }
  fn parse_template_middle_end(&mut self) -> Token {
    Token::Unknown
  }
  fn parse_regexp(&mut self) -> Token {
    Token::Unknown
  }

  fn parse_string(&mut self, c: char) -> Token {
    loop {
      let (next, _, _, _) = self.chars.entry();
      if let Some(next) = next {
        if next == c {
          return Token::StringLiteral;
        }
      } else {
        return Token::Unknown;
      }
    }
  }

  fn parse_comment(&mut self, c: char) -> Token {
    if c == '*' {
      while let (Some(c), next, _, _) = self.chars.entry() {
        if c == '*' {
          if let Some('/') = next {
            self.chars.next();
            return Token::Comment;
          }
        }
      }
    } else {
      while let Some(c) = self.chars.next() {
        match c {
          // TODO: Handle CRLF
          '\u{A}' | '\u{D}' | '\u{2028}' | '\u{2029}' => return Token::Comment,
          _ => (),
        }
      }
    }
    Token::Unknown
  }
}








