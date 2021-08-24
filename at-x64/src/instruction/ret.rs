use crate::{ByteCode, BytesAtMost};

pub struct Ret();

impl Ret {
    pub fn new() -> Ret {
        Ret()
    }

    pub fn bytecode(&self) -> ByteCode {
        let mut code = ByteCode::new();

        code.opcode = BytesAtMost::from([0xc3]);

        code
    }
}
