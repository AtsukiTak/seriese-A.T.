use crate::{
    encoder::Encoder,
    reg::{Reg16, Reg32, Reg64},
    BytesAtMost, Mem64,
};

pub struct Mov<Dst, Src>(pub Dst, pub Src);

impl<Dst, Src> Mov<Dst, Src> {
    pub fn new(dst: Dst, src: Src) -> Self {
        Mov(dst, src)
    }
}

impl Mov<Mem64, Reg64> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let (dst, src) = (self.0, self.1);

        Encoder::new()
            .rex_w(true)
            .opcode(BytesAtMost::from([0x89]))
            .mod_rm(src, dst)
            .encode()
    }
}

impl Mov<Reg64, Reg64> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let (dst, src) = (self.0, self.1);

        Encoder::new()
            .rex_w(true)
            .opcode(BytesAtMost::from([0x89]))
            .mod_rm(src, dst)
            .encode()
    }
}

impl Mov<Reg16, u16> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let (dst_reg, src_imm) = (self.0, self.1);

        Encoder::new()
            .prefix(0x66)
            .rex_b(dst_reg.is_extended())
            .opcode(BytesAtMost::from([0xB8 + dst_reg.register_code()]))
            .imm(BytesAtMost::from(src_imm))
            .encode()
    }
}

impl Mov<Reg32, u32> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let (dst_reg, src_imm) = (self.0, self.1);

        Encoder::new()
            .rex_b(dst_reg.is_extended())
            .opcode(BytesAtMost::from([0xB8 + dst_reg.register_code()]))
            .imm(BytesAtMost::from(src_imm))
            .encode()
    }
}

impl Mov<Reg64, u64> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let (dst_reg, src_imm) = (self.0, self.1);

        Encoder::new()
            .rex_w(true)
            .rex_b(dst_reg.is_extended())
            .opcode(BytesAtMost::from([0xB8 + dst_reg.register_code()]))
            .imm(BytesAtMost::from(src_imm))
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
            assert_eq!(origin.bytecode().bytes(), expected);
        }
    }

    #[test]
    fn test_mov_reg16_imm16() {
        use Reg16::*;

        let cases = [
            (Mov::new(AX, 42), vec![0x66, 0xB8, 0x2A, 0x00]),
            (Mov::new(R9W, 11), vec![0x66, 0x41, 0xB9, 0x0B, 0x00]),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().bytes(), expected);
        }
    }

    #[test]
    fn test_mov_reg32_imm32() {
        use Reg32::*;

        let cases = [
            (Mov::new(EDI, 420), vec![0xBF, 0xA4, 0x01, 0x00, 0x00]),
            (
                Mov::new(R15D, 109),
                vec![0x41, 0xBF, 0x6D, 0x00, 0x00, 0x00],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().bytes(), expected);
        }
    }

    #[test]
    fn test_mov_reg64_imm64() {
        use Reg64::*;

        let cases = [
            (
                Mov::new(RSP, 42),
                vec![0x48, 0xBC, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                Mov::new(R14, 42),
                vec![0x49, 0xBE, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().bytes(), expected);
        }
    }
}
