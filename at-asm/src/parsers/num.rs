use super::ParseStr;

impl ParseStr for u64 {
    fn parse_str(s: &str) -> Self {
        u64::from_str_radix(s, 10).unwrap()
    }
}
