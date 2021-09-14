use super::ParseStr;
use at_x64::reg::{Reg32, Reg64};

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
                std::process::exit(1);
            }
        }
    }
}
