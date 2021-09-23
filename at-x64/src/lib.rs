mod bytes;
mod encoder;

pub mod instruction;
pub mod mem;
pub mod reg;

pub(crate) use encoder::Encoder;
// just for usefulness
pub(crate) use instruction::Instruction;

pub use bytes::BytesAtMost;
