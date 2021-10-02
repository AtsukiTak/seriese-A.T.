mod data;
mod instruction;
mod line;
mod num;
mod reg;

pub use line::{Line, Section};

pub trait ParseStr: Sized {
    fn try_parse_str(s: &str) -> Option<Self>;

    fn parse_str(s: &str) -> Self;
}
