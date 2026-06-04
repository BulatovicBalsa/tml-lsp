use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::block_span::{find_indent, find_enclosing_block, BlockKind, BlockSpan, BlockSpanCollector, KeywordSpan};
use tml_tools::constants::INDENT;

fn span(header_line: u32, end_line: u32, level: usize, kind: BlockKind) -> BlockSpan {
    BlockSpan {
        header:    KeywordSpan { line: header_line, col: 0, len: 2 },
        end:       KeywordSpan { line: end_line,    col: 0, len: 3 },
        block_end: KeywordSpan { line: end_line,    col: 0, len: 3 },
        body_indent_level: level,
        body_col: level * 4,
        kind,
    }
}

fn collect(src: &str) -> Vec<BlockSpan> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let parser = TmlParser::new();
    let ast = parser.parse(&normalized).expect("Parse failed");
    BlockSpanCollector::new().collect(&ast)
}

// ───────────────────────── find_indent unit tests ─────────────────────────
// These test the pure function directly without parsing.

#[test]
fn test_find_indent_empty_spans() {
    assert_eq!(find_indent(&[], 5), 0);
}

#[test]
fn test_find_indent_before_any_block() {
    let spans = vec![span(5, 10, 1, BlockKind::Function)];
    assert_eq!(find_indent(&spans, 3), 0);
}

#[test]
fn test_find_indent_after_block_closes() {
    let spans = vec![span(0, 5, 1, BlockKind::Function)];
    assert_eq!(find_indent(&spans, 6), 0);
}

#[test]
fn test_find_indent_exactly_on_header_line() {
    let spans = vec![span(2, 6, 1, BlockKind::Function)];
    assert_eq!(find_indent(&spans, 2), 0);
}

#[test]
fn test_find_indent_exactly_on_end_line() {
    let spans = vec![span(2, 6, 1, BlockKind::Function)];
    assert_eq!(find_indent(&spans, 6), 0);
}

#[test]
fn test_find_indent_inside_single_block() {
    let spans = vec![span(0, 10, 1, BlockKind::Function)];
    assert_eq!(find_indent(&spans, 5), 1);
}

#[test]
fn test_find_indent_nested_blocks_takes_deepest() {
    let spans = vec![
        span(0, 10, 1, BlockKind::Function),
        span(2, 8,  2, BlockKind::If),
        span(4, 6,  3, BlockKind::For),
    ];
    assert_eq!(find_indent(&spans, 5), 3);
    assert_eq!(find_indent(&spans, 3), 2);
    assert_eq!(find_indent(&spans, 1), 1);
}

#[test]
fn test_find_indent_sibling_blocks_not_mixed() {
    let spans = vec![
        span(0, 4,  1, BlockKind::Function),
        span(5, 10, 1, BlockKind::Function),
    ];
    assert_eq!(find_indent(&spans, 3), 1);
    assert_eq!(find_indent(&spans, 7), 1);
    assert_eq!(find_indent(&spans, 5), 0);
}

// ───────────────────────── Collector: span count ─────────────────────────

