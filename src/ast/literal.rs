use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword};
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
node!(#[derive(Default)] pub struct Null {});
impl NodeDisplay for Null {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Null);
        Ok(())
    }
}


// true/false
node!(pub struct Boolean {
    pub value: bool,
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
impl From<bool> for Boolean {
    fn from(value: bool) -> Boolean {
        Boolean {
            value,
            position: None,
        }
    }
}


// 12
node!(pub struct Numeric {
    pub raw: Option<string::String>,
    pub value: f64,
});
impl NodeDisplay for Numeric {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.number(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl From<f64> for Numeric {
    fn from(value: f64) -> Numeric {
        Numeric {
            value,
            raw: None,
            position: None,
        }
    }
}


// "foo"
node!(pub struct String {
    pub raw: Option<string::String>,
    pub value: string::String,
});
impl NodeDisplay for String {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.string(&self.value, self.raw.as_ref().map(string::String::as_str))
    }
}
impl<T: Into<string::String>> From<T> for String {
    fn from(value: T) -> String {
        String {
            value: value.into(),
            raw: None,
            position: None,
        }
    }
}


// /foo/g
node!(pub struct RegExp {
    pub value: string::String,
    pub flags: Vec<char>,
});
impl NodeDisplay for RegExp {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.regexp(&self.value, &self.flags)
    }
}
