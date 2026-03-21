# meta-notation

[![CI/CD](https://github.com/link-foundation/meta-notation/actions/workflows/ci.yml/badge.svg)](https://github.com/link-foundation/meta-notation/actions/workflows/ci.yml)
[![npm version](https://badge.fury.io/js/meta-notation.svg)](https://www.npmjs.com/package/meta-notation)
[![crates.io](https://img.shields.io/crates/v/meta-notation.svg)](https://crates.io/crates/meta-notation)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://github.com/link-foundation/meta-notation/blob/main/LICENSE)

A notation for the largest possible set of languages. It focuses on parsing common delimiters: `()`, `{}`, `[]`, `''`, `` ` ` ``, `""` and so on.

## Vision

Meta-notation is a simpler version of [links-notation](https://github.com/link-foundation/links-notation).

It supports plain sequences of references and nested structures, but **without** the ability to use or parse `:` as a way to define a link's self reference. This makes it compatible with a much larger set of programming languages.

The implementation is similar to the concepts in [metalanguage](https://github.com/konard/metalanguage), but leverages all the tools from links-notation to do it right and efficiently.

## Implementations

Meta-notation is available in multiple languages with identical behavior:

- **[JavaScript/TypeScript](./js)** - Full-featured implementation with PEG.js grammar
- **[Rust](./rust)** - High-performance implementation with serde support

Both implementations produce the same parsed object structure and pass the same test cases.

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with 25+ programming languages and all natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **Multiple Language Implementations**: JavaScript/TypeScript and Rust with identical output
- **Simple Grammar**: Clean, efficient parsing
- **Comprehensive Tests**: 170+ test cases across both implementations covering programming and natural languages

## Installation

### JavaScript/TypeScript

```bash
npm install meta-notation
```

### Rust

```toml
[dependencies]
meta-notation = "0.1"
```

## Quick Start

### JavaScript/TypeScript

```typescript
import { parse, serialize } from 'meta-notation';

const code = 'function test() { return "hello"; }';
const parsed = parse(code);
const serialized = serialize(parsed);
console.log(serialized === code); // true
```

### Rust

```rust
use meta_notation::{parse, serialize};

let code = r#"function test() { return "hello"; }"#;
let parsed = parse(code);
let serialized = serialize(&parsed);
assert_eq!(serialized, code);
```

## API

### `parse(input) -> Sequence`

Parses text into a sequence of blocks. Each block has a `type` and `content`.

- **Bracket delimiters** (`paren`, `curly`, `square`): `content` is a nested array of blocks
- **Quote delimiters** (`singleQuote`, `doubleQuote`, `backtick`): `content` is a plain string (no nested parsing inside quotes)
- **Plain text** (`text`): `content` is a string

```typescript
// JavaScript/TypeScript
const result = parse('hello (world) {test}');
// Returns:
// [
//   { type: 'text', content: 'hello ' },
//   { type: 'paren', content: [{ type: 'text', content: 'world' }] },
//   { type: 'text', content: ' ' },
//   { type: 'curly', content: [{ type: 'text', content: 'test' }] }
// ]
```

```rust
// Rust
let result = parse("hello (world) {test}");
// Returns:
// [
//   Block::Text("hello "),
//   Block::Paren([Block::Text("world")]),
//   Block::Text(" "),
//   Block::Curly([Block::Text("test")])
// ]
```

#### Nested Structures

Bracket delimiters can be nested arbitrarily:

```typescript
const result = parse('{a [b (c) d] e}');
// Returns:
// [
//   { type: 'curly', content: [
//     { type: 'text', content: 'a ' },
//     { type: 'square', content: [
//       { type: 'text', content: 'b ' },
//       { type: 'paren', content: [{ type: 'text', content: 'c' }] },
//       { type: 'text', content: ' d' }
//     ]},
//     { type: 'text', content: ' e' }
//   ]}
// ]
```

#### Quotes

Quotes capture their content as a plain string without further parsing:

```typescript
const result = parse('"hello {world}"');
// Returns:
// [
//   { type: 'doubleQuote', content: 'hello {world}' }
// ]
// Note: {world} is NOT parsed as a curly block inside quotes
```

#### Real-World Examples

**JavaScript code:**
```typescript
const result = parse('const greet = (name) => { return `Hello, ${name}!`; };');
// Returns:
// [
//   { type: 'text', content: 'const greet = ' },
//   { type: 'paren', content: [{ type: 'text', content: 'name' }] },
//   { type: 'text', content: ' => ' },
//   { type: 'curly', content: [
//     { type: 'text', content: ' return ' },
//     { type: 'backtick', content: 'Hello, ${name}!' },
//     { type: 'text', content: '; ' }
//   ]},
//   { type: 'text', content: ';' }
// ]
```

**JSON:**
```typescript
const result = parse('{"name": "John", "tags": ["dev", "admin"]}');
// Returns:
// [
//   { type: 'curly', content: [
//     { type: 'doubleQuote', content: 'name' },
//     { type: 'text', content: ': ' },
//     { type: 'doubleQuote', content: 'John' },
//     { type: 'text', content: ', ' },
//     { type: 'doubleQuote', content: 'tags' },
//     { type: 'text', content: ': ' },
//     { type: 'square', content: [
//       { type: 'doubleQuote', content: 'dev' },
//       { type: 'text', content: ', ' },
//       { type: 'doubleQuote', content: 'admin' }
//     ]}
//   ]}
// ]
```

**Natural language:**
```typescript
const result = parse('She said, "Hello, world!" and smiled.');
// Returns:
// [
//   { type: 'text', content: 'She said, ' },
//   { type: 'doubleQuote', content: 'Hello, world!' },
//   { type: 'text', content: ' and smiled.' }
// ]
```

### `serialize(sequence) -> string`

Converts a sequence of blocks back to text.

```typescript
// JavaScript/TypeScript
const blocks = [
  { type: 'text', content: 'hello ' },
  { type: 'paren', content: [{ type: 'text', content: 'world' }] }
];
const text = serialize(blocks);
// Returns: "hello (world)"
```

```rust
// Rust
let blocks = vec![
    Block::Text("hello ".to_string()),
    Block::Paren(vec![Block::Text("world".to_string())]),
];
let text = serialize(&blocks);
// Returns: "hello (world)"
```

## Types

### JavaScript/TypeScript

```typescript
type DelimiterType = 'paren' | 'curly' | 'square' | 'singleQuote' | 'doubleQuote' | 'backtick' | 'text';

interface Block {
  type: DelimiterType;
  content: Block[] | string;  // Block[] for brackets, string for quotes and text
}

type Sequence = Block[];
```

### Rust

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
```

The Rust `Block` enum uses serde's `#[serde(tag = "type", content = "content", rename_all = "camelCase")]` attribute, so it serializes to the same JSON structure as the JavaScript implementation:

```json
[
  { "type": "text", "content": "hello " },
  { "type": "paren", "content": [{ "type": "text", "content": "world" }] }
]
```

## Language Support

Meta-notation works seamlessly with both programming languages and natural languages.

### Programming Languages (Tested)

| Language | Delimiters Used | Example |
|----------|----------------|---------|
| JavaScript/TypeScript | `() {} \`\`` | `const greet = (name) => { return \`Hello\`; };` |
| Python | `() {} [] ""` | `def calc(x): return {"sum": x, "list": [x]}` |
| Go | `() {} ""` | `func main() { fmt.Println("Hello") }` |
| Rust | `() {} [] ""` | `fn main() { let x = vec![1]; println!("{}", x); }` |
| C++ | `() {} ""` | `int main() { std::cout << "Hello"; }` |
| Java | `() {} [] ""` | `class Main { void main(String[] args) {} }` |
| C# | `() {} ""` | `void Test() { Console.WriteLine("Done"); }` |
| Ruby | `() ""` | `def greet(name); puts "Hello"; end` |
| PHP | `() {} [] ""` | `function test($x) { return ["key" => "val"]; }` |
| Swift | `() {} ""` | `func greet(name: String) { return "Hello" }` |
| Kotlin | `() {} ""` | `fun main() { println("Hello") }` |
| Scala | `() {}` | `def add(x: Int, y: Int): Int = { x + y }` |
| Perl | `() {} ""` | `sub greet { print "Hello\n"; }` |
| Haskell | `""` | `main = putStrLn "Hello, World!"` |
| Lisp/Scheme | `()` | `(define (factorial n) (if (= n 0) 1 (* n 1)))` |
| Clojure | `() [] ""` | `(defn greet [name] (str "Hello"))` |
| Lua | `() ""` | `function greet(name) return "Hello" end` |
| Elixir | `() ""` | `def greet(name), do: "Hello"` |
| R | `() {} ""` | `greet <- function(name) { paste("Hello") }` |
| MATLAB | `()` | `function y = square(x); y = x .^ 2; end` |
| SQL | `""` | `SELECT name FROM users WHERE status = "active"` |
| JSON | `{} [] ""` | `{"name": "John", "tags": ["dev"]}` |
| YAML | `[] ""` | `dependencies: ["react", "typescript"]` |
| Bash/Shell | `""` | `echo "Hello, ${USER}!" \| grep "Hello"` |
| Markdown | `` \`\` `` | ``Here is code: `const x = 1;` in backticks.`` |

### Natural Languages (Tested)

Meta-notation parses natural language text including:

- **Direct speech** with quotes: `She said, "Hello!"`
- **Parenthetical remarks**: `The conference (next week) is online.`
- **Citations and references**: `According to [Smith, 2020]...`
- **Academic writing** with nested structures
- **Legal text** with section references
- **Technical documentation** mixing code and prose
- **Mathematical expressions**: `f(x) = [a + b] * {c - d}`
- **Multiple languages**: English, Spanish, French, German, Italian, Portuguese, Russian, Japanese, Chinese, and more

Works with any language that uses these common delimiters for structure.

## Examples

See the [examples](./js/src/examples) directory for more detailed usage examples.

## Building

### JavaScript/TypeScript

```bash
cd js
npm install
npm run build
```

### Rust

```bash
cd rust
cargo build --release
```

## Testing

### JavaScript/TypeScript

```bash
cd js
npm test
```

81 test cases covering parser, serializer, programming languages, and natural languages.

### Rust

```bash
cd rust
cargo test
```

92 test cases covering the same scenarios plus dedicated parser and serializer unit tests.

Both implementations verify:
- Exact parsed object structure matches expected output
- Round-trip serialization preserves original text
- All delimiter types are correctly identified
- Nested structures are handled correctly

## Comparison with Links-Notation

| Feature | meta-notation | links-notation |
|---------|---------------|----------------|
| Delimiter parsing | ✅ | ✅ |
| Nested structures | ✅ | ✅ |
| Self-reference (`:`) | ❌ | ✅ |
| Language compatibility | Very high | High |
| Complexity | Low | Medium |

By removing the `:` self-reference syntax, meta-notation can parse a wider variety of languages without conflicts.

## License

[Unlicense](https://github.com/link-foundation/meta-notation/blob/main/LICENSE) (Public Domain)
