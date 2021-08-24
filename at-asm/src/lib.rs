use at_obj::{macho, Object, Symbol};
use std::io::{Read, Write};

pub fn assemble<R: Read, W: Write>(_read: &mut R, write: &mut W) {
    let mut obj = Object::new();
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
