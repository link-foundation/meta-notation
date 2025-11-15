//! Tests for meta-notation with various programming languages

use meta_notation::{parse, serialize, DelimiterType};

// Helper function to check if a delimiter type exists anywhere in the parsed result
fn has_delimiter_type(blocks: &[meta_notation::Block], dtype: &DelimiterType) -> bool {
    blocks.iter().any(|b| b.has_delimiter_type(dtype))
}

#[test]
fn test_parse_javascript_code() {
    let code = "const greet = (name) => { return `Hello, ${name}!`; };";
    let result = parse(code);
    assert!(result.len() > 0);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Backtick));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_python_code() {
    let code = r#"def calculate(x, y): return {"sum": x + y, "list": [x, y]}"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_go_code() {
    let code = r#"func main() { fmt.Println("Hello, World!") }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_rust_code() {
    let code = r#"fn main() { let x = vec![1, 2, 3]; println!("{:?}", x); }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_cpp_code() {
    let code = r#"int main() { std::cout << "Hello" << std::endl; return 0; }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_java_code() {
    let code = r#"public class Main { public static void main(String[] args) { System.out.println("Hello"); } }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_csharp_code() {
    let code =
        r#"public void Test() { var list = new List<int> {1, 2, 3}; Console.WriteLine("Done"); }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_ruby_code() {
    let code = r#"def greet(name); puts "Hello, #{name}!"; end"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_php_code() {
    let code = r#"function test($x) { return ["key" => "value"]; }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_swift_code() {
    let code = r#"func greet(name: String) -> String { return "Hello, \(name)!" }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_kotlin_code() {
    let code = r#"fun main() { val list = listOf(1, 2, 3); println("Hello") }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_typescript_code() {
    let code = "const add = (a: number, b: number): number => { return a + b; };";
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_scala_code() {
    let code = "def add(x: Int, y: Int): Int = { x + y }";
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_perl_code() {
    let code = r#"sub greet { my ($name) = @_; print "Hello, $name!\n"; }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_haskell_code() {
    let code = r#"main = putStrLn "Hello, World!""#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_lisp_code() {
    let code = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_clojure_code() {
    let code = r#"(defn greet [name] (str "Hello, " name "!"))"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_lua_code() {
    let code = r#"function greet(name) return "Hello, " .. name end"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_elixir_code() {
    let code = r#"def greet(name), do: "Hello, #{name}!""#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_r_code() {
    let code = r#"greet <- function(name) { paste("Hello,", name) }"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_matlab_code() {
    let code = "function y = square(x); y = x .^ 2; end";
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Paren));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_sql_code() {
    let code = r#"SELECT name, age FROM users WHERE status = "active" ORDER BY created_at;"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_json() {
    let code = r#"{"name": "John", "age": 30, "tags": ["developer", "designer"]}"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Curly));
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_yaml_with_brackets() {
    let code = r#"dependencies: ["react", "typescript"]"#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Square));
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_bash_code() {
    let code = r#"echo "Hello, ${USER}!" | grep "Hello""#;
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::DoubleQuote));
    assert_eq!(serialize(&result), code);
}

#[test]
fn test_parse_markdown_with_code_blocks() {
    let code = "Here is code: `const x = [1, 2, 3];` in backticks.";
    let result = parse(code);
    assert!(has_delimiter_type(&result, &DelimiterType::Backtick));
    assert_eq!(serialize(&result), code);
}
