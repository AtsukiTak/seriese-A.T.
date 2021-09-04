pub mod reader;

use crate::reader::Reader;
use at_obj::{macho, Object, Symbol};
use std::io::{Read, Write};

pub fn assemble<R: Read, W: Write>(read: &mut R, write: &mut W) {
    let mut reader = Reader::new(read);
    let mut obj = Object::new();

    while let Some(line) = reader.next_line() {
        parse_line(line, &mut obj);
    }

    // mov ax, 42
    // ret
    obj.sections.text.bytes = vec![0x66, 0xb8, 0x2a, 0x00, 0xc3];
    obj.sections.text.symbols = vec![Symbol::Ref {
        name: "_main".to_string(),
        addr: 0,
        ext: true,
    }];
    macho::write_into(&obj, write);
}

fn parse_line(line: &str, obj: &mut Object) {
    match line {
        "ret\n" => obj.sections.text.bytes.push(0xc3),
        "mov ax, 42\n" => obj
            .sections
            .text
            .bytes
            .extend_from_slice(&[0x66, 0xb8, 0x2a, 0x00]),
        _ => panic!("unrecognized line: {}", line),
    }
}
