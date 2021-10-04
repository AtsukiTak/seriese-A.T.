use super::{ParseError, ParseStr};
use at_x64::reg::{Reg16, Reg32, Reg64};

impl ParseStr for Reg64 {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        use Reg64::*;

        let reg = match s {
            "rax" => RAX,
            "rdi" => RDI,
            "rsi" => RSI,
            "rdx" => RDX,
            "rcx" => RCX,
            "rbp" => RBP,
            "rsp" => RSP,
            "rbx" => RBX,
            "r8" => R8,
            "r9" => R9,
            "r10" => R10,
            "r11" => R11,
            "r12" => R12,
            "r13" => R13,
            "r14" => R14,
            "r15" => R15,
            _ => return Ok(None),
        };

        Ok(Some(reg))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(reg) => Ok(reg),
            None => Err(ParseError::new(format!(
                "error: invalid 64bit register : {}",
                s
            ))),
        }
    }
}

impl ParseStr for Reg32 {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        use Reg32::*;

        let reg = match s {
            "eax" => EAX,
            "edi" => EDI,
            "esi" => ESI,
            "edx" => EDX,
            "ecx" => ECX,
            "ebp" => EBP,
            "esp" => ESP,
            "ebx" => EBX,
            "r8d" => R8D,
            "r9d" => R9D,
            "r10d" => R10D,
            "r11d" => R11D,
            "r12d" => R12D,
            "r13d" => R13D,
            "r14d" => R14D,
            "r15d" => R15D,
            _ => return Ok(None),
        };

        Ok(Some(reg))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(reg) => Ok(reg),
            None => Err(ParseError::new(format!(
                "error: invalid 32bit register : {}",
                s
            ))),
        }
    }
}

impl ParseStr for Reg16 {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        use Reg16::*;

        let reg = match s {
            "ax" => AX,
            "di" => DI,
            "si" => SI,
            "dx" => DX,
            "cx" => CX,
            "bp" => BP,
            "sp" => SP,
            "bx" => BX,
            "r8w" => R8W,
            "r9w" => R9W,
            "r10w" => R10W,
            "r11w" => R11W,
            "r12w" => R12W,
            "r13w" => R13W,
            "r14w" => R14W,
            "r15w" => R15W,
            _ => return Ok(None),
        };

        Ok(Some(reg))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(reg) => Ok(reg),
            None => Err(ParseError::new(format!(
                "error: invalid 16bit register : {}",
                s
            ))),
        }
    }
}
