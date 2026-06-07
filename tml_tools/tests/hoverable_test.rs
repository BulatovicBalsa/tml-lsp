use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::hoverable::{HoverableCollector, HoverableKind};

fn collect(src: &str) -> Vec<tml_tools::collectors::hoverable::HoverableNode> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let ast = TmlParser::new().parse(&normalized).expect("Parse failed");
    HoverableCollector::new().collect(&ast)
}

fn has_function_call(nodes: &[tml_tools::collectors::hoverable::HoverableNode], name: &str) -> bool {
    nodes.iter().any(|n| matches!(&n.kind, HoverableKind::FunctionCall { name: n } if n == name))
}

fn has_function_def(nodes: &[tml_tools::collectors::hoverable::HoverableNode], name: &str) -> bool {
    nodes.iter().any(|n| matches!(&n.kind, HoverableKind::FunctionDef { name: n } if n == name))
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
    // foo() called twice as standalone — should appear twice
    let src = "fn test():\n    foo()\n    foo()\nend";
    let nodes = collect(src);
    let count = nodes.iter()
        .filter(|n| matches!(&n.kind, HoverableKind::FunctionCall { name } if name == "foo"))
        .count();
    assert_eq!(count, 2, "Expected 2 FunctionCall nodes for 'foo', got {}", count);
}
