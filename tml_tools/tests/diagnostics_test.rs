use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticsRunner};
use tml_tools::function_call_checker::FunctionCallDiagnosticSource;
use tml_tools::symbol_table::SymbolTableBuilder;
use tml_tools::undefined_variable_checker::UndefinedVariableDiagnosticSource;

fn run_diagnostics(src: &str) -> Vec<Diagnostic> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let parser = TmlParser::new();
    let ast = parser.parse(&normalized).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);

    DiagnosticsRunner::new()
        .add_source(UndefinedVariableDiagnosticSource)
        .add_source(FunctionCallDiagnosticSource)
        .run(&ast, &table)
}

fn has_error(diagnostics: &[Diagnostic], msg: &str) -> bool {
    diagnostics.iter().any(|d| {
        d.severity == DiagnosticSeverity::Error && d.message.contains(msg)
    })
}

fn error_at(diagnostics: &[Diagnostic], line: u32, col: u32) -> bool {
    diagnostics.iter().any(|d| d.line == line && d.column == col)
}

// ───────────────────────── No diagnostics ─────────────────────────

#[test]
fn test_no_diagnostics_for_valid_code() {
    let diags = run_diagnostics("int x = 5\ny = x + 1");
    assert!(diags.is_empty(), "Unexpected diagnostics: {:?}", diags);
}

#[test]
fn test_no_diagnostics_for_valid_function() {
    let diags = run_diagnostics("fn add(int a, int b):\n    return a + b\nend");
    assert!(diags.is_empty(), "Unexpected diagnostics: {:?}", diags);
}

#[test]
fn test_no_diagnostics_for_namespace_access() {
    let diags = run_diagnostics("fn output_fnc():\n    t.out = t.in1 + t.in2\nend");
    assert!(diags.is_empty(), "Unexpected diagnostics: {:?}", diags);
}

// ───────────────────────── Undefined variable ─────────────────────────

#[test]
fn test_undefined_variable_diagnostic() {
    let diags = run_diagnostics("y = undefined_var");
    assert!(has_error(&diags, "undefined_var"));
}

#[test]
fn test_undefined_variable_has_position() {
    let diags = run_diagnostics("y = undefined_var");
    assert!(!diags.is_empty());
    // "undefined_var" starts after "y = " on line 0
    assert!(error_at(&diags, 0, 4), "Expected error at 0:4, got {:?}", diags);
}

#[test]
fn test_undefined_variable_in_function() {
    let diags = run_diagnostics("fn test():\n    y = ghost\nend");
    assert!(has_error(&diags, "ghost"));
}

#[test]
fn test_namespace_redeclaration_diagnostic() {
    let diags = run_diagnostics("int t = 5");
    assert!(has_error(&diags, "Cannot redeclare"));
}

// ───────────────────────── Undefined function ─────────────────────────

#[test]
fn test_undefined_function_diagnostic() {
    let diags = run_diagnostics("y = ghost_fn(1)");
    assert!(has_error(&diags, "ghost_fn"));
}

#[test]
fn test_undefined_function_has_position() {
    let diags = run_diagnostics("y = ghost_fn(1)");
    assert!(!diags.is_empty());
    assert!(error_at(&diags, 0, 4), "Expected error at 0:4, got {:?}", diags);
}

// ───────────────────────── Argument count ─────────────────────────

#[test]
fn test_arg_count_mismatch_diagnostic() {
    let diags = run_diagnostics("fn foo(int a):\n    return a\nend\ny = foo(1, 2)");
    assert!(has_error(&diags, "expects 1 argument(s), got 2"));
}

// ───────────────────────── Entry function call ─────────────────────────

#[test]
fn test_entry_function_call_diagnostic() {
    let diags = run_diagnostics("fn test():\n    output_fnc()\nend");
    assert!(has_error(&diags, "Entry function"));
}

// ───────────────────────── Multiple sources ─────────────────────────

#[test]
fn test_multiple_errors_from_different_sources() {
    // Both undefined variable and undefined function
    let diags = run_diagnostics("y = ghost_var + ghost_fn()");
    assert!(has_error(&diags, "ghost_var"));
    assert!(has_error(&diags, "ghost_fn"));
    assert_eq!(diags.len(), 2, "Expected 2 diagnostics, got {:?}", diags);
}

#[test]
fn test_diagnostics_runner_is_extensible() {
    // Verify that adding no sources produces no diagnostics
    let normalized = "int x = 5".to_string();
    let parser = TmlParser::new();
    let ast = parser.parse(&normalized).expect("Parse failed");
    let (table, _) = SymbolTableBuilder::new().build(&ast);

    let runner = DiagnosticsRunner::new(); // no sources
    let diags = runner.run(&ast, &table);
    assert!(diags.is_empty());
}