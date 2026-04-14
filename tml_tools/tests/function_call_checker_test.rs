use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::symbol_table::SymbolTableBuilder;
use tml_tools::function_call_checker::{CallError, FunctionCallChecker};

fn check(src: &str) -> Vec<CallError> {
    let parser = TmlParser::new();
    let ast = parser.parse(src).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);
    FunctionCallChecker::new(&table).check(&ast)
}

fn has_undefined_fn(errors: &[CallError], name: &str) -> bool {
    errors.iter().any(|e| matches!(e,
        CallError::UndefinedFunction { name: n, .. } if n == name
    ))
}

fn has_arg_count_error(errors: &[CallError], name: &str, expected: usize, got: usize) -> bool {
    errors.iter().any(|e| matches!(e,
        CallError::ArgumentCountMismatch { name: n, expected: ex, got: g, .. }
        if n == name && *ex == expected && *g == got
    ))
}

fn has_named_arg_error(errors: &[CallError], fn_name: &str, arg_name: &str) -> bool {
    errors.iter().any(|e| matches!(e,
        CallError::NamedArgumentNotAllowed { function_name: f, arg_name: a, .. }
        if f == fn_name && a == arg_name
    ))
}

fn has_entry_fn_error(errors: &[CallError], name: &str) -> bool {
    errors.iter().any(|e| matches!(e,
        CallError::EntryFunctionCall { name: n, .. } if n == name
    ))
}

// ───────────────────────── Valid calls ─────────────────────────

