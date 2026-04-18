use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticsRunner};
use tml_tools::empty_body_checker::{EmptyBodyChecker, EmptyBodyDiagnosticSource, EmptyBodyError};
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

fn run_errors(src: &str) -> Vec<EmptyBodyError> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");

    let ast = TmlParser::new()
        .parse(&normalized)
        .expect("Parse failed");

    EmptyBodyChecker::new().check(&ast)
}

// ───────────────────────── No error when body has content ─────────────────────────

#[rstest]
#[case("fn foo():\n    x = 1\nend")]
#[case("fn foo():\n    pass\nend")]
#[case("fn foo():\n    if true:\n        x = 1\n    end\nend")]
#[case("fn foo():\n    for i = 0:10:\n        x = i\n    end\nend")]
#[case("fn foo():\n    int x = 0\n    while x > 0:\n        x = x - 1\n    end\nend")]
fn test_no_error_for_non_empty_body(#[case] src: &str) {
    let diags = run(src);
    assert!(diags.is_empty(), "Expected no diagnostics for:\n{}\nGot: {:?}", src, diags);
}

// ───────────────────────── Error for empty bodies ─────────────────────────

#[rstest]
#[case("fn foo():\nend",                                                         "foo")]
#[case("fn my_function():\nend",                                                 "my_function")]
#[case("fn foo():\n    if true:\n    end\nend",                                  "'if' body")]
#[case("fn foo():\n    if true:\n        pass\n    elseif false:\n    end\nend", "'elseif' body")]
#[case("fn foo():\n    if true:\n        x=1\n    else:\n    end\nend",          "'else' body")]
#[case("fn foo():\n    for i = 0:10:\n    end\nend",                             "'for i'")]
#[case("fn foo():\n    int x=1\n    while x > 0:\n    end\nend",                "'while' body")]
#[case("fn foo():\n    not exists t.a:\n    end\nend",                           "'not exists' body")]
#[case("fn foo():\n    not feedthrough t.a:\n    end\nend",                      "'not feedthrough' body")]
fn test_error_for_empty_body(#[case] src: &str, #[case] msg_fragment: &str) {
    let diags = run(src);
    assert!(
        has_error(&diags, msg_fragment),
        "Expected error containing '{}' for:\n{}\nGot: {:?}",
        msg_fragment, src, diags
    );
}

// ───────────────────────── Severity is Error ─────────────────────────

#[test]
fn test_severity_is_error() {
    let diags = run("fn foo():\nend");
    assert!(diags.iter().all(|d| d.severity == DiagnosticSeverity::Error));
}

// ───────────────────────── Keyword position ─────────────────────────

#[rstest]
#[case(
    "fn foo():\n    if true:\n    end\nend",
    "'if' body", 1, 4, 2
)]
#[case(
    "fn foo():\n    for i = 0:10:\n    end\nend",
    "'for i'", 1, 4, 3
)]
#[case(
    "fn foo():\n    int x = 1\n    while x > 0:\n    end\nend",
    "'while' body", 2, 4, 5
)]
#[case(
    "fn foo():\n    if true:\n        x = 1\n    else:\n    end\nend",
    "'else' body", 3, 4, 4
)]
#[case(
    "fn foo():\n    not exists t.a:\n    end\nend",
    "'not exists' body", 1, 4, 10  // "not exists" = 10 chars
)]
#[case(
    "fn foo():\n    not feedthrough t.a:\n    end\nend",
    "'not feedthrough' body", 1, 4, 15  // "not feedthrough" = 15 chars
)]
fn test_keyword_position(
    #[case] src: &str,
    #[case] msg_fragment: &str,
    #[case] expected_line: u32,
    #[case] expected_col: u32,
    #[case] expected_len: usize,
) {
    let diags = run(src);
    let d = diags
        .iter()
        .find(|d| d.message.contains(msg_fragment))
        .unwrap_or_else(|| panic!(
            "No diagnostic containing '{}'\nsrc:\n{}\ndiags: {:?}",
            msg_fragment, src, diags
        ));

    assert_eq!(
        d.line, expected_line,
        "Wrong line for '{}': expected {}, got {}\nsrc:\n{}",
        msg_fragment, expected_line, d.line, src
    );
    assert_eq!(
        d.column, expected_col,
        "Wrong col for '{}': expected {}, got {}\nsrc:\n{}",
        msg_fragment, expected_col, d.column, src
    );
    assert_eq!(
        d.length, expected_len,
        "Wrong length for '{}': expected {}, got {}\nsrc:\n{}",
        msg_fragment, expected_len, d.length, src
    );
}

// ───────────────────────── Quick fix metadata ─────────────────────────

#[rstest]
#[case(
    "fn foo():\nend",
    "foo",
    1,
    4
)]
#[case(
    "fn foo():\n    if true:\n    end\nend",
    "'if' body",
    2,
    8
)]
#[case(
    "fn foo():\n    for i = 0:10:\n    end\nend",
    "'for i'",
    2,
    8
)]
#[case(
    "fn foo():\n    int x=1\n    while x > 0:\n    end\nend",
    "'while' body",
    3,
    8
)]
#[case(
    "fn foo():\n    if true:\n        x=1\n    else:\n    end\nend",
    "'else' body",
    4,
    8
)]
#[case(
    "  fn foo():\nend",
    "foo",
    1,
    6
)]
fn test_quick_fix_metadata(
    #[case] src: &str,
    #[case] msg_fragment: &str,
    #[case] expected_insert_line: u32,
    #[case] expected_indent_levels: usize,
) {
    let errors = run_errors(src);
    let e = errors
        .iter()
        .find(|e| e.message.contains(msg_fragment))
        .unwrap_or_else(|| panic!(
            "No error containing '{}'\nsrc:\n{}\nerrors: {:?}",
            msg_fragment, src, errors
        ));

    assert_eq!(
        e.insert_line, expected_insert_line,
        "Wrong insert_line for '{}': expected {}, got {}\nsrc:\n{}",
        msg_fragment, expected_insert_line, e.insert_line, src
    );

    let expected_indent = " ".repeat(expected_indent_levels);
    assert_eq!(
        e.indent, expected_indent,
        "Wrong indent for '{}': expected {:?}, got {:?}\nsrc:\n{}",
        msg_fragment, expected_indent, e.indent, src
    );
}