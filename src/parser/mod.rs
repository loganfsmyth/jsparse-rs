#[macro_use]
pub mod utils;

// mod file;
// mod module;
// mod expressions;
// mod declarations;
// mod statements;
// mod classes;
// mod functions;

use std::ops::{Deref, DerefMut};
use tokenizer::{IntoTokenizer, Tokenizer, Hint, tokens};

fn parse_root<'code, T, P>(t: T) -> P
where
    T: IntoTokenizer<'code>,
    P: FromTokenizer
{
    FromTokenizer::from_tokenizer(t)
}

pub trait FromTokenizer {
    fn from_tokenizer<'code, T: IntoTokenizer<'code>>(t: T) -> Self;
}

impl FromTokenizer for () {
    fn from_tokenizer<'code, T: IntoTokenizer<'code>>(t: T) -> () {
        let mut p = Parser {
            tok: t.into_tokenizer(),
            hint: Default::default(),
            lookahead: None,
            token: None,
        };

        // p.parse_module();
    }
}

pub enum Flags {
    In,
    Yield,
    Await,
    Return,
    Default,
}

struct GrammarFlags {
    allow_in: bool,
    allow_yield: bool,
    allow_await: bool,
    allow_return: bool,
    allow_default: bool,
}

pub struct Parser<'code, T: 'code> {
    tok: T,
    hint: Hint,
    token: Option<tokens::Token<'code>>,
    lookahead: Option<LookaheadResult<'code>>,
}

// struct ParserProxy<'code, 'parser: 'code, T> {
//     p: &'parser mut Parser<'code, T>,
//     flag: Flags,
//     val: bool,
// }
// impl<'a, T> ParserProxy<'a, T> {
//     fn new<'b>(p: &mut Parser<'b>, flag: Flags, val: bool) -> ParserProxy<'b> {
//         ParserProxy {
//             p,
//             flag,
//             val,
//         }
//     }

//     pub fn with_flag(&mut self, flag: Flags, val: bool) -> ParserProxy<'a> {
//         ParserProxy::new(self.p, flag, val)
//     }
// }
// impl<'a, T> Deref for ParserProxy<'a, T> {
//     type Target = Parser<'a, T>;

//     fn deref(&self) -> &Parser {
//         self.p
//     }
// }
// impl<'a, T> DerefMut for ParserProxy<'a, T> {
//     fn deref_mut(&mut self) -> &mut Parser {
//         self.p
//     }
// }
// impl<'a, T> Drop for ParserProxy<'a, T> {
//     fn drop(&mut self) -> &mut Parser {

//     }
// }



pub struct LookaheadResult<'code> {
    line: bool,
    token: tokens::Token<'code>,
}

impl<'code, T: Tokenizer<'code>> Parser<'code, T> {
    pub fn expect_expression(&mut self) {
        self.hint.expression(true);
    }

    pub fn semicolon(&mut self) {

    }

    // pub fn with_flag<'parser>(&'parser mut self, flag: Flags, val: bool) -> ParserProxy<'code, 'parser, T> {
    //     ParserProxy::new(self, flag, val)
    // }

    fn read_token(&mut self, hint: &Hint) -> (bool, tokens::Token<'code>) {
        let mut line = false;
        loop {
            let (token, pos) = self.tok.next_token(hint);
            match token {
                tokens::Token::Whitespace(_) => {}
                tokens::Token::LineTerminator(_) => {
                    line = true;
                }
                tokens::Token::Comment(_) => {
                    if pos.start.line != pos.end.line {
                        line = true;
                    }
                }
                t => break (line, t),
            }
        }
    }

    pub fn token(&mut self) -> &tokens::Token {
        if let None = self.token {
            if let Some(ahead) = self.lookahead.take() {
                self.token = Some(ahead.token);
                self.lookahead = None;
            } else {
                // TODO
                let hint = self.hint;
                let token = self.read_token(&hint).1;
                self.token = Some(token);
            }
        }

        match self.token {
            Some(ref t) => t,
            _ => unreachable!(),
        }
    }

    pub fn ident_lookahead(&mut self) -> Option<&LookaheadResult> {
        if let Some(ref lookahead) = self.lookahead {
            return Some(lookahead);
        }

        let expect_expression = if let tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) = *self.token() {
            match &**name {
                "case" | "throw" | "try" | "do" | "return" | "new" | "typeof" | "delete" | "void" | "await" | "yield" => {
                    true
                }
                _ => false,
            }
        } else {
            return None;
        };

        let hint = self.hint.expression(expect_expression);
        let (line, token) = self.read_token(&hint);

        self.lookahead = Some(LookaheadResult { line, token });

        if let Some(ref lookahead) = self.lookahead {
            return Some(lookahead);
        } else {
            unreachable!()
        }
    }

    pub fn try_punc(&mut self, punc: tokens::PunctuatorToken) -> utils::InnerResult<()> {
        if true {
            Ok(())
        } else {
            Err(utils::InnerError::NotFound)
        }
    }

    pub fn eat_punc(&mut self, punc: tokens::PunctuatorToken) -> utils::InnerResult<()> {
        if true {
            Ok(())
        } else {
            Err(utils::InnerError::NotFound)
        }
    }

    pub fn try_identifier(&mut self, keyword: &str) -> utils::InnerResult<()> {
        if true {
            Ok(())
        } else {
            Err(utils::ParseError::UnexpectedToken.into())
        }
    }

    pub fn eat_identifier(&mut self, keyword: &str) -> utils::InnerResult<()> {
        if true {
            Ok(())
        } else {
            Err(utils::ParseError::UnexpectedToken.into())
        }
    }
}
