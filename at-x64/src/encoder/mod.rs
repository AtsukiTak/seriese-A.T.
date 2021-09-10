mod bytecode;

use crate::{
    mem::Mem64,
    reg::{Reg32, Reg64},
    BytesAtMost,
};
use bytecode::{ByteCode, ModRM, Rex, Sib};

pub struct Encoder<R, RM> {
    rex_w: bool,
    opcode: BytesAtMost<3>,
    mod_rm: Option<(R, RM)>,
    imm: BytesAtMost<8>,
}

impl Encoder<Reg64, Mem64> {
    pub fn new() -> Self {
        Encoder {
            rex_w: false,
            opcode: BytesAtMost::new(0),
            mod_rm: None,
            imm: BytesAtMost::new(0),
        }
    }
}

impl<R: RegLike, RM: RegMemLike> Encoder<R, RM> {
    pub fn rex_w(mut self, rex_w: bool) -> Self {
        self.rex_w = rex_w;
        self
    }

    pub fn opcode(mut self, opcode: BytesAtMost<3>) -> Self {
        self.opcode = opcode;
        self
    }

    pub fn mod_rm<R2, RM2>(self, reg: R2, rm: RM2) -> Encoder<R2, RM2> {
        Encoder {
            rex_w: self.rex_w,
            opcode: self.opcode,
            mod_rm: Some((reg, rm)),
            imm: self.imm,
        }
    }

    pub fn imm(mut self, imm: BytesAtMost<8>) -> Self {
        self.imm = imm;
        self
    }

    pub fn encode(self) -> BytesAtMost<15> {
        let mut bytecode = ByteCode::new();

        // opcode
        bytecode.opcode = self.opcode;

        if self.rex_w {
            bytecode.rex.insert(Rex::new()).set_w(true);
        }

        // ModRM byte
        if let Some((reg_like, rm_like)) = self.mod_rm {
            let mut mod_rm = ModRM::new();
            mod_rm.set_reg(reg_like.reg());
            mod_rm.set_mode(rm_like.mode());
            mod_rm.set_rm(rm_like.rm());
            bytecode.mod_rm = Some(mod_rm);

            if reg_like.rex_r() {
                bytecode.rex.get_or_insert(Rex::new()).set_r(true);
            }

            if rm_like.rex_b() {
                bytecode.rex.get_or_insert(Rex::new()).set_b(true);
            }

            // SIB byte
            bytecode.sib = rm_like.sib();
            if rm_like.rex_x() {
                bytecode.rex.get_or_insert(Rex::new()).set_x(true);
            }

            // addr disp
            bytecode.addr_disp = rm_like.disp_bytes();
        }

        // imm
        if self.imm.len() == 8 && self.rex_w != false {
            panic!("64bit immediate value is only valid on 64bit operand mode");
        }
        bytecode.imm = self.imm;

        bytecode.to_bytes()
    }
}

pub trait RegLike {
    /// R bit of REX prefix
    fn rex_r(&self) -> bool;

    /// reg field of ModR/M byte
    fn reg(&self) -> u8;
}

pub trait RegMemLike {
    /// B bit of REX prefix
    fn rex_b(&self) -> bool;

    /// X bit of REX prefix
    fn rex_x(&self) -> bool;

    /// mode field of ModR/M byte
    fn mode(&self) -> u8;

    /// rm field of ModR/M byte
    fn rm(&self) -> u8;

    /// SIB byte if exists
    fn sib(&self) -> Option<Sib>;

    /// address displacement bytes
    fn disp_bytes(&self) -> BytesAtMost<4>;
}

impl RegLike for Reg64 {
    fn rex_r(&self) -> bool {
        self.is_extended()
    }

    fn reg(&self) -> u8 {
        self.register_code()
    }
}

impl RegLike for Reg32 {
    fn rex_r(&self) -> bool {
        self.is_extended()
    }

    fn reg(&self) -> u8 {
        self.register_code()
    }
}

/// in case of opcode expansion
impl RegLike for u8 {
    fn rex_r(&self) -> bool {
        false
    }

    fn reg(&self) -> u8 {
        *self
    }
}

impl RegMemLike for Reg64 {
    fn rex_b(&self) -> bool {
        self.is_extended()
    }

    fn rex_x(&self) -> bool {
        false
    }

