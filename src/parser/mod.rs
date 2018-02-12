#[macro_use]
mod utils;

mod file;
mod module;
mod expressions;
mod declarations;
mod statements;
mod classes;
mod functions;

use failure;
use failure::Fail;

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



use std::ops::{Deref, DerefMut};
use tokenizer::{IntoTokenizer, Tokenizer, Hint, tokens};

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

        p.parse_module().unwrap()
    }
}

pub struct ParserProxy<'parser, 'code: 'parser, T: Tokenizer<'code> + 'code>(&'parser mut Parser<'code, T>);

impl<'parser, 'code, T: Tokenizer<'code>> ParserProxy<'parser, 'code, T>
{
    fn new(p: &'parser mut Parser<'code, T>, flag: Flag, val: bool) -> ParserProxy<'parser, 'code, T> {
        p.push_flags(flag, val);

        ParserProxy(p)
    }

    pub fn with<'p>(&'p mut self, flag: Flag) -> ParserProxy<'p, 'code, T> {
        ParserProxy::new(self.0, flag, true)
    }

    pub fn without<'p>(&'p mut self, flag: Flag) -> ParserProxy<'p, 'code, T> {
        ParserProxy::new(self.0, flag, false)
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
        self.hint.expression(true);
    }

    pub fn unexpected(&self) -> utils::OptResult<()> {
        Err(utils::ParseError {}.into())
    }

    pub fn semicolon(&mut self) -> Option<()> {
        self.semicolon_inner(false)
    }
    pub fn semicolon_dowhile(&mut self) -> Option<()> {
        self.semicolon_inner(true)
    }
    fn semicolon_inner(&mut self, was_do_while: bool) -> Option<()> {
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
                return None;
            }
        };

        if exists {
            self.token = None;
        } else {
            self.hint.expression(true);
        }
        Some(())
    }

    pub fn with<'parser>(&'parser mut self, flags: Flag) -> ParserProxy<'parser, 'code, T> {
        ParserProxy::new(self, flags, true)
    }

    pub fn without<'parser>(&'parser mut self, flags: Flag) -> ParserProxy<'parser, 'code, T> {
        ParserProxy::new(self, flags, true)
    }

    fn push_flags(&mut self, flags: Flag, val: bool) {
        self.flags_stack.push(self.flags);

        match flags {
            Flag::In => { self.flags.allow_in = val; }
            Flag::Yield => { self.flags.allow_yield = val; }
            Flag::Await => { self.flags.allow_await = val; }
            Flag::Return => { self.flags.allow_return = val; }
            Flag::Default => { self.flags.allow_default = val; }
            Flag::Module => { self.flags.is_module = val; }
            Flag::Strict => { self.flags.is_strict = val; }
            Flag::Noop => { /* useful if you want to consistently pass a ParserProxy */}
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
                t => {
                    println!("{:?}", (line, t.clone()));
                    break (line, t);
                },
            }
        }
    }

    fn try_token<V, F: FnOnce(tokens::Token<'code>) -> Result<V, tokens::Token<'code>>>(
        &mut self,
        handler: F,
    ) -> Option<V> {
        self.token_and_line();

        let LookaheadResult { line, token } = self.token.take().unwrap();

        match handler(token) {
            Ok(v) => Some(v),
            Err(result) => {
                self.token = LookaheadResult { line, token: result }.into();
                None
            }
        }
    }

    fn token(&mut self) -> &tokens::Token {
        self.token_and_line().1
    }

    fn token_and_line(&mut self) -> (bool, &tokens::Token) {
        if let None = self.token {
            if let Some(ahead) = self.lookahead.take() {
                self.token = Some(ahead);
                self.lookahead = None;
            } else {
                // TODO
                let hint = self.hint;
                let (line, token) = self.read_token(&hint);
                self.token = Some(LookaheadResult {
                    line,
                    token,
                });
            }
        }

        match self.token {
            Some(LookaheadResult { line, ref token }) => (line, token),
            _ => unreachable!(),
        }
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

    pub fn punc(&mut self, punc: tokens::PunctuatorToken) -> Option<tokens::PunctuatorToken> {
        self.try_token(|t| {
            match t {
                tokens::Token::Punctuator(p) => {
                    if p == punc {
                        Ok(p)
                    } else {
                        Err(p.into())
                    }
                }
                v => Err(v),
            }
        })
    }

    pub fn numeric(&mut self) -> Option<tokens::NumericLiteralToken> {
        self.try_token(|t| {
            match t {
                tokens::Token::NumericLiteral(n) => {
                    Ok(n)
                }
                v => Err(v),
            }
        })
    }

    pub fn string(&mut self) -> Option<tokens::StringLiteralToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::StringLiteral(n) => {
                    Ok(n)
                }
                v => Err(v),
            }
        })
    }

    pub fn regex(&mut self) -> Option<tokens::RegularExpressionLiteralToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::RegularExpressionLiteral(r) => {
                    Ok(r)
                }
                v => Err(v),
            }
        })
    }

    pub fn template(&mut self) -> Option<tokens::TemplateToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::Template(t @ tokens::TemplateToken { format: tokens::TemplateFormat::NoSubstitution, .. }) |
                tokens::Token::Template(t @ tokens::TemplateToken { format: tokens::TemplateFormat::Head, .. }) => {
                    Ok(t)
                }
                v => {
                    Err(v)
                }
            }
        })
    }
    pub fn template_tail(&mut self) -> Option<tokens::TemplateToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::Template(t @ tokens::TemplateToken { format: tokens::TemplateFormat::Middle, .. }) |
                tokens::Token::Template(t @ tokens::TemplateToken { format: tokens::TemplateFormat::Tail, .. }) => {
                    Ok(t)
                }
                v => {
                    Err(v)
                }
            }
        })
    }

    pub fn binding_identifier(&mut self) -> Option<tokens::IdentifierNameToken<'code>> {
        let flags = self.flags;

        self.try_token(|t| {
            match t {
                tokens::Token::IdentifierName(v) => {
                    if is_binding_identifier(&flags, &v.name) {
                        Ok(v)
                    } else {
                        Err(v.into())
                    }
                }
                v => Err(v),
            }
        })
    }

    pub fn reference_identifier(&mut self) -> Option<tokens::IdentifierNameToken<'code>> {
        self.binding_identifier()
    }

    pub fn label_identifier(&mut self) -> Option<tokens::IdentifierNameToken<'code>> {
        self.binding_identifier()
    }

    pub fn keyword(&mut self, keyword: &str) -> Option<tokens::IdentifierNameToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::IdentifierName(v) => {
                    if v.name == keyword {
                        Ok(v)
                    } else {
                        Err(v.into())
                    }
                }
                v => Err(v),
            }
        })
    }

    pub fn identifier(&mut self) -> Option<tokens::IdentifierNameToken<'code>> {
        self.try_token(|t| {
            match t {
                tokens::Token::IdentifierName(v) => {
                    Ok(v)
                }
                v => Err(v),
            }
        })
    }

    pub fn eof(&mut self) -> Option<tokens::EOFToken> {
        self.try_token(|t| {
            match t {
                tokens::Token::EOF(v) => {
                    Ok(v)
                }
                v => Err(v),
            }
        })
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
