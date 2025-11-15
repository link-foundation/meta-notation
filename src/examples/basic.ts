/**
 * Basic examples of using meta-notation
 */

import { parse, serialize, type Block } from '../index.js';

// Example 1: Parse simple text with delimiters
console.log('Example 1: Basic parsing');
console.log('========================');
const text1 = 'hello (world)';
const parsed1 = parse(text1);
console.log('Input:', text1);
console.log('Parsed:', JSON.stringify(parsed1, null, 2));
console.log();

// Example 2: Parse nested structures
console.log('Example 2: Nested structures');
console.log('============================');
const text2 = '{a [b (c) d] e}';
const parsed2 = parse(text2);
console.log('Input:', text2);
console.log('Parsed:', JSON.stringify(parsed2, null, 2));
console.log();

// Example 3: Parse different quote types
console.log('Example 3: Different quote types');
console.log('================================');
const text3 = `'single' "double" \`backtick\``;
const parsed3 = parse(text3);
console.log('Input:', text3);
console.log('Parsed:', JSON.stringify(parsed3, null, 2));
console.log();

// Example 4: Round-trip (parse then serialize)
console.log('Example 4: Round-trip');
console.log('=====================');
const original = 'function test() { return "hello"; }';
const parsed = parse(original);
const serialized = serialize(parsed);
console.log('Original: ', original);
console.log('Serialized:', serialized);
console.log('Match:', original === serialized);
console.log();

// Example 5: Analyze JavaScript code structure
console.log('Example 5: JavaScript analysis');
console.log('==============================');
const jsCode = 'const x = [1, 2, 3];';
const jsParsed = parse(jsCode);
console.log('Code:', jsCode);
console.log('Structure:');
jsParsed.forEach((block: Block, i: number) => {
  console.log(`  [${i}] ${block.type}:`,
    Array.isArray(block.content) ? `${block.content.length} items` : `"${block.content}"`
  );
});
console.log();

// Example 6: Analyze Python code structure
console.log('Example 6: Python analysis');
console.log('==========================');
const pyCode = 'def func(a, b): return {"key": "value"}';
const pyParsed = parse(pyCode);
console.log('Code:', pyCode);
console.log('Structure:');
pyParsed.forEach((block: Block, i: number) => {
  console.log(`  [${i}] ${block.type}:`,
    Array.isArray(block.content) ? `${block.content.length} items` : `"${block.content}"`
  );
});
