/**
 * Tests for the meta-notation parser
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { parse } from './parser.js';

test('parse plain text', () => {
  const result = parse('hello world');
  assert.deepStrictEqual(result, [
    { type: 'text', content: 'hello world' }
  ]);
});

test('parse parentheses', () => {
  const result = parse('(hello)');
  assert.deepStrictEqual(result, [
    { type: 'paren', content: [{ type: 'text', content: 'hello' }] }
  ]);
});

test('parse curly braces', () => {
  const result = parse('{world}');
  assert.deepStrictEqual(result, [
    { type: 'curly', content: [{ type: 'text', content: 'world' }] }
  ]);
});

test('parse square brackets', () => {
  const result = parse('[test]');
  assert.deepStrictEqual(result, [
    { type: 'square', content: [{ type: 'text', content: 'test' }] }
  ]);
});

test('parse single quotes', () => {
  const result = parse("'hello'");
  assert.deepStrictEqual(result, [
    { type: 'singleQuote', content: 'hello' }
  ]);
});

test('parse double quotes', () => {
  const result = parse('"world"');
  assert.deepStrictEqual(result, [
    { type: 'doubleQuote', content: 'world' }
  ]);
});

test('parse backticks', () => {
  const result = parse('`code`');
  assert.deepStrictEqual(result, [
    { type: 'backtick', content: 'code' }
  ]);
});

test('parse mixed delimiters', () => {
  const result = parse('hello (world) {test}');
  assert.deepStrictEqual(result, [
    { type: 'text', content: 'hello ' },
    { type: 'paren', content: [{ type: 'text', content: 'world' }] },
    { type: 'text', content: ' ' },
    { type: 'curly', content: [{ type: 'text', content: 'test' }] }
  ]);
});

test('parse nested structures', () => {
  const result = parse('(a (b) c)');
  assert.deepStrictEqual(result, [
    {
      type: 'paren',
      content: [
        { type: 'text', content: 'a ' },
        { type: 'paren', content: [{ type: 'text', content: 'b' }] },
        { type: 'text', content: ' c' }
      ]
    }
  ]);
});

test('parse complex nested structures', () => {
  const result = parse('{a [b (c) d] e}');
  assert.deepStrictEqual(result, [
    {
      type: 'curly',
      content: [
        { type: 'text', content: 'a ' },
        {
          type: 'square',
          content: [
            { type: 'text', content: 'b ' },
            { type: 'paren', content: [{ type: 'text', content: 'c' }] },
            { type: 'text', content: ' d' }
          ]
        },
        { type: 'text', content: ' e' }
      ]
    }
  ]);
});

test('parse empty delimiters', () => {
  const result = parse('(){}[]');
  assert.deepStrictEqual(result, [
    { type: 'paren', content: [] },
    { type: 'curly', content: [] },
    { type: 'square', content: [] }
  ]);
});

test('parse quotes with special chars', () => {
  const result = parse('"hello {world}"');
  assert.deepStrictEqual(result, [
    { type: 'doubleQuote', content: 'hello {world}' }
  ]);
});

test('parse JavaScript-like code', () => {
  const result = parse('function test() { return "hello"; }');
  assert.strictEqual(result.length, 4);
  assert.strictEqual(result[0].type, 'text');
  assert.strictEqual(result[1].type, 'paren');
  assert.strictEqual(result[3].type, 'curly');
});

test('parse Python-like code', () => {
  const result = parse('def test(): return [1, 2, 3]');
  const types = result.map(b => b.type);
  assert.ok(types.includes('paren'));
  assert.ok(types.includes('square'));
  assert.ok(types.includes('text'));
});

test('parse JSON-like structure', () => {
  const result = parse('{"key": "value", "array": [1, 2, 3]}');
  assert.strictEqual(result[0].type, 'curly');
  assert.ok(Array.isArray(result[0].content));
});
