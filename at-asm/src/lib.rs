pub mod reader;

use crate::reader::Reader;
use at_obj::{macho, Object, Symbol};
use at_x64::{
    instruction::{Mov, Ret},
    reg::Reg32::*,
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

    match line {
        "ret\n" => {
            let bytes = Ret::new().bytecode();
            section_bytes.extend_from_slice(bytes.bytes());
        }
        "mov ax, 42\n" => {
            let bytes = Mov::new(EAX, 42).bytecode();
            section_bytes.extend_from_slice(bytes.bytes());
        }
        _ => panic!("unrecognized line: {}", line),
    }
}
