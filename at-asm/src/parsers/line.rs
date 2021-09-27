use super::{
    instruction::{AnyMov, AnyPush},
    ParseStr,
};
use at_x64::{
    instruction::{Instruction, Ret, Syscall},
    BytesAtMost,
};
use std::process::exit;

pub enum Line {
    Empty,
    Instruction(BytesAtMost<15>),
}

impl ParseStr for Line {
    fn parse_str(s: &str) -> Self {
        // コメントを無視
        let s = match s.split_once(";") {
            Some((s, _)) => s,
            None => s,
        };

        let mut tokens = s.split_whitespace();

        let opcode = match tokens.next() {
            Some(token) => token,
            None => {
                return Line::Empty;
            }
        };

        let bytes = match opcode {
            "ret" => Ret::new().bytecode(),
            "mov" => AnyMov::parse_str(s).bytecode(),
            "push" => AnyPush::parse_str(s).bytecode(),
            "syscall" => Syscall::new().bytecode(),
            _ => {
                eprintln!("error: unknown opcode {}", opcode);
                exit(1);
            }
        };

        Line::Instruction(bytes)
    }

    fn try_parse_str(s: &str) -> Option<Self> {
        Some(Self::parse_str(s))
    }
}
