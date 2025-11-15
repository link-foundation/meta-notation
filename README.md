# meta-notation

A notation for the largest possible set of languages. It focuses on parsing common delimiters: `()`, `{}`, `[]`, `''`, `` ` ` ``, `""` and so on.

## Vision

Meta-notation is a simpler version of [links-notation](https://github.com/link-foundation/links-notation).

It supports plain sequences of references and nested structures, but **without** the ability to use or parse `:` as a way to define a link's self reference. This makes it compatible with a much larger set of programming languages.

The implementation is similar to the concepts in [metalanguage](https://github.com/konard/metalanguage), but leverages all the tools from links-notation to do it right and efficiently.

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with JavaScript, Python, C++, Java, Go, Rust, and more
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **TypeScript Support**: Fully typed API
- **Simple Grammar**: PEG.js-based grammar for efficient parsing

## Installation

```bash
npm install meta-notation
```

## Quick Start

```typescript
import { parse, serialize } from 'meta-notation';

// Parse code with delimiters
const code = 'function test() { return "hello"; }';
const parsed = parse(code);

// Serialize back to string
const serialized = serialize(parsed);
console.log(serialized === code); // true
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

## Examples

See the [examples](./src/examples) directory for more detailed usage examples.

## Building

```bash
npm install
npm run build
```

## Testing

```bash
npm test
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

MIT
