//! Tests for meta-notation with natural language text
//!
//! Meta-notation should work seamlessly with natural languages since they
//! use the same delimiters: quotes for speech, parentheses for asides,
//! brackets for references, etc.

use meta_notation::{parse, serialize, Block, DelimiterType};

// Helper function to check if a delimiter type exists anywhere in the parsed result
fn has_delimiter_type(blocks: &[Block], dtype: &DelimiterType) -> bool {
    blocks.iter().any(|b| b.has_delimiter_type(dtype))
}

#[test]
fn test_parse_english_text_with_quotes() {
    let text = r#"She said, "Hello, world!" and smiled."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_english_text_with_parentheses() {
    let text = "The conference (scheduled for next week) will be online.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_english_text_with_brackets() {
    let text = "According to the report [see page 42], the results were positive.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_english_dialogue() {
    let text = r#""How are you?" she asked. "I'm fine," he replied."#;
    let result = parse(text);
    let quote_count = result.iter().filter(|b| b.delimiter_type() == DelimiterType::DoubleQuote).count();
    assert_eq!(quote_count, 2);
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_english_nested_structure() {
    let text = r#"He said, "I heard her say 'hello' yesterday.""#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_spanish_text_with_quotes() {
    let text = r#"Ella dijo, "¡Hola, mundo!" y sonrió."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_spanish_text_with_parentheses() {
    let text = "La conferencia (programada para la próxima semana) será en línea.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_french_text_with_quotes() {
    let text = r#"Elle a dit, "Bonjour, le monde!" et a souri."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_french_text_with_parentheses() {
    let text = "La conférence (prévue pour la semaine prochaine) sera en ligne.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_german_text_with_quotes() {
    let text = r#"Sie sagte, "Hallo, Welt!" und lächelte."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_german_text_with_parentheses() {
    let text = "Die Konferenz (geplant für nächste Woche) wird online sein.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_italian_text_with_quotes() {
    let text = r#"Lei disse, "Ciao, mondo!" e sorrise."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_portuguese_text_with_quotes() {
    let text = r#"Ela disse, "Olá, mundo!" e sorriu."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_russian_text_transliterated_with_quotes() {
    let text = r#"Ona skazala, "Privet, mir!" i ulyblas."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_japanese_text_romanized_with_quotes() {
    let text = r#"Kanojo wa "Konnichiwa, sekai!" to itta."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_chinese_text_pinyin_with_quotes() {
    let text = r#"Ta shuo, "Nǐ hǎo, shìjiè!" ránhòu xiàole."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_academic_text_with_citations() {
    let text = "The study [Smith et al., 2020] found that performance (measured in ms) improved.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_mathematical_text() {
    let text = "The formula is f(x) = [a + b] * {c - d}.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_literature_with_nested_quotes() {
    let text = r#"He thought, "Did she really say that?" and she said, 'Yes, I did.'"#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert!(has_delimiter_type(&result, &DelimiterType::SingleQuote));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_poetry_with_annotations() {
    let text = "Roses are red (traditionally), violets are blue.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_legal_text_with_section_references() {
    let text = "According to Section 5(a) [see amendment], the party must comply.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_news_article_text() {
    let text = r#"The CEO said, "Our growth exceeded expectations." The increase (50% year-over-year) was significant."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_email_like_text() {
    let text = "Hi [Name], I wanted to follow up on our meeting (yesterday). Thanks!";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_social_media_post() {
    let text = "Just learned something cool! [link] Check it out!";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_recipe_text() {
    let text = "Add ingredients [see list] and mix (gently) for 2 minutes.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_technical_documentation() {
    let text = "The function `process` accepts parameters [x, y] and returns an object {status, data}.";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Backtick));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_mixed_natural_language_and_code() {
    let text = "To debug, run `npm test` (in terminal) and check [output].";
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Backtick));
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_song_lyrics_with_notes() {
    let text = r#""Imagine" (by John Lennon) says "You may say I'm a dreamer.""#;
    let result = parse(text);
    let quote_count = result.iter().filter(|b| b.delimiter_type() == DelimiterType::DoubleQuote).count();
    assert_eq!(quote_count, 2);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_complex_academic_text() {
    let text = r#"The hypothesis (H₁) states that "performance improves" [p < 0.05]."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert_eq!(serialize(&result), text);
}

#[test]
fn test_parse_text_with_empty_delimiters() {
    let text = r#"The list () was empty, the quote "" was blank."#;
    let result = parse(text);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), text);
}
