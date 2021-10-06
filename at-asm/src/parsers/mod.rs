mod data;
mod global_symbol;
mod instruction;
mod line;
mod num;
mod reg;
mod section;

pub use line::Line;
pub use section::Section;

pub trait ParseStr: Sized {
    /// `Ok(Some)` if parsed string is valid for thist format.
    /// `Ok(None)` if parsed string is invalid for this format but maybe valid for another format.
    /// `Err` if parse string is invalid for any format.
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError>;

    fn parse_str(s: &str) -> Result<Self, ParseError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new<I>(msg: I) -> Self
    where
        String: From<I>,
    {
        ParseError {
            msg: String::from(msg),
        }
    }

    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }
}
