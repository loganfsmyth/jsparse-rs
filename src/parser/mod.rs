#[macro_use]
mod utils;

mod file;
mod module;
mod expressions;
mod declarations;
mod statements;
mod classes;
mod functions;

use std::ops::{Deref, DerefMut};
use tokenizer::{IntoTokenizer, Tokenizer, Hint, tokens};

fn parse_root<'code, T: 'code, P>(t: T) -> P
where
    T: IntoTokenizer<'code>,
    P: FromTokenizer
{
    FromTokenizer::from_tokenizer(t)
}

pub trait FromTokenizer {
    fn from_tokenizer<'code, T: IntoTokenizer<'code> + 'code>(t: T) -> Self;
}

impl FromTokenizer for () {
    fn from_tokenizer<'code, T: IntoTokenizer<'code> + 'code>(t: T) -> () {
        let mut p = Parser {
            tok: t.into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],
            lookahead: None,
            token: None,
        };

        // p.parse_module();
    }
}

pub struct ParserProxy<'parser, 'code: 'parser, T: 'code>
where
    T: Tokenizer<'code>
{
    p: &'parser mut Parser<'code, T>,
    flag: Flags,
    val: bool,
}
impl<'parser, 'code, T> ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    fn new(p: &'parser mut Parser<'code, T>, flag: Flags, val: bool) -> ParserProxy<'parser, 'code, T> {
        p.push_flags(flag, val);

        ParserProxy {
            p,
            flag,
            val,
        }
    }

    pub fn with_flag<'p>(&'p mut self, flag: Flags, val: bool) -> ParserProxy<'p, 'code, T> {
        ParserProxy::new(self.p, flag, val)
    }
}
impl<'parser, 'code, T> Deref for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    type Target = Parser<'code, T>;

    fn deref(&self) -> &Parser<'code, T> {
        self.p
    }
}
impl<'parser, 'code, T> DerefMut for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    fn deref_mut(&mut self) -> &mut Parser<'code, T> {
        self.p
    }
}
impl<'parser, 'code, T> Drop for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    fn drop(&mut self) {
        self.p.pop_flags();
    }
}

pub struct LookaheadResult<'code> {
    line: bool,
    token: tokens::Token<'code>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    In,
    Yield,
    Await,
    Return,
    Default,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GrammarFlags {
    allow_in: bool,
    allow_yield: bool,
    allow_await: bool,
    allow_return: bool,
    allow_default: bool,
}

pub struct Parser<'code, T: 'code>
where
    T: Tokenizer<'code>
{
    tok: T,
    hint: Hint,
    flags: GrammarFlags,
    flags_stack: Vec<GrammarFlags>,
    token: Option<tokens::Token<'code>>,
    lookahead: Option<LookaheadResult<'code>>,
}

impl<'code, T: Tokenizer<'code>> Parser<'code, T> {
    pub fn expect_expression(&mut self) {
        self.hint.expression(true);
    }

    pub fn semicolon(&mut self) {

    }

    pub fn with<'parser>(&'parser mut self, flags: Flags) -> ParserProxy<'parser, 'code, T> {
        ParserProxy::new(self, flags, true)
    }

    pub fn without<'parser>(&'parser mut self, flags: Flags) -> ParserProxy<'parser, 'code, T> {
        ParserProxy::new(self, flags, true)
    }

    fn push_flags(&mut self, flags: Flags, val: bool) {
        self.flags_stack.push(self.flags);

        match flags {
            Flags::In => { self.flags.allow_in = val; }
            Flags::Yield => { self.flags.allow_yield = val; }
            Flags::Await => { self.flags.allow_await = val; }
            Flags::Return => { self.flags.allow_return = val; }
            Flags::Default => { self.flags.allow_default = val; }
        }
    }
    fn pop_flags(&mut self) {
        self.flags_stack.pop();
    }

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
