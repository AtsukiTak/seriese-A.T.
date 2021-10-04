mod data;
mod instruction;
mod line;
mod num;
mod reg;
mod section;

pub use line::Line;
pub use section::Section;

pub trait ParseStr: Sized {
    fn try_parse_str(s: &str) -> Option<Self>;

    fn parse_str(s: &str) -> Self;
}
