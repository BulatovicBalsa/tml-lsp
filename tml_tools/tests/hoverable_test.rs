use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::hoverable::{HoverableCollector, HoverableKind, HoverableNode};
use tml_tools::symbol_table::{SymbolTable, SymbolTableBuilder};

fn collect_with_table(src: &str) -> (Vec<HoverableNode>, SymbolTable) {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let ast = TmlParser::new().parse(&normalized).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);
    let nodes = HoverableCollector::new().collect(&ast);
    (nodes, table)
}

fn collect(src: &str) -> Vec<HoverableNode> {
    collect_with_table(src).0
}

fn has_function_call(nodes: &[HoverableNode], name: &str) -> bool {
    nodes.iter().any(|n| matches!(&n.kind, HoverableKind::FunctionCall { name: n } if n == name))
}

fn has_function_def(nodes: &[HoverableNode], name: &str) -> bool {
    nodes.iter().any(|n| matches!(&n.kind, HoverableKind::FunctionDef { name: n } if n == name))
}

fn find_at(nodes: &[HoverableNode], line: u32, col: u32) -> Option<&HoverableNode> {
    HoverableCollector::find_at(nodes, line, col)
}

// ───────────────────────── Function definition ─────────────────────────

#[test]
fn test_function_def_registered() {
    let nodes = collect("fn foo():\n    pass\nend");
    assert!(has_function_def(&nodes, "foo"),
        "Expected FunctionDef for 'foo', got: {:?}", nodes);
}

#[test]
fn test_multiple_function_defs_registered() {
    let nodes = collect("fn foo():\n    pass\nend\nfn bar():\n    pass\nend");
    assert!(has_function_def(&nodes, "foo"), "Expected FunctionDef for 'foo'");
    assert!(has_function_def(&nodes, "bar"), "Expected FunctionDef for 'bar'");
}

// ───────────────────────── Function call in expression ─────────────────────────

#[test]
fn test_function_call_in_expression_registered() {
    // a = foo(6) — call in rvalue expression
    let nodes = collect("fn bar():\n    a = foo(6)\nend");
    assert!(has_function_call(&nodes, "foo"),
        "Expected FunctionCall for 'foo' in expression, got: {:?}", nodes);
}

// ───────────────────────── Standalone function call statement ─────────────────────────

#[test]
fn test_standalone_function_call_registered() {
    // foo(6) as standalone statement — FunctionCallStatement
    let nodes = collect("fn bar():\n    foo(6)\nend");
    assert!(has_function_call(&nodes, "foo"),
        "Expected FunctionCall for standalone 'foo(6)', got: {:?}", nodes);
}

#[test]
fn test_standalone_function_call_position() {
    // foo is at col 4, line 1
    let nodes = collect("fn bar():\n    foo(6)\nend");
    let node = nodes.iter().find(|n| matches!(&n.kind, HoverableKind::FunctionCall { name } if name == "foo"))
        .expect("Expected FunctionCall node for 'foo'");
    assert_eq!(node.position.line, 1, "Expected call on line 1");
    assert_eq!(node.position.column, 4, "Expected call at col 4");
}

#[test]
fn test_standalone_call_and_expression_call_both_registered() {
    // Both foo() as statement and bar() in expression should be registered
    let src = "fn test():\n    foo()\n    a = bar()\nend";
    let nodes = collect(src);
    assert!(has_function_call(&nodes, "foo"),
        "Expected FunctionCall for standalone 'foo()', got: {:?}", nodes);
    assert!(has_function_call(&nodes, "bar"),
        "Expected FunctionCall for 'bar()' in expression, got: {:?}", nodes);
}

#[test]
fn test_function_call_count_standalone() {
    let src = "fn test():\n    foo()\n    foo()\nend";
    let nodes = collect(src);
    let count = nodes.iter()
        .filter(|n| matches!(&n.kind, HoverableKind::FunctionCall { name } if name == "foo"))
        .count();
    assert_eq!(count, 2, "Expected 2 FunctionCall nodes for 'foo', got {}", count);
}

// ───────────────────────── find_at ─────────────────────────

#[test]
fn test_find_at_function_def() {
    // "fn foo():" -> fn name at line 0, col 3
    let nodes = collect("fn foo():\n    pass\nend");
    let node = find_at(&nodes, 0, 3).expect("Expected node at (0, 3)");
    assert!(matches!(&node.kind, HoverableKind::FunctionDef { name } if name == "foo"));
}

#[test]
fn test_find_at_returns_none_outside_token() {
    let nodes = collect("fn foo():\n    pass\nend");
    assert!(find_at(&nodes, 0, 20).is_none(),
        "Expected None for cursor outside any token");
}

#[test]
fn test_find_at_function_call() {
    // foo() at line 1, col 4
    let nodes = collect("fn bar():\n    foo()\nend");
    let node = find_at(&nodes, 1, 4).expect("Expected node at (1, 4)");
    assert!(matches!(&node.kind, HoverableKind::FunctionCall { name } if name == "foo"));
}

#[test]
fn test_find_at_variable_ref() {
    // x at line 1, col 8 in "a = x + 1"
    let nodes = collect("fn test():\n    int x = 5\n    a = x\nend");
    let node = find_at(&nodes, 2, 8).expect("Expected variable ref at (2, 8)");
    assert!(matches!(&node.kind, HoverableKind::VariableRef { name } if name == "x"),
        "Expected VariableRef for 'x', got: {:?}", node.kind);
}

// ───────────────────────── Hover content ─────────────────────────

#[test]
fn test_hover_function_def_shows_signature() {
    let (nodes, table) = collect_with_table("fn foo(int x) int:\n    pass\nend");
    let node = find_at(&nodes, 0, 3).expect("Expected node at fn name");
    let content = node.hover_content(&table).expect("Expected hover content");
    assert!(content.contains("foo"), "Hover should contain function name");
    assert!(content.contains("int"), "Hover should contain param type");
}

#[test]
fn test_hover_function_call_shows_signature() {
    let (nodes, table) = collect_with_table(
        "fn foo(int x) int:\n    pass\nend\nfn bar():\n    foo(5)\nend"
    );
    let node = find_at(&nodes, 4, 4).expect("Expected node at foo call");
    let content = node.hover_content(&table).expect("Expected hover content");
    assert!(content.contains("foo"), "Hover should contain function name");
}

#[test]
fn test_hover_variable_decl_shows_type() {
    let (nodes, table) = collect_with_table("fn test():\n    int x = 5\nend");
    // int x at line 1 — decl node
    let node = find_at(&nodes, 1, 8).expect("Expected node at 'x'");
    let content = node.hover_content(&table);
    assert!(content.is_some(), "Expected hover content for variable decl");
    let content = content.unwrap();
    assert!(content.contains("x"), "Hover should contain variable name");
}

#[test]
fn test_hover_unknown_function_returns_none() {
    let (nodes, table) = collect_with_table("fn bar():\n    foo()\nend");
    let node = find_at(&nodes, 1, 4).expect("Expected node at foo call");
    // foo is not defined, hover_content returns None
    assert!(node.hover_content(&table).is_none(),
        "Expected None hover for undefined function");
}
