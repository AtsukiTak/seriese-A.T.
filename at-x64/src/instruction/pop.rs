use crate::{
    reg::{Reg16, Reg64},
    BytesAtMost, Encoder, Instruction,
};

pub struct Pop<T>(T);

impl<T> Pop<T> {
    pub fn new(operand: T) -> Self {
        Pop(operand)
    }
}

impl Instruction for Pop<Reg16> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Pop(operand) = *self;

        Encoder::new()
            .prefix(0x66)
            .rex_b(operand.is_extended())
            .opcode([0x58 + operand.register_code()])
            .encode()
    }
}

impl Instruction for Pop<Reg64> {
    fn bytecode(&self) -> BytesAtMost<15> {
        let Pop(operand) = *self;

        Encoder::new()
            .rex_b(operand.is_extended())
            .opcode([0x58 + operand.register_code()])
            .encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_reg16() {
        use Reg16::*;

        let cases = [
            (Pop::new(DI), vec![0x66, 0x5f]),
            (Pop::new(R15W), vec![0x66, 0x41, 0x5f]),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected)
        }
    }

    #[test]
    fn test_pop_reg64() {
        use Reg64::*;

        let cases = [
            (Pop::new(RDI), vec![0x5f]),
            (Pop::new(R15), vec![0x41, 0x5f]),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().as_ref(), expected)
        }
    }
}
