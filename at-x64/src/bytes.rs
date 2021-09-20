use byteorder::{WriteBytesExt as _, LE};

pub struct BytesAtMost<const MAX: usize> {
    bytes: [u8; MAX],
    len: usize,
}

impl<const MAX: usize> BytesAtMost<MAX> {
    pub fn with_len(len: usize) -> Self {
        assert!(len <= MAX);

        BytesAtMost {
            bytes: [0; MAX],
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[0..self.len]
    }
}

impl<const N: usize, const MAX: usize> From<[u8; N]> for BytesAtMost<MAX> {
    fn from(array: [u8; N]) -> Self {
        assert!(N <= MAX);

        let mut bytes = BytesAtMost::with_len(N);
        bytes.bytes_mut().copy_from_slice(&array[..]);
        bytes
    }
}

impl<const MAX: usize> From<u8> for BytesAtMost<MAX> {
    fn from(n: u8) -> Self {
        assert!(MAX >= 1);

        BytesAtMost::from([n])
    }
}

impl<const MAX: usize> From<u16> for BytesAtMost<MAX> {
    fn from(n: u16) -> Self {
        assert!(MAX >= 1);

        let mut bytes = BytesAtMost::with_len(2);
        bytes.bytes_mut().write_u16::<LE>(n).unwrap();
        bytes
    }
}

impl<const MAX: usize> From<u32> for BytesAtMost<MAX> {
    fn from(n: u32) -> Self {
        assert!(MAX >= 4);

        let mut bytes = BytesAtMost::with_len(4);
        bytes.bytes_mut().write_u32::<LE>(n).unwrap();
        bytes
    }
}

impl<const MAX: usize> From<u64> for BytesAtMost<MAX> {
    fn from(n: u64) -> Self {
        assert!(MAX >= 8);

        let mut bytes = BytesAtMost::with_len(8);
        bytes.bytes_mut().write_u64::<LE>(n).unwrap();
        bytes
    }
}
