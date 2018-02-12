use std::result;
use std::fmt;
use std::error;
use failure::Fail;
use failure::Error;

pub struct NotFound;

pub type Result<T> = result::Result<T, Error>;
pub type OptResult<T> = Result<TokenResult<T>>;

// pub type TokenResult<T> = result::Result<T, NotFound>;

#[deny(unused_must_use)]
pub enum TokenResult<T> {
    Some(T),
    None
}

impl<T> TokenResult<T> {
    pub fn map<F, U>(self, op: F) -> TokenResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TokenResult::Some(value) => TokenResult::Some(op(value)),
            TokenResult::None => TokenResult::None,
        }
    }
}

// impl From<TokenResult> for O


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
            $crate::parser::utils::TokenResult::None => {
                try_sequence!($($t)*)
            }
            $crate::parser::utils::TokenResult::Some(val) => {
                $crate::parser::utils::TokenResult::Some(val)
            }
        }
    );
    () => {
        $crate::parser::utils::TokenResult::None
    };
}

// Execute an OptResult-returning function and "throw" if not found.
#[macro_export]
macro_rules! eat_value {
    ($e:expr) => (
        match $e {
            $crate::parser::utils::TokenResult::Some(val) => {
                val
            }
            $crate::parser::utils::TokenResult::None => {
                return ::std::result::Result::Err(From::from($crate::parser::utils::ParseError {}));
            }
        }
    );
}

// Execute an OptResult-returning function, and do nothing if not found.
#[macro_export]
macro_rules! try_value {
    ($e:expr) => (
        match $e {
            $crate::parser::utils::TokenResult::Some(val) => {
                val
            }
            $crate::parser::utils::TokenResult::None => {
                return ::std::result::Result::Ok($crate::parser::utils::TokenResult::None);
            }
        }
    );
}

// Execute an OptResult-returning function, and do nothing if not found.
#[macro_export]
macro_rules! opt_value {
    ($e:expr) => (
        match $e {
            $crate::parser::utils::TokenResult::Some(val) => {
                ::std::option::Option::Some(val)
            }
            $crate::parser::utils::TokenResult::None => {
                ::std::option::Option::None
            }
        }
    );
}
