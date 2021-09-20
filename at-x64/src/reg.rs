use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    Reg64(Reg64),
    Reg32(Reg32),
    Reg16(Reg16),
    Reg8(Reg8),
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Reg::Reg64(reg) => reg.fmt(f),
            Reg::Reg32(reg) => reg.fmt(f),
            Reg::Reg16(reg) => reg.fmt(f),
            Reg::Reg8(reg) => reg.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg64 {
    /// Accumulator Register
    RAX,
    /// Destination Index Register
    RDI,
    /// Source Index Register
    RSI,
    /// Data Register
    RDX,
    /// Counter Register
    RCX,
    /// Stack Base Pointer Register
    RBP,
    /// Stack Pointer Register
    RSP,
    /// Base Register
    RBX,
    /// x64で追加されたレジスタ
    R8,
    /// x64で追加されたレジスタ
    R9,
    /// x64で追加されたレジスタ
    R10,
    /// x64で追加されたレジスタ
    R11,
    /// x64で追加されたレジスタ
    R12,
    /// x64で追加されたレジスタ
    R13,
    /// x64で追加されたレジスタ
    R14,
    /// x64で追加されたレジスタ
    R15,
}

impl Reg64 {
    pub fn register_code(&self) -> u8 {
        use Reg64::*;

        match self {
            RAX | R8 => 0b000,
            RCX | R9 => 0b001,
            RDX | R10 => 0b010,
            RBX | R11 => 0b011,
            RSP | R12 => 0b100,
            RBP | R13 => 0b101,
            RSI | R14 => 0b110,
            RDI | R15 => 0b111,
        }
    }

    pub fn is_extended(&self) -> bool {
        use Reg64::*;

        match self {
            RAX | RCX | RDX | RBX | RSP | RBP | RSI | RDI => false,
            R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 => true,
        }
    }

    pub fn rex_r_bit(&self) -> bool {
        self.is_extended()
    }

    pub fn rex_b_bit(&self) -> bool {
        self.is_extended()
    }

    /// mode bits of ModR/M field (2bit)
    pub fn mode_bits(&self) -> u8 {
        0b11
    }

    /// reg bits of ModR/M field (3bit)
    pub fn reg_bits(&self) -> u8 {
        self.register_code()
    }

    /// rm bits of ModR/M field (3bit)
    pub fn rm_bits(&self) -> u8 {
        self.register_code()
    }
}

