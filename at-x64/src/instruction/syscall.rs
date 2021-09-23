use crate::{BytesAtMost, Encoder};

pub struct Syscall();

impl Syscall {
    pub fn new() -> Self {
        Syscall()
    }

    pub fn bytecode(&self) -> BytesAtMost<15> {
        Encoder::new()
            .opcode(BytesAtMost::from([0x0f, 0x05]))
            .encode()
    }
}
