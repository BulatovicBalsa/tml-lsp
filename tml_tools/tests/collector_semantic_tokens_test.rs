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

// ───────────────────────── Function call arguments ─────────────────────────

#[test]
fn test_function_call_name_is_function_token() {
    let tokens = collect("fn bar():\n    a = foo(x, y)\nend");
    assert!(has_token(&tokens, 1, 8, &TokenType::Function, TokenModifiers::NONE),
        "Expected Function token for 'foo' call at (1, 8), got: {:?}", tokens);
}

#[test]
fn test_function_call_simple_argument_is_variable() {
    // "foo(x)" -> argument "x" is Variable
    let tokens = collect("fn bar():\n    a = foo(x)\nend");
    let vars = find_type(&tokens, &TokenType::Variable);
    assert!(vars.iter().any(|t| t.modifiers == TokenModifiers::NONE),
        "Expected Variable token for argument 'x', got: {:?}", tokens);
}

#[test]
fn test_function_call_multiple_arguments_are_variables() {
    // "name(x, 2)" -> x is Variable, 2 is skipped (Constant has no position)
    // fn bar():\n    a = name(x, y)\nend
    let tokens = collect("fn bar():\n    a = name(x, y)\nend");
    let vars = find_type(&tokens, &TokenType::Variable);
    // x and y should be Variable tokens (plus 'a' from assignment)
    assert!(vars.len() >= 2,
        "Expected at least 2 Variable tokens for x and y arguments, got: {:?}", tokens);
}

#[test]
fn test_function_call_property_argument() {
    let tokens = collect("fn bar():\n    a = foo(p.x)\nend");
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property token for 'x' in p.x argument, got: {:?}", tokens);
}

// ───────────────────────── Control flow keywords ─────────────────────────

