/**
 * Meta-Notation
 *
 * A notation for the largest possible set of languages.
 * Focuses on parsing common delimiters: (), {}, [], '', "", ``
 *
 * This is a simpler version of links-notation without the : self-reference syntax,
 * making it compatible with a much larger set of languages.
 *
 * @module meta-notation
 */

export { parse, MetaNotationParser } from './parser.js';
export { serialize, MetaNotationSerializer } from './serializer.js';
export type { Block, Sequence, DelimiterType, Parser, Serializer } from './types.js';

/**
 * Main API for convenience
 */
import { parse } from './parser.js';
import { serialize } from './serializer.js';

export default {
  parse,
  serialize
};
