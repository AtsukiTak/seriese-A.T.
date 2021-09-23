use crate::{BytesAtMost, Encoder, Mem64, Reg64};

pub struct Lea<Dst, Src>(Dst, Src);

impl<Dst, Src> Lea<Dst, Src> {
    pub fn new(dst: Dst, src: Src) -> Self {
        Lea(dst, src)
    }
}

impl Lea<Reg64, Mem64> {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        let Lea(dst_reg, src_mem) = *self;

        Encoder::new()
            .rex_w(true)
            .opcode(BytesAtMost::from([0x8D]))
            .mod_rm(dst_reg, src_mem)
            .encode()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use Reg64::*;

        let cases = [
            (Lea::new(RAX, Mem64::reg(RDI)), vec![0x48, 0x8D, 0x07]),
            (
                Lea::new(RAX, Mem64::reg_offset(RDI, 42)),
                vec![0x48, 0x8D, 0x47, 0x2A],
            ),
            (Lea::new(RSP, Mem64::reg(RSP)), vec![0x48, 0x8D, 0x24, 0x24]),
            (Lea::new(RAX, Mem64::reg(RSP)), vec![0x48, 0x8D, 0x04, 0x24]),
            (
                Lea::new(RDI, Mem64::rip_offset(42)),
                vec![0x48, 0x8D, 0x3D, 0x2A, 0x00, 0x00, 0x00],
            ),
            (
                Lea::new(RDI, Mem64::sib(Some(RAX), 0, RDI, 1)),
                vec![0x48, 0x8D, 0x3c, 0x78],
            ),
            (
                Lea::new(RDI, Mem64::sib(None, 0, RDI, 1)),
                vec![0x48, 0x8D, 0x3c, 0x7d, 0x00, 0x00, 0x00, 0x00],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().bytes(), expected);
        }
    }
}
