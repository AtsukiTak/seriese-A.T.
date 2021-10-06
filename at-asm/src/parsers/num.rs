use super::{ParseError, ParseStr};
use std::any::type_name;

macro_rules! impl_parse_str {
    ($($ty:ty)*) => {$(
        impl ParseStr for $ty {
            /// Never error.
            /// Because another number format possibly fit.
            fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
                if s.starts_with("0x") {
                    let s = s.trim_start_matches("0x");
                    Ok(Self::from_str_radix(s, 16).ok())
                } else {
                    Ok(Self::from_str_radix(s, 10).ok())
                }
            }

            fn parse_str(s: &str) -> Result<Self, ParseError> {
                match Self::try_parse_str(s)? {
                    Some(parsed) => Ok(parsed),
                    None => Err(ParseError::new(format!("{} is invalid {}", s, type_name::<$ty>())))
                }
            }
        }
    )*};
}

impl_parse_str!(u64 u32 u16 u8);
