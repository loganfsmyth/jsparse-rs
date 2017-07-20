extern crate ucd;

pub mod tokenizer;

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
