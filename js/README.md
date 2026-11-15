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

Parses text into a sequence of blocks. Each block has a `type` (the delimiter kind) and `content` (either a nested array of blocks for bracket delimiters, or a plain string for quotes and text).

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

Nested structures are supported:

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

Quotes capture content as a plain string (no nested parsing):

```typescript
const result = parse('"hello {world}"');
// Returns:
// [
//   { type: 'doubleQuote', content: 'hello {world}' }
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

### Class-based API

```typescript
import { MetaNotationParser, MetaNotationSerializer } from 'meta-notation';

const parser = new MetaNotationParser();
const serializer = new MetaNotationSerializer();

const parsed = parser.parse('hello (world)');
const text = serializer.serialize(parsed);
```

## Types

```typescript
type DelimiterType = 'paren' | 'curly' | 'square' | 'singleQuote' | 'doubleQuote' | 'backtick' | 'text';

interface Block {
  type: DelimiterType;
  content: Block[] | string;  // Block[] for brackets, string for quotes and text
}

type Sequence = Block[];

interface Parser {
  parse(input: string): Sequence;
}

interface Serializer {
  serialize(sequence: Sequence): string;
}
```

### Content types by delimiter

| Delimiter | Type Name | Content Type | Example |
|-----------|-----------|-------------|---------|
| `()` | `paren` | `Block[]` | `{ type: 'paren', content: [{ type: 'text', content: 'x' }] }` |
| `{}` | `curly` | `Block[]` | `{ type: 'curly', content: [{ type: 'text', content: 'x' }] }` |
| `[]` | `square` | `Block[]` | `{ type: 'square', content: [{ type: 'text', content: 'x' }] }` |
| `''` | `singleQuote` | `string` | `{ type: 'singleQuote', content: 'hello' }` |
| `""` | `doubleQuote` | `string` | `{ type: 'doubleQuote', content: 'hello' }` |
| `` `` `` | `backtick` | `string` | `{ type: 'backtick', content: 'hello' }` |
| plain text | `text` | `string` | `{ type: 'text', content: 'hello' }` |

## Building

```bash
npm install
npm run build
```

## Testing

```bash
npm test
```

81 test cases covering:
- **Parser tests**: Each delimiter type, mixed delimiters, nested structures, empty delimiters, quotes with special characters, JavaScript/Python/JSON-like code structures, with exact expected parsed object verification
- **Serializer tests**: Serialization of each delimiter type, round-trip consistency
- **Programming language tests**: 25+ languages with delimiter type and round-trip verification, key tests include full expected parsed object structure
- **Natural language tests**: 25 tests for English, Spanish, French, German, Italian, Portuguese, Russian, Japanese, Chinese text, academic citations, mathematical expressions, legal text, and more

## Publishing to npm

Published GitHub releases run `.github/workflows/publish-npm.yml` on a
GitHub-hosted runner with Node 24 and npm 11.20.0. Normal releases use npm
trusted publishing (OIDC), with `id-token: write` and no npm token secret.
Publish failures fail the job. Version 0.1.1 is prepared for the next release;
0.1.0 is already on npm and cannot be republished.

Before releasing, a package owner must attach the trusted publisher using an
authenticated npm CLI with support for `npm trust`:

```bash
npm trust github meta-notation --repo link-foundation/meta-notation --file publish-npm.yml --allow-publish --yes
npm trust list meta-notation
```

Alternatively, configure the publisher in the package settings on npmjs.com:
organization `link-foundation`, repository `meta-notation`, workflow filename
`publish-npm.yml`, no environment, with permission to publish directly. This
account operation requires package write access and npm authentication; GitHub
repository access alone cannot configure it. See the
[npm trusted publishing documentation](https://docs.npmjs.com/trusted-publishers/).

After configuration, publish a GitHub release for the intended version and check
that the **Publish to npm** step succeeds while **Publish with token fallback**
is skipped. Confirm the published version and provenance on npm. A dry run or
passing workflow unit tests do not verify the registry trust relationship.

For an emergency fallback, create a fresh, short-lived granular npm token with
package write access and the required publishing permissions, store it as the
`NPM_TOKEN` repository secret, and manually dispatch **Publish to npm** at the
intended release tag with `use_token` selected. Only this explicit path receives
the secret; an OIDC failure never automatically retries with a token. Remove or
rotate the fallback secret after use. Do not reuse the bootstrap token.

Workflow regression tests run in CI and can be run locally from the repository
root with PyYAML installed:

```bash
python -m unittest discover -s experiments -p 'test_*.py' -v
```

## Features

- **Universal Delimiter Parsing**: Parses `()`, `{}`, `[]`, `''`, `""`, `` ` ` ``
- **Language Agnostic**: Works with 25+ programming languages and all natural languages
- **Nested Structures**: Supports arbitrary nesting of delimiters
- **Round-trip Serialization**: Parse and serialize back to original text
- **TypeScript Support**: Fully typed API with exported interfaces
- **PEG.js Grammar**: Clean, maintainable grammar definition
- **Identical Output**: Produces the same parsed structure as the Rust implementation

See the [main README](../README.md) for more information.
