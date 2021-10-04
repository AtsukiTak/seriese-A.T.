use super::{ParseError, ParseStr};
use byteorder::{WriteBytesExt as _, LE};

pub struct Data(pub Vec<u8>);

impl ParseStr for Data {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty());

        let mut data = Vec::new();

        match tokens.next() {
            Some("db") => {
                for token in tokens {
                    let n = u8::parse_str(token)?;
                    data.write_u8(n).unwrap();
                }
            }
            Some("dw") => {
                for token in tokens {
                    let n = u16::parse_str(token)?;
                    data.write_u16::<LE>(n).unwrap();
                }
            }
            Some("dd") => {
                for token in tokens {
                    let n = u32::parse_str(token)?;
                    data.write_u32::<LE>(n).unwrap();
                }
            }
            Some("dq") => {
                for token in tokens {
                    let n = u64::parse_str(token)?;
                    data.write_u64::<LE>(n).unwrap();
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(Data(data)))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(s) => Ok(s),
            None => Err(ParseError::new("invalid data format")),
        }
    }
}
