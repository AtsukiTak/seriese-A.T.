use super::{ParseError, ParseStr};
use std::convert::TryFrom as _;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Expr(u64);

impl Expr {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_u32(&self) -> Option<u32> {
        u32::try_from(self.0).ok()
    }

    pub fn as_u16(&self) -> Option<u16> {
        u16::try_from(self.0).ok()
    }
}

impl ParseStr for Expr {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        let mut tokens = s.split_whitespace();

        let token1 = match tokens.next() {
            Some(t) => t,
            None => return Ok(None),
        };
        let lhs = match Token::try_parse_str(token1)? {
            Some(Token::Num(n)) => n,
            Some(Token::Plus | Token::Minus) => {
                return Err(ParseError::new("unary op is not supported"))
            }
            None => return Ok(None),
        };

        let token2 = match tokens.next() {
            Some(t) => t,
            None => return Ok(Some(Expr(lhs))),
        };
        let sign = match Token::try_parse_str(token2)? {
            Some(Token::Plus) => true,
            Some(Token::Minus) => false,
            Some(_) => return Err(ParseError::new("expected + or -")),
            None => return Err(ParseError::new("expected + or -")),
        };

        let token3 = match tokens.next() {
            Some(t) => t,
            None => return Err(ParseError::new("rhs not found")),
        };
        let rhs = match Token::try_parse_str(token3)? {
            Some(Token::Num(n)) => n,
            _ => return Err(ParseError::new("invalid rhs")),
        };

        let res = lhs as i64 + sign_to_u64(sign) * rhs as i64;

        Ok(Some(Expr(res as u64)))
    }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        match Self::try_parse_str(s)? {
            Some(t) => Ok(t),
            None => Err(ParseError::new("invalid expr format")),
        }
    }
}

fn sign_to_u64(sign: bool) -> i64 {
    sign as i64 * 2 - 1
}

enum Token {
    Num(u64),
    Plus,
    Minus,
}

impl ParseStr for Token {
    fn try_parse_str(s: &str) -> Result<Option<Self>, ParseError> {
        match s {
            "+" => Ok(Some(Token::Plus)),
            "-" => Ok(Some(Token::Minus)),
            s => match u64::try_parse_str(s)? {
                Some(n) => Ok(Some(Token::Num(n))),
                None => Ok(None),
            },
        }
    }

    fn parse_str(_: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(Expr::parse_str("0x200000 + 4").unwrap(), Expr(0x200000 + 4));
        assert_eq!(Expr::parse_str("200000 - 4").unwrap(), Expr(200000 - 4));
        assert_eq!(Expr::parse_str("0x200 + 0x15").unwrap(), Expr(0x200 + 0x15));
    }
}
