use crate::reg::Reg64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mem64 {
    /// [reg + u32]
    ///
    /// ## NOTE
    /// - [RBP],[R13]は[RBP+0],[R13+0] として扱う
    /// - [RSP+d],[R12+d]は[SIB+d]として扱い、
    ///   SIB.base = RSP/R12, SIB.index = RSP
    ///   を設定する
    RegOffset(Reg64, u32),
    /// [RIP + u32]
    RipOffset(u32),
    /// [base + disp + index * scale]
    ///
    /// ## NOTE
    /// indexフィールドにRSPを指定した場合、
    /// 「index無し」として扱われる。
    Sib {
        base: Option<Reg64>,
        disp: u32,
        index: Reg64,
        scale: u8, // 0 ~ 3,
    },
}

impl Mem64 {
    pub fn reg(reg: Reg64) -> Self {
        Mem64::RegOffset(reg, 0)
    }

    pub fn reg_offset(reg: Reg64, offset: u32) -> Self {
        Mem64::RegOffset(reg, offset)
    }

    pub fn rip_offset(offset: u32) -> Self {
        Mem64::RipOffset(offset)
    }

    pub fn sib(base: Option<Reg64>, disp: u32, index: Reg64, scale: u8) -> Self {
        assert!(scale <= 3);
        Mem64::Sib {
            base,
            disp,
            index,
            scale,
        }
    }
}
