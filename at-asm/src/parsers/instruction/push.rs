use super::{
    super::{ParseError, ParseStr},
    Instruction,
};
use at_x64::{instruction::Push, reg::Reg64};

pub fn parse_str(s: &str) -> Result<Instruction, ParseError> {
    let mut tokens = s.split_whitespace();

    if tokens.next() != Some("push") {
        return Err(ParseError::new("error: invalid push instruction format"));
    }

    let operand_str = match tokens.next() {
        Some(s) => s,
        None => {
            return Err(ParseError::new(
                "error: operand is expected after push opcode",
            ));
        }
    };

    // if operand is Reg64
    if let Some(reg) = Reg64::try_parse_str(operand_str)? {
        let push = Push::new(reg);
        return Ok(Instruction::new(push));
    }

    // if operand is u8
    if let Some(imm) = u8::try_parse_str(operand_str)? {
        let push = Push::new(imm);
        return Ok(Instruction::new(push));
    }

    // if operand is u32
    if let Some(imm) = u32::try_parse_str(operand_str)? {
        let push = Push::new(imm);
        return Ok(Instruction::new(push));
    }

    // otherwise error
    Err(ParseError::new(format!(
        "error: invalid push operand : {}",
        operand_str
    )))
}
