use super::{ParseError, ParseStr};
use byteorder::{WriteBytesExt as _, LE};
use std::convert::TryFrom as _;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Data {
    pub label: Option<String>,
    pub bytes: Vec<u8>,
}

impl ParseStr for Data {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        // label, align, data の3パートに分ける
        let (label, align, data_str) = match split_parts(s) {
            Some(tuple) => tuple,
            None => return Ok(None),
        };

        let mut bytes = Vec::new();

        for data in DataDefIter::new(data_str) {
            data?.write_to(&mut bytes, align)?;
        }

        let label = label.map(str::to_string);

        Ok(Some(Data { label, bytes }))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(s) => Ok(s),
            None => Err(ParseError::new("invalid data format")),
        }
    }
}

#[derive(Clone, Copy)]
enum Align {
    DB,
    DW,
    DD,
    DQ,
}

// 入力文字列を (label, dx, data) に分ける
fn split_parts(s: &str) -> Option<(Option<&str>, Align, &str)> {
    use Align::*;

    let s = s.trim();

    let (t1, rmn) = split_whitespace_once(s.trim())?;

    match t1 {
        "db" => Some((None, DB, rmn)),
        "dw" => Some((None, DW, rmn)),
        "dd" => Some((None, DD, rmn)),
        "dq" => Some((None, DQ, rmn)),
        label => {
            let label = Some(label);
            let (t2, rmn) = split_whitespace_once(rmn)?;
            match t2 {
                "db" => Some((label, DB, rmn)),
                "dw" => Some((label, DW, rmn)),
                "dd" => Some((label, DD, rmn)),
                "dq" => Some((label, DQ, rmn)),
                _ => None,
            }
        }
    }
}

fn split_whitespace_once(s: &str) -> Option<(&str, &str)> {
    match s.split_once(char::is_whitespace) {
        Some((s1, s2)) => Some((s1.trim(), s2.trim())),
        None => None,
    }
}

enum DataDef<'a> {
    Num(u64),
    Str(&'a str),
}

impl<'a> DataDef<'a> {
    // align: 0 -> db, 1 -> dw, 2 -> dd, 3 -> dq
    fn write_to(&self, buf: &mut Vec<u8>, align: Align) -> Result<(), ParseError> {
        match self {
            DataDef::Num(n) => write_num(*n, buf, align),
            DataDef::Str(s) => write_str(s, buf, align),
        }
    }
}

fn write_num(n: u64, buf: &mut Vec<u8>, align: Align) -> Result<(), ParseError> {
    use Align::*;

    match align {
        DB => {
            if let Some(n) = u8::try_from(n).ok() {
                buf.write_u8(n).unwrap();
                Ok(())
            } else {
                Err(ParseError::new(format!("{} is not a 8 bit number", n)))
            }
        }
        DW => {
            if let Some(n) = u16::try_from(n).ok() {
                buf.write_u16::<LE>(n).unwrap();
                Ok(())
            } else {
                Err(ParseError::new(format!("{} is not a 16 bit number", n)))
            }
        }
        DD => {
            if let Some(n) = u32::try_from(n).ok() {
                buf.write_u32::<LE>(n).unwrap();
                Ok(())
            } else {
                Err(ParseError::new(format!("{} is not a 32 bit number", n)))
            }
        }
        DQ => {
            buf.write_u64::<LE>(n).unwrap();
            Ok(())
        }
    }
}

fn write_str(s: &str, buf: &mut Vec<u8>, align: Align) -> Result<(), ParseError> {
    use Align::*;

    buf.extend_from_slice(s.as_bytes());

    let pad = match align {
        DB => 0,
        DW => s.len() % 2,
        DD => (4 - s.len() % 4) % 4,
        DQ => (8 - s.len() % 8) % 8,
    };

    for _ in 0..pad {
        buf.push(0);
    }

    Ok(())
}

struct DataDefIter<'a> {
    s: &'a str,
}

impl<'a> DataDefIter<'a> {
    fn new(s: &'a str) -> Self {
        DataDefIter { s }
    }
}

impl<'a> Iterator for DataDefIter<'a> {
    type Item = Result<DataDef<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        fn split_delim_once(s: &str) -> (&str, &str) {
            match s.split_once(|c: char| c.is_whitespace() || c == ',') {
                Some((s1, s2)) => (s1, s2.trim_start()),
                None => (s, ""),
            }
        }

        if self.s.is_empty() {
            return None;
        }

        if self.s.starts_with("\"") {
            let (_, s) = self.s.split_at(1);
            match s.split_once('\"') {
                None => Some(Err(ParseError::new("unclosed \" found"))),
                Some((str_data, rmn)) => {
                    self.s = rmn.trim_start_matches(|c: char| c.is_whitespace() || c == ',');
                    Some(Ok(DataDef::Str(str_data)))
                }
            }
        } else {
            let (s1, s2) = split_delim_once(self.s);

            self.s = s2;

            match u64::parse_str(s1) {
                Ok(n) => Some(Ok(DataDef::Num(n))),
                Err(e) => Some(Err(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_bytes {
        ($s:expr, $bytes:expr) => {
            assert_eq!(Data::try_parse_str($s).unwrap().unwrap().bytes, $bytes);
        };
    }

    macro_rules! assert_label_none {
        ($s:expr) => {
            assert!(Data::try_parse_str($s).unwrap().unwrap().label.is_none());
        };
    }

    macro_rules! assert_label {
        ($s:expr, $label:expr) => {
            assert_eq!(
                Data::try_parse_str($s).unwrap().unwrap().label.unwrap(),
                $label
            );
        };
    }

    #[test]
    fn should_ok() {
        assert_label_none!("db 0x42");

        assert_bytes!("db 0x42,42,0x11", vec![0x42, 42, 0x11]);

        assert_label!("hoge db 0x42", "hoge");

        assert_bytes!("dw 0x12, 0x34", vec![0x12, 0x00, 0x34, 0x00]);

        assert_bytes!("dd 0x12, 0x34", vec![0x12, 0, 0, 0, 0x34, 0, 0, 0]);

        assert_bytes!(
            "db \"Hello, World\"",
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64]
        );

        assert_label!("mdb db 0x42", "mdb");

        assert_bytes!(
            "db \"Hello\", 0x42",
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x42]
        );

        assert_bytes!(
            "dd \"Hello\", 0x42",
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0, 0, 0, 0x42, 0, 0, 0]
        );
    }
}
