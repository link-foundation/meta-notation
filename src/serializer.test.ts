/**
 * Tests for the meta-notation serializer
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { parse } from './parser.js';
import { serialize } from './serializer.js';

test('serialize plain text', () => {
  const blocks = [{ type: 'text' as const, content: 'hello world' }];
  const result = serialize(blocks);
  assert.strictEqual(result, 'hello world');
});

test('serialize parentheses', () => {
  const blocks = [
    { type: 'paren' as const, content: [{ type: 'text' as const, content: 'hello' }] }
  ];
  const result = serialize(blocks);
  assert.strictEqual(result, '(hello)');
});

test('serialize curly braces', () => {
  const blocks = [
    { type: 'curly' as const, content: [{ type: 'text' as const, content: 'world' }] }
  ];
  const result = serialize(blocks);
  assert.strictEqual(result, '{world}');
});

test('serialize square brackets', () => {
  const blocks = [
    { type: 'square' as const, content: [{ type: 'text' as const, content: 'test' }] }
  ];
  const result = serialize(blocks);
  assert.strictEqual(result, '[test]');
});

test('serialize quotes', () => {
  const blocks = [
    { type: 'singleQuote' as const, content: 'hello' },
    { type: 'text' as const, content: ' ' },
    { type: 'doubleQuote' as const, content: 'world' }
  ];
  const result = serialize(blocks);
  assert.strictEqual(result, `'hello' "world"`);
});

test('serialize backticks', () => {
  const blocks = [
    { type: 'backtick' as const, content: 'code' }
  ];
  const result = serialize(blocks);
  assert.strictEqual(result, '`code`');
});

test('round-trip: parse then serialize', () => {
  const original = 'hello (world) {test} [array] "string" `code`';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.strictEqual(serialized, original);
});

test('round-trip: nested structures', () => {
  const original = '{a [b (c) d] e}';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.strictEqual(serialized, original);
});

test('round-trip: empty delimiters', () => {
  const original = '(){}[]';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.strictEqual(serialized, original);
});

test('round-trip: complex code', () => {
  const original = 'function test() { return "hello"; }';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.strictEqual(serialized, original);
});
