use super::{
    data::Data,
    instruction::{AnyMov, AnyPush},
    section::Section,
    ParseStr,
};
use at_x64::{
    instruction::{Instruction, Ret, Syscall},
    BytesAtMost,
};
use std::process::exit;

pub enum Line {
    Empty,
    Section(Section),
    Symbol(String),
    Data(Vec<u8>),
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

        let token = match tokens.next() {
            Some(token) => token,
            None => {
                return Line::Empty;
            }
        };

        // section
        if let Some(section) = Section::try_parse_str(s) {
            return Line::Section(section);
        }

        // シンボル
        if token.ends_with(":") {
            if tokens.next().is_some() {
                eprintln!("error: expected end of line.");
                exit(1);
            }
            let symbol = token.trim_end_matches(":");
            return Line::Symbol(symbol.to_string());
        }

        // data
        if let Some(Data(data)) = Data::try_parse_str(s) {
            return Line::Data(data);
        }

        // instruction
        let bytes = match token {
            "ret" => Ret::new().bytecode(),
            "mov" => AnyMov::parse_str(s).bytecode(),
            "push" => AnyPush::parse_str(s).bytecode(),
            "syscall" => Syscall::new().bytecode(),
            _ => {
                eprintln!("error: unknown opcode {}", token);
                exit(1);
            }
        };

        Line::Instruction(bytes)
    }

    fn try_parse_str(s: &str) -> Option<Self> {
        Some(Self::parse_str(s))
    }
}