#[test]
fn test_function_produces_one_span() {
    let spans = collect("fn foo():\n    pass\nend");
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_if_inside_function_produces_two_spans() {
    let spans = collect("fn foo():\n    if true:\n        pass\n    end\nend");
    assert_eq!(spans.len(), 2);
}

#[test]
fn test_if_else_produces_three_spans() {
    // fn + if + else = 3 spans
    let spans = collect("fn foo():\n    if true:\n        pass\n    else:\n        pass\n    end\nend");
    assert_eq!(spans.len(), 3);
}

#[test]
fn test_for_inside_function_produces_two_spans() {
    let spans = collect("fn foo():\n    for i = 0:5:\n        pass\n    end\nend");
    assert_eq!(spans.len(), 2);
}

#[test]
fn test_while_inside_function_produces_two_spans() {
    let spans = collect("fn foo():\n    int x = 1\n    while x > 0:\n        pass\n    end\nend");
    assert_eq!(spans.len(), 2);
}

// ───────────────────────── Collector: indent levels ─────────────────────────

#[test]
fn test_function_body_indent_level() {
    // fn is at col 0 → body indent level = 1
    let spans = collect("fn foo():\n    pass\nend");
    assert_eq!(spans[0].body_indent_level, 1);
}

#[test]
fn test_if_body_indent_level_inside_function() {
    // if is at col 4 (1 INDENT) → body indent level = 2
    let spans = collect("fn foo():\n    if true:\n        pass\n    end\nend");
    let if_span = spans.iter().find(|s| s.body_indent_level == 2);
    assert!(if_span.is_some(), "Expected a span with body_indent_level=2, got: {:?}", spans);
}

#[test]
fn test_for_body_indent_level() {
    let spans = collect("fn foo():\n    for i = 0:5:\n        pass\n    end\nend");
    let for_span = spans.iter().find(|s| s.body_indent_level == 2);
    assert!(for_span.is_some(), "Expected a span with body_indent_level=2, got: {:?}", spans);
}

// ───────────────────────── Collector + find_indent integration ─────────────────────────
//
// Format: src line numbers are 0-based.
// "fn foo():"  → line 0, header_colon on line 0, end on line 2
// "    pass"   → line 1
// "end"        → line 2
//
// Cursor on line 1 (inside function) → level 1

#[rstest]
#[case(
    // fn body — cursor on line 1
    "fn foo():\n    pass\nend",
    1, 1
)]
#[case(
    // if body inside fn — cursor on line 2
    "fn foo():\n    if true:\n        pass\n    end\nend",
    2, 2
)]
#[case(
    // for body inside fn — cursor on line 2
    "fn foo():\n    for i = 0:5:\n        pass\n    end\nend",
    2, 2
)]
#[case(
    // while body inside fn — cursor on line 3 (after "int x = 1" on line 1)
    "fn foo():\n    int x = 1\n    while x > 0:\n        pass\n    end\nend",
    3, 2
)]
#[case(
    // cursor on end line of fn - outside
    "fn foo():\n    pass\nend",
    2, 0
)]
#[case(
    // cursor before fn starts — global scope, no indent
    "fn foo():\n    pass\nend",
    0, 0
)]
fn test_find_indent_integration(
    #[case] src: &str,
    #[case] cursor_line: u32,
    #[case] expected_level: usize,
) {
    let spans = collect(src);
    let level = find_indent(&spans, cursor_line);
    assert_eq!(
        level, expected_level,
        "Wrong indent level at line {} for:\n{}\nspans: {:?}",
        cursor_line, src, spans
    );
}

// ───────────────────────── Indent string matches INDENT constant ─────────────────────────

#[test]
fn test_indent_level_matches_indent_constant() {
    // body_indent_level == 1 should correspond to exactly one INDENT repetition
    let spans = collect("fn foo():\n    pass\nend");
    let level = spans[0].body_indent_level;
    let expected_indent = INDENT.repeat(level);
    assert_eq!(expected_indent, "    "); // 4 spaces = 1 * INDENT
}

// ───────────────────────── Elseif coverage ─────────────────────────

#[test]
fn test_find_indent_inside_last_elseif_block_regular() {
    let src = "fn foo():\n    if true:\n        pass\n    elseif false:\n        pass\n    elseif true:\n\n    end\nend";
    let spans = collect(src);
    let level = find_indent(&spans, 6);
    assert_eq!(
        level, 2,
        "Cursor inside last elseif block should be at level 2, spans: {:?}", spans
    );
}

// ───────────────────────── Macro elseif coverage ─────────────────────────

#[test]
fn test_find_indent_inside_last_elseif_block() {
    let src = "macro if true:\n    pass\nelseif false:\n    pass\nelseif true:\n\nend";
    let spans = collect(src);
    let level = find_indent(&spans, 5);
    assert_eq!(
        level, 1,
        "Cursor inside last elseif block should be at level 1, spans: {:?}", spans
    );
}

// ───────────────────────── BlockKind ─────────────────────────

