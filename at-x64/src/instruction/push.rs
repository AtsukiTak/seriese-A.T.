use crate::{
    reg::{Reg16, Reg64},
    BytesAtMost, Encoder, Instruction,
};

pub struct Push<T>(T);

impl<T> Push<T> {
    pub fn new(item: T) -> Self {
        Push(item)
    }
}

impl Instruction for Push<u8> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Push(imm) = *self;

        Encoder::new().opcode([0x6A]).imm(imm).encode()
    }
}

impl Instruction for Push<u16> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Push(imm) = *self;

        Encoder::new().prefix(0x66).opcode([0x68]).imm(imm).encode()
    }
}

impl Instruction for Push<u32> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Push(imm) = *self;

        Encoder::new().opcode([0x68]).imm(imm).encode()
    }
}

/// Note that this instruction push 16bit data.
/// It breaks 64bit stack alignment.
impl Instruction for Push<Reg16> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Push(reg) = *self;

        Encoder::new()
            .prefix(0x66)
            .rex_b(reg.is_extended())
            .opcode([0x50 + reg.register_code()])
            .encode()
    }
}

impl Instruction for Push<Reg64> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Push(reg) = *self;

        Encoder::new()
            .rex_b(reg.is_extended())
            .opcode([0x50 + reg.register_code()])
            .encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_imm8() {
        let cases = [(Push::new(42_u8), vec![0x6A, 0x2A])];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected);
        }
    }

    #[test]
    fn test_push_imm16() {
        let cases = [(Push::new(420_u16), vec![0x66, 0x68, 0xA4, 0x01])];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected);
        }
    }

    #[test]
    fn test_push_imm32() {
        let cases = [(Push::new(420_u32), vec![0x68, 0xA4, 0x01, 0x00, 0x00])];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected);
        }
    }

    #[test]
    fn test_push_reg16() {
        use Reg16::*;

        let cases = [
            (Push::new(AX), vec![0x66, 0x50]),
            (Push::new(R10W), vec![0x66, 0x41, 0x52]),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected);
        }
    }

    #[test]
    fn test_push_reg64() {
        use Reg64::*;

        let cases = [
            (Push::new(RAX), vec![0x50]),
            (Push::new(R10), vec![0x41, 0x52]),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected);
        }
    }
}
