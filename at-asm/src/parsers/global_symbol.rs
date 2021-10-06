use super::{ParseError, ParseStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GlobalSymbol(pub String);

impl ParseStr for GlobalSymbol {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("global") {
            return Ok(None);
        }

        let label = match tokens.next() {
            Some(label) => label.to_string(),
            None => {
                return Err(ParseError::new("symbol name is expected"));
            }
        };

        if tokens.next().is_some() {
            return Err(ParseError::new(
                "expected end of line after global symbol definition",
            ));
        }

        Ok(Some(GlobalSymbol(label)))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(t) => Ok(t),
            None => Err(ParseError::new("invalid global symbol definition")),
        }
    }
}
