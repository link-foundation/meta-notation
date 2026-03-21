//! Tests for the meta-notation parser
//!
//! These tests verify the exact parsed object structure matches the expected
//! output, ensuring consistency with the JavaScript implementation.

use meta_notation::{parse, Block};

#[test]
fn test_parse_plain_text() {
    let result = parse("hello world");
    assert_eq!(result, vec![Block::Text("hello world".to_string())]);
}

#[test]
fn test_parse_parentheses() {
    let result = parse("(hello)");
    assert_eq!(result, vec![Block::Paren(vec![Block::Text("hello".to_string())])]);
}

#[test]
fn test_parse_curly_braces() {
    let result = parse("{world}");
    assert_eq!(result, vec![Block::Curly(vec![Block::Text("world".to_string())])]);
}

#[test]
fn test_parse_square_brackets() {
    let result = parse("[test]");
    assert_eq!(result, vec![Block::Square(vec![Block::Text("test".to_string())])]);
}

#[test]
fn test_parse_single_quotes() {
    let result = parse("'hello'");
    assert_eq!(result, vec![Block::SingleQuote("hello".to_string())]);
}

#[test]
fn test_parse_double_quotes() {
    let result = parse("\"world\"");
    assert_eq!(result, vec![Block::DoubleQuote("world".to_string())]);
}

#[test]
fn test_parse_backticks() {
    let result = parse("`code`");
    assert_eq!(result, vec![Block::Backtick("code".to_string())]);
}

#[test]
fn test_parse_mixed_delimiters() {
    let result = parse("hello (world) {test}");
    assert_eq!(
        result,
        vec![
            Block::Text("hello ".to_string()),
            Block::Paren(vec![Block::Text("world".to_string())]),
            Block::Text(" ".to_string()),
            Block::Curly(vec![Block::Text("test".to_string())]),
        ]
    );
}

#[test]
fn test_parse_nested_structures() {
    let result = parse("(a (b) c)");
    assert_eq!(
        result,
        vec![Block::Paren(vec![
            Block::Text("a ".to_string()),
            Block::Paren(vec![Block::Text("b".to_string())]),
            Block::Text(" c".to_string()),
        ])]
    );
}

#[test]
fn test_parse_complex_nested_structures() {
    let result = parse("{a [b (c) d] e}");
    assert_eq!(
        result,
        vec![Block::Curly(vec![
            Block::Text("a ".to_string()),
            Block::Square(vec![
                Block::Text("b ".to_string()),
                Block::Paren(vec![Block::Text("c".to_string())]),
                Block::Text(" d".to_string()),
            ]),
            Block::Text(" e".to_string()),
        ])]
    );
}

#[test]
fn test_parse_empty_delimiters() {
    let result = parse("(){}[]");
    assert_eq!(
        result,
        vec![
            Block::Paren(vec![]),
            Block::Curly(vec![]),
            Block::Square(vec![]),
        ]
    );
}

#[test]
fn test_parse_quotes_with_special_chars() {
    let result = parse("\"hello {world}\"");
    assert_eq!(result, vec![Block::DoubleQuote("hello {world}".to_string())]);
}

#[test]
fn test_parse_javascript_like_code() {
    let result = parse("function test() { return \"hello\"; }");
    assert_eq!(result.len(), 4);
    assert!(matches!(result[0], Block::Text(_)));
    assert!(matches!(result[1], Block::Paren(_)));
    assert!(matches!(result[3], Block::Curly(_)));
}

#[test]
fn test_parse_python_like_code() {
    let result = parse("def test(): return [1, 2, 3]");
    let has_paren = result.iter().any(|b| matches!(b, Block::Paren(_)));
    let has_square = result.iter().any(|b| matches!(b, Block::Square(_)));
    let has_text = result.iter().any(|b| matches!(b, Block::Text(_)));
    assert!(has_paren);
    assert!(has_square);
    assert!(has_text);
}

#[test]
fn test_parse_json_like_structure() {
    let result = parse("{\"key\": \"value\", \"array\": [1, 2, 3]}");
    assert!(matches!(result[0], Block::Curly(_)));
    if let Block::Curly(ref content) = result[0] {
        assert!(!content.is_empty());
    }
}
