use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::constants::{PREDEFINED_LITERALS, PREDEFINED_LITERAL_TYPES};
use tml_tools::symbol_table::*;
use tml_tools::types::{Scope, SimpleTypeKind, Symbol, SymbolError, SymbolType};

fn build_table(src: &str) -> (SymbolTable, Vec<SymbolError>) {
    let parser = TmlParser::new();
    let ast = parser.parse(src).expect("Parse failed");
    SymbolTableBuilder::new().build(&ast)
}

fn get_symbol<'a>(table: &'a SymbolTable, name: &str, scope: &Scope) -> &'a Symbol {
    table.lookup(name, scope).unwrap_or_else(|| {
        panic!("Symbol '{}' not found in scope {:?}", name, scope)
    })
}

/// Find symbol in any Function scope with the given function name, ignoring ID.
fn get_symbol_in_fn<'a>(table: &'a SymbolTable, sym_name: &str, fn_name: &str) -> &'a Symbol {
    table.symbols.iter()
        .find(|s| s.name == sym_name && matches!(&s.scope, Scope::Function { name, .. } if name == fn_name))
        .unwrap_or_else(|| panic!("Symbol '{}' not found in function '{}'", sym_name, fn_name))
}

/// Returns true if the scope is a local scope (not global or transparent).
fn in_local_scope(scope: &Scope) -> bool {
    !matches!(scope, Scope::Global | Scope::TransparentBlock | Scope::MacroIndexBlock { .. })
}

// ───────────────────────── Global declarations ─────────────────────────

#[test]
fn test_global_int_declaration() {
    let (table, errors) = build_table("int x = 5");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "x", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_global_bool_declaration() {
    let (table, errors) = build_table("bool a = false");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Bool));
}

#[test]
fn test_global_real_declaration() {
    let (table, errors) = build_table("real pi = 3.14");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "pi", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_multiple_global_declarations() {
    let (table, errors) = build_table("int x = 1\nreal y = 2.0\nbool z = true");
    assert!(errors.is_empty());
    assert_eq!(table.symbols.len(), 3);
    assert_eq!(get_symbol(&table, "x", &Scope::Global).ty, SymbolType::Simple(SimpleTypeKind::Int));
    assert_eq!(get_symbol(&table, "y", &Scope::Global).ty, SymbolType::Simple(SimpleTypeKind::Real));
    assert_eq!(get_symbol(&table, "z", &Scope::Global).ty, SymbolType::Simple(SimpleTypeKind::Bool));
}

// ───────────────────────── Derived type ─────────────────────────

#[test]
fn test_derived_from_terminal() {
    let (table, errors) = build_table("t.in.type a = 1");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Derived("t.in".to_string()));
}

#[test]
fn test_derived_from_variable() {
    let (table, errors) = build_table("int x = 1\na.type b = 2");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "b", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Derived("a".to_string()));
}

#[test]
fn test_derived_with_brackets() {
    let (table, errors) = build_table("t.in_operand[].type c = 3");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "c", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Derived("t.in_operand[]".to_string()));
}

// ───────────────────────── Tensor type ─────────────────────────

#[test]
fn test_tensor_simple() {
    let (table, errors) = build_table("tensor<int, 3> a = [1, 2, 3]");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(
            Box::new(SymbolType::Simple(SimpleTypeKind::Int)),
            vec!["3".to_string()]
        )
    );
}

#[test]
fn test_tensor_multidimensional() {
    let (table, errors) = build_table("tensor<int, 2, 2, 2> a = [1, 2; 3, 4 | 1, 2; 3, 4]");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(
            Box::new(SymbolType::Simple(SimpleTypeKind::Int)),
            vec!["2".to_string(), "2".to_string(), "2".to_string()]
        )
    );
}

#[test]
fn test_tensor_of_tensors() {
    let (table, errors) = build_table("tensor<tensor<int, 2>, 2> a = [[1, 2], [3, 4]]");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(
            Box::new(
                SymbolType::Tensor(
                    Box::new(SymbolType::Simple(SimpleTypeKind::Int)),
                    vec!["2".to_string()]
            )),
            vec!["2".to_string()]
        )
    );
}

