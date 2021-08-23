pub mod build_version;
pub mod dysymtab;
pub mod segment64;
pub mod symtab;

pub use self::{
    build_version::{BuildToolVersion, BuildVersionCommand},
    dysymtab::DysymtabCommand,
    segment64::{Section64, SegmentCommand64},
    symtab::SymtabCommand,
};

use crate::io::{Endian, ReadExt as _};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt as _};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(SegmentCommand64, Vec<Section64>),
    Symtab(SymtabCommand),
    Dysymtab(DysymtabCommand),
    BuildVersion(BuildVersionCommand, Vec<BuildToolVersion>),
}

impl LoadCommand {
    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmd,
            LC::Symtab(cmd) => cmd.cmd,
            LC::Dysymtab(cmd) => cmd.cmd,
            LC::BuildVersion(cmd, _) => cmd.cmd,
        }
    }

    pub fn cmd_size(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmdsize,
            LC::Symtab(cmd) => cmd.cmdsize,
            LC::Dysymtab(cmd) => cmd.cmdsize,
            LC::BuildVersion(cmd, _) => cmd.cmdsize,
        }
    }

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        use LoadCommand as LC;

        let cmd = read.read_u32_in(endian);

        let mut cmd_bytes = [0u8; 4];
        match endian {
            Endian::Little => (&mut cmd_bytes[..]).write_u32::<LittleEndian>(cmd).unwrap(),
            Endian::Big => (&mut cmd_bytes[..]).write_u32::<BigEndian>(cmd).unwrap(),
        };

        let mut read = cmd_bytes.chain(read);

        match cmd {
            SegmentCommand64::TYPE => {
                let cmd = SegmentCommand64::read_from_in(&mut read, endian);

                let mut sections = Vec::with_capacity(cmd.nsects as usize);
                for _ in 0..cmd.nsects {
                    sections.push(Section64::read_from_in(&mut read, endian));
                }

                LC::Segment64(cmd, sections)
            }
            SymtabCommand::TYPE => {
                let cmd = SymtabCommand::read_from_in(&mut read, endian);
                LC::Symtab(cmd)
            }
            DysymtabCommand::TYPE => {
                let cmd = DysymtabCommand::read_from_in(&mut read, endian);
                LC::Dysymtab(cmd)
            }
            BuildVersionCommand::TYPE => {
                let cmd = BuildVersionCommand::read_from_in(&mut read, endian);

                let mut tools = Vec::with_capacity(cmd.ntools as usize);
                for _ in 0..cmd.ntools {
                    tools.push(BuildToolVersion::read_from_in(&mut read, endian));
                }
                LC::BuildVersion(cmd, tools)
            }
            _ => unimplemented!(),
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, sections) => {
                cmd.write_into(write);
                for section in sections.iter() {
                    section.write_into(write);
                }
            }
            LC::Symtab(cmd) => {
                cmd.write_into(write);
            }
            LC::Dysymtab(cmd) => {
                cmd.write_into(write);
            }
            LC::BuildVersion(cmd, tools) => {
                cmd.write_into(write);
                for tool in tools.iter() {
                    tool.write_into(write);
                }
            }
        }
    }
}
