use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::symbol_table::*;

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
    let scope = Scope::Function("add".to_string());
    let a = get_symbol(&table, "a", &scope);
    let b = get_symbol(&table, "b", &scope);
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
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "x", &scope);
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
    let func_scope = Scope::Function("test".to_string());
    // lookup should find x as global fallback
    assert!(table.lookup("x", &func_scope).is_some());
}

// ───────────────────────── For loop index ─────────────────────────

#[test]
fn test_for_index_in_scope() {
    let (table, errors) = build_table(
        "fn test():\n    for i = 0:10:\n        x = i\n    end\nend"
    );
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "i", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
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

    let func = table.symbols_in_scope(&Scope::Function("test".to_string()));
    assert_eq!(func.len(), 1);
    assert_eq!(func[0].name, "z");
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
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_type_from_param_in_function() {
    let (table, errors) = build_table("fn test(real x):\n    a = x\nend");
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Real));
}

#[test]
fn test_infer_type_from_global_in_function() {
    let (table, errors) = build_table("real g = 1.0\nfn test():\n    a = g\nend");
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
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
    let scope = Scope::Function("main".to_string());
    let sym = get_symbol(&table, "x", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

// ───────────────────────── Tensor indexing type inference ─────────────────────────

#[test]
fn test_infer_type_from_tensor_index() {
    let (table, errors) = build_table(
        "tensor<int, 3> buf = [1, 2, 3]\nfn test():\n    a = buf[0]\nend"
    );
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}

#[test]
fn test_infer_type_from_nested_tensor_index() {
    let (table, errors) = build_table(
        "tensor<tensor<int, 2>, 3> buf = [[1, 2], [3, 4], [5, 6]]\nfn test():\n    a = buf[0]\nend"
    );
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
    assert_eq!(
        sym.ty,
        SymbolType::Tensor(Box::new(SymbolType::Simple(SimpleTypeKind::Int)), vec!["2".to_string()])
    );
}

#[test]
fn test_infer_type_from_double_tensor_index() {
    // buf[i][j] → int
    let (table, errors) = build_table(
        "tensor<tensor<int, 2>, 3> buf = [[1, 2], [3, 4], [5, 6]]\nfn test():\n    a = buf[0][1]\nend"
    );
    assert!(errors.is_empty());
    let scope = Scope::Function("test".to_string());
    let sym = get_symbol(&table, "a", &scope);
    assert_eq!(sym.ty, SymbolType::Simple(SimpleTypeKind::Int));
}