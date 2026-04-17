use rustemo::Parser;
use tml_parser::tml::TmlParser;

fn assert_parses(input: &str, error_msg: &str) {
    let parser = TmlParser::new();
    let ast = parser.parse(input);
    assert!(ast.is_ok(), "{}", error_msg);
}

#[test]
fn test_empty_function() {
    let input = "fn empty_fn():\nend\n";
    assert_parses(input, "Failed to parse empty function");
}

#[test]
fn test_empty_if() {
    let input = "fn test():\n    if true:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty if statement");
}

#[test]
fn test_empty_else() {
    let input = "fn test():\n    if false:\n        return\n    else:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty else statement");
}

#[test]
fn test_empty_elseif() {
    let input = "fn test():\n    if false:\n        return\n    elseif true:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty elseif statement");
}

#[test]
fn test_empty_while() {
    let input = "fn test():\n    while false:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty while loop");
}

#[test]
fn test_empty_for() {
    let input = "fn test():\n    for i in 0:10:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty for loop");
}

#[test]
fn test_empty_feedthrough() {
    let input = "fn test():\n    feedthrough condition:\n    end\nend\n";
    assert_parses(input, "Failed to parse empty feedthrough block");
}