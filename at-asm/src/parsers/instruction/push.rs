use super::super::ParseStr;
use at_x64::{
    instruction::{Instruction, Push},
    reg::Reg64,
    BytesAtMost,
};
use std::process::exit;

pub struct AnyPush(BytesAtMost<15>);

impl AnyPush {
    fn from<T>(push: Push<T>) -> Self
    where
        Push<T>: Instruction,
    {
        AnyPush(push.bytecode())
    }

    pub fn bytecode(&self) -> BytesAtMost<15> {
        self.0
    }
}

impl ParseStr for AnyPush {
    fn try_parse_str(s: &str) -> Option<Self> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("push") {
            return None;
        }

        let operand_str = match tokens.next() {
            Some(s) => s,
            None => {
                eprintln!("error: operand is expected after push opcode");
                exit(1);
            }
        };

        // if operand is Reg64
        if let Some(reg) = Reg64::try_parse_str(operand_str) {
            let push = Push::new(reg);
            return Some(AnyPush::from(push));
        }

        // if operand is u8
        if let Some(imm) = u8::try_parse_str(operand_str) {
            let push = Push::new(imm);
            return Some(AnyPush::from(push));
        }

        // if operand is u32
        if let Some(imm) = u32::try_parse_str(operand_str) {
            let push = Push::new(imm);
            return Some(AnyPush::from(push));
        }

        // otherwise error
        eprintln!("error: invalid push operand : {}", operand_str);
        exit(1);
    }

    fn parse_str(s: &str) -> Self {
        match Self::try_parse_str(s) {
            Some(t) => t,
            None => {
                eprintln!("error: invalid push instruction format");
                exit(1);
            }
        }
    }
}
