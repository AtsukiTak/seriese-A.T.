use super::{parse, ParseStr};
use at_x64::{
    instruction::{Mov, Ret},
    reg::Reg32,
    BytesAtMost,
};
use std::process::exit;

pub enum Line {
    Instruction(BytesAtMost<15>),
}

impl ParseStr for Line {
    fn parse_str(s: &str) -> Self {
        let mut tokens = s.split_whitespace();

        let opcode = tokens.next().unwrap();

        let bytes = match opcode {
            "ret" => Ret::new().bytecode(),
            "mov" => {
                let operand1_str = tokens.next().unwrap();
                if !operand1_str.ends_with(",") {
                    eprintln!("error: comma expected after first operand");
                    exit(1);
                }
                let operand1_str = operand1_str.split_at(operand1_str.len() - 1).0;
                let operand1 = parse::<Reg32>(operand1_str);

                let operand2_str = tokens.next().unwrap();
                let operand2 = parse::<u64>(operand2_str) as u32;

                if tokens.next().is_some() {
                    eprintln!("error: end of line expected after second operand");
                    exit(1);
                }

                Mov::new(operand1, operand2).bytecode()
            }
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
