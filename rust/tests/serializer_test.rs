//! Tests for the meta-notation serializer
//!
//! These tests verify that serialization produces the expected output and
//! that round-trip (parse -> serialize) preserves the original text.

use meta_notation::{parse, serialize, Block};

#[test]
fn test_serialize_plain_text() {
    let blocks = vec![Block::Text("hello world".to_string())];
    let result = serialize(&blocks);
    assert_eq!(result, "hello world");
}

#[test]
fn test_serialize_parentheses() {
    let blocks = vec![Block::Paren(vec![Block::Text("hello".to_string())])];
    let result = serialize(&blocks);
    assert_eq!(result, "(hello)");
}

#[test]
fn test_serialize_curly_braces() {
    let blocks = vec![Block::Curly(vec![Block::Text("world".to_string())])];
    let result = serialize(&blocks);
    assert_eq!(result, "{world}");
}

#[test]
fn test_serialize_square_brackets() {
    let blocks = vec![Block::Square(vec![Block::Text("test".to_string())])];
    let result = serialize(&blocks);
    assert_eq!(result, "[test]");
}

#[test]
fn test_serialize_quotes() {
    let blocks = vec![
        Block::SingleQuote("hello".to_string()),
        Block::Text(" ".to_string()),
        Block::DoubleQuote("world".to_string()),
    ];
    let result = serialize(&blocks);
    assert_eq!(result, "'hello' \"world\"");
}

#[test]
fn test_serialize_backticks() {
    let blocks = vec![Block::Backtick("code".to_string())];
    let result = serialize(&blocks);
    assert_eq!(result, "`code`");
}

#[test]
fn test_round_trip_parse_then_serialize() {
    let original = "hello (world) {test} [array] \"string\" `code`";
    let parsed = parse(original);
    let serialized = serialize(&parsed);
    assert_eq!(serialized, original);
}

#[test]
fn test_round_trip_nested_structures() {
    let original = "{a [b (c) d] e}";
    let parsed = parse(original);
    let serialized = serialize(&parsed);
    assert_eq!(serialized, original);
}

#[test]
fn test_round_trip_empty_delimiters() {
    let original = "(){}[]";
    let parsed = parse(original);
    let serialized = serialize(&parsed);
    assert_eq!(serialized, original);
}

#[test]
fn test_round_trip_complex_code() {
    let original = "function test() { return \"hello\"; }";
    let parsed = parse(original);
    let serialized = serialize(&parsed);
    assert_eq!(serialized, original);
}
