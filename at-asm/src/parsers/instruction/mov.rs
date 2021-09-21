use super::super::ParseStr;
use at_x64::{
    instruction::Mov,
    reg::{Reg16, Reg32},
    BytesAtMost,
};
use std::process::exit;

pub struct AnyMov(BytesAtMost<15>);

impl AnyMov {
    pub fn bytecode(&self) -> BytesAtMost<15> {
        self.0
    }
}

impl ParseStr for AnyMov {
    fn try_parse_str(s: &str) -> Option<Self> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("mov") {
            return None;
        }

        let operand1_str = match tokens.next() {
            Some(s) if s.ends_with(",") => s.split_at(s.len() - 1).0,
            Some(_) => {
                eprintln!("error: comma expected after first operand");
                exit(1);
            }
            None => {
                eprintln!("error: first operand is expected after mov opcode");
                exit(1);
            }
        };

        // if first operand is reg32
        if let Some(dst_reg) = Reg32::try_parse_str(operand1_str) {
            let operand2_str = match tokens.next() {
                Some(s) => s,
                None => {
                    eprintln!("error: second operand is expected");
                    exit(1);
                }
            };

            // if second operand is u32
            if let Some(src_imm) = u32::try_parse_str(operand2_str) {
                if tokens.next().is_some() {
                    eprintln!("error: end of line expected after second operand");
                    exit(1);
                }

                Some(AnyMov(Mov::new(dst_reg, src_imm).bytecode()))

            // otherwise, error
            } else {
                eprintln!("error: invalid second operand");
                exit(1);
            }

        // if first operand is reg16
        } else if let Some(dst_reg) = Reg16::try_parse_str(operand1_str) {
            let operand2_str = match tokens.next() {
                Some(s) => s,
                None => {
                    eprintln!("error: second operand is expected");
                    exit(1);
                }
            };

            // if second operand i u16
            if let Some(src_imm) = u16::try_parse_str(operand2_str) {
                Some(AnyMov(Mov::new(dst_reg, src_imm).bytecode()))

            // otherwise, error
            } else {
                eprintln!("error: invalid second operand");
                exit(1);
            }

        // otherwise, error
        } else {
            eprintln!("error: invalid first operand");
            exit(1);
        }
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(parsed) => parsed,
            None => {
                eprintln!("error: invalid mov instruction format");
                exit(1);
            }
        }
    }
}
