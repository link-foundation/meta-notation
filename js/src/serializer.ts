/**
 * Meta-Notation Serializer
 *
 * Converts meta-notation blocks back to text.
 */

import type { Block, Sequence, Serializer as ISerializer } from './types.js';

/**
 * Get the opening delimiter for a block type
 */
function getOpenDelimiter(type: string): string {
  switch (type) {
    case 'paren': return '(';
    case 'curly': return '{';
    case 'square': return '[';
    case 'singleQuote': return "'";
    case 'doubleQuote': return '"';
    case 'backtick': return '`';
    default: return '';
  }
}

/**
 * Get the closing delimiter for a block type
 */
function getCloseDelimiter(type: string): string {
  switch (type) {
    case 'paren': return ')';
    case 'curly': return '}';
    case 'square': return ']';
    case 'singleQuote': return "'";
    case 'doubleQuote': return '"';
    case 'backtick': return '`';
    default: return '';
  }
}

/**
 * Serialize a single block back to text
 */
function serializeBlock(block: Block): string {
  if (block.type === 'text') {
    return block.content as string;
  }

  const open = getOpenDelimiter(block.type);
  const close = getCloseDelimiter(block.type);

  if (Array.isArray(block.content)) {
    // Nested sequence
    const inner = serialize(block.content);
    return `${open}${inner}${close}`;
  } else {
    // String content (for quotes and backticks)
    return `${open}${block.content}${close}`;
  }
}

/**
 * Serialize a sequence of blocks back to text
 *
 * @param sequence - The sequence to serialize
 * @returns The serialized text
 *
 * @example
 * ```typescript
 * const blocks = [
 *   { type: 'text', content: 'hello ' },
 *   { type: 'paren', content: [{ type: 'text', content: 'world' }] }
 * ];
 * const text = serialize(blocks);
 * // Returns: "hello (world)"
 * ```
 */
export function serialize(sequence: Sequence): string {
  return sequence.map(serializeBlock).join('');
}

/**
 * Serializer class implementing the Serializer interface
 */
export class MetaNotationSerializer implements ISerializer {
  serialize(sequence: Sequence): string {
    return serialize(sequence);
  }
}

export default { serialize, MetaNotationSerializer };
