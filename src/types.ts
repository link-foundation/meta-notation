/**
 * Meta-Notation Types
 *
 * A notation for the largest possible set of languages.
 * Focuses on parsing common delimiters without language-specific syntax.
 */

/**
 * Delimiter types supported by meta-notation
 */
export type DelimiterType =
  | 'paren'        // ()
  | 'curly'        // {}
  | 'square'       // []
  | 'singleQuote'  // ''
  | 'doubleQuote'  // ""
  | 'backtick'     // ``
  | 'text';        // plain text

/**
 * A block is either a delimited structure or plain text
 */
export interface Block {
  type: DelimiterType;
  content: Block[] | string;
}

/**
 * A sequence is an array of blocks
 */
export type Sequence = Block[];

/**
 * Parser interface
 */
export interface Parser {
  parse(input: string): Sequence;
}

/**
 * Serializer interface
 */
export interface Serializer {
  serialize(sequence: Sequence): string;
}
