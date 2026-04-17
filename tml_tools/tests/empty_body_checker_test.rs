use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticsRunner};
use tml_tools::empty_body_checker::EmptyBodyDiagnosticSource;
use tml_tools::symbol_table::SymbolTableBuilder;

fn run(src: &str) -> Vec<Diagnostic> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let parser = TmlParser::new();
    let ast = parser.parse(&normalized).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);
    DiagnosticsRunner::new()
        .add_source(EmptyBodyDiagnosticSource)
        .run(&ast, &table)
}

fn has_error(diags: &[Diagnostic], msg: &str) -> bool {
    diags.iter().any(|d| d.severity == DiagnosticSeverity::Error && d.message.contains(msg))
}

// ───────────────────────── No warning when body has content ─────────────────────────

#[test]
fn test_no_warning_for_non_empty_function() {
    let diags = run("fn foo():\n    x = 1\nend");
    assert!(diags.is_empty(), "{:?}", diags);
}

#[test]
fn test_no_warning_when_pass_present() {
    // `pass` counts as content — body is not empty
    let diags = run("fn foo():\n    pass\nend");
    assert!(diags.is_empty(), "{:?}", diags);
}

#[test]
fn test_no_warning_for_non_empty_if() {
    let diags = run("fn foo():\n    if true:\n        x = 1\n    end\nend");
    assert!(diags.is_empty(), "{:?}", diags);
}

#[test]
fn test_no_warning_for_non_empty_for() {
    let diags = run("fn foo():\n    for i = 0:10:\n        x = i\n    end\nend");
    assert!(diags.is_empty(), "{:?}", diags);
}

#[test]
fn test_no_warning_for_non_empty_while() {
    let diags = run("fn foo():\n    int x = 0\n    while x > 0:\n        x = x - 1\n    end\nend");
    assert!(diags.is_empty(), "{:?}", diags);
}

// ───────────────────────── Warning for empty function ─────────────────────────

#[test]
fn test_warning_for_empty_function() {
    let diags = run("fn foo():\nend");
    assert!(has_error(&diags, "foo"), "{:?}", diags);
    assert!(has_error(&diags, "empty body"), "{:?}", diags);
}

#[test]
fn test_warning_includes_function_name() {
    let diags = run("fn my_function():\nend");
    assert!(has_error(&diags, "my_function"), "{:?}", diags);
}

// ───────────────────────── Warning for empty if/elseif/else ─────────────────────────

#[test]
fn test_warning_for_empty_if_body() {
    let diags = run("fn foo():\n    if true:\n    end\nend");
    assert!(has_error(&diags, "'if' body"), "{:?}", diags);
}

#[test]
fn test_warning_for_empty_else_body() {
    let diags = run("fn foo():\n    if true:\n        x = 1\n    else:\n    end\nend");
    assert!(has_error(&diags, "'else' body"), "{:?}", diags);
}

// ───────────────────────── Warning for empty for/while ─────────────────────────

#[test]
fn test_warning_for_empty_for_body() {
    let diags = run("fn foo():\n    for i = 0:10:\n    end\nend");
    assert!(has_error(&diags, "'for"), "{:?}", diags);
}

#[test]
fn test_warning_for_empty_while_body() {
    let diags = run("fn foo():\n    int x = 1\n    while x > 0:\n    end\nend");
    assert!(has_error(&diags, "'while' body"), "{:?}", diags);
}

// ───────────────────────── Severity is Error ─────────────────────────

#[test]
fn test_severity_is_error() {
    let diags = run("fn foo():\nend");
    assert!(diags.iter().all(|d| d.severity == DiagnosticSeverity::Error));
}
