use rustemo::Parser;
use rstest::rstest;
use tml_parser::tml::TmlParser;

#[test]
fn test_fn_prefix_identifier() {
    let parser = TmlParser::new();
    let input = "fn_x = 0\nfn test_fn():\n    real if_x = 0\nend\n";
    assert!(parser.parse(input).is_ok(), "Failed to parse: {}", input);
}

#[rstest]
#[case("if_x")]
#[case("for_x")]
#[case("while_x")]
#[case("feedthrough_x")]
#[case("exists_x")]
#[case("input_x")]
#[case("in_x")]
#[case("out_x")]
#[case("int_x")]
#[case("real_x")]
#[case("bool_x")]
#[case("str_x")]
#[case("fn_x")]
#[case("return_x")]
#[case("break_x")]
#[case("continue_x")]
#[case("not_x")]
#[case("and_x")]
#[case("or_x")]
#[case("tensor_x")]
#[case("macro_x")]
#[case("pass_x")]
#[case("narrow_x")]
fn test_keyword_prefix_identifier(#[case] var: &str) {
    let parser = TmlParser::new();
    let input = format!("fn test():\n    {} = 0\nend\n", var);
    assert!(parser.parse(&input).is_ok(), "Failed to parse identifier: {}", var);
}