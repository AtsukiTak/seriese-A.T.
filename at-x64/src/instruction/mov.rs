use crate::{
    encoder::Encoder,
    reg::{Reg32, Reg64},
    ByteCode, BytesAtMost, Mem64,
};

pub struct Mov<Dst, Src>(pub Dst, pub Src);

impl<Dst, Src> Mov<Dst, Src> {
    pub fn new(dst: Dst, src: Src) -> Self {
        Mov(dst, src)
    }
}

impl Mov<Mem64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        Encoder::new()
            .rex_w(true)
            .opcode(BytesAtMost::from([0x89]))
            .mod_rm(src, dst)
            .encode()
    }
}

impl Mov<Reg64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        Encoder::new()
            .rex_w(true)
            .opcode(BytesAtMost::from([0x89]))
            .mod_rm(src, dst)
            .encode()
    }
}

impl Mov<Reg32, u32> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        Encoder::new()
            .opcode(BytesAtMost::from([0xB8 + dst.register_code()]))
            .imm(BytesAtMost::from(src))
            .encode()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mov_mem64_reg64() {
        use Reg64::*;

        let cases = [
            (Mov(Mem64::reg(RDI), RAX), vec![0x48, 0x89, 0x07]),
            (
                Mov(Mem64::reg_offset(RDI, 42), RAX),
                vec![0x48, 0x89, 0x47, 0x2A],
            ),
            (
                Mov(Mem64::rip_offset(42), RAX),
                vec![0x48, 0x89, 0x05, 0x2A, 0x00, 0x00, 0x00],
            ),
            (
                Mov(Mem64::sib(Some(RBP), 42, RAX, 3), R13),
                vec![0x4C, 0x89, 0x6C, 0xC5, 0x2A],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().to_bytes().bytes(), expected);
        }
    }
}
