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

fn find_def<'a>(nodes: &'a [HoverableNode], target_name: &str, is_function: bool) -> Vec<&'a HoverableNode> {
    nodes.iter().filter(|n| match &n.kind {
        HoverableKind::FunctionDef { name } if is_function => name == target_name,
        HoverableKind::VariableDecl { name, .. } if !is_function => name == target_name,
        _ => false,
    }).collect()
}

#[test]
fn test_goto_def_function_single() {
    let nodes = collect("fn foo():\n    pass\nend\nfn bar():\n    foo()\nend");
    let defs = find_def(&nodes, "foo", true);
    assert_eq!(defs.len(), 1, "Expected 1 definition for 'foo'");
    assert_eq!(defs[0].position.line, 0);
}

#[test]
fn test_goto_def_duplicate_function_two_results() {
    // Two functions with same name -> goto def should find both
    let src = "fn foo():\n    pass\nend\nfn foo():\n    pass\nend\nfn bar():\n    foo()\nend";
    let nodes = collect(src);
    let defs = find_def(&nodes, "foo", true);
    assert_eq!(defs.len(), 2, "Expected 2 definitions for duplicate 'foo'");
}

#[test]
fn test_goto_def_variable() {
    // int x = 5 -> goto def on x reference should find declaration
    let nodes = collect("fn test():\n    int x = 5\n    a = x\nend");
    let defs = find_def(&nodes, "x", false);
    assert_eq!(defs.len(), 1, "Expected 1 declaration for 'x'");
    assert_eq!(defs[0].position.line, 1);
}

#[test]
fn test_goto_def_param() {
    // parameter x -> goto def on x reference should find param declaration
    let nodes = collect("fn foo(int x):\n    a = x\nend");
    let defs = find_def(&nodes, "x", false);
    assert_eq!(defs.len(), 1, "Expected 1 declaration for param 'x'");
    assert_eq!(defs[0].position.line, 0);
}
