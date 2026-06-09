use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::symbol_table::SymbolTableBuilder;
use tml_tools::checkers::undefined_variable::{CheckError, UndefinedVariableChecker};

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
    let errors = check("a = 5");
    assert!(errors.is_empty());
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

// ───────────────────────── Namespace assignment ─────────────────────────

#[test]
fn test_bare_namespace_assignment_is_error() {
    // "p = 5" assigns to bare namespace root — should be an error
    let errors = check("fn test():\n    p = 5\nend");
    assert!(has_redeclaration(&errors, "p"),
        "Expected redeclaration error for 'p = 5', got: {:?}", errors);
}

#[test]
fn test_namespace_dot_access_assignment_is_ok() {
    // "p.x = 5" assigns to a property, not the namespace root — should be valid
    let errors = check("fn test():\n    p.x = 5\nend");
    let has_error = errors.iter().any(|e| matches!(e,
        CheckError::RedeclaredNamespace { name, .. } if name.starts_with('p')
    ));
    assert!(!has_error,
        "'p.x = 5' should not trigger redeclaration error, got: {:?}", errors);
}

#[test]
fn test_bare_t_assignment_is_error() {
    let errors = check("fn test():\n    t = 5\nend");
    assert!(has_redeclaration(&errors, "t"),
        "Expected redeclaration error for 't = 5', got: {:?}", errors);
}

#[test]
fn test_bare_n_assignment_is_error() {
    let errors = check("fn test():\n    n = 5\nend");
    assert!(has_redeclaration(&errors, "n"),
        "Expected redeclaration error for 'n = 5', got: {:?}", errors);
}

#[test]
fn test_namespace_deep_dot_access_assignment_is_ok() {
    let errors = check("fn test():\n    t.s_ctrl.length = 5\nend");
    let has_error = errors.iter().any(|e| matches!(e,
        CheckError::RedeclaredNamespace { .. }
    ));
    assert!(!has_error,
        "Deep namespace dot access should not trigger redeclaration error, got: {:?}", errors);
}

// ───────────────────────── Block scope ─────────────────────────

#[test]
fn test_variable_defined_in_if_not_visible_after() {
    let errors = check("fn test():\n    if true:\n        int x = 5\n    end\n    y = x\nend");
    assert!(has_undefined(&errors, "x"),
        "Expected undefined for 'x' used outside if block, got: {:?}", errors);
}

#[test]
fn test_variable_defined_in_if_visible_inside() {
    let errors = check("fn test():\n    if true:\n        int x = 5\n        y = x\n    end\nend");
    assert!(!has_undefined(&errors, "x"),
        "'x' should be visible inside if block, got: {:?}", errors);
}

#[test]
fn test_variable_defined_in_for_not_visible_after() {
    let errors = check("fn test():\n    for i = 0:5:\n        int x = i\n    end\n    y = x\nend");
    assert!(has_undefined(&errors, "x"),
        "Expected undefined for 'x' used outside for block, got: {:?}", errors);
}

#[test]
fn test_for_index_not_visible_after_loop() {
    let errors = check("fn test():\n    for i = 0:5:\n        pass\n    end\n    y = i\nend");
    assert!(has_undefined(&errors, "i"),
        "Expected undefined for 'i' used outside for block, got: {:?}", errors);
}

// ───────────────────────── Macro for ─────────────────────────

#[test]
fn test_macro_for_index_visible_inside() {
    // macro for index should be visible inside the loop body
    let errors = check("macro for i = 0:5:\n    x = i\nend");
    assert!(!has_undefined(&errors, "i"),
        "'i' should be visible inside macro for body, got: {:?}", errors);
}

#[test]
fn test_macro_for_index_not_visible_after() {
    // macro for index should not be visible after the loop
    let errors = check("fn test():\n    macro for i = 0:5:\n        pass\n    end\n    y = i\nend");
    assert!(has_undefined(&errors, "i"),
        "Expected undefined for 'i' used outside macro for block, got: {:?}", errors);
}

#[test]
fn test_macro_for_namespace_address_is_valid() {
    // n.x_rows in address expression should not trigger undefined error
    let errors = check("macro for i = 0:n.x_rows:\n    x = i\nend");
    assert!(!has_undefined(&errors, "i"),
        "'i' should be visible inside macro for, got: {:?}", errors);
    assert!(!has_undefined(&errors, "n"),
        "namespace 'n' should be valid, got: {:?}", errors);
}

#[test]
fn test_variable_defined_in_while_not_visible_after() {
    let errors = check("fn test():\n    int x = 1\n    while x > 0:\n        int y = x\n        x = x - 1\n    end\n    z = y\nend");
    assert!(has_undefined(&errors, "y"),
        "Expected undefined for 'y' used outside while block, got: {:?}", errors);
}

#[test]
fn test_outer_variable_visible_inside_block() {
    let errors = check("fn test():\n    int x = 5\n    if true:\n        y = x\n    end\nend");
    assert!(!has_undefined(&errors, "x"),
        "'x' defined in function scope should be visible inside if block, got: {:?}", errors);
}

