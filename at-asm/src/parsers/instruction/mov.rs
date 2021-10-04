use super::super::{ParseError, ParseStr};
use at_x64::{
    instruction::{Instruction, Mov},
    reg::{Reg16, Reg32, Reg64},
    BytesAtMost,
};

pub struct AnyMov(BytesAtMost<15>);

impl AnyMov {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        self.0
    }
}

impl ParseStr for AnyMov {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("mov") {
            return Ok(None);
        }

        let operand1_str = match tokens.next() {
            Some(s) if s.ends_with(",") => s.split_at(s.len() - 1).0,
            Some(_) => return Err(ParseError::new("error: comma expected after first operand")),
            None => {
                return Err(ParseError::new(
                    "error: first operand is expected after mov opcode",
                ));
            }
        };

        // if first operand is reg64
        if let Some(dst_reg) = Reg64::try_parse_str(operand1_str)? {
            let operand2_str = match tokens.next() {
                Some(s) => s,
                None => {
                    return Err(ParseError::new("error: second operand is expected"));
                }
            };

            // if second operand is u64
            if let Some(src_imm) = u64::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_imm);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // if second operand is Reg64
            if let Some(src_reg) = Reg64::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_reg);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // otherwise, error
            return Err(ParseError::new("error: invalid second operand"));
        }

        // if first operand is reg32
        if let Some(dst_reg) = Reg32::try_parse_str(operand1_str)? {
            let operand2_str = match tokens.next() {
                Some(s) => s,
                None => {
                    return Err(ParseError::new("error: second operand is expected"));
                }
            };

            // if second operand is u32
            if let Some(src_imm) = u32::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_imm);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // if second operand is Reg32
            if let Some(src_reg) = Reg32::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_reg);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // otherwise, error
            return Err(ParseError::new("error: invalid second operand"));
        }

        // if first operand is Reg16
        if let Some(dst_reg) = Reg16::try_parse_str(operand1_str)? {
            let operand2_str = match tokens.next() {
                Some(s) => s,
                None => {
                    return Err(ParseError::new("error: second operand is expected"));
                }
            };

            // if second operand i u16
            if let Some(src_imm) = u16::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_imm);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // if second operand is Reg16
            if let Some(src_reg) = Reg16::try_parse_str(operand2_str)? {
                let mov = Mov::new(dst_reg, src_reg);
                return Ok(Some(AnyMov(mov.bytecode())));
            }

            // otherwise, error
            return Err(ParseError::new("error: invalid second operand"));
        }

        // otherwise, error
        Err(ParseError::new("error: invalid first operand"))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(parsed) => Ok(parsed),
            None => Err(ParseError::new("error: invalid mov instruction format")),
        }
    }
}
