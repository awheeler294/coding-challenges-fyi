use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum JsonValue<'v> {
    Object(JsonObject<'v>),
    Array(Vec<Self>),
    String(&'v str),
    //TODO: support decimal types
    Number(i32),
    Bool(bool),
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub struct JsonObject<'o>(HashMap<&'o str, JsonValue<'o>>);

impl<'o> JsonObject<'o> {
    fn new() -> Self {
        Self { 0: HashMap::new() }
    }

    pub fn from_str(input: &'o str) -> Result<Self> {
        let mut tokens = Token::tokenize(&input);
        let mut input = input.trim();

        if input.starts_with('{') == false {
            return Err(anyhow!(""));
        }

        let mut kv_store = HashMap::new();

        input = &input[1..];

        Ok(Self { 0: kv_store })
    }

    fn parse(tokens: VecDeque<Token>) -> Result<Self> {
        // if let Some(Token::OpenBracket) = tokens.pop_front() {
        //     let
        //
        //     match tokens.pop_front() {
        //         Some(Str(s)) =>
        //     }
        // } else {
        Err(anyhow!("json objects must start with a `{{`"))
        // }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'t> {
    OpenBrace,
    ClosedBrace,
    OpenBracket,
    ClosedBracket,
    Colon,
    True,
    False,
    Null,
    Str(&'t str),
    Number(&'t str),
}

impl<'t> Token<'t> {
    fn tokenize(input: &'t str) -> Result<VecDeque<Token<'t>>> {
        let mut tokens = VecDeque::new();
        let mut skip = 0;

        for (i, ch) in input.chars().enumerate() {
            if skip > 0 {
                skip -= 1;
            } else {
                if ch == '{' {
                    tokens.push_back(Self::OpenBrace);
                } else if ch == '}' {
                    tokens.push_back(Self::ClosedBrace);
                } else if ch == '[' {
                    tokens.push_back(Self::OpenBracket);
                } else if ch == ']' {
                    tokens.push_back(Self::ClosedBracket);
                } else if ch == ':' {
                    tokens.push_back(Self::Colon);
                } else if let Some(str) = Self::parse_str(&input[i..]) {
                    skip = str.len() + 1;
                    tokens.push_back(Self::Str(str));
                } else if let Some(num) = Self::parse_number(&input[i..]) {
                    skip = num.len() + 1;
                    tokens.push_back(Token::Number(num));
                }
            }
        }

        Ok(tokens)
    }

    fn parse_str(input: &'t str) -> Option<&'t str> {
        let mut chars = input.chars();

        if chars.next() != Some('"') {
            return None;
        }

        let mut len = 0;
        let mut escape = false;
        let mut end_quote_found = false;

        while let Some(ch) = chars.next() {
            len += 1;

            if escape == false {
                if ch == '"' {
                    end_quote_found = true;
                    break;
                }

                if ch == '\\' {
                    escape = true;
                }
            } else {
                escape = false;
            }
        }

        if end_quote_found {
            Some(&input[1..len])
        } else {
            None
        }
    }

    fn parse_number(input: &'t str) -> Option<&'t str> {
        let mut chars = input.chars();

        let mut len = 0;

        while let Some(ch) = chars.next() {
            // TODO: handle more types of numbers, like decimals and hex
            if ch.is_numeric() {
                len += 1;
            } else {
                break;
            }
        }

        if len > 0 {
            Some(&input[..len])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = r#"{
            "key": "value",
            [
                1,
                2,
                3,
            ]
        }"#;

        let expected = VecDeque::from([
            Token::OpenBrace,
            Token::Str("key"),
            Token::Colon,
            Token::Str("value"),
            Token::OpenBracket,
            Token::Number("1"),
            Token::Number("2"),
            Token::Number("3"),
            Token::ClosedBracket,
            Token::ClosedBrace,
        ]);

        let actual = Token::tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_simple_object() {
        let input = "{}";
        let expected = JsonObject::new();

        let actual = JsonObject::from_str(&input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_str() {
        for (input, expected) in [
            (r#""a string""#, Some("a string")),
            (r#""""#, Some("")),
            (
                r#""A string with matching \"escaped\" quotes in the middle""#,
                Some(r#"A string with matching \"escaped\" quotes in the middle"#),
            ),
            (
                r#""A string with a single \"escaped quote in the middle""#,
                Some(r#"A string with a single \"escaped quote in the middle"#),
            ),
            (
                r#""A string with characters after the closing quote"abc"#,
                Some("A string with characters after the closing quote"),
            ),
            (
                r#"abc"A string with characters before the opening quote""#,
                None,
            ),
            (r#"Not a string; no open quotes""#, None),
            (r#""Not a string; no closing quotes"#, None),
            (r#"\"Not a string; escaped open quotes""#, None),
            (r#""Not a string; escaped closing quotes\""#, None),
        ] {
            let actual = Token::parse_str(&input);
            assert_eq!(actual, expected);
        }
    }
}
