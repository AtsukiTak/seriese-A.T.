use crate::io::{Endian, ReadExt as _, WriteExt as _};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{Read, Write};

/// The build_version_command contains the min OS version on which this
/// binary was built to run for its platform.  The list of known platforms and
/// tool values following it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildVersionCommand {
    pub cmd: u32,
    /// BuildVersion::SIZE + ntools * BuildToolVersion::SIZE
    pub cmdsize: u32,
    pub platform: Platform,
    /// X.Y.Z is encoded in nibbles xxxx.yy.zz
    pub minos: Version,
    /// X.Y.Z is encoded in nibbles xxxx.yy.zz
    pub sdk: Version,
    pub ntools: u32,
}

impl BuildVersionCommand {
    pub const TYPE: u32 = 0x32;

    pub const SIZE: u32 = 0x18; // 24

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let cmd = read.read_u32_in(endian);
        assert_eq!(cmd, Self::TYPE);

        let cmdsize = read.read_u32_in(endian);

        let platform_n = read.read_u32_in(endian);
        let platform = Platform::from_u32(platform_n);

        let minos_n = read.read_u32_in(endian);
        let minos = Version::from_u32(minos_n);

        let sdk_n = read.read_u32_in(endian);
        let sdk = Version::from_u32(sdk_n);

        let ntools = read.read_u32_in(endian);

        assert_eq!(cmdsize, Self::SIZE + BuildToolVersion::SIZE * ntools);

        BuildVersionCommand {
            cmd,
            cmdsize,
            platform,
            minos,
            sdk,
            ntools,
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.cmd);
        write.write_u32_native(self.cmdsize);
        write.write_u32_native(self.platform.to_u32());
        write.write_u32_native(self.minos.to_u32());
        write.write_u32_native(self.sdk.to_u32());
        write.write_u32_native(self.ntools);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Platform {
    MacOS = 1,
    IOS = 2,
    TvOS = 3,
    WatchOS = 4,
    BridgeOS = 5,
    MacCatalyst = 6,
    IOSSimulator = 7,
    TvOSSimulator = 8,
    WatchOSSimulator = 9,
    Driverkit = 10,
}

impl Platform {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Invalid platform number {}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u8,
    pub release: u8,
}

impl Version {
    /// version is represented as "xxxx.yy.zz"
    pub fn from_u32(n: u32) -> Self {
        let major = ((n & 0xFFFF_0000) >> 16) as u16;
        let minor = ((n & 0x0000_FF00) >> 8) as u8;
        let release = (n & 0x0000_00FF) as u8;
        Version {
            major,
            minor,
            release,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut n = 0;
        n |= (self.major as u32) << 16;
        n |= (self.minor as u32) << 8;
        n |= self.release as u32;
        n
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildToolVersion {
    pub tool: Tool,
    pub version: u32,
}

impl BuildToolVersion {
    pub const SIZE: u32 = 0x8;

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> Self {
        let tool_n = read.read_u32_in(endian);
        let tool = Tool::from_u32(tool_n);
        let version = read.read_u32_in(endian);

        BuildToolVersion { tool, version }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        write.write_u32_native(self.tool.to_u32());
        write.write_u32_native(self.version);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Tool {
    Clang = 1,
    Swift = 2,
    LD = 3,
}

impl Tool {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Unsupported tool number {}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_build_version_command() {
        let cmd = BuildVersionCommand {
            cmd: BuildVersionCommand::TYPE,
            cmdsize: BuildVersionCommand::SIZE,
            platform: Platform::MacOS,
            minos: Version {
                major: 3,
                minor: 10,
                release: 42,
            },
            sdk: Version {
                major: 1,
                minor: 11,
                release: 13,
            },
            ntools: 0,
        };

        let mut buf = Vec::new();

        cmd.write_into(&mut buf);

        assert_eq!(buf.len(), BuildVersionCommand::SIZE as usize);

        let read_cmd = BuildVersionCommand::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_cmd, cmd);
    }

    #[test]
    fn write_and_read_build_tool_version() {
        let version = BuildToolVersion {
            tool: Tool::Clang,
            version: 42,
        };

        let mut buf = Vec::new();

        version.write_into(&mut buf);

        assert_eq!(buf.len(), BuildToolVersion::SIZE as usize);

        let read_version = BuildToolVersion::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_version, version);
    }
}