// ───────────────────────── Function scope ─────────────────────────

#[test]
fn test_function_registered() {
    let (table, errors) = build_table("fn test():\n    x = 1\nend");
    assert!(errors.is_empty());
    assert!(table.lookup_function("test").is_some());
}

#[test]
fn test_function_params_in_scope() {
    let (table, errors) = build_table("fn add(int a, int b):\n    return a + b\nend");
    assert!(errors.is_empty());
    let a = get_symbol_in_fn(&table, "a", "add");
    let b = get_symbol_in_fn(&table, "b", "add");
    assert_eq!(a.ty, SymbolType::Simple(SimpleTypeKind::Int));
    assert_eq!(b.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_function_params_not_in_global_scope() {
    let (table, _) = build_table("fn add(int a, int b):\n    return a + b\nend");
    assert!(table.lookup("a", &Scope::Global).is_none());
    assert!(table.lookup("b", &Scope::Global).is_none());
}

#[test]
fn test_local_declaration_in_function() {
    let (table, errors) = build_table("fn test():\n    int x = 5\nend");
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "x" && in_local_scope(&s.scope))
        .expect("Expected 'x' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_function_signature_params() {
    let (table, _) = build_table("fn add(int a, real b):\n    return a\nend");
    let func = table.lookup_function("add").unwrap();
    assert_eq!(func.params.len(), 2);
    assert_eq!(func.params[0], (SymbolType::Simple(SimpleTypeKind::Int), "a".to_string()));
    assert_eq!(func.params[1], (SymbolType::Simple(SimpleTypeKind::Real), "b".to_string()));
}

#[test]
fn test_global_visible_in_function_scope() {
    let (table, _) = build_table("int x = 5\nfn test():\n    y = x\nend");
    // x should be found via global fallback when looking inside function scope
    assert!(table.symbols.iter().any(|s| s.name == "x" && s.scope == Scope::Global));
}

// ───────────────────────── For loop index ─────────────────────────

#[test]
fn test_for_index_in_scope() {
    let (table, errors) = build_table(
        "fn test():\n    for i = 0:10:\n        x = i\n    end\nend"
    );
    assert!(errors.is_empty());
    // i should be in some block scope inside function test
    let found = table.symbols.iter().any(|s| s.name == "i" && in_local_scope(&s.scope));
    assert!(found, "Expected 'i' in a block scope, got: {:?}", table.symbols);
}

// ───────────────────────── Duplicate detection ─────────────────────────

#[test]
fn test_duplicate_global() {
    let (_, errors) = build_table("int x = 1\nint x = 2");
    assert!(!errors.is_empty());
    assert_eq!(errors[0].symbol_name, "x");
}

#[test]
fn test_duplicate_in_function() {
    let (_, errors) = build_table("fn test():\n    int x = 1\n    int x = 2\nend");
    assert!(!errors.is_empty());
    assert_eq!(errors[0].symbol_name, "x");
}

#[test]
fn test_same_name_different_scopes_ok() {
    // This should be allowed since the global x and function-local x are in different scopes
    let (_, errors) = build_table("int x = 1\nfn test():\n    int x = 2\nend");
    assert!(errors.is_empty());
}

// ───────────────────────── symbols_in_scope ─────────────────────────

#[test]
fn test_symbols_in_scope() {
    let (table, _) = build_table(
        "int x = 1\nreal y = 2.0\nfn test():\n    bool z = true\nend"
    );
    let global = table.symbols_in_scope(&Scope::Global);
    assert_eq!(global.len(), 2);

    let fn_symbols: Vec<_> = table.symbols.iter()
        .filter(|s| in_local_scope(&s.scope))
        .collect();
    assert_eq!(fn_symbols.len(), 1);
    assert_eq!(fn_symbols[0].name, "z");
}

// ───────────────────────── Type inference from constants ─────────────────────────

#[test]
fn test_infer_int_from_assignment() {
    let (table, errors) = build_table("a = 5");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_real_from_assignment() {
    let (table, errors) = build_table("b = 6.2");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "b", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_infer_bool_from_assignment() {
    let (table, errors) = build_table("c = true");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "c", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Bool));
}

#[test]
fn test_infer_str_from_assignment() {
    let (table, errors) = build_table(r#"d = "hello""#);
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "d", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Str));
}

#[test]
fn test_infer_uint_from_assignment() {
    let (table, errors) = build_table("e = 5u");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "e", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Uint));
}

