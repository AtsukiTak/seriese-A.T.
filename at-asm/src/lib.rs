mod parsers;
pub mod reader;

use crate::reader::Reader;
use at_obj::{macho, Object, Symbol};
use at_x64::{
    instruction::{Mov, Ret},
    reg::Reg32,
};
use std::io::{Read, Write};

pub fn assemble<R: Read, W: Write>(read: &mut R, write: &mut W) {
    let mut reader = Reader::new(read);
    let mut obj = Object::new();

    while let Some(line) = reader.next_line() {
        parse_line(line, &mut obj);
    }

    obj.sections.text.symbols = vec![Symbol::Ref {
        name: "_main".to_string(),
        addr: 0,
        ext: true,
    }];
    macho::write_into(&obj, write);
}

fn parse_line(line: &str, obj: &mut Object) {
    let section_bytes = &mut obj.sections.text.bytes;

    let mut tokens = line.split_whitespace();

    match tokens.next() {
        Some("ret") => {
            let bytes = Ret::new().bytecode();
            section_bytes.extend_from_slice(bytes.bytes());
        }
        Some("mov") => {
            let reg = match tokens.next().unwrap() {
                "ax," => Reg32::EAX,
                _ => todo!(),
            };
            let num = {
                let s = tokens.next().unwrap();
                parsers::parse::<u64>(s) as u32
            };
            let bytes = Mov::new(reg, num).bytecode();
            section_bytes.extend_from_slice(bytes.bytes());
        }
        _ => panic!("unrecognized line: {}", line),
    }
}
