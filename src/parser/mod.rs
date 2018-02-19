#[macro_use]
mod utils;

mod file;
mod module;
mod expressions;
mod declarations;
mod statements;
mod classes;
mod functions;

// use failure;
// use failure::Fail;

use time;

// impl<'code, T> Parser<'code, T>
// where
//     T: Tokenizer<'code>
// {
//     // TODO: Whereever these are called needs to call `self.expect_expression()`.
//     pub fn parse_expression(&mut self) -> utils::OptResult<()> {
//         self.parse_assignment_expression()
//     }
//     pub fn parse_assignment_expression(&mut self) -> utils::OptResult<()> {
//         self.parse_left_hand_side_expression()
//     }
//     pub fn parse_left_hand_side_expression(&mut self) -> utils::OptResult<()> {
//         try_token!(self.keyword("this"));
//         Ok(Some(()))
//     }
// }

// use flame;
// use std::fs::File;

use std::ops::{Deref, DerefMut};
use tokenizer::{IntoTokenizer, Tokenizer, Hint, tokens};
use self::utils::TokenResult;

// struct Timer {
//     t: u64
// }
// impl Timer {
//     fn new() -> Timer {
//         Timer {
//             t: time::precise_time_ns()
//         }
//     }
// }
// impl Drop for Timer {
//     fn drop(&mut self) {
//         let t_ns = time::precise_time_ns() - self.t;

//         println!("time: {:?}", (t_ns / 1_000_000_000, t_ns % 1_000_000_000  ));
//     }
// }

pub fn parse_root<'code, T: 'code, P>(t: T) -> P
where
    T: IntoTokenizer<'code>,
    P: FromTokenizer
{
    println!("starting");
    // let _timer = Timer::new();

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


        let t_start = time::precise_time_ns();
        {
            // let _g = flame::start_guard("jsparse");
            let _ = p.parse_module().unwrap();
        }
        let total_parse = time::precise_time_ns() - t_start;

        println!("Total parsing time: {}ms", total_parse as f64 / 1e6);

        // let mut total_tok = 0;
        // for (_name, &(_count, ns, _chars)) in p.tok.stats() {
        //     total_tok += ns;
        //     // println!("{} took {} tokens in {}ns, averaging {} each, processing at {} cp/us", _name, _count, ns, ns as f64 / _count as f64, 1e3 * _chars as f64 / ns as f64);
        // }
        // println!("Total tokenizing time: {:.3}ms, roughly {:.2}%", total_tok as f64 / 1e6, 100.0 * total_tok as f64 / total_parse as f64);


        // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

        ()
    }
}

pub struct ParserProxy<'parser, 'code: 'parser, T: Tokenizer<'code> + 'code>(&'parser mut Parser<'code, T>);

impl<'parser, 'code, T: Tokenizer<'code>> ParserProxy<'parser, 'code, T>
{
    fn new(p: &'parser mut Parser<'code, T>) -> ParserProxy<'parser, 'code, T> {
        ParserProxy(p)
    }

