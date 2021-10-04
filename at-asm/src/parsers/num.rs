use super::{ParseError, ParseStr};
use std::any::type_name;

macro_rules! impl_parse_str {
    ($($ty:ty)*) => {$(
        impl ParseStr for $ty {
            fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
                if !s.starts_with(|c: char| c.is_ascii_digit()) {
                    return Ok(None);
                }

                if s.starts_with("0x") {
                    let s = s.trim_start_matches("0x");
                    match Self::from_str_radix(s, 16) {
                        Ok(n) => Ok(Some(n)),
                        Err(_) => Err(ParseError::new(format!("{} is invalid {}", s, type_name::<$ty>()))),
                    }
                } else {
                    match Self::from_str_radix(s, 10) {
                        Ok(n) => Ok(Some(n)),
                        Err(_) => Err(ParseError::new(format!("{} is invalid {}", s, type_name::<$ty>()))),
                    }
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
