use std::fmt;

pub struct StringTable {
    data: Vec<u8>,
}

impl StringTable {
    pub fn with_null() -> Self {
        StringTable { data: vec![0] }
    }

    pub fn get(&self, idx: usize) -> &str {
        let bytes = self.data[idx..].split(|n| *n == 0).next().unwrap();
        std::str::from_utf8(bytes).unwrap()
    }

    pub fn push_with_null(&mut self, s: &str) {
        for c in s.chars() {
            if !c.is_ascii() {
                panic!("could not push non-ascii char");
            }
            self.data.push(c as u8);
        }
        self.data.push(0);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.as_ref()
            .split(|b| *b == 0)
            .skip(1)
            .map(|bytes| std::str::from_utf8(bytes).unwrap())
    }
}

impl AsRef<[u8]> for StringTable {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl From<Vec<u8>> for StringTable {
    fn from(data: Vec<u8>) -> Self {
        assert!(data.starts_with(&[0]));
        assert!(data.ends_with(&[0]));
        assert!(data.iter().all(|n| *n == 0 || n.is_ascii()));

        StringTable { data }
    }
}

impl fmt::Debug for StringTable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_list();

        for n in self.data.iter() {
            match n {
                0 => {
                    debug.entry(&"");
                }
                _ => {
                    debug.entry(&char::from(*n));
                }
            }
        }

        debug.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_0_always_return_empty_string() {
        let data = vec![0x00, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x00];
        let table = StringTable::from(data);
        assert_eq!(table.get(0), "");

        let mut table = StringTable::with_null();
        table.push_with_null("hoge");
        assert_eq!(table.get(0), "");
    }

    #[test]
    fn get_string_from_vec() {
        let data = vec![0x00, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x00];
        let table = StringTable::from(data);

        assert_eq!(table.get(1), "_main");
        assert_eq!(table.get(2), "main");
    }

    #[test]
    fn get_string() {
        let mut table = StringTable::with_null();
        table.push_with_null("hoge");

        assert_eq!(table.get(1), "hoge");
    }
}
