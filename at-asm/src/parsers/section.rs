use super::ParseStr;
use std::process::exit;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Section {
    Text,
    Data,
    Bss,
}

impl ParseStr for Section {
    fn try_parse_str(s: &str) -> Option<Self> {
        let mut tokens = s.split_whitespace();

        if tokens.next() != Some("section") {
            return None;
        }

        let section = match tokens.next() {
            Some(".text") => Section::Text,
            Some(".data") => Section::Data,
            Some(".bss") => Section::Bss,
            Some(other) => {
                eprintln!("error: unrecognized section {}", other);
                exit(1);
            }
            None => {
                eprintln!("error: section name is expected");
                exit(1);
            }
        };

        Some(section)
    }

    fn parse_str(s: &str) -> Self {
        match Section::try_parse_str(s) {
            Some(sect) => sect,
            None => {
                eprintln!("error: invalid section declaration");
                exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_valid_str() {
        assert_eq!(Section::parse_str("section .text"), Section::Text);
        assert_eq!(Section::parse_str("section .data"), Section::Data);
        assert_eq!(Section::parse_str("section .bss"), Section::Bss);
    }
}
