mod parsers;
pub mod reader;

use self::parsers::{parse, Line};
use crate::reader::Reader;
use at_obj::{macho, Object, Symbol};
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

fn parse_line(line_str: &str, obj: &mut Object) {
    match parse::<Line>(line_str) {
        Line::Instruction(bytes) => obj.sections.text.bytes.extend_from_slice(bytes.as_ref()),
    };
}
