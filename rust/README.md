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

### `serialize(blocks: &[Block]) -> String`

Converts a sequence of blocks back to text.

## Types

```rust
pub enum Block {
    Paren(Vec<Block>),
    Curly(Vec<Block>),
    Square(Vec<Block>),
    SingleQuote(String),
    DoubleQuote(String),
    Backtick(String),
    Text(String),
}
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with programming and natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **Serde Support**: Serialize/deserialize to JSON
- **Zero Dependencies**: Only uses `serde` for serialization

See the [main README](../README.md) for more information.
