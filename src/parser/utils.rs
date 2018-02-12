use std::result;
use std::fmt;
use std::error;
use failure::Fail;
use failure::Error;

pub struct NotFound;

pub type Result<T> = result::Result<T, Error>;
pub type OptResult<T> = Result<TokenResult<T>>;

pub type TokenResult<T> = result::Result<T, NotFound>;


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
            Err(NotFound) => {
                try_sequence!($($t)*)
            }
            Ok(val) => {
                Ok(val)
            }
        }
    );
    () => {
        Err($crate::parser::utils::NotFound)
    };
}

// Execute an OptResult-returning function and "throw" if not found.
#[macro_export]
macro_rules! eat_token {
    ($e:expr) => (
        match $e {
            ::std::result::Result::Ok(val) => {
                val
            }
            ::std::result::Result::Err($crate::parser::utils::NotFound) => {
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
            ::std::result::Result::Ok(val) => {
                val
            }
            ::std::result::Result::Err($crate::parser::utils::NotFound) => {
                return ::std::result::Result::Ok(::std::result::Result::Err($crate::parser::utils::NotFound));
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
