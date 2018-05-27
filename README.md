## jsparse

This project contains a partially-complete JavaScript parser implemented in Rust. I wrote it because I wanted to learn Rust and was curious what kind of performance I could expect from a systems language compared to existing JS parsers. It could certainly stabilize eventually, but I don't think I'd recommend anyone use this code currently.

This project contains semi-functional implementations of:

* [A JS parser](src/parser) - "Works" in that it parses plenty of structures, but doesn't actually create an AST yet, just directs the tokenizer based on the next expected token
* [A JS tokenizer](src/tokenizer) - Pretty much works for tokenizing slices
* [A AST structure](src/ast) - Defined, but otherwise unused in the current parser.