// ───────────────────────── Type inference from expressions ─────────────────────────

#[test]
fn test_infer_type_from_unary_minus() {
    let (table, errors) = build_table("a = -5");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_bool_from_logical_expression() {
    let (table, errors) = build_table("int x = 1\na = x > 0");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Bool));
}

#[test]
fn test_infer_bool_from_not() {
    let (table, errors) = build_table("bool x = true\na = not x");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Bool));
}

// ───────────────────────── Type promotion ─────────────────────────

#[test]
fn test_promote_int_plus_real() {
    let (table, errors) = build_table("int x = 1\nreal y = 2.0\na = x + y");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_promote_uint_plus_int() {
    let (table, errors) = build_table("uint x = 1u\nint y = 2\na = x + y");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_promote_uint_plus_uint() {
    let (table, errors) = build_table("uint x = 1u\nuint y = 2u\na = x + y");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Uint));
}

#[test]
fn test_promote_int_plus_real_chain() {
    let (table, errors) = build_table("int x = 1\nint y = 2\nreal z = 3.0\na = x + y + z");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

// ───────────────────────── Type inference from variable reference ─────────────────────────

#[test]
fn test_infer_type_from_variable() {
    let (table, errors) = build_table("int x = 5\na = x");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_type_from_real_variable() {
    let (table, errors) = build_table("real x = 5.0\na = x");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_infer_type_chain() {
    let (table, errors) = build_table("b = 5\na = b");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

// ───────────────────────── Type inference in function scope ─────────────────────────

#[test]
fn test_infer_type_in_function() {
    let (table, errors) = build_table("fn test():\n    a = 5\nend");
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_type_from_param_in_function() {
    let (table, errors) = build_table("fn test(real x):\n    a = x\nend");
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_infer_type_from_global_in_function() {
    let (table, errors) = build_table("real g = 1.0\nfn test():\n    a = g\nend");
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

// ───────────────────────── No duplicate on reassignment ─────────────────────────

#[test]
fn test_no_duplicate_on_reassignment() {
    // Reassigning an already declared variable should not create a duplicate
    let (table, errors) = build_table("int x = 5\nx = 10");
    assert!(errors.is_empty());
    let count = table.symbols.iter().filter(|s| s.name == "x").count();
    assert_eq!(count, 1, "Expected only one symbol 'x', got {}", count);
}

#[test]
fn test_function_forward_reference() {
    let (table, errors) = build_table(
        "fn main():\n    x = foo()\nend\nfn foo() int:\n    return 5\nend"
    );
    assert!(errors.is_empty());
    // x is in a block scope inside main
    let sym = table.symbols.iter()
        .find(|s| s.name == "x" && in_local_scope(&s.scope))
        .unwrap_or_else(|| panic!("Symbol 'x' not found in local scope, symbols: {:?}", table.symbols));
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

// ───────────────────────── Tensor indexing type inference ─────────────────────────

#[test]
fn test_infer_type_from_tensor_index() {
    let (table, errors) = build_table(
        "tensor<int, 3> buf = [1, 2, 3]\nfn test():\n    a = buf[0]\nend"
    );
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_type_from_nested_tensor_index() {
    let (table, errors) = build_table(
        "tensor<tensor<int, 2>, 3> buf = [[1, 2], [3, 4], [5, 6]]\nfn test():\n    a = buf[0]\nend"
    );
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(Box::new(SymbolType::Simple(SimpleTypeKind::Int)), vec!["2".to_string()])
    );
}

#[test]
fn test_infer_type_from_double_tensor_index() {
    let (table, errors) = build_table(
        "tensor<tensor<int, 2>, 3> buf = [[1, 2], [3, 4], [5, 6]]\nfn test():\n    a = buf[0][1]\nend"
    );
    assert!(errors.is_empty());
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

// ───────────────────────── Namespace type inference ─────────────────────────

#[test]
fn test_namespace_ref_inferred_as_derived() {
    // p.x is inferred as Derived("p.x")
    let (table, errors) = build_table("a = p.x");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Derived("p.x".to_string()));
}

#[test]
fn test_namespace_arithmetic_result_added_to_table() {
    // amplitude = p.max - p.min should produce a symbol in the table
    let (table, errors) = build_table("amplitude = p.max_val - p.min_val");
    assert!(errors.is_empty());
    assert!(table.lookup("amplitude", &Scope::Global).is_some());
}

#[test]
fn test_namespace_then_used_in_next_assignment() {
    // symbol built from namespace ref is usable immediately after
    let (table, errors) = build_table(
        "amplitude = p.max_val - p.min_val\nx = amplitude"
    );
    assert!(errors.is_empty());
    assert!(table.lookup("x", &Scope::Global).is_some());
}

#[test]
fn test_namespace_mixed_with_concrete_type_promotes() {
    // real gain + p.x (Derived) should produce real (concrete wins over Derived)
    let (table, errors) = build_table("real gain = 2.0\nresult = gain + p.offset");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "result", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_namespace_mixed_with_int_promotes() {
    let (table, errors) = build_table("int x = 1\nresult = x + p.offset");
    assert!(errors.is_empty());
    let sym = get_symbol(&table, "result", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[rstest]
#[case::ns_t("t")]
#[case::ns_p("p")]
#[case::ns_n("n")]
fn test_all_namespaces_inferred_as_derived(#[case] ns: &str) {
    let (table, errors) = build_table(
        &format!("result = {}.offset", ns)
    );
    assert!(errors.is_empty());
    assert!(matches!(get_symbol(&table, "result", &Scope::Global).ty, SymbolType::Derived(_)));
}

#[test]
fn test_namespace_in_function_scope() {
    let (table, errors) = build_table(
        "fn update():\n    v_out = p.gain * t.in1\nend"
    );
    assert!(errors.is_empty());
    let found = table.symbols.iter().any(|s| s.name == "v_out" && in_local_scope(&s.scope));
    assert!(found, "Expected 'v_out' in block scope, got: {:?}", table.symbols);
}

#[test]
fn test_namespace_bare_root_not_inferred() {
    let (table, _) = build_table("x = p");
    assert!(table.lookup("x", &Scope::Global).is_none(),
        "bare 'p' should not be inferred as Derived");
}

// ───────────────────────── Predefined literal type inference ─────────────────────────
#[test]
fn test_all_predefined_literals_have_type_mapping() {
    for name in PREDEFINED_LITERALS {
        let has_mapping = PREDEFINED_LITERAL_TYPES.iter().any(|(lit, _)| lit == name);
        assert!(has_mapping,
            "Predefined literal '{}' has no type mapping in PREDEFINED_LITERAL_TYPES", name);
    }
}

#[rstest]
#[case("M_PI", SymbolType::Simple(SimpleTypeKind::Real))]
#[case("M_E",  SymbolType::Simple(SimpleTypeKind::Real))]
#[case("inf",  SymbolType::Simple(SimpleTypeKind::Real))]
fn test_infer_type_from_predefined_literal(
    #[case] literal: &str,
    #[case] expected: SymbolType,
) {
    let (table, errors) = build_table(&format!("a = {}", literal));
    assert!(errors.is_empty(),
        "Unexpected errors for literal '{}': {:?}", literal, errors);
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, expected,
        "Expected {:?} inferred from '{}', got: {:?}", expected, literal, sym.ty);
}

#[test]
fn test_predefined_literal_in_expression_promotes_to_real() {
    // int + M_PI -> real (real wins in promotion)
    let (table, errors) = build_table("int x = 5\na = x + M_PI");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = get_symbol(&table, "a", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real),
        "Expected Real when mixing int with M_PI, got: {:?}", sym.ty);
}

#[test]
fn test_sign_scalar_returns_int() {
    let (table, errors) = build_table("fn test():\n    a = sign(-5.0)\nend");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int),
        "sign() on scalar should return Int, got: {:?}", sym.ty);
}

#[test]
fn test_sign_int_returns_int() {
    let (table, errors) = build_table("fn test():\n    int x = 5\n    a = sign(x)\nend");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int),
        "sign() on int should return Int, got: {:?}", sym.ty);
}

#[test]
fn test_sign_tensor_returns_tensor_of_int() {
    let (table, errors) = build_table(
        "tensor<real, 3> buf = [1.0, -2.0, 0.0]\nfn test():\n    a = sign(buf)\nend"
    );
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(Box::new(SymbolType::Simple(SimpleTypeKind::Int)), vec!["3".to_string()]),
        "sign() on tensor<real, 3> should return tensor<int, 3>, got: {:?}", sym.ty
    );
}

#[test]
fn test_predefined_literal_in_function_body() {
    let (table, errors) = build_table("fn test():\n    a = M_PI * 2\nend");
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = table.symbols.iter()
        .find(|s| s.name == "a" && in_local_scope(&s.scope))
        .expect("Expected 'a' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real),
        "Expected Real from M_PI * 2, got: {:?}", sym.ty);
}

// ───────────────────────── IO read type inference ─────────────────────────

#[test]
fn test_io_read_infers_type_from_declaration() {
    // link_up declared as uint, reading it via <- should infer uint
    let src = "in<uint, 0x0> link_up\nlink_up_val = <-link_up";
    let (table, errors) = build_table(src);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = get_symbol(&table, "link_up_val", &Scope::Global);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Uint),
        "Expected Uint inferred from IO read of 'link_up', got: {:?}", sym.ty);
}

#[test]
fn test_io_read_result_usable_in_next_assignment() {
    // link_up_val should be visible after IO read assignment
    let src = "in<uint, 0x0> link_up\nlink_up_val = <-link_up\nx = link_up_val";
    let (table, errors) = build_table(src);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    assert!(table.lookup("x", &Scope::Global).is_some(),
        "Expected 'x' in table after using IO read result");
}

#[test]
fn test_io_read_in_function_infers_type() {
    let src = concat!(
        "in<int, 0x0> sensor\n",
        "fn test():\n",
        "    val = <-sensor\n",
        "end"
    );
    let (table, errors) = build_table(src);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    let sym = table.symbols.iter()
        .find(|s| s.name == "val" && in_local_scope(&s.scope))
        .expect("Expected 'val' in local scope");
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int),
        "Expected Int inferred from IO read of 'sensor', got: {:?}", sym.ty);
}

#[test]
fn test_duplicate_function_definition() {
    let (_, errors) = build_table("fn foo():\n    pass\nend\nfn foo():\n    pass\nend");
    assert!(!errors.is_empty(), "Expected error for duplicate function 'foo'");
    assert!(errors.iter().any(|e| e.symbol_name == "foo"));
}

#[test]
fn test_two_functions_same_name_params_isolated() {
    // Variables from first fn should not be visible in second fn
    let src = concat!(
        "fn name(real x, int n) int:\n",
        "    x = 5\n",
        "end\n",
        "fn name():\n",
        "    y = x\n", // x is NOT defined in this scope
        "end"
    );
    let (table, _) = build_table(src);
    // y should not be in table since x is not visible in second fn
    let y_in_second = table.symbols.iter().any(|s| s.name == "y");
    assert!(!y_in_second,
        "'y = x' in second fn should not produce symbol since x is not visible");
}
