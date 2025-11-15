/**
 * Tests for meta-notation with natural language text
 *
 * Meta-notation should work seamlessly with natural languages since they
 * use the same delimiters: quotes for speech, parentheses for asides,
 * brackets for references, etc.
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

// English
test('parse English text with quotes', () => {
  const text = 'She said, "Hello, world!" and smiled.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

test('parse English text with parentheses', () => {
  const text = 'The conference (scheduled for next week) will be online.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

test('parse English text with brackets', () => {
  const text = 'According to the report [see page 42], the results were positive.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.equal(serialize(result), text);
});

test('parse English dialogue', () => {
  const text = `"How are you?" she asked. "I'm fine," he replied.`;
  const result = parse(text);
  assert.ok(result.filter(b => b.type === 'doubleQuote').length === 2);
  assert.equal(serialize(result), text);
});

test('parse English nested structure', () => {
  const text = 'He said, "I heard her say \'hello\' yesterday."';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Spanish
test('parse Spanish text with quotes', () => {
  const text = 'Ella dijo, "¡Hola, mundo!" y sonrió.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

test('parse Spanish text with parentheses', () => {
  const text = 'La conferencia (programada para la próxima semana) será en línea.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// French
test('parse French text with quotes', () => {
  const text = 'Elle a dit, "Bonjour, le monde!" et a souri.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

test('parse French text with parentheses', () => {
  const text = 'La conférence (prévue pour la semaine prochaine) sera en ligne.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// German
test('parse German text with quotes', () => {
  const text = 'Sie sagte, "Hallo, Welt!" und lächelte.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

test('parse German text with parentheses', () => {
  const text = 'Die Konferenz (geplant für nächste Woche) wird online sein.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Italian
test('parse Italian text with quotes', () => {
  const text = 'Lei disse, "Ciao, mondo!" e sorrise.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Portuguese
test('parse Portuguese text with quotes', () => {
  const text = 'Ela disse, "Olá, mundo!" e sorriu.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Russian (with Latin transliteration)
test('parse Russian text (transliterated) with quotes', () => {
  const text = 'Ona skazala, "Privet, mir!" i ulyblas.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Japanese (romanized)
test('parse Japanese text (romanized) with quotes', () => {
  const text = 'Kanojo wa "Konnichiwa, sekai!" to itta.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Chinese (Pinyin)
test('parse Chinese text (Pinyin) with quotes', () => {
  const text = 'Ta shuo, "Nǐ hǎo, shìjiè!" ránhòu xiàole.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});

// Academic citations
test('parse academic text with citations', () => {
  const text = 'The study [Smith et al., 2020] found that performance (measured in ms) improved.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Mathematical expressions
test('parse mathematical text', () => {
  const text = 'The formula is f(x) = [a + b] * {c - d}.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.equal(serialize(result), text);
});

// Literature with nested quotes
test('parse literature with nested quotes', () => {
  const text = `He thought, "Did she really say that?" and she said, 'Yes, I did.'`;
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.ok(hasDelimiterType(result, 'singleQuote'));
  assert.equal(serialize(result), text);
});

// Poetry with parenthetical notes
test('parse poetry with annotations', () => {
  const text = 'Roses are red (traditionally), violets are blue.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Legal text with references
test('parse legal text with section references', () => {
  const text = 'According to Section 5(a) [see amendment], the party must comply.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.equal(serialize(result), text);
});

// News article
test('parse news article text', () => {
  const text = 'The CEO said, "Our growth exceeded expectations." The increase (50% year-over-year) was significant.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Email format
test('parse email-like text', () => {
  const text = 'Hi [Name], I wanted to follow up on our meeting (yesterday). Thanks!';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Social media post
test('parse social media post', () => {
  const text = 'Just learned something cool! [link] Check it out!';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.equal(serialize(result), text);
});

// Recipe format
test('parse recipe text', () => {
  const text = 'Add ingredients [see list] and mix (gently) for 2 minutes.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Technical documentation
test('parse technical documentation', () => {
  const text = 'The function `process` accepts parameters [x, y] and returns an object {status, data}.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'backtick'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.ok(hasDelimiterType(result, 'curly'));
  assert.equal(serialize(result), text);
});

// Mixed language and code
test('parse mixed natural language and code', () => {
  const text = 'To debug, run `npm test` (in terminal) and check [output].';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'backtick'));
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.equal(serialize(result), text);
});

// Lyrics with annotations
test('parse song lyrics with notes', () => {
  const text = `"Imagine" (by John Lennon) says "You may say I'm a dreamer."`;
  const result = parse(text);
  assert.ok(result.filter(b => b.type === 'doubleQuote').length === 2);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.equal(serialize(result), text);
});

// Complex academic text
test('parse complex academic text', () => {
  const text = 'The hypothesis (H₁) states that "performance improves" [p < 0.05].';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.ok(hasDelimiterType(result, 'square'));
  assert.equal(serialize(result), text);
});

// Empty delimiters in text
test('parse text with empty delimiters', () => {
  const text = 'The list () was empty, the quote "" was blank.';
  const result = parse(text);
  assert.ok(hasDelimiterType(result, 'paren'));
  assert.ok(hasDelimiterType(result, 'doubleQuote'));
  assert.equal(serialize(result), text);
});
