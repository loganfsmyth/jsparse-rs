use std::result;
use std::fmt;
use std::error;
use failure::Fail;
use failure::Error;

pub type Result<T> = result::Result<T, Error>;
pub type OptResult<T> = Result<Option<T>>;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Fail)]
pub struct ParseError { }
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnexpectedToken")
    }
}

// trait OptionExt {
//     fn eat(self) -> OptResult<tokens::Token>;
//     fn try(self) -> OptResult<tokens::Token>;
// }

// impl OptionExt for Option<tokens::Token> {
//     fn eat(self) -> OptResult<tokens::Token> {
//         match self {
//             Some(t) => Ok(t),
//             None => ParseError {}.into()
//         }
//     }
//     fn try(self) -> OptResult<tokens::Token> {
//         match self {
//             Some(t) => Ok(t),
//             None => ParseError {}.into()
//         }
//     }

// }


// Try a list of OptResult-returning functions, in order.
#[macro_export]
macro_rules! try_sequence {
    ($e:expr, $($t:tt)*) => (
        match $e {
            ::std::result::Result::Ok(None) => {
                try_sequence!($($t)*)
            }
            ::std::result::Result::Ok(Some(val)) => {
                ::std::result::Result::Ok(Some(val))
            }
            ::std::result::Result::Err(e) => {
                ::std::result::Result::Err(From::from(e))
            }
        }
    );
    () => {
        ::std::result::Result::Ok(None)
    };
}

// Execute an OptResult-returning function and "throw" if not found.
#[macro_export]
macro_rules! eat_token {
    ($e:expr) => (
        match $e {
            ::std::option::Option::Some(val) => {
                val
            }
            ::std::option::Option::None => {
                return ::std::result::Result::Err(From::from($crate::parser::utils::ParseError {}));
            }
        }
    );
}

// Execute an OptResult-returning function, and do nothing if not found.
#[macro_export]
macro_rules! try_token {
    ($e:expr) => (
        match $e {
            ::std::option::Option::Some(val) => {
                val
            }
            ::std::option::Option::None => {
                return ::std::result::Result::Ok(None);
            }
        }
    );
}

// Execute an OptResult-returning function and "throw" if not found.
#[macro_export]
macro_rules! eat_fn {
    ($e:expr) => (
        match $e {
            ::std::result::Result::Ok(Some(val)) => {
                val
            }
            ::std::result::Result::Ok(None) => {
                return ::std::result::Result::Err(From::from($crate::parser::utils::ParseError {}));
            }
            ::std::result::Result::Err(e) => {
                return ::std::result::Result::Err(From::from(e));
            }
        }
    );
}

// Execute an OptResult-returning function, and do nothing if not found.
#[macro_export]
macro_rules! try_fn {
    ($e:expr) => (
        match $e {
            ::std::result::Result::Ok(Some(v)) => {
                v
            }
            ::std::result::Result::Ok(None) => {
                return ::std::result::Result::Ok(None)
            }
            ::std::result::Result::Err(e) => {
                return ::std::result::Result::Err(From::from(e));
            }
        }
    );
}
