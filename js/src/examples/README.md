# Meta-Notation Examples

This directory contains examples demonstrating the use of meta-notation.

## Running Examples

First, build the project:

```bash
npm install
npm run build
```

Then run the examples:

```bash
node dist/examples/basic.js
```

## What is Meta-Notation?

Meta-notation is a simple, language-agnostic notation system that parses common delimiters found in most programming languages:

- Parentheses: `()`
- Curly braces: `{}`
- Square brackets: `[]`
- Single quotes: `''`
- Double quotes: `""`
- Backticks: `` ` ` ``

Unlike more complex notation systems, meta-notation intentionally avoids language-specific features (like the `:` self-reference in links-notation), making it compatible with a much larger set of programming languages.

## Use Cases

1. **Code Analysis**: Parse code structure without full language parsing
2. **Syntax Highlighting**: Quick delimiter matching
3. **Code Transformation**: Preserve structure while modifying content
4. **Multi-language Tools**: Build tools that work across many languages
5. **Educational**: Understand code structure at a basic level

## Design Philosophy

Meta-notation is inspired by:
- [links-notation](https://github.com/link-foundation/links-notation) - A more complex notation system with self-references
- [metalanguage](https://github.com/konard/metalanguage) - An earlier meta programming language concept

The key difference is simplicity and universality. By focusing only on common delimiters and avoiding language-specific syntax, meta-notation can parse code from JavaScript, Python, C++, Java, Go, Rust, and many other languages with the same simple grammar.
