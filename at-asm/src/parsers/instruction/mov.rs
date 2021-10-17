use super::{
    super::{expr::Expr, ParseError, ParseStr},
    Instruction, LocalReloc,
};
use at_x64::{
    instruction::Mov,
    reg::{Reg16, Reg32, Reg64},
};

pub fn parse_str(s: &str) -> Result<Instruction, ParseError> {
    let (opcode, remain) = s
        .trim()
        .split_once(" ")
        .ok_or_else(|| ParseError::new("invalid mov instruction format"))?;
    if opcode != "mov" {
        return Err(ParseError::new("not mov opcode"));
    }
    let (operand1_str, operand2_str) = remain
        .split_once(",")
        .map(|(s1, s2)| (s1.trim(), s2.trim()))
        .ok_or_else(|| ParseError::new("invalid mov instruction format"))?;

    // if first operand is reg64
    if let Some(dst_reg) = Reg64::try_parse_str(operand1_str)? {
        // if second operand is Expr
        if let Some(expr) = Expr::try_parse_str(operand2_str)? {
            let mov = Mov::new(dst_reg, expr.as_u64());
            return Ok(Instruction::new(mov));
        }

        // if second operand is Reg64
        if let Some(src_reg) = Reg64::try_parse_str(operand2_str)? {
            let mov = Mov::new(dst_reg, src_reg);
            return Ok(Instruction::new(mov));
        }

        // otherwise, second operand is symbol
        let symbol = operand2_str.to_string();
        let mov = Mov::new(dst_reg, 0_u64);
        let reloc = LocalReloc {
            offset: mov.offset_to_imm_bytes(),
            symbol,
            pcrel: false,
            len: 3, // 2^3 byte = 64bit
        };
        let anymov = Instruction::new(mov).with_reloc(reloc);
        return Ok(anymov);
    }

    // if first operand is reg32
    if let Some(dst_reg) = Reg32::try_parse_str(operand1_str)? {
        // if second operand is Expr
        if let Some(expr) = Expr::try_parse_str(operand2_str)? {
            if let Some(src_imm) = expr.as_u32() {
                let mov = Mov::new(dst_reg, src_imm);
                return Ok(Instruction::new(mov));
            } else {
                return Err(ParseError::new(format!("{} is not 32bit", expr.as_u64())));
            }
        }

        // if second operand is Reg32
        if let Some(src_reg) = Reg32::try_parse_str(operand2_str)? {
            let mov = Mov::new(dst_reg, src_reg);
            return Ok(Instruction::new(mov));
        }

        // otherwise, second operand is symbol
        let symbol = operand2_str.to_string();
        let mov = Mov::new(dst_reg, 0_u32);
        let reloc = LocalReloc {
            offset: mov.offset_to_imm_bytes(),
            symbol,
            pcrel: false,
            len: 2, // 2^2 byte = 32bit
        };
        let anymov = Instruction::new(mov).with_reloc(reloc);
        return Ok(anymov);
    }

    // if first operand is Reg16
    if let Some(dst_reg) = Reg16::try_parse_str(operand1_str)? {
        // if second operand i u16
        if let Some(expr) = Expr::try_parse_str(operand2_str)? {
            if let Some(src_imm) = expr.as_u16() {
                let mov = Mov::new(dst_reg, src_imm);
                return Ok(Instruction::new(mov));
            } else {
                return Err(ParseError::new(format!("{} is not 16bit", expr.as_u64())));
            }
        }

        // if second operand is Reg16
        if let Some(src_reg) = Reg16::try_parse_str(operand2_str)? {
            let mov = Mov::new(dst_reg, src_reg);
            return Ok(Instruction::new(mov));
        }

        // otherwise, error
        return Err(ParseError::new("invalid second operand"));
    }

    // otherwise, error
    Err(ParseError::new("invalid first operand"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use at_x64::{
        instruction::Instruction as Inst,
        reg::{Reg16::*, Reg32::*, Reg64::*},
    };

    fn assert_bytes<T: Inst>(raw: &'static str, inst: T) {
        assert_eq!(parse_str(raw).unwrap().bytes, inst.bytecode());
    }

    #[test]
    fn should_parse() {
        assert_bytes("mov eax, 42", Mov::new(EAX, 42));
        assert_bytes("mov eax, esp", Mov::new(EAX, ESP));
        assert_bytes("mov cx, r8w", Mov::new(CX, R8W));
        assert_bytes("mov rax, rcx", Mov::new(RAX, RCX));
        assert_bytes("mov rax, 0x200004", Mov::new(RAX, 0x200004));
        assert_bytes("mov rax, 0x200000 + 4", Mov::new(RAX, 0x200004));
    }
}