impl Display for Reg64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Reg64::RAX => write!(f, "rax"),
            Reg64::RDI => write!(f, "rdi"),
            Reg64::RSI => write!(f, "rsi"),
            Reg64::RDX => write!(f, "rdx"),
            Reg64::RCX => write!(f, "rcx"),
            Reg64::RBP => write!(f, "rbp"),
            Reg64::RSP => write!(f, "rsp"),
            Reg64::RBX => write!(f, "rbx"),
            Reg64::R8 => write!(f, "r8"),
            Reg64::R9 => write!(f, "r9"),
            Reg64::R10 => write!(f, "r10"),
            Reg64::R11 => write!(f, "r11"),
            Reg64::R12 => write!(f, "r12"),
            Reg64::R13 => write!(f, "r13"),
            Reg64::R14 => write!(f, "r14"),
            Reg64::R15 => write!(f, "r15"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg32 {
    /// Lower 32-bits of RAX Register
    EAX,
    /// Lower 32-bits of RDI Register
    EDI,
    /// Lower 32-bits of RSI Register
    ESI,
    /// Lower 32-bits of RDX Register
    EDX,
    /// Lower 32-bits of RCX Register
    ECX,
    /// Lower 32-bits of RBP Register
    EBP,
    /// Lower 32-bits of RSP Register
    ESP,
    /// Lower 32-bits of RBX Register
    EBX,
    /// Lower 32-bits of R8 Register
    R8D,
    /// Lower 32-bits of R9 Register
    R9D,
    /// Lower 32-bits of R10 Register
    R10D,
    /// Lower 32-bits of R11 Register
    R11D,
    /// Lower 32-bits of R12 Register
    R12D,
    /// Lower 32-bits of R13 Register
    R13D,
    /// Lower 32-bits of R14 Register
    R14D,
    /// Lower 32-bits of R15 Register
    R15D,
}

impl Reg32 {
    pub fn to_reg64(self) -> Reg64 {
        match self {
            Reg32::EAX => Reg64::RAX,
            Reg32::EDI => Reg64::RDI,
            Reg32::ESI => Reg64::RSI,
            Reg32::EDX => Reg64::RDX,
            Reg32::ECX => Reg64::RCX,
            Reg32::EBP => Reg64::RBP,
            Reg32::ESP => Reg64::RSP,
            Reg32::EBX => Reg64::RBX,
            Reg32::R8D => Reg64::R8,
            Reg32::R9D => Reg64::R9,
            Reg32::R10D => Reg64::R10,
            Reg32::R11D => Reg64::R11,
            Reg32::R12D => Reg64::R12,
            Reg32::R13D => Reg64::R13,
            Reg32::R14D => Reg64::R14,
            Reg32::R15D => Reg64::R15,
        }
    }

    pub fn register_code(&self) -> u8 {
        self.to_reg64().register_code()
    }

    /// true if it is added at x64
    pub fn is_extended(&self) -> bool {
        self.to_reg64().is_extended()
    }
}

impl Display for Reg32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Reg32::EAX => write!(f, "eax"),
            Reg32::EDI => write!(f, "edi"),
            Reg32::ESI => write!(f, "esi"),
            Reg32::EDX => write!(f, "edx"),
            Reg32::ECX => write!(f, "ecx"),
            Reg32::EBP => write!(f, "ebp"),
            Reg32::ESP => write!(f, "esp"),
            Reg32::EBX => write!(f, "ebx"),
            Reg32::R8D => write!(f, "r8d"),
            Reg32::R9D => write!(f, "r9d"),
            Reg32::R10D => write!(f, "r10d"),
            Reg32::R11D => write!(f, "r11d"),
            Reg32::R12D => write!(f, "r12d"),
            Reg32::R13D => write!(f, "r13d"),
            Reg32::R14D => write!(f, "r14d"),
            Reg32::R15D => write!(f, "r15d"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg16 {
    /// Lower 16-bits of RAX Register
    AX,
    /// Lower 16-bits of RDI Register
    DI,
    /// Lower 16-bits of RSI Register
    SI,
    /// Lower 16-bits of RDX Register
    DX,
    /// Lower 16-bits of RCX Register
    CX,
    /// Lower 16-bits of RBP Register
    BP,
    /// Lower 16-bits of RSP Register
    SP,
    /// Lower 16-bits of RBX Register
    BX,
    /// Lower 16-bits of R8 Register
    R8W,
    /// Lower 16-bits of R9 Register
    R9W,
    /// Lower 16-bits of R10 Register
    R10W,
    /// Lower 16-bits of R11 Register
    R11W,
    /// Lower 16-bits of R12 Register
    R12W,
    /// Lower 16-bits of R13 Register
    R13W,
    /// Lower 16-bits of R14 Register
    R14W,
    /// Lower 16-bits of R15 Register
    R15W,
}

impl Reg16 {
    pub fn to_reg32(self) -> Reg32 {
        match self {
            Reg16::AX => Reg32::EAX,
            Reg16::DI => Reg32::EDI,
            Reg16::SI => Reg32::ESI,
            Reg16::DX => Reg32::EDX,
            Reg16::CX => Reg32::ECX,
            Reg16::BP => Reg32::EBP,
            Reg16::SP => Reg32::ESP,
            Reg16::BX => Reg32::EBX,
            Reg16::R8W => Reg32::R8D,
            Reg16::R9W => Reg32::R9D,
            Reg16::R10W => Reg32::R10D,
            Reg16::R11W => Reg32::R11D,
            Reg16::R12W => Reg32::R12D,
            Reg16::R13W => Reg32::R13D,
            Reg16::R14W => Reg32::R14D,
            Reg16::R15W => Reg32::R15D,
        }
    }

    pub fn register_code(&self) -> u8 {
        self.to_reg32().register_code()
    }

    /// true if it is added at x64
    pub fn is_extended(&self) -> bool {
        self.to_reg32().is_extended()
    }
}

impl Display for Reg16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Reg16::AX => write!(f, "ax"),
            Reg16::DI => write!(f, "di"),
            Reg16::SI => write!(f, "si"),
            Reg16::DX => write!(f, "dx"),
            Reg16::CX => write!(f, "cx"),
            Reg16::BP => write!(f, "bp"),
            Reg16::SP => write!(f, "sp"),
            Reg16::BX => write!(f, "bx"),
            Reg16::R8W => write!(f, "r8w"),
            Reg16::R9W => write!(f, "r9w"),
            Reg16::R10W => write!(f, "r10w"),
            Reg16::R11W => write!(f, "r11w"),
            Reg16::R12W => write!(f, "r12w"),
            Reg16::R13W => write!(f, "r13w"),
            Reg16::R14W => write!(f, "r14w"),
            Reg16::R15W => write!(f, "r15w"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg8 {
    /// Lower 8-bits of RAX Register
    AL,
    /// Lower 8-bits of RDI Register
    DIL,
    /// Lower 8-bits of RSI Register
    SIL,
    /// Lower 8-bits of RDX Register
    DL,
    /// Lower 8-bits of RCX Register
    CL,
    /// Lower 8-bits of RBP Register
    BPL,
    /// Lower 8-bits of RSP Register
    SPL,
    /// Lower 8-bits of RBL Register
    BL,
    /// Lower 8-bits of R8 Register
    R8B,
    /// Lower 8-bits of R9 Register
    R9B,
    /// Lower 8-bits of R10 Register
    R10B,
    /// Lower 8-bits of R11 Register
    R11B,
    /// Lower 8-bits of R12 Register
    R12B,
    /// Lower 8-bits of R13 Register
    R13B,
    /// Lower 8-bits of R14 Register
    R14B,
    /// Lower 8-bits of R15 Register
    R15B,
}

impl Display for Reg8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Reg8::AL => write!(f, "al"),
            Reg8::DIL => write!(f, "dil"),
            Reg8::SIL => write!(f, "sil"),
            Reg8::DL => write!(f, "dl"),
            Reg8::CL => write!(f, "cl"),
            Reg8::BPL => write!(f, "bpl"),
            Reg8::SPL => write!(f, "spl"),
            Reg8::BL => write!(f, "bl"),
            Reg8::R8B => write!(f, "r8b"),
            Reg8::R9B => write!(f, "r9b"),
            Reg8::R10B => write!(f, "r10b"),
            Reg8::R11B => write!(f, "r11b"),
            Reg8::R12B => write!(f, "r12b"),
            Reg8::R13B => write!(f, "r13b"),
            Reg8::R14B => write!(f, "r14b"),
            Reg8::R15B => write!(f, "r15b"),
        }
    }
}
