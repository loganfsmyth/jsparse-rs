use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, HasInOperator, FirstSpecialToken};
// use super::misc;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_prints_boolean() {
//         assert_serialize!(Boolean, { value: true }, "true");
//         assert_serialize!(Boolean, { value: false }, "false");
//     }

//     #[test]
//     fn it_prints_null() {
//         assert_serialize!(Null, { }, "null");
//     }

//     #[test]
//     fn it_prints_number() {
//         assert_serialize!(Numeric, { value: 42.0, raw: None }, "42");
//         assert_serialize!(Numeric, { value: 42.3, raw: None }, "42.3");
//         assert_serialize!(Numeric, { value: 42.9, raw: None }, "42.9");
//         assert_serialize!(Numeric, { value: 0.1, raw: None }, "0.1");
//         assert_serialize!(Numeric, { value: 32e10, raw: None }, "320000000000");
//     }

//     #[test]
//     fn it_prints_string() {
//         assert_serialize!(String, { value: "hello".into(), raw: None }, "'hello'");
//     }
//     #[test]
//     fn it_prints_regexp() {
//         assert_serialize!(RegExp, { value: "hello".into(), flags: vec!['g', 'u'] }, "/hello/gu");
//     }
// }

// null
node!(pub struct Null {});
impl NodeDisplay for Null {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Null);
        Ok(())
    }
}
impl HasInOperator for Null {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for Null {}


// true/false
node!(pub struct Boolean {
    value: bool,
});
impl NodeDisplay for Boolean {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        if self.value {
            f.keyword(Keyword::True);
        } else {
            f.keyword(Keyword::False);
        }
        Ok(())
    }
}
impl HasInOperator for Boolean {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for Boolean {}


// 12
node!(pub struct Numeric {
    raw: Option<string::String>,
    value: f64,
});
impl NodeDisplay for Numeric {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.number(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl HasInOperator for Numeric {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for Numeric {}


// "foo"
node!(pub struct String {
    raw: Option<string::String>,
    value: string::String,
});
impl NodeDisplay for String {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.string(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl HasInOperator for String {
    fn has_in_operator(&self) -> bool {
        false
    }
}
impl FirstSpecialToken for String {}


// /foo/g
node!(pub struct RegExp {
    value: string::String,
    flags: Vec<char>,
});
impl NodeDisplay for RegExp {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.regexp(&self.value, &self.flags)
    }
}
impl FirstSpecialToken for RegExp {}
impl HasInOperator for RegExp {}
