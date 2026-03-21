# Meta-Notation (Rust)

Rust implementation of meta-notation parser.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
meta-notation = "0.1"
```

## Usage

```rust
use meta_notation::{parse, serialize};

fn main() {
    // Parse code with delimiters
    let code = r#"function test() { return "hello"; }"#;
    let parsed = parse(code);

    // Serialize back to string
    let serialized = serialize(&parsed);
    assert_eq!(serialized, code);
}
```

## API

### `parse(input: &str) -> Vec<Block>`

Parses text into a sequence of blocks.

- **Bracket delimiters** (`Paren`, `Curly`, `Square`): contain `Vec<Block>` (nested blocks)
- **Quote delimiters** (`SingleQuote`, `DoubleQuote`, `Backtick`): contain `String` (no nested parsing inside quotes)
- **Plain text** (`Text`): contains `String`

```rust
let result = parse("hello (world) {test}");
// Returns:
// [
//   Block::Text("hello "),
//   Block::Paren([Block::Text("world")]),
//   Block::Text(" "),
//   Block::Curly([Block::Text("test")])
// ]
```

Nested structures are supported:

```rust
let result = parse("{a [b (c) d] e}");
// Returns:
// [
//   Block::Curly([
//     Block::Text("a "),
//     Block::Square([
//       Block::Text("b "),
//       Block::Paren([Block::Text("c")]),
//       Block::Text(" d")
//     ]),
//     Block::Text(" e")
//   ])
// ]
```

Quotes capture content as a plain string (no nested parsing):

```rust
let result = parse("\"hello {world}\"");
// Returns:
// [
//   Block::DoubleQuote("hello {world}")
// ]
```

### `serialize(blocks: &[Block]) -> String`

Converts a sequence of blocks back to text.

```rust
let blocks = vec![
    Block::Text("hello ".to_string()),
    Block::Paren(vec![Block::Text("world".to_string())]),
];
let text = serialize(&blocks);
assert_eq!(text, "hello (world)");
```

### Block Methods

```rust
// Get the delimiter type of a block
let block = Block::Paren(vec![]);
assert_eq!(block.delimiter_type(), DelimiterType::Paren);

// Check if a block or its children contain a specific delimiter type (recursive)
let result = parse("(a {b} c)");
assert!(result[0].has_delimiter_type(&DelimiterType::Curly));
```

## Types

```rust
pub enum Block {
    Paren(Vec<Block>),       // ()  - content is nested blocks
    Curly(Vec<Block>),       // {}  - content is nested blocks
    Square(Vec<Block>),      // []  - content is nested blocks
    SingleQuote(String),     // ''  - content is a plain string
    DoubleQuote(String),     // ""  - content is a plain string
    Backtick(String),        // ``  - content is a plain string
    Text(String),            // plain text
}

pub enum DelimiterType {
    Paren,
    Curly,
    Square,
    SingleQuote,
    DoubleQuote,
    Backtick,
    Text,
}
```

### Content types by delimiter

| Delimiter | Variant | Content Type | Example |
|-----------|---------|-------------|---------|
| `()` | `Block::Paren(...)` | `Vec<Block>` | `Block::Paren(vec![Block::Text("x".into())])` |
| `{}` | `Block::Curly(...)` | `Vec<Block>` | `Block::Curly(vec![Block::Text("x".into())])` |
| `[]` | `Block::Square(...)` | `Vec<Block>` | `Block::Square(vec![Block::Text("x".into())])` |
| `''` | `Block::SingleQuote(...)` | `String` | `Block::SingleQuote("hello".into())` |
| `""` | `Block::DoubleQuote(...)` | `String` | `Block::DoubleQuote("hello".into())` |
| `` `` `` | `Block::Backtick(...)` | `String` | `Block::Backtick("hello".into())` |
| plain text | `Block::Text(...)` | `String` | `Block::Text("hello".into())` |

### Serde JSON Serialization

The `Block` enum uses serde's `#[serde(tag = "type", content = "content", rename_all = "camelCase")]` attribute, producing JSON identical to the JavaScript implementation:

```rust
let result = parse("hello (world)");
let json = serde_json::to_string_pretty(&result).unwrap();
// Produces:
// [
//   { "type": "text", "content": "hello " },
//   { "type": "paren", "content": [{ "type": "text", "content": "world" }] }
// ]
```

This ensures interoperability between the JavaScript and Rust implementations.

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

92 test cases covering:
- **Parser tests** (15 tests): Each delimiter type, mixed delimiters, nested structures, empty delimiters, quotes with special characters, JavaScript/Python/JSON-like code structures, with exact expected parsed object verification
- **Serializer tests** (10 tests): Serialization of each delimiter type, round-trip consistency
- **Programming language tests** (26 tests): 25+ languages with delimiter type and round-trip verification, key tests include full expected parsed object structure
- **Natural language tests** (30 tests): English, Spanish, French, German, Italian, Portuguese, Russian, Japanese, Chinese text, academic citations, mathematical expressions, legal text, and more
- **Unit tests** (9 tests): Internal parser tests in lib.rs
- **Doc tests** (2 tests): Examples in documentation

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with 25+ programming languages and all natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **Serde Support**: Serialize/deserialize to JSON with structure identical to JavaScript implementation
- **Display Trait**: Blocks implement `Display` for convenient string formatting
- **Identical Output**: Produces the same parsed structure as the JavaScript implementation

See the [main README](../README.md) for more information.
