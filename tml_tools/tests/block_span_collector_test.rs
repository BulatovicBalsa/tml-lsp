use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::block_span::{find_indent, BlockSpan, BlockSpanCollector};
use tml_tools::formatter::INDENT;

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
    let spans = vec![BlockSpan { header_line: 5, end_line: 10, body_indent_level: 1 }];
    // Line 3 is before the block opens at line 5
    assert_eq!(find_indent(&spans, 3), 0);
}

#[test]
fn test_find_indent_after_block_closes() {
    let spans = vec![BlockSpan { header_line: 0, end_line: 5, body_indent_level: 1 }];
    // Line 6 is after the block closes at line 5
    assert_eq!(find_indent(&spans, 6), 0);
}

#[test]
fn test_find_indent_exactly_on_header_line() {
    // header_line is exclusive — cursor ON header line means we are NOT inside the block yet
    let spans = vec![BlockSpan { header_line: 2, end_line: 6, body_indent_level: 1 }];
    assert_eq!(find_indent(&spans, 2), 0);
}

#[test]
fn test_find_indent_exactly_on_end_line() {
    let spans = vec![BlockSpan { header_line: 2, end_line: 6, body_indent_level: 1 }];
    assert_eq!(find_indent(&spans, 6), 0);
}

#[test]
fn test_find_indent_inside_single_block() {
    let spans = vec![BlockSpan { header_line: 0, end_line: 10, body_indent_level: 1 }];
    assert_eq!(find_indent(&spans, 5), 1);
}

#[test]
fn test_find_indent_nested_blocks_takes_deepest() {
    let spans = vec![
        BlockSpan { header_line: 0, end_line: 10, body_indent_level: 1 },
        BlockSpan { header_line: 2, end_line: 8,  body_indent_level: 2 },
        BlockSpan { header_line: 4, end_line: 6,  body_indent_level: 3 },
    ];
    // Line 5 is inside all three — should return deepest (3)
    assert_eq!(find_indent(&spans, 5), 3);
    // Line 3 is inside first two only — should return 2
    assert_eq!(find_indent(&spans, 3), 2);
    // Line 1 is inside only the outermost — should return 1
    assert_eq!(find_indent(&spans, 1), 1);
}

#[test]
fn test_find_indent_sibling_blocks_not_mixed() {
    let spans = vec![
        BlockSpan { header_line: 0, end_line: 4,  body_indent_level: 1 },
        BlockSpan { header_line: 5, end_line: 10, body_indent_level: 1 },
    ];
    // Line 3 is in first block only
    assert_eq!(find_indent(&spans, 3), 1);
    // Line 7 is in second block only
    assert_eq!(find_indent(&spans, 7), 1);
    // Line 5 is the header of second block — not inside it yet
    // (but IS inside... wait, end of first is 4, second starts at 5 — gap at line 5 header)
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
