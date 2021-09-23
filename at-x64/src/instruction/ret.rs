use crate::{BytesAtMost, Encoder, Instruction};

pub struct Ret();

impl Ret {
    pub fn new() -> Ret {
        Ret()
    }
}

impl Instruction for Ret {
    fn bytecode(&self) -> BytesAtMost<15> {
        Encoder::new().opcode([0xc3]).encode()
    }
}
