use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::semantic_tokens::{
    RawToken, SemanticTokenCollector, TokenModifiers, TokenType,
};

fn collect(src: &str) -> Vec<RawToken> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let ast = TmlParser::new().parse(&normalized).expect("Parse failed");
    SemanticTokenCollector::new().collect(&ast)
}

fn find(tokens: &[RawToken], line: u32, col: u32) -> Option<&RawToken> {
    tokens.iter().find(|t| t.line == line && t.col == col)
}

fn find_type<'a>(tokens: &'a [RawToken], token_type: &TokenType) -> Vec<&'a RawToken> {
    let ty = token_type.clone() as u32;
    tokens.iter().filter(|t| t.token_type.clone() as u32 == ty).collect()
}

fn find_modifier(tokens: Vec<&RawToken>, token_modifier: u32) -> Vec<&RawToken> {
    tokens.into_iter().filter(|t| t.modifiers == token_modifier).collect()
}

fn has_token(tokens: &[RawToken], line: u32, col: u32, token_type: &TokenType, modifiers: u32) -> bool {
    let ty = token_type.clone() as u32;
    tokens.iter().any(|t| {
        t.line == line
            && t.col == col
            && t.token_type.clone() as u32 == ty
            && t.modifiers == modifiers
    })
}

#[test]
fn test_fn_keyword_token() {
    let tokens = collect("fn foo():\n    pass\nend");
    assert!(has_token(&tokens, 0, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected keyword at (0, 0), got: {:?}", tokens);
}

#[test]
fn test_fn_keyword_len() {
    let tokens = collect("fn foo():\n    pass\nend");
    let t = find(&tokens, 0, 0).expect("Expected token at (0,0)");
    assert_eq!(t.len, 2);
}

#[test]
fn test_function_name_token() {
    let tokens = collect("fn foo():\n    pass\nend");
    assert!(has_token(&tokens, 0, 3, &TokenType::Function, TokenModifiers::DECLARATION),
        "Expected function declaration at (0, 3), got: {:?}", tokens);
}

#[test]
fn test_function_name_len() {
    let tokens = collect("fn foo():\n    pass\nend");
    let t = find(&tokens, 0, 3).expect("Expected token at (0, 3)");
    assert_eq!(t.len, 3);
}

#[test]
fn test_function_name_longer() {
    let tokens = collect("fn my_function():\n    pass\nend");
    assert!(has_token(&tokens, 0, 3, &TokenType::Function, TokenModifiers::DECLARATION),
        "Expected function declaration at (0, 3), got: {:?}", tokens);
    let t = find(&tokens, 0, 3).unwrap();
    assert_eq!(t.len, 11);
}

#[test]
fn test_end_keyword_token() {
    let tokens = collect("fn foo():\n    pass\nend");
    assert!(has_token(&tokens, 2, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected keyword at (2, 0), got: {:?}", tokens);
}

#[test]
fn test_end_keyword_len() {
    let tokens = collect("fn foo():\n    pass\nend");
    let t = find(&tokens, 2, 0).expect("Expected token at (2, 0)");
    assert_eq!(t.len, 3);
}

#[test]
fn test_parameter_type_token() {
    let tokens = collect("fn foo(int x):\n    pass\nend");
    assert!(has_token(&tokens, 0, 7, &TokenType::Type, TokenModifiers::NONE),
        "Expected type at (0, 7), got: {:?}", tokens);
}

#[test]
fn test_parameter_name_token() {
    let tokens = collect("fn foo(int x):\n    pass\nend");
    assert!(has_token(&tokens, 0, 11, &TokenType::Parameter, TokenModifiers::DECLARATION),
        "Expected parameter at (0, 11), got: {:?}", tokens);
}

#[test]
fn test_multiple_parameters() {
    let tokens = collect("fn foo(int x, real y):\n    pass\nend");
    assert!(has_token(&tokens, 0, 7,  &TokenType::Type,      TokenModifiers::NONE),        "int type");
    assert!(has_token(&tokens, 0, 11, &TokenType::Parameter, TokenModifiers::DECLARATION), "x param");
    assert!(has_token(&tokens, 0, 14, &TokenType::Type,      TokenModifiers::NONE),        "real type");
    assert!(has_token(&tokens, 0, 19, &TokenType::Parameter, TokenModifiers::DECLARATION), "y param");
}

#[test]
fn test_no_parameters() {
    let tokens = collect("fn foo():\n    pass\nend");
    assert_eq!(find_type(&tokens, &TokenType::Parameter).len(), 0);
}

#[test]
fn test_return_type_token() {
    let tokens = collect("fn foo() int:\n    pass\nend");
    assert!(!find_type(&tokens, &TokenType::Type).is_empty(),
        "Expected at least one type token for return type, got: {:?}", tokens);
}

#[test]
fn test_fn_tokens_in_order() {
    let tokens = collect("fn foo():\n    pass\nend");
    let keyword_tokens = find_type(&tokens, &TokenType::Keyword);
    assert!(keyword_tokens.len() >= 2);
    assert!(keyword_tokens[0].line < keyword_tokens[1].line,
        "fn keyword should come before end keyword");
}

#[test]
fn test_multiple_functions() {
    let src = "fn foo():\n    pass\nend\nfn bar():\n    pass\nend";
    let tokens = collect(src);
    assert_eq!(find_type(&tokens, &TokenType::Function).len(), 2, "Expected 2 function name tokens");
}

#[test]
fn test_derived_type_first_identifier_is_parameter() {
    // "p" at start of dot access is Parameter
    let tokens = collect("fn foo(p.MyType.type x):\n    pass\nend");
    assert!(has_token(&tokens, 0, 7, &TokenType::Parameter, TokenModifiers::NONE),
        "Expected Parameter for 'p' at (0, 7), got: {:?}", tokens);
}

#[test]
fn test_derived_type_middle_identifiers_are_parameter() {
    // "p.someId.type a" -> p and someId are Parameter (no DECLARATION)
    let tokens = collect("fn foo(p.someId.type x):\n    pass\nend");
    let params = find_type(&tokens, &TokenType::Parameter);
    let dot_params = find_modifier(params, TokenModifiers::NONE);
    assert_eq!(dot_params.len(), 2,
        "Expected 2 Parameter tokens for p.someId, got: {:?}", tokens);
}

#[test]
fn test_derived_type_type_at_end_is_type_token() {
    // ".type" at end of dot access is TokenType::Type
    let tokens = collect("fn foo(p.someId.type x):\n    pass\nend");
    assert!(!find_type(&tokens, &TokenType::Type).is_empty(),
        "Expected Type token for 'type' at end of dot access, got: {:?}", tokens);
}

#[test]
fn test_derived_type_parameter_name_is_declaration() {
    // parameter name after derived type is Parameter + DECLARATION
    let tokens = collect("fn foo(p.someId.type x):\n    pass\nend");
    let decl_params: Vec<_> = find_type(&tokens, &TokenType::Parameter)
        .into_iter()
        .filter(|t| t.modifiers == TokenModifiers::DECLARATION)
        .collect();
    assert_eq!(decl_params.len(), 1,
        "Expected 1 Parameter+DECLARATION for 'x', got: {:?}", tokens);
}