#[test]
fn test_valid_call_no_args() {
    let errors = check("fn foo():\n    return\nend\ny = foo()");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_valid_call_one_arg() {
    let errors = check("fn foo(int a):\n    return\nend\ny = foo(1)");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_valid_call_multiple_args() {
    let errors = check("fn foo(int a, int b, int c):\n    return\nend\ny = foo(1, 2, 3)");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_valid_call_inside_function() {
    let errors = check(
        "fn helper(int x):\n    return\nend\nfn main():\n    helper(42)\nend"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_valid_call_in_expression() {
    let errors = check("fn double(int x):\n    return x * 2\nend\ny = double(5)");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_valid_nested_calls() {
    let errors = check(
        "fn inc(int x):\n    return x + 1\nend\ny = inc(inc(1))"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

// ───────────────────────── Built-in functions ─────────────────────────

#[test]
fn test_builtin_min_ok() {
    let errors = check("y = min(a)");
    assert!(errors.is_empty(), "Built-in min should be valid");
}

#[test]
fn test_builtin_max_ok() {
    let errors = check("y = max(a)");
    assert!(errors.is_empty());
}

#[test]
fn test_builtin_any_ok() {
    let errors = check("y = any(true)");
    assert!(errors.is_empty());
}

#[test]
fn test_builtin_alternative_ok() {
    let alt_methods = vec!["min!", "max!", "all!", "any!"];
    for method in alt_methods {
        let code = format!(
            "fn test():\n    if {}(cond):\n        return\n    end\nend",
            method
        );
        let errors = check(&code);
        assert!(errors.is_empty());
    }
}

#[test]
fn test_builtin_math_ok() {
    let errors = check("y = sin(x)");
    assert!(errors.is_empty());
}

#[test]
fn test_builtin_sqrt_ok() {
    let errors = check("y = sqrt(x)");
    assert!(errors.is_empty());
}

#[test]
fn test_builtin_atan2_ok() {
    let errors = check("y = atan2(a, b)");
    assert!(errors.is_empty());
}


#[test]
fn test_builtin_min_wrong_arg_count() {
    let errors = check("y = min(a, b)"); // min prima 1, dato 2
    assert!(has_arg_count_error(&errors, "min", 1, 2));
}

#[test]
fn test_builtin_setbit_ok() {
    let errors = check("y = setbit(x, 3, 1)"); // exactly 3
    assert!(errors.is_empty());
}

#[test]
fn test_builtin_setbit_wrong_count() {
    let errors = check("y = setbit(x, 3)"); // one is missing
    assert!(has_arg_count_error(&errors, "setbit", 3, 2));
}

// ───────────────────────── Undefined ─────────────────────────

#[test]
fn test_undefined_function() {
    let errors = check("y = undefined_func(1, 2)");
    assert!(has_undefined_fn(&errors, "undefined_func"));
}

#[test]
fn test_undefined_function_in_function_body() {
    let errors = check("fn test():\n    y = ghost()\nend");
    assert!(has_undefined_fn(&errors, "ghost"));
}

#[test]
fn test_undefined_function_in_if_condition() {
    let errors = check("fn test():\n    if unknown_fn():\n        return\n    end\nend");
    assert!(has_undefined_fn(&errors, "unknown_fn"));
}

// ───────────────────────── Arg count mismatch ─────────────────────────

#[test]
fn test_too_few_args() {
    let errors = check("fn foo(int a, int b):\n    return\nend\ny = foo(1)");
    assert!(has_arg_count_error(&errors, "foo", 2, 1));
}

#[test]
fn test_too_many_args() {
    let errors = check("fn foo(int a):\n    return\nend\ny = foo(1, 2, 3)");
    assert!(has_arg_count_error(&errors, "foo", 1, 3));
}

#[test]
fn test_no_args_but_expects_some() {
    let errors = check("fn foo(int a, int b):\n    return\nend\ny = foo()");
    assert!(has_arg_count_error(&errors, "foo", 2, 0));
}

#[test]
fn test_args_but_expects_none() {
    let errors = check("fn foo():\n    return\nend\ny = foo(1, 2)");
    assert!(has_arg_count_error(&errors, "foo", 0, 2));
}

#[test]
fn test_arg_count_mismatch_in_expression() {
    let errors = check("fn double(int x):\n    return x * 2\nend\ny = double(1, 2)");
    assert!(has_arg_count_error(&errors, "double", 1, 2));
}

// ───────────────────────── Named args ─────────────────────────

#[test]
fn test_named_arg_not_allowed() {
    let errors = check("fn foo(int a, int b):\n    return\nend\ny = foo(a = 1, b = 2)");
    assert!(has_named_arg_error(&errors, "foo", "a"));
    assert!(has_named_arg_error(&errors, "foo", "b"));
}

#[test]
fn test_named_arg_mixed_with_positional() {
    let errors = check("fn foo(int a, int b):\n    return\nend\ny = foo(1, b = 2)");
    assert!(has_named_arg_error(&errors, "foo", "b"));
}

#[test]
fn test_named_arg_on_builtin() {
    let errors = check("y = min(a = 1, b = 2)");
    assert!(has_named_arg_error(&errors, "min", "a"));
}

// ───────────────────────── Calls inside expressions ─────────────────────────

#[test]
fn test_call_in_tensor_index() {
    let errors = check("fn idx(int x):\n    return x\nend\ny = buf[idx(0)]");
    assert!(errors.is_empty());
}

#[test]
fn test_call_in_for_range() {
    let errors = check("fn limit():\n    return 10\nend\nfn test():\n    for i = 0:limit():\n        return\n    end\nend");
    assert!(errors.is_empty());
}

#[test]
fn test_invalid_call_in_for_range() {
    let errors = check("fn test():\n    for i = 0:undefined_fn():\n        return\n    end\nend");
    assert!(has_undefined_fn(&errors, "undefined_fn"));
}

// ───────────────────────── Entry functions ─────────────────────────

#[test]
fn test_entry_function_call_from_user_code() {
    let errors = check("fn test():\n    output_fnc()\nend");
    assert!(has_entry_fn_error(&errors, "output_fnc"));
}

#[test]
fn test_entry_function_call_init_fnc() {
    let errors = check("fn test():\n    init_fnc()\nend");
    assert!(has_entry_fn_error(&errors, "init_fnc"));
}

#[test]
fn test_entry_function_call_update_fnc() {
    let errors = check("fn test():\n    update_fnc()\nend");
    assert!(has_entry_fn_error(&errors, "update_fnc"));
}

#[test]
fn test_entry_function_call_from_global() {
    let errors = check("output_fnc()");
    assert!(has_entry_fn_error(&errors, "output_fnc"));
}

#[test]
fn test_entry_function_definition_is_valid() {
    // Defining entry functions is valid, only calling them is not
    let errors = check("fn output_fnc():\n    x = 1\nend");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_entry_function_call_from_nested_function() {
    let errors = check(
        "fn helper():\n    output_fnc()\nend\nfn test():\n    helper()\nend"
    );
    assert!(has_entry_fn_error(&errors, "output_fnc"));
}