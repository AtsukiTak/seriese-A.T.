use crate::parsers::{Line, ParseStr as _, Section};
use crate::reader::Reader;
use at_obj::{macho, BssSection, DataSection, Object, Symbol, TextSection};
use std::{
    io::{Read, Write},
    process::exit,
};

pub struct Assembler {
    cur_sect: Section,
    cur_line: u32,
    obj: Object,
    ext_symbols: Vec<String>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            cur_sect: Section::Text,
            cur_line: 0,
            obj: Object::new(),
            ext_symbols: Vec::new(),
        }
    }

    fn cur_sect_mut<'a>(&'a mut self) -> SectionMut<'a> {
        match self.cur_sect {
            Section::Text => SectionMut::Text(&mut self.obj.sections.text),
            Section::Data => SectionMut::Data(&mut self.obj.sections.data),
            Section::Bss => SectionMut::Bss(&mut self.obj.sections.bss),
        }
    }

    pub fn set_cur_sect(&mut self, cur_sect: Section) {
        self.cur_sect = cur_sect;
    }

    pub fn read_from<R: Read>(&mut self, read: &mut R) -> &mut Self {
        let mut reader = Reader::new(read);

        while let Some(line) = reader.next_line() {
            self.cur_line += 1;

            let parsed_line = match Line::parse_str(line) {
                Ok(parsed) => parsed,
                Err(err) => {
                    eprintln!("error at line {}: {}", self.cur_line, err.msg());
                    exit(1);
                }
            };

            match parsed_line {
                Line::Empty => {}
                Line::Section(sect) => {
                    self.set_cur_sect(sect);
                }
                Line::GlobalSymbol(symbol_name) => {
                    self.ext_symbols.push(symbol_name);
                }
                Line::Symbol(name) => {
                    let mut cur_sect_mut = self.cur_sect_mut();
                    let symbol = Symbol::Ref {
                        name,
                        addr: cur_sect_mut.size(),
                        ext: false,
                    };
                    cur_sect_mut.push_symbol(symbol);
                }
                Line::Data(data) => {
                    let mut cur_sect_mut = self.cur_sect_mut();
                    if let Some(label) = data.label {
                        let symbol = Symbol::Ref {
                            name: label,
                            addr: cur_sect_mut.size(),
                            ext: false,
                        };
                        cur_sect_mut.push_symbol(symbol);
                    }
                    cur_sect_mut.write_bytes(data.bytes.as_slice());
                }
                Line::Instruction(bytes) => {
                    self.cur_sect_mut().write_bytes(bytes.as_ref());
                }
            };
        }

        let ext_symbols = &self.ext_symbols;
        let obj = &mut self.obj;

        obj.symbols_mut()
            .filter(|symbol| ext_symbols.iter().any(|s| s == symbol.name()))
            .for_each(|symbol| match symbol {
                Symbol::Undef { .. } => {}
                Symbol::Abs { ext, .. } => *ext = true,
                Symbol::Ref { ext, .. } => *ext = true,
            });

        self
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        macho::write_into(&self.obj, write);
    }
}

enum SectionMut<'a> {
    Text(&'a mut TextSection),
    Data(&'a mut DataSection),
    Bss(&'a mut BssSection),
}

impl<'a> SectionMut<'a> {
    fn size(&self) -> u64 {
        use SectionMut::*;

        match self {
            Text(text) => text.bytes.len() as u64,
            Data(data) => data.bytes.len() as u64,
            Bss(bss) => bss.size,
        }
    }

    fn push_symbol(&mut self, symbol: Symbol) {
        use SectionMut::*;

        match self {
            Text(text) => text.symbols.push(symbol),
            Data(data) => data.symbols.push(symbol),
            Bss(bss) => bss.symbols.push(symbol),
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        use SectionMut::*;

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
