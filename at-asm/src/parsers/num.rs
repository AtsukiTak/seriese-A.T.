use super::ParseStr;

impl ParseStr for u64 {
    fn try_parse_str(s: &str) -> Option<Self> {
        u64::from_str_radix(s, 10).ok()
    }

    fn parse_str(s: &str) -> Self {
        Self::try_parse_str(s).expect(format!("invalid 64bit number : {}", s).as_str())
    }
}
