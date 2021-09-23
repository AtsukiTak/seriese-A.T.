mod lea;
mod mov;
mod pop;
mod push;
mod ret;
mod syscall;

pub use lea::Lea;
pub use mov::Mov;
pub use pop::Pop;
pub use push::Push;
pub use ret::Ret;
pub use syscall::Syscall;

use crate::BytesAtMost;

pub trait Instruction {
    fn bytecode(&self) -> BytesAtMost<15>;
}
