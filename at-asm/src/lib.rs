mod parsers;
pub mod reader;

use self::parsers::{Line, ParseStr as _, Section};
use crate::reader::Reader;
use at_obj::{macho, BssSection, DataSection, Object, Symbol, TextSection};
use std::{
    io::{Read, Write},
    process::exit,
};

pub fn assemble<R: Read, W: Write>(read: &mut R, write: &mut W) {
    let mut reader = Reader::new(read);
    let mut obj = Object::new();

    let mut cur_sect = TargetSection::Text(&mut obj.sections.text);
    let mut n_line = 0;

    while let Some(line) = reader.next_line() {
        n_line += 1;

        let parsed_line = match Line::parse_str(line) {
            Ok(parsed) => parsed,
            Err(err) => {
                eprintln!("error at line {}: {}", n_line, err.msg());
                exit(1);
            }
        };

        match parsed_line {
            Line::Empty => {}
            Line::Section(sect) => {
                cur_sect = match sect {
                    Section::Text => TargetSection::Text(&mut obj.sections.text),
                    Section::Data => TargetSection::Data(&mut obj.sections.data),
                    Section::Bss => TargetSection::Bss(&mut obj.sections.bss),
                };
            }
            Line::Symbol(name) => {
                let symbol = Symbol::Ref {
                    name,
                    addr: cur_sect.size(),
                    ext: true,
                };
                cur_sect.push_symbol(symbol);
            }
            Line::Data(data) => {
                cur_sect.write_bytes(data.bytes());
            }
            Line::Instruction(bytes) => {
                cur_sect.write_bytes(bytes.as_ref());
            }
        };
    }

    macho::write_into(&obj, write);
}

enum TargetSection<'a> {
    Text(&'a mut TextSection),
    Data(&'a mut DataSection),
    Bss(&'a mut BssSection),
}

impl<'a> TargetSection<'a> {
    fn size(&self) -> u64 {
        use TargetSection::*;

        match self {
            Text(text) => text.bytes.len() as u64,
            Data(data) => data.bytes.len() as u64,
            Bss(bss) => bss.size,
        }
    }

    fn push_symbol(&mut self, symbol: Symbol) {
        use TargetSection::*;

        match self {
            Text(text) => text.symbols.push(symbol),
            Data(data) => data.symbols.push(symbol),
            Bss(bss) => bss.symbols.push(symbol),
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        use TargetSection::*;

        match self {
            Text(text) => text.bytes.extend_from_slice(bytes),
            Data(data) => data.bytes.extend_from_slice(bytes),
            Bss(_) => {
                eprintln!("error: can't write data to .bss section");
                exit(1);
            }
        }
    }
}
