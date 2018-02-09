use std::result;

pub type Result<T> = result::Result<T, ParseError>;
pub type InnerResult<T> = result::Result<T, InnerError>;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken,
}
impl From<ParseError> for InnerError {
    fn from(e: ParseError) -> Self {
        InnerError::Parse(e)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnerError {
    NotFound,
    Parse(ParseError),
}

#[macro_export]
macro_rules! try_sequence {
    ($e:expr, $($t:tt)*) => (
        match $e {
            ::std::result::Result::Err(InnerError::NotFound) => {
                try_sequence!($($t)*)
            }
            ::std::result::Result::Err(InnerError::Parse(e)) => {
                ::std::result::Result::Err(From::from(e))
            }
            ::std::result::Result::Ok(val) => {
                ::std::result::Result::Ok(val)
            }
        }
    );
    () => {
        Err(InnerError::NotFound)
    };
}

#[macro_export]
macro_rules! eat {
    ($e:expr) => (
        match $e {
            ::std::result::Result::Ok(val) => {
                ::std::result::Result::Ok(val)
            }
            ::std::result::Result::Err(InnerError::NotFound) => {
                ::std::result::Result::Err($crate::parser::utils::ParseError::UnexpectedToken)
            }
            ::std::result::Result::Err(InnerError::Parse(e)) => {
                ::std::result::Result::Err(From::from(e))
            }
        }
    );
}
