//! # Meta-Notation
//!
//! A notation for the largest possible set of languages.
//! Focuses on parsing common delimiters: (), {}, [], '', "", ``
//!
//! This is a simpler version of links-notation without the : self-reference syntax,
//! making it compatible with a much larger set of programming languages.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Delimiter types supported by meta-notation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DelimiterType {
    Paren,
    Curly,
    Square,
    SingleQuote,
    DoubleQuote,
    Backtick,
    Text,
}

/// A block is either a delimited structure or plain text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum Block {
    Paren(Vec<Block>),
    Curly(Vec<Block>),
    Square(Vec<Block>),
    SingleQuote(String),
    DoubleQuote(String),
    Backtick(String),
    Text(String),
}

impl Block {
    /// Get the delimiter type of this block
    pub fn delimiter_type(&self) -> DelimiterType {
        match self {
            Block::Paren(_) => DelimiterType::Paren,
            Block::Curly(_) => DelimiterType::Curly,
            Block::Square(_) => DelimiterType::Square,
            Block::SingleQuote(_) => DelimiterType::SingleQuote,
            Block::DoubleQuote(_) => DelimiterType::DoubleQuote,
            Block::Backtick(_) => DelimiterType::Backtick,
            Block::Text(_) => DelimiterType::Text,
        }
    }

    /// Check if this block contains a specific delimiter type (recursively)
    pub fn has_delimiter_type(&self, dtype: &DelimiterType) -> bool {
        if &self.delimiter_type() == dtype {
            return true;
        }

        match self {
            Block::Paren(blocks) | Block::Curly(blocks) | Block::Square(blocks) => {
                blocks.iter().any(|b| b.has_delimiter_type(dtype))
            }
            _ => false,
        }
    }
}

/// Parser for meta-notation
pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given input
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// Get the current character without consuming it
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Consume and return the current character
    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Check if we're at the end of input
    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Parse a sequence of blocks
    pub fn parse_sequence(&mut self) -> Vec<Block> {
        let mut blocks = Vec::new();

        while !self.is_eof() {
            if let Some(block) = self.parse_block() {
                blocks.push(block);
            } else {
                break;
            }
        }

        blocks
    }

    /// Parse a single block
    fn parse_block(&mut self) -> Option<Block> {
        match self.peek()? {
            '(' => self.parse_delimited('(', ')', |blocks| Block::Paren(blocks)),
            '{' => self.parse_delimited('{', '}', |blocks| Block::Curly(blocks)),
            '[' => self.parse_delimited('[', ']', |blocks| Block::Square(blocks)),
            '\'' => self.parse_quote('\'', |s| Block::SingleQuote(s)),
            '"' => self.parse_quote('"', |s| Block::DoubleQuote(s)),
            '`' => self.parse_quote('`', |s| Block::Backtick(s)),
            ')' | '}' | ']' => {
                // Unmatched closing delimiter - treat as text
                let c = self.consume()?;
                Some(Block::Text(c.to_string()))
            }
            _ => self.parse_text(),
        }
    }

    /// Parse a delimited block (parentheses, braces, brackets)
    fn parse_delimited<F>(&mut self, open: char, close: char, constructor: F) -> Option<Block>
    where
        F: FnOnce(Vec<Block>) -> Block,
    {
        // Consume opening delimiter
        if self.consume()? != open {
            return None;
        }

        let mut blocks = Vec::new();

        // Parse until we find the closing delimiter
        while let Some(c) = self.peek() {
            if c == close {
                self.consume(); // Consume closing delimiter
                return Some(constructor(blocks));
            }

            if let Some(block) = self.parse_block() {
                blocks.push(block);
            } else {
                break;
            }
        }

        // If we reach here, we didn't find the closing delimiter
        // Return what we have anyway
        Some(constructor(blocks))
    }

    /// Parse a quoted string
    fn parse_quote<F>(&mut self, quote: char, constructor: F) -> Option<Block>
    where
        F: FnOnce(String) -> Block,
    {
        // Consume opening quote
        if self.consume()? != quote {
            return None;
        }

        let mut content = String::new();

        // Collect until closing quote
        while let Some(c) = self.peek() {
            if c == quote {
                self.consume(); // Consume closing quote
                return Some(constructor(content));
            }
            self.consume();
            content.push(c);
        }

        // If we reach here, we didn't find the closing quote
        // Return what we have anyway
        Some(constructor(content))
    }

    /// Parse plain text (non-delimiter characters)
    fn parse_text(&mut self) -> Option<Block> {
        let mut content = String::new();

        while let Some(c) = self.peek() {
            match c {
                '(' | ')' | '{' | '}' | '[' | ']' | '\'' | '"' | '`' => break,
                _ => {
                    self.consume();
                    content.push(c);
                }
            }
        }

        if content.is_empty() {
            None
        } else {
            Some(Block::Text(content))
        }
    }
}

