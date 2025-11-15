/**
 * Tests for meta-notation with various programming languages
 */

import { test, assert } from 'test-anywhere';
import { parse } from '../src/parser.js';
import { serialize } from '../src/serializer.js';
import type { Block, Sequence, DelimiterType } from '../src/types.js';

// Helper function to check if a delimiter type exists anywhere in the parsed result
function hasDelimiterType(sequence: Sequence, type: DelimiterType): boolean {
  for (const block of sequence) {
    if (block.type === type) {
      return true;
    }
    if (Array.isArray(block.content)) {
      if (hasDelimiterType(block.content, type)) {
        return true;
      }
    }
  }
  return false;
}

// JavaScript
test('parse JavaScript code', () => {
  const code = 'const greet = (name) => { return `Hello, ${name}!`; };';
  const result = parse(code);
  assert.ok(result.length > 0);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'backtick'));
  // Round-trip test
  assert.equal(serialize(result), code);
});

// Python
test('parse Python code', () => {
  const code = 'def calculate(x, y): return {"sum": x + y, "list": [x, y]}';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Go
test('parse Go code', () => {
  const code = 'func main() { fmt.Println("Hello, World!") }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Rust
test('parse Rust code', () => {
  const code = 'fn main() { let x = vec![1, 2, 3]; println!("{:?}", x); }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// C++
test('parse C++ code', () => {
  const code = 'int main() { std::cout << "Hello" << std::endl; return 0; }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Java
test('parse Java code', () => {
  const code = 'public class Main { public static void main(String[] args) { System.out.println("Hello"); } }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// C#
test('parse C# code', () => {
  const code = 'public void Test() { var list = new List<int> {1, 2, 3}; Console.WriteLine("Done"); }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Ruby
test('parse Ruby code', () => {
  const code = 'def greet(name); puts "Hello, #{name}!"; end';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// PHP
test('parse PHP code', () => {
  const code = 'function test($x) { return ["key" => "value"]; }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Swift
test('parse Swift code', () => {
  const code = 'func greet(name: String) -> String { return "Hello, \\(name)!" }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Kotlin
test('parse Kotlin code', () => {
  const code = 'fun main() { val list = listOf(1, 2, 3); println("Hello") }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// TypeScript
test('parse TypeScript code', () => {
  const code = 'const add = (a: number, b: number): number => { return a + b; };';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.equal(serialize(result), code);
});

// Scala
test('parse Scala code', () => {
  const code = 'def add(x: Int, y: Int): Int = { x + y }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.equal(serialize(result), code);
});

// Perl
test('parse Perl code', () => {
  const code = 'sub greet { my ($name) = @_; print "Hello, $name!\\n"; }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Haskell
test('parse Haskell code', () => {
  const code = 'main = putStrLn "Hello, World!"';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Lisp/Scheme
test('parse Lisp code', () => {
  const code = '(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), code);
});

// Clojure
test('parse Clojure code', () => {
  const code = '(defn greet [name] (str "Hello, " name "!"))';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Lua
test('parse Lua code', () => {
  const code = 'function greet(name) return "Hello, " .. name end';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Elixir
test('parse Elixir code', () => {
  const code = 'def greet(name), do: "Hello, #{name}!"';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// R
test('parse R code', () => {
  const code = 'greet <- function(name) { paste("Hello,", name) }';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// MATLAB
test('parse MATLAB code', () => {
  const code = 'function y = square(x); y = x .^ 2; end';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), code);
});

// SQL
test('parse SQL code', () => {
  const code = 'SELECT name, age FROM users WHERE status = "active" ORDER BY created_at;';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// JSON
test('parse JSON', () => {
  const code = '{"name": "John", "age": 30, "tags": ["developer", "designer"]}';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// YAML-like (with brackets)
test('parse YAML with brackets', () => {
  const code = 'dependencies: ["react", "typescript"]';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Shell/Bash
test('parse Bash code', () => {
  const code = 'echo "Hello, ${USER}!" | grep "Hello"';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), code);
});

// Markdown code blocks
test('parse Markdown with code blocks', () => {
  const code = 'Here is code: `const x = [1, 2, 3];` in backticks.';
  const result = parse(code);
  assert.ok(hasDelimiterType(result, 'backtick'));
  assert.equal(serialize(result), code);
});
