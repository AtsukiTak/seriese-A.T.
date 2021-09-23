use super::ParseStr;
use at_x64::reg::{Reg16, Reg32, Reg64};
use std::process::exit;

impl ParseStr for Reg64 {
    fn try_parse_str(s: &str) -> Option<Self> {
        use Reg64::*;

        match s {
            "rax" => Some(RAX),
            "rdi" => Some(RDI),
            "rsi" => Some(RSI),
            "rdx" => Some(RDX),
            "rcx" => Some(RCX),
            "rbp" => Some(RBP),
            "rsp" => Some(RSP),
            "rbx" => Some(RBX),
            "r8" => Some(R8),
            "r9" => Some(R9),
            "r10" => Some(R10),
            "r11" => Some(R11),
            "r12" => Some(R12),
            "r13" => Some(R13),
            "r14" => Some(R14),
            "r15" => Some(R15),
            _ => None,
        }
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(reg) => reg,
            None => {
                eprintln!("error: invalid 64bit register : {}", s);
                exit(1);
            }
        }
    }
}

impl ParseStr for Reg32 {
    fn try_parse_str(s: &str) -> Option<Self> {
        use Reg32::*;

        match s {
            "eax" => Some(EAX),
            "edi" => Some(EDI),
            "esi" => Some(ESI),
            "edx" => Some(EDX),
            "ecx" => Some(ECX),
            "ebp" => Some(EBP),
            "esp" => Some(ESP),
            "ebx" => Some(EBX),
            "r8d" => Some(R8D),
            "r9d" => Some(R9D),
            "r10d" => Some(R10D),
            "r11d" => Some(R11D),
            "r12d" => Some(R12D),
            "r13d" => Some(R13D),
            "r14d" => Some(R14D),
            "r15d" => Some(R15D),
            _ => None,
        }
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(reg) => reg,
            None => {
                eprintln!("error: invalid 32bit register : {}", s);
                exit(1);
            }
        }
    }
}

impl ParseStr for Reg16 {
    fn try_parse_str(s: &str) -> Option<Self> {
        use Reg16::*;

        match s {
            "ax" => Some(AX),
            "di" => Some(DI),
            "si" => Some(SI),
            "dx" => Some(DX),
            "cx" => Some(CX),
            "bp" => Some(BP),
            "sp" => Some(SP),
            "bx" => Some(BX),
            "r8w" => Some(R8W),
            "r9w" => Some(R9W),
            "r10w" => Some(R10W),
            "r11w" => Some(R11W),
            "r12w" => Some(R12W),
            "r13w" => Some(R13W),
            "r14w" => Some(R14W),
            "r15w" => Some(R15W),
            _ => None,
        }
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(reg) => reg,
            None => {
                eprintln!("error: invalid 16bit register : {}", s);
                exit(1);
            }
        }
    }
}
