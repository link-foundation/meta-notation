/**
 * Tests for the meta-notation serializer
 */

import { test, assert } from 'test-anywhere';
import { parse } from '../src/parser.js';
import { serialize } from '../src/serializer.js';

test('serialize plain text', () => {
  const blocks = [{ type: 'text' as const, content: 'hello world' }];
  const result = serialize(blocks);
  assert.equal(result, 'hello world');
});

test('serialize parentheses', () => {
  const blocks = [
    { type: 'paren' as const, content: [{ type: 'text' as const, content: 'hello' }] }
  ];
  const result = serialize(blocks);
  assert.equal(result, '(hello)');
});

test('serialize curly braces', () => {
  const blocks = [
    { type: 'curly' as const, content: [{ type: 'text' as const, content: 'world' }] }
  ];
  const result = serialize(blocks);
  assert.equal(result, '{world}');
});

test('serialize square brackets', () => {
  const blocks = [
    { type: 'square' as const, content: [{ type: 'text' as const, content: 'test' }] }
  ];
  const result = serialize(blocks);
  assert.equal(result, '[test]');
});

test('serialize quotes', () => {
  const blocks = [
    { type: 'singleQuote' as const, content: 'hello' },
    { type: 'text' as const, content: ' ' },
    { type: 'doubleQuote' as const, content: 'world' }
  ];
  const result = serialize(blocks);
  assert.equal(result, `'hello' "world"`);
});

test('serialize backticks', () => {
  const blocks = [
    { type: 'backtick' as const, content: 'code' }
  ];
  const result = serialize(blocks);
  assert.equal(result, '`code`');
});

test('round-trip: parse then serialize', () => {
  const original = 'hello (world) {test} [array] "string" `code`';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.equal(serialized, original);
});

test('round-trip: nested structures', () => {
  const original = '{a [b (c) d] e}';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.equal(serialized, original);
});

test('round-trip: empty delimiters', () => {
  const original = '(){}[]';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.equal(serialized, original);
});

test('round-trip: complex code', () => {
  const original = 'function test() { return "hello"; }';
  const parsed = parse(original);
  const serialized = serialize(parsed);
  assert.equal(serialized, original);
});
