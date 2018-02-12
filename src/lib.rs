extern crate ucd;
extern crate time;

#[macro_use] extern crate failure;

// #[macro_use]
// pub mod ast;

mod tokenizer;

pub use tokenizer::IntoTokenizer;
pub use tokenizer::Tokenizer;

pub mod parser;


// impl parser::Parser for ast::Ast {
//     type Root = ast::Ast;
//     type Directive = ();

//     fn root(directives: Vec<Self::Directive>) -> Self::Root {

//     }
// }


// struct TokenList {
//     tokens: Vec<tokenizer::tokens::Token>,
// }

// impl parser::FromParser for TokenList {

// }
