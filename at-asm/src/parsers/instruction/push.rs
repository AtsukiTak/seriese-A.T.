use super::super::{ParseError, ParseStr};
use at_x64::{
    instruction::{Instruction, Push},
    reg::Reg64,
    BytesAtMost,
};

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
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("push") {
            return Ok(None);
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
            return Ok(Some(AnyPush::from(push)));
        }

        // if operand is u8
        if let Some(imm) = u8::try_parse_str(operand_str)? {
            let push = Push::new(imm);
            return Ok(Some(AnyPush::from(push)));
        }

        // if operand is u32
        if let Some(imm) = u32::try_parse_str(operand_str)? {
            let push = Push::new(imm);
            return Ok(Some(AnyPush::from(push)));
        }

        // otherwise error
        Err(ParseError::new(format!(
            "error: invalid push operand : {}",
            operand_str
        )))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(t) => Ok(t),
            None => Err(ParseError::new("error: invalid push instruction format")),
        }
    }
}