#[test]
fn test_if_keyword_token() {
    let tokens = collect("fn foo():\n    if true:\n        pass\n    end\nend");
    assert!(has_token(&tokens, 1, 4, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected keyword at (1, 4), got: {:?}", tokens);
}

#[test]
fn test_elseif_keyword_token() {
    let tokens = collect("fn foo():\n    if true:\n        pass\n    elseif false:\n        pass\n    end\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 3),
        "Expected elseif keyword token on line 3, got: {:?}", tokens);
}

#[test]
fn test_else_keyword_token() {
    let tokens = collect("fn foo():\n    if true:\n        pass\n    else:\n        pass\n    end\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 3),
        "Expected else keyword token on line 3, got: {:?}", tokens);
}

#[test]
fn test_for_keyword_token() {
    let tokens = collect("fn foo():\n    for i = 0:5:\n        pass\n    end\nend");
    assert!(has_token(&tokens, 1, 4, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected for keyword at (1, 4), got: {:?}", tokens);
}

#[test]
fn test_for_index_is_variable_declaration() {
    // for index variable is Variable + DECLARATION
    let tokens = collect("fn foo():\n    for i = 0:5:\n        pass\n    end\nend");
    let vars = find_type(&tokens, &TokenType::Variable);
    let decl = find_modifier(vars, TokenModifiers::DECLARATION);
    assert!(!decl.is_empty(),
        "Expected Variable+DECLARATION for for index 'i', got: {:?}", tokens);
}

#[test]
fn test_while_keyword_token() {
    let tokens = collect("fn foo():\n    int x = 1\n    while x > 0:\n        pass\n    end\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected while keyword token on line 2, got: {:?}", tokens);
}

// ───────────────────────── Assignment ─────────────────────────

#[test]
fn test_assignment_lhs_is_variable() {
    // "a = 5" -> "a" is Variable (no DECLARATION — inferred)
    let tokens = collect("fn foo():\n    a = 5\nend");
    assert!(has_token(&tokens, 1, 4, &TokenType::Variable, TokenModifiers::NONE),
        "Expected Variable at (1, 4), got: {:?}", tokens);
}

#[test]
fn test_assignment_dot_access_lhs() {
    // "p.x = 5" -> "p" is Namespace, "x" is Property
    let tokens = collect("fn foo():\n    p.x = 5\nend");
    assert!(has_token(&tokens, 1, 4, &TokenType::Namespace, TokenModifiers::NONE),
        "Expected Namespace for 'p' at (1, 4), got: {:?}", tokens);
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property for 'x', got: {:?}", tokens);
}

// ───────────────────────── Declaration ─────────────────────────

#[test]
fn test_declaration_type_is_type_token() {
    // "int x = 5" -> "int" is Type
    let tokens = collect("fn foo():\n    int x = 5\nend");
    assert!(!find_type(&tokens, &TokenType::Type).is_empty(),
        "Expected Type token for 'int', got: {:?}", tokens);
}

#[test]
fn test_declaration_name_is_variable_declaration() {
    // "int x = 5" -> "x" is Variable + DECLARATION
    let tokens = collect("fn foo():\n    int x = 5\nend");
    let vars = find_type(&tokens, &TokenType::Variable);
    let decls = find_modifier(vars, TokenModifiers::DECLARATION);
    assert!(!decls.is_empty(),
        "Expected Variable+DECLARATION for 'x', got: {:?}", tokens);
}

#[test]
fn test_global_declaration() {
    // global declaration outside function
    let tokens = collect("int x = 5");
    assert!(!find_type(&tokens, &TokenType::Type).is_empty(),
        "Expected Type token for global 'int', got: {:?}", tokens);
    let vars = find_type(&tokens, &TokenType::Variable);
    assert!(!find_modifier(vars, TokenModifiers::DECLARATION).is_empty(),
        "Expected Variable+DECLARATION for global 'x', got: {:?}", tokens);
}

// ───────────────────────── Constants ─────────────────────────

#[test]
fn test_integer_constant_is_number() {
    let tokens = collect("fn foo():\n    a = 42\nend");
    assert!(!find_type(&tokens, &TokenType::Number).is_empty(),
        "Expected Number token for integer constant, got: {:?}", tokens);
}

#[test]
fn test_float_constant_is_number() {
    let tokens = collect("fn foo():\n    a = 3.14\nend");
    assert!(!find_type(&tokens, &TokenType::Number).is_empty(),
        "Expected Number token for float constant, got: {:?}", tokens);
}

#[test]
fn test_boolean_constant_is_keyword() {
    let tokens = collect("fn foo():\n    a = true\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.len() >= 2,
        "Expected keyword tokens including 'true', got: {:?}", tokens);
}

// ───────────────────────── Macro if / macro for ─────────────────────────

#[test]
fn test_macro_if_macro_keyword_token() {
    // "macro if true:" -> "macro" at col 0 is Keyword
    let src = "macro if true:\n    pass\nend";
    let tokens = collect(src);
    assert!(has_token(&tokens, 0, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected Keyword for 'macro' at (0, 0), got: {:?}", tokens);
}

#[test]
fn test_macro_if_end_keyword_token() {
    let src = "macro if true:\n    pass\nend";
    let tokens = collect(src);
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected end keyword on line 2, got: {:?}", tokens);
}

#[test]
fn test_macro_if_elseif_keyword_token() {
    // macro if with elseif
    let src = "macro if true:\n    pass\nelseif false:\n    pass\nend";
    let tokens = collect(src);
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected elseif keyword token on line 2, got: {:?}", tokens);
}

#[test]
fn test_macro_if_else_keyword_token() {
    let src = "macro if true:\n    pass\nelse:\n    pass\nend";
    let tokens = collect(src);
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected else keyword token on line 2, got: {:?}", tokens);
}

#[test]
fn test_macro_for_macro_keyword_token() {
    // "macro for i = 0:5:" -> "macro" at col 0 is Keyword
    let src = "macro for i = 0:5:\n    pass\nend";
    let tokens = collect(src);
    assert!(has_token(&tokens, 0, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected Keyword for 'macro' at (0, 0), got: {:?}", tokens);
}

#[test]
fn test_macro_for_index_is_variable_declaration() {
    let src = "macro for i = 0:5:\n    pass\nend";
    let tokens = collect(src);
    let vars = find_type(&tokens, &TokenType::Variable);
    let decls = find_modifier(vars, TokenModifiers::DECLARATION);
    assert!(!decls.is_empty(),
        "Expected Variable+DECLARATION for macro for index 'i', got: {:?}", tokens);
}

#[test]
fn test_macro_for_end_keyword_token() {
    let src = "macro for i = 0:5:\n    pass\nend";
    let tokens = collect(src);
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected end keyword on line 2, got: {:?}", tokens);
}

#[test]
fn test_macro_if_if_keyword_token() {
    // "macro if true:" -> "if" keyword should also be colored
    let src = "macro if true:\n    pass\nend";
    let tokens = collect(src);
    let kws = find_type(&tokens, &TokenType::Keyword);
    // expect: macro (col 0), if (col 6), end (line 2)
    assert!(kws.iter().any(|t| t.line == 0 && t.col > 0),
        "Expected 'if' keyword token on line 0 after 'macro', got: {:?}", tokens);
}

#[test]
fn test_macro_if_has_both_macro_and_if_keywords() {
    let src = "macro if x == 5:\n    pass\nend";
    let tokens = collect(src);
    let kws_on_line_0: Vec<_> = find_type(&tokens, &TokenType::Keyword)
        .into_iter()
        .filter(|t| t.line == 0)
        .collect();
    assert_eq!(kws_on_line_0.len(), 2,
        "Expected both 'macro' and 'if' keyword tokens on line 0, got: {:?}", kws_on_line_0);
}

#[test]
fn test_macro_for_for_keyword_token() {
    let src = "macro for i = 0:5:\n    pass\nend";
    let tokens = collect(src);
    let kws_on_line_0: Vec<_> = find_type(&tokens, &TokenType::Keyword)
        .into_iter()
        .filter(|t| t.line == 0)
        .collect();
    assert_eq!(kws_on_line_0.len(), 2,
        "Expected both 'macro' and 'for' keyword tokens on line 0, got: {:?}", kws_on_line_0);
}

#[test]
fn test_io_in_keyword_is_keyword_token() {
    let tokens = collect("in<int, 0x0> x");
    assert!(has_token(&tokens, 0, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected Keyword for 'in' at (0, 0), got: {:?}", tokens);
}

#[test]
fn test_io_out_keyword_is_keyword_token() {
    let tokens = collect("out<real, 0x0> x");
    assert!(has_token(&tokens, 0, 0, &TokenType::Keyword, TokenModifiers::NONE),
        "Expected Keyword for 'out' at (0, 0), got: {:?}", tokens);
}

#[test]
fn test_io_declaration_type_is_type_token() {
    let tokens = collect("in<int, 0x0> x");
    assert!(!find_type(&tokens, &TokenType::Type).is_empty(),
        "Expected Type token for 'int' in IO declaration, got: {:?}", tokens);
}

#[test]
fn test_io_declaration_name_is_variable_declaration() {
    let tokens = collect("in<int, 0x0> x");
    let vars = find_type(&tokens, &TokenType::Variable);
    let decls = find_modifier(vars, TokenModifiers::DECLARATION);
    assert!(!decls.is_empty(),
        "Expected Variable+DECLARATION for 'x' in IO declaration, got: {:?}", tokens);
}

#[test]
fn test_io_declaration_with_namespace_address() {
    // address expression contains namespace references -> Variable + Property
    let tokens = collect("in<int, n.rd_ds> x");
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property token for 'rd_ds' in namespace address, got: {:?}", tokens);
}

#[test]
fn test_io_write_lhs_is_variable() {
    let tokens = collect("fn foo():\n    in<int, 0x0> x\n    x = 5\nend");
    let vars = find_type(&tokens, &TokenType::Variable);
    assert!(vars.iter().any(|t| t.modifiers == TokenModifiers::NONE),
        "Expected Variable token for IO write lhs, got: {:?}", tokens);
}

#[test]
fn test_io_write_dot_access_lhs() {
    let tokens = collect("fn foo():\n    in<int, 0x0> p.x\n    p.x = 5\nend");
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property token for dot access in IO write, got: {:?}", tokens);
}

#[test]
fn test_namespace_first_id_is_variable() {
    let tokens = collect("fn foo():\n    x = t.gain\nend");
    assert!(has_token(&tokens, 1, 8, &TokenType::Namespace, TokenModifiers::NONE),
        "Expected Namespace for 't' at (1, 8), got: {:?}", tokens);
}

#[test]
fn test_namespace_second_id_is_property() {
    let tokens = collect("fn foo():\n    x = t.gain\nend");
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property for 'gain', got: {:?}", tokens);
}

#[test]
fn test_namespace_p_is_namespace_token() {
    let tokens = collect("fn foo():\n    x = p.input\nend");
    assert!(!find_type(&tokens, &TokenType::Namespace).is_empty(),
        "Expected Namespace token for 'p', got: {:?}", tokens);
}

#[test]
fn test_namespace_n_is_namespace_token() {
    let tokens = collect("fn foo():\n    x = n.count\nend");
    assert!(!find_type(&tokens, &TokenType::Namespace).is_empty(),
        "Expected Namespace token for 'n', got: {:?}", tokens);
}

#[test]
fn test_namespace_deep_chain_rest_are_property() {
    // "t.s_ctrl.length" -> t=Namespace, s_ctrl=Property, length=Property
    let tokens = collect("fn foo():\n    x = t.s_ctrl.length\nend");
    let props = find_type(&tokens, &TokenType::Property);
    assert!(props.len() >= 2,
        "Expected at least 2 Property tokens for 's_ctrl' and 'len', got: {:?}", tokens);
}

#[test]
fn test_plain_variable_is_not_namespace() {
    // "myvar" is not a reserved namespace -> Variable, not Namespace
    let tokens = collect("fn foo():\n    myvar = 5\nend");
    assert!(find_type(&tokens, &TokenType::Namespace).is_empty(),
        "Expected no Namespace tokens for plain variable, got: {:?}", tokens);
}

#[test]
fn test_namespace_in_assignment_lhs() {
    let tokens = collect("fn foo():\n    p.x = 5\nend");
    assert!(!find_type(&tokens, &TokenType::Namespace).is_empty(),
        "Expected Namespace for 'p' in assignment lhs, got: {:?}", tokens);
    assert!(!find_type(&tokens, &TokenType::Property).is_empty(),
        "Expected Property for 'x' in assignment lhs, got: {:?}", tokens);
}

#[test]
fn test_pass_is_keyword() {
    let tokens = collect("fn foo():\n    pass\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 1),
        "Expected 'pass' keyword token on line 1, got: {:?}", tokens);
}

#[test]
fn test_return_is_keyword() {
    let tokens = collect("fn foo():\n    return\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 1),
        "Expected 'return' keyword token on line 1, got: {:?}", tokens);
}

#[test]
fn test_return_value_is_keyword() {
    let tokens = collect("fn foo():\n    return 42\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 1),
        "Expected 'return' keyword token on line 1, got: {:?}", tokens);
}

#[test]
fn test_break_is_keyword() {
    let tokens = collect("fn foo():\n    for i = 0:5:\n        break\n    end\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected 'break' keyword token on line 2, got: {:?}", tokens);
}

#[test]
fn test_continue_is_keyword() {
    let tokens = collect("fn foo():\n    for i = 0:5:\n        continue\n    end\nend");
    let kws = find_type(&tokens, &TokenType::Keyword);
    assert!(kws.iter().any(|t| t.line == 2),
        "Expected 'continue' keyword token on line 2, got: {:?}", tokens);
}
