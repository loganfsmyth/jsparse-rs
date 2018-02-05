use std::result;

pub type Result<T> = result::Result<T, ParseError>;
pub enum ParseError {
    UnexpectedToken,
}
impl From<ParseError> for InnerError {
    fn from(e: ParseError) -> Self {
        InnerError::Parse(e)
    }
}

pub type InnerResult<T> = result::Result<T, InnerError>;
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
    ($t:tt) => ({
        compile_error!("Unknown try sequence");
    });
}