    pub fn with<'p>(&'p mut self, flag: Flag) -> ParserProxy<'p, 'code, T> {
        self.push_flags(flag, true);

        ParserProxy::new(self.0)
    }

    pub fn without<'p>(&'p mut self, flag: Flag) -> ParserProxy<'p, 'code, T> {
        self.push_flags(flag, false);

        ParserProxy::new(self.0)
    }
}
impl<'parser, 'code, T> Deref for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    type Target = Parser<'code, T>;

    fn deref(&self) -> &Parser<'code, T> {
        self.0
    }
}
impl<'parser, 'code, T> DerefMut for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    fn deref_mut(&mut self) -> &mut Parser<'code, T> {
        self.0
    }
}
impl<'parser, 'code, T> Drop for ParserProxy<'parser, 'code, T>
where
    T: Tokenizer<'code>
{
    fn drop(&mut self) {
        self.0.pop_flags();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookaheadResult<'code> {
    line: bool,
    token: tokens::Token<'code>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    In,
    Yield,
    Await,
    Return,
    Default,
    Module,
    Strict,
    Noop,

    Template,
    // Curly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GrammarFlags {
    allow_in: bool,
    allow_yield: bool,
    allow_await: bool,
    allow_return: bool,
    allow_default: bool,
    is_module: bool,
    is_strict: bool,
    // TODO: Where to handle directives?

    expect_template: bool,
}

#[derive(Debug)]
pub struct Parser<'code, T: 'code>
where
    T: Tokenizer<'code>
{
    tok: T,
    hint: Hint,
    flags: GrammarFlags,
    flags_stack: Vec<GrammarFlags>,
    token: Option<LookaheadResult<'code>>,
    lookahead: Option<LookaheadResult<'code>>,
}

impl<'code, T: Tokenizer<'code>> Parser<'code, T> {
    pub fn expect_expression(&mut self) {
        self.hint = self.hint.expression(true);
    }

    pub fn semicolon(&mut self) -> TokenResult<()> {
        self.semicolon_inner(false)
    }
    pub fn semicolon_dowhile(&mut self) -> TokenResult<()> {
        self.semicolon_inner(true)
    }
    fn semicolon_inner(&mut self, was_do_while: bool) -> TokenResult<()> {
        let exists = {
            let (line, token) = self.token_and_line();

            if let tokens::Token::Punctuator(tokens::PunctuatorToken::Semicolon) = *token {
                true
            } else if let tokens::Token::Punctuator(tokens::PunctuatorToken::CurlyClose) = *token {
                false
            } else if let tokens::Token::EOF(_) = *token {
                false
            } else if was_do_while || line {
                false
            } else {
                return TokenResult::None;
            }
        };

        if exists {
            self.token = None;
        } else {
            self.hint.expression(true);
        }
        TokenResult::Some(())
    }

    pub fn with<'parser>(&'parser mut self, flag: Flag) -> ParserProxy<'parser, 'code, T> {
        self.push_flags(flag, true);

        ParserProxy::new(self)
    }

    pub fn without<'parser>(&'parser mut self, flag: Flag) -> ParserProxy<'parser, 'code, T> {
        self.push_flags(flag, false);

        ParserProxy::new(self)
    }

    fn push_flags(&mut self, flag: Flag, val: bool) {
        self.flags_stack.push(self.flags);

        // println!("pushed {:?} as {:?}", flag, val);

        match flag {
            Flag::In => { self.flags.allow_in = val; }
            Flag::Yield => { self.flags.allow_yield = val; }
            Flag::Await => { self.flags.allow_await = val; }
            Flag::Return => { self.flags.allow_return = val; }
            Flag::Default => { self.flags.allow_default = val; }
            Flag::Module => { self.flags.is_module = val; }
            Flag::Strict => { self.flags.is_strict = val; }
            Flag::Template => {
                self.flags.expect_template = val;
                self.hint = self.hint.template(val);
            }
            Flag::Noop => { /* useful if you want to consistently pass a ParserProxy */}
        }
    }
    fn pop_flags(&mut self) {
        self.flags = self.flags_stack.pop().unwrap();
        self.hint = self.hint.template(self.flags.expect_template);
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
                t => {
                    self.hint = self.hint.expression(false);

                    // println!("{:?}", (line, t.clone()));
                    break (line, t);
                },
            }
        }
    }

    pub fn token(&mut self) -> &tokens::Token {
        self.token_and_line().1
    }

    pub fn pop(&mut self) -> tokens::Token<'code> {
        self.token.take().unwrap().token
    }

    fn token_and_line(&mut self) -> (bool, &tokens::Token) {
        if let Some(LookaheadResult { line, ref token }) = self.token {
            (line, token)
        } else {
            match self.lookahead.take() {
                Some(LookaheadResult { line, token }) => {
                    self.token = Some(LookaheadResult { line, token });
                    (line, &self.token.as_ref().unwrap().token)
                }
                _ => {
                    // TODO
                    let hint = self.hint;
                    let (line, token) = self.read_token(&hint);
                    self.token = Some(LookaheadResult {
                        line,
                        token,
                    });

                    (line, &self.token.as_ref().unwrap().token)
                }
            }
        }

        // match self.token {
        //     Some(LookaheadResult { line, ref token }) => (line, token),
        //     _ => unreachable!(),
        // }
    }

    pub fn no_line_terminator(&mut self) -> bool {
        !self.token_and_line().0
    }

    pub fn is_binding_identifier(&self, name: &str) -> bool {
        is_binding_identifier(&self.flags, name)
    }

    pub fn ident_lookahead(&mut self) -> Option<&LookaheadResult> {
        if let Some(ref lookahead) = self.lookahead {
            return Some(lookahead);
        }

        let flags = self.flags;
        let expect_expression = if let tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) = *self.token() {
            !is_binding_identifier(&flags, name)
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

    pub fn punc(&mut self, punc: tokens::PunctuatorToken) -> TokenResult<tokens::PunctuatorToken> {
        let same = match *self.token() {
            tokens::Token::Punctuator(ref p) if *p == punc => { true }
            _ => false
        };

        if same {
            if let tokens::Token::Punctuator(p) = self.pop() {
                TokenResult::Some(p)
            } else {
                unreachable!("already matched punc");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn numeric(&mut self) -> TokenResult<tokens::NumericLiteralToken> {
        let same = match *self.token() {
            tokens::Token::NumericLiteral(_) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::NumericLiteral(n) = self.pop() {
                TokenResult::Some(n)
            } else {
                unreachable!("already matched number");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn string(&mut self) -> TokenResult<tokens::StringLiteralToken<'code>> {
        let same = match *self.token() {
            tokens::Token::StringLiteral(_) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::StringLiteral(s) = self.pop() {
                TokenResult::Some(s)
            } else {
                unreachable!("already matched string");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn regex(&mut self) -> TokenResult<tokens::RegularExpressionLiteralToken<'code>> {
        let same = match *self.token() {
            tokens::Token::RegularExpressionLiteral(_) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::RegularExpressionLiteral(r) = self.pop() {
                TokenResult::Some(r)
            } else {
                unreachable!("already matched string");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn template(&mut self) -> TokenResult<tokens::TemplateToken<'code>> {
        let same = match *self.token() {
            tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::NoSubstitution, .. }) |
            tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::Head, .. }) => true,
            _ => false
        };

        if same {
            if let tokens::Token::Template(t) = self.pop() {
                TokenResult::Some(t)
            } else {
                unreachable!("already matched template");
            }
        } else {
            TokenResult::None
        }
    }
    pub fn template_tail(&mut self) -> TokenResult<tokens::TemplateToken<'code>> {
        let same = match *self.token() {
            tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::Middle, .. }) |
            tokens::Token::Template(tokens::TemplateToken { format: tokens::TemplateFormat::Tail, .. }) => true,
            _ => false
        };

        if same {
            if let tokens::Token::Template(t) = self.pop() {
                TokenResult::Some(t)
            } else {
                unreachable!("already matched template");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn binding_identifier(&mut self) -> TokenResult<tokens::IdentifierNameToken<'code>> {
        let flags = self.flags;

        let same = match *self.token() {
            tokens::Token::IdentifierName(ref v) if is_binding_identifier(&flags, &v.name) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::IdentifierName(ident) = self.pop() {
                TokenResult::Some(ident)
            } else {
                unreachable!("already matched ident");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn reference_identifier(&mut self) -> TokenResult<tokens::IdentifierNameToken<'code>> {
        self.binding_identifier()
    }

    pub fn label_identifier(&mut self) -> TokenResult<tokens::IdentifierNameToken<'code>> {
        self.binding_identifier()
    }

    pub fn keyword(&mut self, keyword: &'static str) -> TokenResult<tokens::IdentifierNameToken<'code>> {
        let same = match *self.token() {
            tokens::Token::IdentifierName(ref v) if &v.name == keyword => { true }
            _ => false
        };

        if same {
            if let tokens::Token::IdentifierName(ident) = self.pop() {
                TokenResult::Some(ident)
            } else {
                unreachable!("already matched keyword");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn identifier(&mut self) -> TokenResult<tokens::IdentifierNameToken<'code>> {
        let same = match *self.token() {
            tokens::Token::IdentifierName(_) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::IdentifierName(ident) = self.pop() {
                TokenResult::Some(ident)
            } else {
                unreachable!("already matched identifier");
            }
        } else {
            TokenResult::None
        }
    }

    pub fn eof(&mut self) -> TokenResult<tokens::EOFToken> {
        let same = match *self.token() {
            tokens::Token::EOF(_) => { true }
            _ => false
        };

        if same {
            if let tokens::Token::EOF(v) = self.pop() {
                TokenResult::Some(v)
            } else {
                unreachable!("already matched keyword");
            }
        } else {
            TokenResult::None
        }
    }
}

fn is_binding_identifier(flags: &GrammarFlags, s: &str) -> bool {
    // TODO: In strict mode specifically, 'arguments' and 'eval' aren't allowed as binding names,
    // just labels and references.

    match s {
        // Conditional keywords
        "yield" if !flags.allow_yield => false,
        "await" if !flags.allow_await => false,

        // Keywords
        "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger" | "default" | "delete" |
        "do" | "else" | "export" | "extends" | "finally" | "for" | "function" | "if" | "import" | "in" |
        "instanceof" | "new" | "return" | "super" | "switch" | "this" | "throw" | "try" | "typeof" |
        "var" | "void" | "while" | "with" | "yield" => false,

        // Strict Keywords
        "let" | "static" if flags.is_strict => false,

        // Future Keywords
        "enum" => false,

        // Future Strict Keywords
        "implements" | "package" | "protected" | "interface" | "private" | "public" if flags.is_strict => false,

        // Literals
        "null" | "true" | "false" => false,

        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let mut p = Parser {
            tok: "this;".into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],
            lookahead: None,
            token: None,
        };

        p.parse_module().unwrap();
    }
}