#[test]
fn test_variable_in_sibling_elseif_not_visible() {
    let errors = check(concat!(
        "fn test():\n",
        "    if true:\n",
        "        int x = 5\n",
        "    elseif false:\n",
        "        y = x\n",
        "    end\n",
        "end"
    ));
    assert!(has_undefined(&errors, "x"),
        "Expected undefined for 'x' in sibling elseif block, got: {:?}", errors);
}

#[test]
fn test_variable_defined_before_if_visible_inside() {
    let errors = check(concat!(
        "fn test():\n",
        "    int x = 5\n",
        "    if true:\n",
        "        y = x + 1\n",
        "    end\n",
        "end"
    ));
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_tensor_index_undefined_variable() {
    let errors = check("fn test():\n    int x = 5\n    x[b] = 3\nend");
    assert!(has_undefined(&errors, "b"),
            "Expected undefined for 'b' in tensor index, got: {:?}", errors);
}

#[test]
fn test_tensor_index_defined_variable_ok() {
    let errors = check("fn test():\n    int x = 5\n    int b = 0\n    x[b] = 3\nend");
    assert!(!has_undefined(&errors, "b"),
            "'b' should be visible as tensor index, got: {:?}", errors);
}

#[test]
fn test_predefined_variable_in_expression() {
    let errors = check("result = M_PI * 2");
    assert!(errors.is_empty(), "Predefined literal 'M_PI' should not be undefined, got: {:?}", errors);
}

#[test]
fn test_predefined_variable_declaration() {
    let errors = check("M_PI = 3.14");
    assert!(has_redeclaration(&errors, "M_PI"),
        "Expected redeclaration error for 'M_PI', got: {:?}", errors);
}

#[test]
fn test_predefined_variable_used_in_function() {
    let errors = check("fn test():\n    result = M_E * 2\nend");
    assert!(errors.is_empty(), "Predefined literal 'M_E' should be valid inside function, got: {:?}", errors);
}

#[test]
fn test_predefined_variable_declaration_with_type() {
    let errors = check("int M_E = 3");
    assert!(has_redeclaration(&errors, "M_E"),
        "Expected redeclaration error for 'M_E', got: {:?}", errors);
}

// ───────────────────────── Macro if scope ─────────────────────────

#[test]
fn test_macro_if_variable_visible_after() {
    // macro if is compile-time — x defined inside is visible outside
    let errors = check("macro if true:\n    x = 5\nend\ny = x");
    assert!(!has_undefined(&errors, "x"),
        "'x' defined inside macro if should be visible after, got: {:?}", errors);
}

#[test]
fn test_macro_if_variable_visible_in_body() {
    let errors = check("macro if true:\n    x = 5\n    y = x\nend");
    assert!(!has_undefined(&errors, "x"),
        "'x' should be visible within macro if body, got: {:?}", errors);
}

#[test]
fn test_macro_if_else_variable_visible_after() {
    // variable defined in either branch is visible after
    let errors = check("macro if true:\n    x = 5\nelse:\n    x = 0\nend\ny = x");
    assert!(!has_undefined(&errors, "x"),
        "'x' defined in macro if/else should be visible after, got: {:?}", errors);
}

#[test]
fn test_macro_for_variable_visible_after() {
    // macro for is compile-time — x defined inside is visible outside
    let errors = check("macro for i = 0:3:\n    x = i\nend\ny = x");
    assert!(!has_undefined(&errors, "x"),
        "'x' defined inside macro for should be visible after, got: {:?}", errors);
}

#[test]
fn test_macro_if_else_declaration_visible_from_outer_scope() {
    // variable defined in macro if should be visible in outer scope
    let errors = check("macro if true:\n    int b = 5\nelse:\n    int x = 0\nend\ny = x");
    assert!(!has_undefined(&errors, "x"),
        "'x' defined in macro if/else should be visible after, got: {:?}", errors);
}

#[test]
fn test_macro_if_elseif_variable_visible_after() {
    // variable defined in any branch should be visible after
    let errors = check("macro if true:\n    x = 5\nelseif false:\n    y = 3\nelse:\n    z = 0\nend\nf = y");
    assert!(!has_undefined(&errors, "y"),
        "'y' defined in macro elseif should be visible after, got: {:?}", errors);
}

#[test]
fn test_predefined_literal_used_in_expression() {
    let errors = check("conversion_constant = M_PI / 3\nphase_state = conversion_constant");
    assert!(errors.is_empty(), "Predefined literal 'M_PI' should be valid, got: {:?}", errors);
}

#[test]
fn test_predefined_literal_used_in_expression_with_declared_var() {
    let errors = check("real conversion_constant = M_PI / 3\nphase_state = conversion_constant");
    assert!(errors.is_empty(), "Predefined literal 'M_PI' should be valid in expression with declared var, got: {:?}", errors);
}