pub fn parse_num(s: &str) -> u64 {
    u64::from_str_radix(s, 10).unwrap()
}
