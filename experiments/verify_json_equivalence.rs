// Experiment: Verify that Rust serde JSON output matches JS parsed object structure
// Run with: cd /tmp/gh-issue-solver-1774090289115 && cargo test --test verify_json_equivalence -- --nocapture

use meta_notation::{parse, Block};

fn main() {
    let test_cases = vec![
        "hello world",
        "(hello)",
        "{world}",
        "[test]",
        "'hello'",
        "\"world\"",
        "`code`",
        "hello (world) {test}",
        "(a (b) c)",
        "{a [b (c) d] e}",
        "(){}[]",
        "\"hello {world}\"",
    ];

    for input in test_cases {
        let result = parse(input);
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("Input: {:?}", input);
        println!("JSON: {}", json);
        println!("---");
    }
}
