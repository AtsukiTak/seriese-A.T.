mod instruction;
mod line;
mod num;
mod reg;

pub use line::Line;

pub fn parse<T>(s: &str) -> T
where
    T: ParseStr,
{
    T::parse_str(s)
}

pub fn try_parse<T>(s: &str) -> Option<T>
where
    T: ParseStr,
{
    T::try_parse_str(s)
}

pub trait ParseStr: Sized {
    fn try_parse_str(s: &str) -> Option<Self>;

    fn parse_str(s: &str) -> Self;
}
