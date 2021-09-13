pub mod num;

pub fn parse<T>(s: &str) -> T
where
    T: ParseStr,
{
    T::parse_str(s)
}

pub trait ParseStr {
    fn parse_str(s: &str) -> Self;
}
