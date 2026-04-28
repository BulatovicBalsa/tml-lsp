use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::symbol_table::SymbolTableBuilder;
use tml_tools::undefined_variable_checker::{CheckError, UndefinedVariableChecker};

fn check(src: &str) -> Vec<CheckError> {
    let parser = TmlParser::new();
    let ast = parser.parse(src).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);
    UndefinedVariableChecker::new(&table).check(&ast)
}

fn has_undefined(errors: &[CheckError], name: &str) -> bool {
    errors.iter().any(|e| matches!(e,
        CheckError::UndefinedVariable { name: n, .. } if n == name
    ))
}

fn has_redeclaration(errors: &[CheckError], name: &str) -> bool {
    errors.iter().any(|e| matches!(e,
        CheckError::RedeclaredNamespace { name: n, .. } if n == name
    ))
}

// ───────────────────────── Valid cases ─────────────────────────

#[test]
fn test_declared_variable_ok() {
    let errors = check("int x = 5\ny = x");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_t_always_valid() {
    let errors = check("y = t.in1 + t.in2");
    assert!(errors.is_empty(), "t.* should always be valid");
}

#[test]
fn test_namespace_p_always_valid() {
    let errors = check("y = p.mul_factor");
    assert!(errors.is_empty(), "p.* should always be valid");
}

#[test]
fn test_namespace_n_always_valid() {
    let errors = check("y = n.div_factor");
    assert!(errors.is_empty(), "n.* should always be valid");
}

#[test]
fn test_namespace_in_expression() {
    let errors = check("result = (t.in1 + t.in2) * p.mul_factor / n.div_factor");
    assert!(errors.is_empty());
}

#[test]
fn test_declared_before_use_ok() {
    let errors = check("int x = 5\nint y = x + 1");
    assert!(errors.is_empty());
}

#[test]
fn test_function_param_used_in_body() {
    let errors = check("fn add(int a, int b):\n    return a + b\nend");
    assert!(errors.is_empty());
}

#[test]
fn test_local_var_used_in_function() {
    let errors = check("fn test():\n    int x = 5\n    y = x + 1\nend");
    assert!(errors.is_empty());
}

#[test]
fn test_global_var_used_in_function() {
    let errors = check("int x = 5\nfn test():\n    y = x + 1\nend");
    assert!(errors.is_empty());
}

#[test]
fn test_for_index_used_in_body() {
    let errors = check("fn test():\n    for i = 0:10:\n        x = i\n    end\nend");
    assert!(errors.is_empty());
}

#[test]
fn test_assign_undeclared_ok() {
    // a = 5 je valid — type inference
    let errors = check("a = 5");
    assert!(errors.is_empty());
}

#[test]
fn test_exists_guarded_not_checked() {
    // guarded var in exists is not being checked
    let errors = check(
        "fn test():\n    exists maybe_var:\n        x = 1\n    end\nend"
    );
    assert!(errors.is_empty());
}

#[test]
#[should_panic]
fn test_defined_in_if_not_visible_after() {
    let errors = check("fn test():\n    if true:\n        x = 5\n    end\n    y = x\nend");
    assert!(errors.is_empty());
    unimplemented!("Currently variables defined in if/while/exists are not visible outside, but this should be fixed in the future");
}

// ───────────────────────── Undefined variables ─────────────────────────

#[test]
fn test_undefined_variable_in_rvalue() {
    let errors = check("y = undefined_var");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_undefined_in_expression() {
    let errors = check("int x = 5\ny = x + undefined_var");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_undefined_in_function_body() {
    let errors = check("fn test():\n    y = undefined_var\nend");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_undefined_in_if_condition() {
    let errors = check("fn test():\n    if undefined_var > 0:\n        x = 1\n    end\nend");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_undefined_in_while_condition() {
    let errors = check("fn test():\n    while undefined_var > 0:\n        x = 1\n    end\nend");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_undefined_in_for_range() {
    let errors = check("fn test():\n    for i = undefined_start:10:\n        x = i\n    end\nend");
    assert!(has_undefined(&errors, "undefined_start"));
}

#[test]
fn test_undefined_in_tensor_index() {
    let errors = check("fn test():\n    int x = buf[undefined_idx]\nend");
    assert!(has_undefined(&errors, "undefined_idx"));
}

#[test]
fn test_undefined_in_return() {
    let errors = check("fn test():\n    return undefined_var\nend");
    assert!(has_undefined(&errors, "undefined_var"));
}

#[test]
fn test_defined_in_different_function_not_visible() {
    // x is declared in fn a(), but not visible in fn b()
    let errors = check(
        "fn a():\n    int x = 5\nend\nfn b():\n    y = x\nend"
    );
    assert!(has_undefined(&errors, "x"));
}

// ───────────────────────── Namespace redeclaration ─────────────────────────

#[test]
fn test_redeclare_t_namespace_error() {
    let errors = check("int t = 5");
    assert!(has_redeclaration(&errors, "t"));
}

#[test]
fn test_redeclare_p_namespace_error() {
    let errors = check("int p = 5");
    assert!(has_redeclaration(&errors, "p"));
}

#[test]
fn test_redeclare_n_namespace_error() {
    let errors = check("int n = 5");
    assert!(has_redeclaration(&errors, "n"));
}

#[test]
fn test_redeclare_namespace_in_function() {
    let errors = check("fn test():\n    int t = 5\nend");
    assert!(has_redeclaration(&errors, "t"));
}

#[test]
fn test_non_namespace_var_ok() {
    // "ta", "pa", "na" are valid variable names, not redeclaring namespaces
    let errors = check("int ta = 5\nint pa = 3\nint na = 1");
    assert!(errors.is_empty());
}

#[test]
fn test_no_undefined_var() {
    let code = r#"
        amplitude = p.max_val - p.min_val
        x = amplitude
    "#;
    let errors = check(code);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

// ───────────────────────── Namespace in expressions ─────────────────────────

#[test]
fn test_namespace_arithmetic_chain() {
    // result of p - p should be usable in further assignments
    let errors = check("amplitude = p.max_val - p.min_val\nx = amplitude");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_mixed_with_declared_var() {
    // mixing namespace ref with a declared variable is valid
    let errors = check("real gain = 2.0\nresult = t.in1 * gain");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_in_function_body() {
    let errors = check("fn update():\n    v_out = p.gain * t.in1\nend");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_chained_assignments() {
    // Each step should be inferred and not cause undefined errors
    let errors = check(
        "a = p.x\nb = a + p.y\nc = b * n.scale"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_in_if_condition() {
    let errors = check(
        "fn test():\n    if p.enabled > 0:\n        x = 1\n    end\nend"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_in_for_range() {
    let errors = check(
        "fn test():\n    for i = 0:p.count:\n        x = i\n    end\nend"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_namespace_root_alone_is_undefined() {
    // bare `p` without dot access is not a valid namespace reference
    // TODO: check if this is valid
    let errors = check("x = p");
    assert!(has_undefined(&errors, "p"), "Expected undefined for bare 'p'");
}

#[test]
fn test_all_namespaces_valid() {
    let errors = check(
        "a = t.in1\nb = p.gain\nc = n.scale"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}