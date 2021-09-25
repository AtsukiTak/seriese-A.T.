use super::ParseStr;
use std::{any::type_name, process::exit};

macro_rules! impl_parse_str {
    ($($ty:ty)*) => {$(
        impl ParseStr for $ty {
            fn try_parse_str(s: &str) -> Option<Self> {
                if s.starts_with("0x") {
                    let s = s.trim_start_matches("0x");
                    Self::from_str_radix(s, 16).ok()
                } else {
                    Self::from_str_radix(s, 10).ok()
                }
            }

            fn parse_str(s: &str) -> Self {
                match Self::try_parse_str(s) {
                    Some(parsed) => parsed,
                    None => {
                        eprintln!("{} is invalid {}", s, type_name::<$ty>());
                        exit(1);
                    }
                }
            }
        }
    )*};
}

impl_parse_str!(u64 u32 u16 u8);
