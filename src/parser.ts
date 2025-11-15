/**
 * Meta-Notation Parser
 *
 * Parses text using the meta-notation grammar.
 */

import { parse as grammarParse } from './meta.grammar.js';
import type { Parser, Sequence } from './types.js';

/**
 * Parse meta-notation text into a sequence of blocks
 *
 * @param input - The text to parse
 * @returns A sequence of blocks representing the parsed structure
 *
 * @example
 * ```typescript
 * const result = parse('hello (world) {test}');
 * // Returns: [
 * //   { type: 'text', content: 'hello ' },
 * //   { type: 'paren', content: [{ type: 'text', content: 'world' }] },
 * //   { type: 'text', content: ' ' },
 * //   { type: 'curly', content: [{ type: 'text', content: 'test' }] }
 * // ]
 * ```
 */
export function parse(input: string): Sequence {
  return grammarParse(input) as Sequence;
}

/**
 * Parser class implementing the Parser interface
 */
export class MetaNotationParser implements Parser {
  parse(input: string): Sequence {
    return parse(input);
  }
}

export default { parse, MetaNotationParser };