    fn mode(&self) -> u8 {
        0b11
    }

    fn rm(&self) -> u8 {
        self.register_code()
    }

    fn sib(&self) -> Option<Sib> {
        None
    }

    fn disp_bytes(&self) -> BytesAtMost<4> {
        BytesAtMost::new(0)
    }
}

impl RegMemLike for Reg32 {
    fn rex_b(&self) -> bool {
        self.is_extended()
    }

    fn rex_x(&self) -> bool {
        false
    }

    fn mode(&self) -> u8 {
        0b11
    }

    fn rm(&self) -> u8 {
        self.register_code()
    }

    fn sib(&self) -> Option<Sib> {
        None
    }

    fn disp_bytes(&self) -> BytesAtMost<4> {
        BytesAtMost::new(0)
    }
}

impl RegMemLike for Mem64 {
    fn rex_b(&self) -> bool {
        match self {
            Mem64::RegOffset(reg, _) => reg.rex_b_bit(),
            Mem64::RipOffset(_) => false,
            Mem64::Sib {
                base: Some(base), ..
            } => base.rex_b_bit(),
            Mem64::Sib { base: None, .. } => false,
        }
    }

    fn rex_x(&self) -> bool {
        match self {
            Mem64::Sib { index, .. } => index.is_extended(),
            _ => false,
        }
    }

    fn mode(&self) -> u8 {
        use {Mem64::*, Reg64::*};

        match self {
            // treat [RBP] and [R13] as [RBP + 0] and [R13 + 0] resprectively
            RegOffset(RBP | R13, 0) => 0b01,
            RegOffset(_, 0) => 0b00,
            RegOffset(_, 1..=256) => 0b01,
            RegOffset(_, _) => 0b10,
            RipOffset(_) => 0b00,
            Sib { base: None, .. } => 0b00,
            Sib {
                base: Some(RBP | R13),
                disp: 0..=256,
                ..
            } => 0b01,
            Sib {
                base: Some(RBP | R13),
                ..
            } => 0b10,
            Sib { disp: 0, .. } => 0b00,
            Sib { disp: 1..=256, .. } => 0b01,
            Sib { .. } => 0b10,
        }
    }

    fn rm(&self) -> u8 {
        match self {
            Mem64::RegOffset(reg, _) => reg.rm(),
            Mem64::RipOffset(_) => 0b101,
            Mem64::Sib { .. } => 0b100,
        }
    }

    fn sib(&self) -> Option<Sib> {
        use Reg64::*;

        match self {
            // treat [RSP + disp] and [R12 + disp] as
            // [SIB + disp] setting each field to
            // scale = 0, index = RSP, base = RSP/R12.
            // Note that setting RSP to index field
            // means no-index.
            Mem64::RegOffset(RSP | R12, _) => Some(Sib::new(0, 0b100, 0b100)),
            Mem64::RegOffset(_, _) => None,
            Mem64::RipOffset(_) => None,
            Mem64::Sib {
                base: None,
                index,
                scale,
                ..
            } => Some(Sib::new(*scale, index.reg_bits(), 0b101)),
            Mem64::Sib {
                base: Some(base),
                index,
                scale,
                ..
            } => Some(Sib::new(*scale, index.reg_bits(), base.reg_bits())),
        }
    }

    fn disp_bytes(&self) -> BytesAtMost<4> {
        use {Mem64::*, Reg64::*};

        match self {
            RegOffset(RBP | R13, 0) => BytesAtMost::from(0 as u8),
            RegOffset(_, 0) => BytesAtMost::new(0),
            RegOffset(_, disp @ 1..=256) => BytesAtMost::from(*disp as u8),
            RegOffset(_, disp) => BytesAtMost::from(*disp),
            RipOffset(disp) => BytesAtMost::from(*disp),
            Sib {
                base: None, disp, ..
            } => BytesAtMost::from(*disp),
            Sib {
                base: Some(RBP | R13),
                disp: 0,
                ..
            } => BytesAtMost::from(0u8),
            Sib { disp: 0, .. } => BytesAtMost::new(0),
            Sib {
                disp: disp @ 1..=256,
                ..
            } => BytesAtMost::from(*disp as u8),
            Sib { disp, .. } => BytesAtMost::from(*disp),
        }
    }
}
