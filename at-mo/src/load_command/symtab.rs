use crate::io::{Endian, ReadExt as _, WriteExt as _};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    /// the byte offset from the start of the file to the location of the
    /// symbol table entries
    pub symoff: u32,
    /// number of symbol table entries
    pub nsyms: u32,
    /// the byte offset from the start of the file to the location of the string table.
    pub stroff: u32,
    /// the size (in bytes) of the string table.
    pub strsize: u32,
}

impl SymtabCommand {
    pub const TYPE: u32 = 0x2;

    pub const SIZE: u32 = 0x18; // 24

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        assert_eq!(cmd, SymtabCommand::TYPE);

        let cmdsize = read.read_u32_in(endian);
        assert_eq!(cmdsize, SymtabCommand::SIZE);

        let symoff = read.read_u32_in(endian);
        let nsyms = read.read_u32_in(endian);
        let stroff = read.read_u32_in(endian);
        let strsize = read.read_u32_in(endian);

        SymtabCommand {
            cmd,
            cmdsize,
            symoff,
            nsyms,
            stroff,
            strsize,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        write.write_u32_native(self.symoff);
        write.write_u32_native(self.nsyms);
        write.write_u32_native(self.stroff);
        write.write_u32_native(self.strsize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_symtab_command() {
        let cmd = SymtabCommand {
            cmd: SymtabCommand::TYPE,
            cmdsize: SymtabCommand::SIZE,
            symoff: 42,
            nsyms: 1,
            stroff: 100,
            strsize: 7,
        };

        let mut buf = Vec::new();

        cmd.write_into(&mut buf);

        assert_eq!(buf.len(), SymtabCommand::SIZE as usize);

        let read_cmd = SymtabCommand::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_cmd, cmd);
    }
}