#[rstest]
#[case("fn foo():\n    pass\nend",                               BlockKind::Function)]
#[case("fn foo():\n    if true:\n        pass\n    end\nend",    BlockKind::If)]
#[case("fn foo():\n    for i=0:5:\n        pass\n    end\nend",  BlockKind::For)]
#[case("fn foo():\n    int x=1\n    while x>0:\n        pass\n    end\nend", BlockKind::While)]
fn test_block_kind(#[case] src: &str, #[case] expected_kind: BlockKind) {
    let spans = collect(src);
    let found = spans.iter().any(|s| s.kind == expected_kind);
    assert!(found, "Expected span with kind {:?} in: {:?}", expected_kind, spans);
}

#[test]
fn test_if_else_span_kinds() {
    let src = "fn foo():\n    if true:\n        pass\n    else:\n        pass\n    end\nend";
    let spans = collect(src);
    assert!(spans.iter().any(|s| s.kind == BlockKind::If));
    assert!(spans.iter().any(|s| s.kind == BlockKind::Else));
}

#[test]
fn test_if_elseif_span_kinds() {
    let src = "fn foo():\n    if true:\n        pass\n    elseif false:\n        pass\n    end\nend";
    let spans = collect(src);
    assert!(spans.iter().any(|s| s.kind == BlockKind::If));
    assert!(spans.iter().any(|s| s.kind == BlockKind::Elseif));
}

// ───────────────────────── find_enclosing_block ─────────────────────────

#[test]
fn test_enclosing_block_none_outside() {
    let spans = collect("fn foo():\n    pass\nend");
    // Line 3 is outside — no enclosing block
    assert!(find_enclosing_block(&spans, 3).is_none());
}

#[test]
fn test_enclosing_block_function() {
    let spans = collect("fn foo():\n    pass\nend");
    let block = find_enclosing_block(&spans, 1).expect("Expected enclosing block");
    assert_eq!(block.kind, BlockKind::Function);
}

#[test]
fn test_enclosing_block_if_inside_function() {
    let src = "fn foo():\n    if true:\n        pass\n    end\nend";
    let spans = collect(src);
    let block = find_enclosing_block(&spans, 2).expect("Expected enclosing block");
    // Deepest enclosing block should be If, not Function
    assert_eq!(block.kind, BlockKind::If);
}

#[test]
fn test_enclosing_block_for_inside_function() {
    let src = "fn foo():\n    for i=0:5:\n        pass\n    end\nend";
    let spans = collect(src);
    let block = find_enclosing_block(&spans, 2).expect("Expected enclosing block");
    assert_eq!(block.kind, BlockKind::For);
}

#[test]
fn test_enclosing_block_is_not_if_for_for_loop() {
    // Verifies that else/elseif snippets would NOT be shown inside a for loop
    let src = "fn foo():\n    for i=0:5:\n        pass\n    end\nend";
    let spans = collect(src);
    let block = find_enclosing_block(&spans, 2).expect("Expected enclosing block");
    let in_if_context = matches!(block.kind, BlockKind::If | BlockKind::Elseif | BlockKind::MacroIf);
    assert!(!in_if_context, "For loop should not be treated as if context");
}

#[test]
fn test_enclosing_block_is_if_context_for_if() {
    // Verifies that else/elseif snippets WOULD be shown inside an if block
    let src = "fn foo():\n    if true:\n        pass\n    end\nend";
    let spans = collect(src);
    let block = find_enclosing_block(&spans, 2).expect("Expected enclosing block");
    let in_if_context = matches!(block.kind, BlockKind::If | BlockKind::Elseif | BlockKind::MacroIf);
    assert!(in_if_context, "If block should be treated as if context");
}

#[test]
fn test_enclosing_block_elseif_is_if_context() {
    // Cursor inside elseif body — elseif snippet should still be available
    let src = "fn foo():\n    if true:\n        pass\n    elseif false:\n        pass\n    end\nend";
    let spans = collect(src);
    let block = find_enclosing_block(&spans, 4).expect("Expected enclosing block");
    let in_if_context = matches!(block.kind, BlockKind::If | BlockKind::Elseif | BlockKind::MacroIf);
    assert!(in_if_context, "Elseif block should be treated as if context");
}