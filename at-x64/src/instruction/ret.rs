use crate::{BytesAtMost, Encoder};

pub struct Ret();

impl Ret {
    pub fn new() -> Ret {
        Ret()
    }

    pub fn bytecode(&self) -> BytesAtMost<15> {
        Encoder::new().opcode(BytesAtMost::from([0xc3])).encode()
    }
}
