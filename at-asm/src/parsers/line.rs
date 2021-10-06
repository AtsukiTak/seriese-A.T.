use super::{
    data::Data,
    global_symbol::GlobalSymbol,
    instruction::{AnyMov, AnyPush},
    section::Section,
    ParseError, ParseStr,
};
use at_x64::{
    instruction::{Instruction, Ret, Syscall},
    BytesAtMost,
};

pub enum Line {
    Empty,
    Section(Section),
    Symbol(String),
    GlobalSymbol(String),
    Data(Data),
    Instruction(BytesAtMost<15>),
}

impl ParseStr for Line {
    fn parse_str(s: &str) -> Result<Self, ParseError> {
        // コメントを無視
        let s = match s.split_once(";") {
            Some((s, _)) => s,
            None => s,
        };

        let mut tokens = s.split_whitespace();

        // 空行
        let token = match tokens.next() {
            Some(token) => token,
            None => {
                return Ok(Line::Empty);
            }
        };

        // section
        if let Some(section) = Section::try_parse_str(s)? {
            return Ok(Line::Section(section));
        }

        // global symbol
        if let Some(symbol) = GlobalSymbol::try_parse_str(s)? {
            return Ok(Line::GlobalSymbol(symbol.0));
        }

        // シンボル
        if token.ends_with(":") {
            if tokens.next().is_some() {
                return Err(ParseError::new("error: expected end of line."));
            }
            let symbol = token.trim_end_matches(":");
            return Ok(Line::Symbol(symbol.to_string()));
        }

        // data
        if let Some(data) = Data::try_parse_str(s)? {
            return Ok(Line::Data(data));
        }

        // instruction
        let bytes = match token {
            "ret" => Ret::new().bytecode(),
            "mov" => AnyMov::parse_str(s)?.bytecode(),
            "push" => AnyPush::parse_str(s)?.bytecode(),
            "syscall" => Syscall::new().bytecode(),
            _ => return Err(ParseError::new(format!("error: unknown opcode {}", token))),
        };

        Ok(Line::Instruction(bytes))
    }

    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        Ok(Some(Self::parse_str(s)?))
    }
}
