{{
  // Meta-Notation Grammar
  // A simple notation for parsing delimiters across multiple languages
  // Supports: (), {}, [], '', "", ``
  // Does NOT support : for self-reference (unlike links-notation)
}}

text = s:sequence { return s; }

sequence = blocks:block* { return blocks; }

block
  = "(" s:sequence ")" { return { type: 'paren', content: s }; }
  / "{" s:sequence "}" { return { type: 'curly', content: s }; }
  / "[" s:sequence "]" { return { type: 'square', content: s }; }
  / "'" s:singleQuoteContent "'" { return { type: 'singleQuote', content: s }; }
  / '"' s:doubleQuoteContent '"' { return { type: 'doubleQuote', content: s }; }
  / "`" s:backtickContent "`" { return { type: 'backtick', content: s }; }
  / c:plainText { return { type: 'text', content: c }; }

// Content inside quotes - different rules since they don't nest the same way
singleQuoteContent = c:[^']* { return c.join(''); }
doubleQuoteContent = c:[^"]* { return c.join(''); }
backtickContent = c:[^`]* { return c.join(''); }

// Plain text outside delimiters
plainText = c:[^(){}[\]'"`]+ { return c.join(''); }
