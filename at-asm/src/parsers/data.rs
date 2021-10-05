use super::{ParseError, ParseStr};
use byteorder::{WriteBytesExt as _, LE};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Data {
    label: Option<String>,
    bytes: Vec<u8>,
}

impl Data {
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl ParseStr for Data {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty());

        let (label, dx) = match tokens.next() {
            dx @ Some("db" | "dw" | "dd" | "dq") => (None, dx.unwrap()),
            Some(label) => match tokens.next() {
                dx @ Some("db" | "dw" | "dd" | "dq") => (Some(label.to_string()), dx.unwrap()),
                _ => return Ok(None),
            },
            None => return Ok(None),
        };

        let mut bytes = Vec::new();

        match dx {
            "db" => {
                for token in tokens {
                    let n = u8::parse_str(token)?;
                    bytes.write_u8(n).unwrap();
                }
            }
            "dw" => {
                for token in tokens {
                    let n = u16::parse_str(token)?;
                    bytes.write_u16::<LE>(n).unwrap();
                }
            }
            "dd" => {
                for token in tokens {
                    let n = u32::parse_str(token)?;
                    bytes.write_u32::<LE>(n).unwrap();
                }
            }
            "dq" => {
                for token in tokens {
                    let n = u64::parse_str(token)?;
                    bytes.write_u64::<LE>(n).unwrap();
                }
            }
            _ => unreachable!(),
        };

        Ok(Some(Data { label, bytes }))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(s) => Ok(s),
            None => Err(ParseError::new("invalid data format")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ok() {
        assert_eq!(
            Data::try_parse_str("db 0x42,42,0x11"),
            Ok(Some(Data {
                label: None,
                bytes: vec![0x42, 42, 0x11]
            }))
        );

        assert_eq!(
            Data::try_parse_str("hoge db 0x42"),
            Ok(Some(Data {
                label: Some("hoge".to_string()),
                bytes: vec![0x42]
            }))
        );
    }
}
