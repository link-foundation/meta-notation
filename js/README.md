# Meta-Notation (JavaScript/TypeScript)

JavaScript/TypeScript implementation of meta-notation parser.

## Installation

```bash
npm install meta-notation
```

## Usage

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

### `serialize(sequence: Sequence): string`

Converts a sequence of blocks back to text.

## Building

```bash
npm install
npm run build
```

## Testing

```bash
npm test
```

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with 25+ programming languages and all natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **TypeScript Support**: Fully typed API
- **81 Test Cases**: Comprehensive test coverage

See the [main README](../README.md) for more information.
