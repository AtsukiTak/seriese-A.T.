use super::ParseStr;
use std::process::exit;

impl ParseStr for u64 {
    fn try_parse_str(s: &str) -> Option<Self> {
        u64::from_str_radix(s, 10).ok()
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(parsed) => parsed,
            None => {
                eprintln!("invalid 64bit number, {}", s);
                exit(1);
            }
        }
    }
}

impl ParseStr for u32 {
    fn try_parse_str(s: &str) -> Option<Self> {
        u32::from_str_radix(s, 10).ok()
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(parsed) => parsed,
            None => {
                eprintln!("invalid 32bit number, {}", s);
                exit(1);
            }
        }
    }
}

impl ParseStr for u16 {
    fn try_parse_str(s: &str) -> Option<Self> {
        u16::from_str_radix(s, 10).ok()
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(parsed) => parsed,
            None => {
                eprintln!("invalid 16bit number, {}", s);
                exit(1);
            }
        }
    }
}
