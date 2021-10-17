mod mov;
mod push;

use super::{ParseError, ParseStr};
use at_x64::{
    instruction::{Instruction as InstructionTrait, Ret, Syscall},
    BytesAtMost,
};

pub struct Instruction {
    pub bytes: BytesAtMost<15>,
    pub reloc: Option<LocalReloc>,
}

pub struct LocalReloc {
    // offset from the start of the bytecode
    // to the byte being relocated.
    pub offset: u8,
    pub symbol: String,
    pub pcrel: bool,
    pub len: u8,
}

impl Instruction {
    pub fn new<T>(inst: T) -> Self
    where
        T: InstructionTrait,
    {
        Instruction {
            bytes: inst.bytecode(),
            reloc: None,
        }
    }

    pub fn with_reloc(mut self, reloc: LocalReloc) -> Self {
        self.reloc = Some(reloc);
        self
    }
}

impl ParseStr for Instruction {
    fn try_parse_str(s: &str) -> Result<Option<Instruction>, ParseError> {
        match s.split_whitespace().next() {
            Some("mov") => Ok(Some(mov::parse_str(s)?)),
            Some("push") => Ok(Some(push::parse_str(s)?)),
            Some("ret") => Ok(Some(Instruction::new(Ret::new()))),
            Some("syscall") => Ok(Some(Instruction::new(Syscall::new()))),
            _ => Ok(None),
        }
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(t) => Ok(t),
            None => Err(ParseError::new("unrecognized opcode")),
        }
    }
}
