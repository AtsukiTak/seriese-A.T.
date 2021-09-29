mod parsers;
pub mod reader;

use self::parsers::{Line, ParseStr as _};
use crate::reader::Reader;
use at_obj::{macho, Object, Symbol};
use std::io::{Read, Write};

pub fn assemble<R: Read, W: Write>(read: &mut R, write: &mut W) {
    let mut reader = Reader::new(read);
    let mut obj = Object::new();

    while let Some(line) = reader.next_line() {
        parse_line(line, &mut obj);
    }

    macho::write_into(&obj, write);
}

fn parse_line(line_str: &str, obj: &mut Object) {
    match Line::parse_str(line_str) {
        Line::Empty => {}
        Line::Symbol(name) => {
            let symbol = Symbol::Ref {
                name,
                addr: obj.sections.text.bytes.len() as u64,
                ext: true,
            };
            obj.sections.text.symbols.push(symbol);
        }
        Line::Instruction(bytes) => obj.sections.text.bytes.extend_from_slice(bytes.as_ref()),
    };
}
