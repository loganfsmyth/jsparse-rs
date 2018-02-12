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

// Try a list of OptResult-returning functions, in order.
#[macro_export]
macro_rules! try_sequence {
    ($e:expr, $($t:tt)*) => (
        match $e {
            None => {
                try_sequence!($($t)*)
            }
            Some(val) => {
                Some(val)
            }
        }
    );
    () => {
        None
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
        eat_token!($e)
    );
}

// Execute an OptResult-returning function, and do nothing if not found.
#[macro_export]
macro_rules! try_fn {
    ($e:expr) => (
        try_token!($e)
    );
}
