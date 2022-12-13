use std::str::Chars;

use itertools::{Itertools, MultiPeek};

pub struct Parser<'c> {
    content: MultiPeek<Chars<'c>>,
}

pub trait Parse<'c> {
    fn parse(parser: &mut Parser<'c>) -> Self;
}

impl<'c> Parser<'c> {
    pub fn new(content: &'c str) -> Self {
        Self {
            content: content.chars().multipeek(),
        }
    }

    pub fn parse<T: Parse<'c>>(&mut self) -> T {
        T::parse(self)
    }

    pub fn one(&mut self, c: char) -> Option<char> {
        self.one_with_fn(|x| x == c)
    }

    pub fn one_with_fn(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(c) = self.content.peek() {
            if f(*c) {
                self.content.next()
            } else {
                self.content.reset_peek();
                None
            }
        } else {
            None
        }
    }

    pub fn parse_str(&mut self, s: &str) -> Option<String> {
        let mut result = String::new();
        for c in s.chars() {
            if let Some(c) = self.one(c) {
                result.push(c);
            } else {
                return None;
            }
        }
        Some(result)
    }

    pub fn parse_single_digit(&mut self) -> Option<u8> {
        self.one_with_fn(|c| c.is_ascii_digit())
            .and_then(|c| c.to_digit(10))
            .map(|x| x as u8)
    }

    pub fn parse_int(&mut self) -> Option<i64> {
        let mut result = 0;
        let mut sign = 1;
        if self.one('-').is_some() {
            sign = -1;
        } else {
            self.one('+');
        }
        let mut first = true;
        while let Some(digit) = self.parse_single_digit() {
            first = false;
            result = result * 10 + digit as i64;
        }
        if first {
            None
        } else {
            Some(result * sign)
        }
    }

    pub fn matches(&mut self, s: &str) -> bool {
        if self.parse_str(s).is_some() {
            true
        } else {
            self.content.reset_peek();
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        let mut parser = super::Parser::new("abc");
        assert_eq!(parser.one('a'), Some('a'));
        assert_eq!(parser.one('b'), Some('b'));
        assert_eq!(parser.one('c'), Some('c'));
        assert_eq!(parser.one('d'), None);
    }

    #[test]
    fn test_one_with_fn() {
        let mut parser = super::Parser::new("abc");
        assert_eq!(parser.one_with_fn(|c| c == 'a' || c == 'b'), Some('a'));
        assert_eq!(parser.one_with_fn(|c| c == 'a' || c == 'b'), Some('b'));
        assert_eq!(parser.one_with_fn(|c| c == 'c'), Some('c'));
        assert_eq!(parser.one_with_fn(|c| c == 'd'), None);
    }

    #[test]
    fn test_parse_str() {
        let mut parser = super::Parser::new("abc");
        assert_eq!(parser.parse_str("abc"), Some("abc".to_string()));
        assert_eq!(parser.parse_str("abc"), None);
    }

    #[test]
    fn test_parse_single_digit() {
        let mut parser = super::Parser::new("123");
        assert_eq!(parser.parse_single_digit(), Some(1));
        assert_eq!(parser.parse_single_digit(), Some(2));
        assert_eq!(parser.parse_single_digit(), Some(3));
        assert_eq!(parser.parse_single_digit(), None);
    }

    #[test]
    fn test_parse_int() {
        let mut parser = super::Parser::new("123");
        assert_eq!(parser.parse_int(), Some(123));
        assert_eq!(parser.parse_int(), None);
        let mut parser = super::Parser::new("-123");
        assert_eq!(parser.parse_int(), Some(-123));
    }
}
