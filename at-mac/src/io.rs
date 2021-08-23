use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

#[cfg(target_endian = "little")]
impl Endian {
    pub const NATIVE: Endian = Endian::Little;
    pub const REVERSE: Endian = Endian::Big;
}

#[cfg(target_endian = "big")]
impl Endian {
    pub const NATIVE: Endian = Endian::Big;
    pub const REVERSE: Endian = Endian::Little;
}

macro_rules! read_in {
    ($func:ident, $endian:expr) => {
        match $endian {
            Endian::Little => ReadBytesExt::$func::<LittleEndian>,
            Endian::Big => ReadBytesExt::$func::<BigEndian>,
        }
    };
}

pub trait ReadExt: Read + ReadBytesExt {
    fn read_u8(&mut self) -> u8 {
        ReadBytesExt::read_u8(self).unwrap()
    }

    fn read_u16_in(&mut self, endian: Endian) -> u16 {
        read_in!(read_u16, endian)(self).unwrap()
    }

    fn read_i32_in(&mut self, endian: Endian) -> i32 {
        read_in!(read_i32, endian)(self).unwrap()
    }

    fn read_u32_in(&mut self, endian: Endian) -> u32 {
        read_in!(read_u32, endian)(self).unwrap()
    }

    fn read_u64_in(&mut self, endian: Endian) -> u64 {
        read_in!(read_u64, endian)(self).unwrap()
    }

    fn read_fixed_size_string(&mut self, size: usize) -> String {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf).unwrap();

        let valid_len = buf.split(|&b| b == 0).next().unwrap().len();
        buf.truncate(valid_len);
        String::from_utf8(buf).unwrap()
    }
}

impl<T> ReadExt for T where T: Read {}

pub trait WriteExt: Write + WriteBytesExt {
    fn write_u8(&mut self, n: u8) {
        WriteBytesExt::write_u8(self, n).unwrap()
    }

    fn write_u16_native(&mut self, n: u16) {
        self.write_u16::<NativeEndian>(n).unwrap()
    }

    fn write_i32_native(&mut self, n: i32) {
        self.write_i32::<NativeEndian>(n).unwrap()
    }

    fn write_u32_native(&mut self, n: u32) {
        self.write_u32::<NativeEndian>(n).unwrap()
    }

    fn write_u64_native(&mut self, n: u64) {
        self.write_u64::<NativeEndian>(n).unwrap()
    }

    fn write_fixed_size_string(&mut self, s: &str, size: usize) {
        assert!(s.is_ascii());
        assert!(s.len() <= size);

        let mut buf = vec![0u8; size];

        for (i, c) in s.chars().enumerate() {
            buf[i] = c as u8;
        }

        self.write_all(&buf).unwrap();
    }
}

impl<T> WriteExt for T where T: Write {}
