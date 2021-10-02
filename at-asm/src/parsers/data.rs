use super::ParseStr;
use byteorder::{WriteBytesExt as _, LE};

pub struct Data(pub Vec<u8>);

impl ParseStr for Data {
    fn try_parse_str(s: &str) -> Option<Self> {
        let mut tokens = s
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty());

        let mut data = Vec::new();

        match tokens.next() {
            Some("db") => {
                tokens
                    .map(u8::parse_str)
                    .for_each(|n| data.write_u8(n).unwrap());
            }
            Some("dw") => {
                tokens
                    .map(u16::parse_str)
                    .for_each(|n| data.write_u16::<LE>(n).unwrap());
            }
            Some("dd") => {
                tokens
                    .map(u32::parse_str)
                    .for_each(|n| data.write_u32::<LE>(n).unwrap());
            }
            Some("dq") => {
                tokens
                    .map(u64::parse_str)
                    .for_each(|n| data.write_u64::<LE>(n).unwrap());
            }
            _ => return None,
        };

        Some(Data(data))
    }

    fn parse_str(_s: &str) -> Self {
        unimplemented!()
    }
}
