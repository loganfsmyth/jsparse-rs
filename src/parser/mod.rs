#[macro_use]
mod utils;

mod file;
mod module;
mod expressions;
mod declarations;
mod statements;
mod classes;
mod functions;

use time;

use std::ops::{Deref, DerefMut};
use tokenizer::{self, IntoTokenizer, Tokenizer, Hint, tokens};
use self::utils::TokenResult;

pub fn parse_root<'code, T: 'code, P>(t: T) -> P
where
    T: IntoTokenizer<'code>,
    P: FromTokenizer
{
    FromTokenizer::from_tokenizer(t)
}

pub trait FromTokenizer {
    fn from_tokenizer<'code, T: IntoTokenizer<'code> + 'code>(t: T) -> Self;
}

struct Test {}

impl FromTokenizer for Test {
    fn from_tokenizer<'code, T: IntoTokenizer<'code> + 'code>(t: T) -> Test {
        let mut p = Parser {
            tok: t.into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],

            tokens: Default::default(),
            index: 0,
            count: 0,
        };

        p.parse_module().unwrap();

        Test {}
    }
}


impl FromTokenizer for () {
    fn from_tokenizer<'code, T: IntoTokenizer<'code> + 'code>(t: T) -> () {
        let mut p = Parser {
            tok: t.into_tokenizer(),
            hint: Default::default(),
            flags: Default::default(),
            flags_stack: vec![],

            tokens: Default::default(),
            index: 0,
            count: 0,
        };

        println!("starting");

        let t_start = time::precise_time_ns();

        p.parse_module().unwrap();

        let total_parse = time::precise_time_ns() - t_start;

        println!("Total parsing time: {}ms", total_parse as f64 / 1e6);

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

#[derive(Debug, Default, Clone, PartialEq)]
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
    tokens: [LookaheadResult<'code>; 2],
    index: u8,
    count: u8,

    // Track first location of non-object-literal single-name assignment
    // { foo = 4 } = {} -> allowed in patterns, not objects
    // cover_pattern


    // Track if there was a rest element in the expression
    // cover_arrow_formal

    // cover_async_arrow_formal
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
                // println!("No ASI");
                return TokenResult::None;
            }
        };

        // println!("Performing ASI");
        if exists {
            self.pop();
        } else {
            self.expect_expression();
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

    pub fn token(&mut self) -> &tokens::Token {
        self.token_and_line().1
    }

    pub fn pop(&mut self) -> tokens::Token<'code> {
        let tok = ::std::mem::replace(
            &mut self.tokens[self.index as usize].token,
            tokens::EOFToken {}.into(),
        );

        self.index = (self.index + 1) % 2;
        self.count -= 1;

        // println!("{:?} {} {} => {:?}", tok, self.count, self.index, self.tokens);

        tok
    }

    fn token_and_line(&mut self) -> (bool, &tokens::Token) {
        if self.count == 0 {
            read_token(&mut self.tok, &mut self.hint, &mut self.tokens[0]);
            self.index = 0;
            self.count += 1;

            // println!("Populated {:?}", self.tokens[0]);
        }

        let LookaheadResult { line, ref token } = self.tokens[self.index as usize];
        (line, token)
    }

    pub fn no_line_terminator(&mut self) -> bool {
        !self.token_and_line().0
    }

    pub fn is_binding_identifier(&self, name: &str) -> bool {
        is_binding_identifier(&self.flags, name)
    }

    pub fn ident_lookahead(&mut self) -> Option<&LookaheadResult> {
        self.token_and_line();

        let look_index = ((self.index + 1) % 2) as usize;

        if self.count < 2 {
            let flags = self.flags;
            let expect_expression = if let tokens::Token::IdentifierName(tokens::IdentifierNameToken { ref name }) = *self.token() {
                !is_binding_identifier(&flags, name)
            } else {
                return None;
            };

            let mut hint = self.hint.expression(expect_expression);

            read_token(&mut self.tok, &mut hint, &mut self.tokens[look_index]);
            self.count += 1;
        }

        Some(&self.tokens[look_index])
    }

    pub fn punc(&mut self, punc: tokens::PunctuatorToken) -> TokenResult<tokens::PunctuatorToken> {
        let same = match *self.token() {
            tokens::Token::Punctuator(ref p) if *p == punc => { true }
            _ => false
        };

        // println!("Punc: {:?}, got {:?}", punc, same);

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

        // println!("checking {:?}, got {:?}", keyword, same);

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

fn read_token<'code, T>(tok: &mut T, hint: &mut Hint, out: &mut LookaheadResult<'code>)
where
    T: Tokenizer<'code> + 'code
{
    out.line = false;

    let mut pos = tokenizer::TokenRange::default();
    loop {
        // TODO: Explore allocating a token and passing it into next_token

        tok.next_token(hint, (&mut out.token, &mut pos));
        match out.token {
            tokens::Token::Whitespace(_) => {}
            tokens::Token::LineTerminator(_) => {
                out.line = true;
            }
            tokens::Token::Comment(_) => {
                if pos.start.line != pos.end.line {
                    out.line = true;
                }
            }
            _ => {
                *hint = hint.expression(false);

                // println!("{:?}", (line, t.clone()));
                break;
            },
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
    use parser;

    #[test]
    fn it_parses() {
        parser::Test::from_tokenizer("this;");
    }
}
