use crate::io::{Endian, ReadExt as _, WriteExt as _};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    fmt,
    io::{Read, Write},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentCommand64 {
    /// SegmentCommand64::TYPE
    pub cmd: u32,
    /// includes sizeof Section64 structs
    pub cmdsize: u32,
    /// segment name. 16byte
    pub segname: String,
    /// memory address of this segment
    pub vmaddr: u64,
    /// memory size of this segment
    pub vmsize: u64,
    /// file offset of this segment
    pub fileoff: u64,
    /// amount to map from the file
    pub filesize: u64,
    /// maximum VM protection
    pub maxprot: i32,
    /// initial VM protection
    pub initprot: i32,
    /// number of sections in segment
    pub nsects: u32,
    /// flags
    pub flags: u32,
}

impl SegmentCommand64 {
    pub const TYPE: u32 = 0x19;

    /// Byte size of `SegmentCommand64` command.
    /// This does not include `Section64` command size.
    /// So this is constant.
    pub const SIZE: u32 = 0x48; // 72

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        assert_eq!(cmd, Self::TYPE);

        let cmdsize = read.read_u32_in(endian);
        let segname = read.read_fixed_size_string(16);
        let vmaddr = read.read_u64_in(endian);
        let vmsize = read.read_u64_in(endian);
        let fileoff = read.read_u64_in(endian);
        let filesize = read.read_u64_in(endian);
        let maxprot = read.read_i32_in(endian);
        let initprot = read.read_i32_in(endian);
        let nsects = read.read_u32_in(endian);
        let flags = read.read_u32_in(endian);

        assert_eq!(cmdsize, Self::SIZE + nsects * Section64::SIZE);

