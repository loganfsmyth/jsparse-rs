use super::super::ast;

mod parser;

// struct Flags {
//   // "yield" toggle
//   generator: bool,

//   // "await" toggle
//   async: bool,

//   // "return" toggle
//   function: bool,

//   // "in" toggle
//   in_op: bool,
// }

pub fn module(code: &str) -> ast::Module {
    let p = Parser::with_flags(ParserFlags {
        module: true,
        ..Default::default()
    });

    p.module()
}

pub fn script(code: &str) -> ast::Script {
    let p = Parser::with_flags(ParserFlags { ..Default::default() });

    p.script()
}

pub fn script_annexb(code: &str) -> ast::Script {
    let p = Parser::with_flags(ParserFlags {
        annexb: true,
        ..Default::default()
    });

    p.script()
}




pub struct ExpressionContext {
    // Allow super.foo
    method: bool,

    // Allow super()
    subclass_constructor: bool,

    generator: bool,
    async: bool,
    function: bool,

    module: bool,
    strict: bool,
}

pub fn expression(code: &str, context: &ExpressionContext) -> ast::alias::Expression {}




pub struct EvalContext {
    // Allow super.foo
    method: bool,

    // Allow super()
    subclass_constructor: bool,

    generator: bool,
    async: bool,
    function: bool,

    module: bool,
    strict: bool,
}

pub fn eval(code: &str, context: &EvalContext) {}
