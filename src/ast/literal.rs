use std::string;

use ast::{MaybeTokenPosition, KeywordData};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword};


// null
node!(#[derive(Default)] pub struct Null {});
impl NodeDisplay for Null {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Null);
        Ok(())
    }
}

#[cfg(test)]
mod tests_null {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(Null::default(), "null");
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

#[cfg(test)]
mod tests_boolean {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(Boolean::from(true), "true");
        assert_serialize!(Boolean::from(false), "false");
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

#[cfg(test)]
mod tests_numeric {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(Numeric::from(42.0), "42");
        assert_serialize!(Numeric::from(42.3), "42.3");
        assert_serialize!(Numeric::from(42.9), "42.9");
        assert_serialize!(Numeric::from(0.1), "0.1");
        assert_serialize!(Numeric::from(32e10), "320000000000");
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

#[cfg(test)]
mod tests_string {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(String::from("hello"), "'hello'");
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

#[cfg(test)]
mod tests_regexp {
    use super::*;

    #[test]
    fn it_prints() {
        assert_serialize!(
            RegExp {
                value: "hello".into(),
                flags: vec!['g', 'u'],
                position: None,
            },
            "/hello/gu"
        );
    }
}