        SegmentCommand64 {
            cmd,
            cmdsize,
            segname,
            vmaddr,
            vmsize,
            fileoff,
            filesize,
            maxprot,
            initprot,
            nsects,
            flags,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        write.write_fixed_size_string(self.segname.as_str(), 16);
        write.write_u64_native(self.vmaddr);
        write.write_u64_native(self.vmsize);
        write.write_u64_native(self.fileoff);
        write.write_u64_native(self.filesize);
        write.write_i32_native(self.maxprot);
        write.write_i32_native(self.initprot);
        write.write_u32_native(self.nsects);
        write.write_u32_native(self.flags);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section64 {
    /// 16-byte string
    pub sectname: String,
    /// 16-byte string
    pub segname: String,
    /// memory address of this section
    pub addr: u64,
    /// size in bytes of this section
    pub size: u64,
    /// file offset of this section
    pub offset: u32,
    /// section alignment (power of 2)
    pub align: u32,
    /// file offset of the first relocation entry for this section
    pub reloff: u32,
    /// number of relocation entries for this section
    pub nreloc: u32,
    /// represented as u32.
    /// higher 3 bytes represent SectionAttrs,
    /// lower 1 byte represent SectionType.
    pub flags: (SectionAttrs, SectionType),
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

impl Section64 {
    pub const SIZE: u32 = 0x50; // 80

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let sectname = read.read_fixed_size_string(16);
        let segname = read.read_fixed_size_string(16);
        let addr = read.read_u64_in(endian);
        let size = read.read_u64_in(endian);
        let offset = read.read_u32_in(endian);
        let align = read.read_u32_in(endian);
        let reloff = read.read_u32_in(endian);
        let nreloc = read.read_u32_in(endian);

        let flags_n = read.read_u32_in(endian);
        let sect_type = SectionType::from_u32(flags_n & SectionType::BIT_MASK);
        let sect_attrs = SectionAttrs::from_u32(flags_n & SectionAttrs::BIT_MASK);

        let reserved1 = read.read_u32_in(endian);
        let reserved2 = read.read_u32_in(endian);
        let reserved3 = read.read_u32_in(endian);

        Section64 {
            sectname,
            segname,
            addr,
            size,
            offset,
            align,
            reloff,
            nreloc,
            flags: (sect_attrs, sect_type),
            reserved1,
            reserved2,
            reserved3,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_fixed_size_string(self.sectname.as_str(), 16);
        write.write_fixed_size_string(self.segname.as_str(), 16);
        write.write_u64_native(self.addr);
        write.write_u64_native(self.size);
        write.write_u32_native(self.offset);
        write.write_u32_native(self.align);
        write.write_u32_native(self.reloff);
        write.write_u32_native(self.nreloc);

        let flags_n = self.flags.0.to_u32() | self.flags.1.to_u32();
        write.write_u32_native(flags_n);

        write.write_u32_native(self.reserved1);
        write.write_u32_native(self.reserved2);
        write.write_u32_native(self.reserved3);
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Regular = 0x0,
    Zerofill = 0x1,
    CstringLiterals = 0x2,
    FourByteLiterals = 0x3,
    EightByteLiterals = 0x4,
    LiteralPointers = 0x5,
    Coalesced = 0xB,
}

impl SectionType {
    pub const BIT_MASK: u32 = 0x000000ff;

    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Unsupported section type 0x{:X}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAttr {
    /// This section contains only executable machine instructions. The standard tools set this
    /// flag for the sections __TEXT,__text, __TEXT,__symbol_stub, and __TEXT,__picsymbol_stub.
    PureInstructions = 0x80000000,
    /// section contains coalesced symbols that are not to be
    /// in a ranlib table of contents
    NoToc = 0x40000000,
    /// ok to strip static symbols in this section in files with the MH_DYLDLINK flag
    StripStaticSyms = 0x20000000,
    /// blocks are live if they reference live blocks
    LiveSupport = 0x08000000,
    /// If a segment contains any sections marked with S_ATTR_DEBUG then all
    /// sections in that segment must have this attribute.  No section other than
    /// a section marked with this attribute may reference the contents of this
    /// section.  A section with this attribute may contain no symbols and must have
    /// a section type S_REGULAR.  The static linker will not copy section contents
    /// from sections with this attribute into its output file.  These sections
    /// generally contain DWARF debugging info.
    Debug = 0x02000000,
    /// section contains some executable machine instructions.
    SomeInstructions = 0x00000400,
    /// section has external relocation entries.
    ExtReloc = 0x00000200,
    /// section has local relocation entries.
    LocReloc = 0x00000100,
}

impl SectionAttr {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n)
            .unwrap_or_else(|| panic!("Unsupported section attribute 0x{:X}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SectionAttrs {
    attrs: Vec<SectionAttr>,
}

impl SectionAttrs {
    pub const BIT_MASK: u32 = 0xffffff00;

    pub fn new() -> SectionAttrs {
        SectionAttrs { attrs: Vec::new() }
    }

    pub fn push(&mut self, attr: SectionAttr) {
        self.attrs.push(attr);
    }

    pub fn from_u32(flags: u32) -> Self {
        let mut attrs = SectionAttrs::new();
        for i in 8..=31 {
            let attr_n = flags & (1 << i);
            if attr_n != 0 {
                attrs.push(SectionAttr::from_u32(attr_n));
            }
        }
        attrs
    }

    pub fn to_u32(&self) -> u32 {
        let mut n = 0;
        for attr in self.attrs.iter() {
            n |= attr.to_u32();
        }
        n
    }
}

impl fmt::Debug for SectionAttrs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.attrs.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_segment64_command() {
        let cmd = SegmentCommand64 {
            cmd: SegmentCommand64::TYPE,
            cmdsize: SegmentCommand64::SIZE + Section64::SIZE,
            segname: String::new(),
            vmaddr: 0,
            vmsize: 42,
            fileoff: 100,
            filesize: 42,
            maxprot: 7,
            initprot: 7,
            nsects: 1,
            flags: 0,
        };

        let mut buf = Vec::new();

        cmd.write_into(&mut buf);

        assert_eq!(buf.len(), SegmentCommand64::SIZE as usize);

        let read_cmd = SegmentCommand64::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_cmd, cmd);
    }

    #[test]
    fn write_and_read_section64() {
        let cmd = Section64 {
            sectname: "__TEXT,__text".to_string(),
            segname: String::new(),
            addr: 0,
            size: 42,
            offset: 100,
            align: 0,
            reloff: 53,
            nreloc: 1,
            flags: (SectionAttrs::new(), SectionType::Regular),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        };

        let mut buf = Vec::new();

        cmd.write_into(&mut buf);

        assert_eq!(buf.len(), Section64::SIZE as usize);

        let read_cmd = Section64::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_cmd, cmd);
    }
}