/// Parse meta-notation text into a sequence of blocks
///
/// # Examples
///
/// ```
/// use meta_notation::parse;
///
/// let result = parse("hello (world) {test}");
/// assert_eq!(result.len(), 4);
/// ```
pub fn parse(input: &str) -> Vec<Block> {
    let mut parser = Parser::new(input);
    parser.parse_sequence()
}

/// Serialize a sequence of blocks back to text
///
/// # Examples
///
/// ```
/// use meta_notation::{parse, serialize};
///
/// let text = "hello (world)";
/// let blocks = parse(text);
/// let serialized = serialize(&blocks);
/// assert_eq!(serialized, text);
/// ```
pub fn serialize(blocks: &[Block]) -> String {
    blocks.iter().map(|b| serialize_block(b)).collect()
}

/// Serialize a single block
fn serialize_block(block: &Block) -> String {
    match block {
        Block::Text(s) => s.clone(),
        Block::Paren(blocks) => format!("({})", serialize(blocks)),
        Block::Curly(blocks) => format!("{{{}}}", serialize(blocks)),
        Block::Square(blocks) => format!("[{}]", serialize(blocks)),
        Block::SingleQuote(s) => format!("'{}'", s),
        Block::DoubleQuote(s) => format!("\"{}\"", s),
        Block::Backtick(s) => format!("`{}`", s),
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serialize_block(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_text() {
        let result = parse("hello world");
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Block::Text(_)));
    }

    #[test]
    fn test_parse_parentheses() {
        let result = parse("(hello)");
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Block::Paren(_)));
    }

    #[test]
    fn test_parse_mixed_delimiters() {
        let result = parse("hello (world) {test}");
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_round_trip() {
        let original = "hello (world) {test}";
        let parsed = parse(original);
        let serialized = serialize(&parsed);
        assert_eq!(serialized, original);
    }

    #[test]
    fn test_nested_structures() {
        let original = "{a [b (c) d] e}";
        let parsed = parse(original);
        let serialized = serialize(&parsed);
        assert_eq!(serialized, original);
    }

    #[test]
    fn test_quotes() {
        let original = r#"'single' "double" `backtick`"#;
        let parsed = parse(original);
        assert_eq!(parsed.len(), 5); // 3 quotes + 2 spaces
        let serialized = serialize(&parsed);
        assert_eq!(serialized, original);
    }

    #[test]
    fn test_has_delimiter_type() {
        let parsed = parse("hello (world) {test}");
        assert!(parsed[1].has_delimiter_type(&DelimiterType::Paren));
        assert!(parsed[3].has_delimiter_type(&DelimiterType::Curly));
    }

    #[test]
    fn test_javascript_code() {
        let code = "const greet = (name) => { return `Hello, ${name}!`; };";
        let parsed = parse(code);
        let serialized = serialize(&parsed);
        assert_eq!(serialized, code);
    }

    #[test]
    fn test_natural_language() {
        let text = "She said, \"Hello, world!\" and smiled.";
        let parsed = parse(text);
        let serialized = serialize(&parsed);
        assert_eq!(serialized, text);
    }
}
