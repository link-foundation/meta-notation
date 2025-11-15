# meta-notation

A notation for the largest possible set of languages. It focuses on parsing common delimiters: `()`, `{}`, `[]`, `''`, `` ` ` ``, `""` and so on.

## Vision

Meta-notation is a simpler version of [links-notation](https://github.com/link-foundation/links-notation).

It supports plain sequences of references and nested structures, but **without** the ability to use or parse `:` as a way to define a link's self reference. This makes it compatible with a much larger set of programming languages.

The implementation is similar to the concepts in [metalanguage](https://github.com/konard/metalanguage), but leverages all the tools from links-notation to do it right and efficiently.

## Implementations

Meta-notation is available in multiple languages:

- **[JavaScript/TypeScript](./js)** - Full-featured implementation with PEG.js grammar
- **[Rust](./rust)** - High-performance implementation with serde support

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with 25+ programming languages and all natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **Multiple Language Implementations**: JavaScript/TypeScript and Rust
- **Simple Grammar**: Clean, efficient parsing
- **Comprehensive Tests**: 81+ test cases for programming and natural languages

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

### `parse(input: string): Sequence`

Parses text into a sequence of blocks.

```typescript
const result = parse('hello (world) {test}');
// Returns:
// [
//   { type: 'text', content: 'hello ' },
//   { type: 'paren', content: [{ type: 'text', content: 'world' }] },
//   { type: 'text', content: ' ' },
//   { type: 'curly', content: [{ type: 'text', content: 'test' }] }
// ]
```

### `serialize(sequence: Sequence): string`

Converts a sequence of blocks back to text.

```typescript
const blocks = [
  { type: 'text', content: 'hello ' },
  { type: 'paren', content: [{ type: 'text', content: 'world' }] }
];
const text = serialize(blocks);
// Returns: "hello (world)"
```

## Types

```typescript
type DelimiterType = 'paren' | 'curly' | 'square' | 'singleQuote' | 'doubleQuote' | 'backtick' | 'text';

interface Block {
  type: DelimiterType;
  content: Block[] | string;
}

type Sequence = Block[];
```

## Language Support

Meta-notation works seamlessly with both programming languages and natural languages.

### Programming Languages (Tested)

- **JavaScript/TypeScript** - Functions, arrow functions, template literals
- **Python** - Dictionaries, lists, function definitions
- **Go** - Functions, print statements
- **Rust** - Vectors, macros, format strings
- **C++** - Streams, functions, return statements
- **Java** - Classes, methods, arrays
- **C#** - LINQ, collections, generics
- **Ruby** - Methods, string interpolation
- **PHP** - Functions, arrays, associative arrays
- **Swift** - Functions, string interpolation
- **Kotlin** - Functions, lists
- **Scala** - Functions, type annotations
- **Perl** - Subroutines, arrays
- **Haskell** - Pure functions
- **Lisp/Scheme** - S-expressions
- **Clojure** - Vectors, strings
- **Lua** - Functions, string concatenation
- **Elixir** - Functions, string interpolation
- **R** - Functions, paste
- **MATLAB** - Functions
- **SQL** - SELECT statements with WHERE clauses
- **JSON** - Objects and arrays
- **YAML** - Arrays (with bracket syntax)
- **Bash/Shell** - Echo, variables, pipes
- **Markdown** - Code blocks with backticks

### Natural Languages (Tested)

Meta-notation parses natural language text including:

- **Direct speech** with quotes: `She said, "Hello!"`
- **Parenthetical remarks**: `The conference (next week) is online.`
- **Citations and references**: `According to [Smith, 2020]...`
- **Academic writing** with nested structures
- **Legal text** with section references
- **Technical documentation** mixing code and prose
- **Multiple languages**: English, Spanish, French, German, Italian, Portuguese, and more

Works with any language that uses these common delimiters for structure.

## Examples

See the [examples](./src/examples) directory for more detailed usage examples.

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

### Rust

```bash
cd rust
cargo test
```

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
