extern crate ucd;

mod ast;

pub use ast::misc;
pub use ast::alias;
pub use ast::jsx;
pub use ast::flow;
pub use ast::expression;
pub use ast::statement;
pub use ast::declaration;
pub use ast::literal;


// pub mod tokenizer;
// pub mod parser;

// #[cfg(test)]
// mod tests {
//     use tokenizer::Tokenizer;

//     #[test]
//     fn it_should_run() {
//         // let tokens = Tokenizer::parse("one;'foo';`foo`;0.3;08.2;`a\\u{123}c`;");
//         let tokens = Tokenizer::parse("08.2;1.2e4;`a\\u{123}c`;");

//         println!("{:#?}", tokens);
//     }


//     #[test]
//     fn it_should_tokenize_operators() {}
// }
