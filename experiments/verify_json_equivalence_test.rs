use meta_notation::parse;

#[test]
fn verify_json_equivalence() {
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
