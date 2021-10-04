use super::{ParseError, ParseStr};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Section {
    Text,
    Data,
    Bss,
}

impl ParseStr for Section {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("section") {
            return Ok(None);
        }

        let sect = match tokens.next() {
            Some(".text") => Section::Text,
            Some(".data") => Section::Data,
            Some(".bss") => Section::Bss,
            Some(other) => {
                return Err(ParseError::new(format!("unrecognized section {}", other)));
            }
            None => {
                return Err(ParseError::new("section name is expected"));
            }
        };

        if tokens.next().is_some() {
            return Err(ParseError::new(
                "expected end of line after section declaration",
            ));
        }

        Ok(Some(sect))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Section::try_parse_str(s) {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Err(ParseError::new("invalid section declaration")),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ok_some() {
        assert_eq!(Section::parse_str("section .text").unwrap(), Section::Text);
        assert_eq!(Section::parse_str("section .data").unwrap(), Section::Data);
        assert_eq!(Section::parse_str("section .bss").unwrap(), Section::Bss);
    }

    #[test]
    fn should_ok_none() {
        assert!(Section::try_parse_str("mov rax, 42").unwrap().is_none());
        assert!(Section::try_parse_str("hoge").unwrap().is_none());
    }

    #[test]
    fn should_err() {
        assert!(Section::try_parse_str("section .hoge").is_err());
        assert!(Section::try_parse_str("section .text .data").is_err());
    }
}
