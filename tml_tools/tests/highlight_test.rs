use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::block_span::{BlockSpan, BlockSpanCollector, find_highlight};

fn collect(src: &str) -> Vec<BlockSpan> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let ast = TmlParser::new().parse(&normalized).expect("Parse failed");
    BlockSpanCollector::new().collect(&ast)
}

// ───────────────────────── None cases ─────────────────────────

#[test]
fn test_no_highlight_on_body_line() {
    let spans = collect("fn foo():\n    pass\nend");
    assert!(find_highlight(&spans, 1, 0).is_none());
}

#[test]
fn test_no_highlight_outside_keyword_range() {
    // fn is at col 0..2, cursor at col 5 is past the keyword
    let spans = collect("fn foo():\n    pass\nend");
    assert!(find_highlight(&spans, 0, 5).is_none());
}

#[test]
fn test_no_highlight_on_empty_spans() {
    assert!(find_highlight(&[], 0, 0).is_none());
}

// ───────────────────────── fn / end ─────────────────────────

#[test]
fn test_highlight_fn_keyword() {
    // fn foo():   <- line 0, fn at col 0..2
    //     pass    <- line 1
    // end         <- line 2, end at col 0..3
    let spans = collect("fn foo():\n    pass\nend");

    let (header, end) = find_highlight(&spans, 0, 0)
        .expect("Expected highlight on fn keyword");

    assert_eq!(header.line, 0);
    assert_eq!(header.col,  0);
    assert_eq!(header.len,  2); // "fn"
    assert_eq!(end.line,    2);
    assert_eq!(end.col,     0);
    assert_eq!(end.len,     3); // "end"
}

#[test]
fn test_highlight_fn_keyword_cursor_at_last_char() {
    // cursor at col 1 - still inside "fn"
    let spans = collect("fn foo():\n    pass\nend");
    assert!(find_highlight(&spans, 0, 1).is_some());
}

#[test]
fn test_highlight_end_keyword() {
    let spans = collect("fn foo():\n    pass\nend");

    let (header, end) = find_highlight(&spans, 2, 0)
        .expect("Expected highlight on end keyword");

    assert_eq!(header.line, 0);
    assert_eq!(end.line,    2);
}

#[test]
fn test_highlight_end_cursor_at_last_char() {
    // end is at col 0..3, cursor at col 2 - still inside "end"
    let spans = collect("fn foo():\n    pass\nend");
    assert!(find_highlight(&spans, 2, 2).is_some());
}

// ───────────────────────── if / end ─────────────────────────

#[test]
fn test_highlight_if_keyword() {
    // fn foo():     <- line 0
    //     if true:  <- line 1, if at col 4..6
    //         pass  <- line 2
    //     end       <- line 3, end at col 4..7
    // end           <- line 4
    let src = "fn foo():\n    if true:\n        pass\n    end\nend";
    let spans = collect(src);

    let (header, end) = find_highlight(&spans, 1, 4)
        .expect("Expected highlight on if keyword");

    assert_eq!(header.line, 1);
    assert_eq!(header.col,  4);
    assert_eq!(header.len,  2); // "if"
    assert_eq!(end.line,    3);
    assert_eq!(end.col,     4);
    assert_eq!(end.len,     3); // "end"
}

#[test]
fn test_highlight_if_end_returns_if_header() {
    let src = "fn foo():\n    if true:\n        pass\n    end\nend";
    let spans = collect(src);

    let (header, _) = find_highlight(&spans, 3, 4)
        .expect("Expected highlight on if end keyword");

    assert_eq!(header.line, 1); // highlights back to if
}

// ───────────────────────── Nested blocks ─────────────────────────

#[rstest]
#[case("fn foo():\n    pass\nend",                               0, 0, 0, 2)]
#[case("fn foo():\n    if true:\n        pass\n    end\nend",    1, 4, 1, 3)]
#[case("fn foo():\n    for i=0:5:\n        pass\n    end\nend",  1, 4, 1, 3)]
fn test_highlight_header_and_end_lines(
    #[case] src: &str,
    #[case] cursor_line: u32,
    #[case] cursor_col: u32,
    #[case] expected_header_line: u32,
    #[case] expected_end_line: u32,
) {
    let spans = collect(src);
    let (header, end) = find_highlight(&spans, cursor_line, cursor_col)
        .unwrap_or_else(|| panic!(
            "Expected highlight at ({}, {}) for:\n{}\nspans: {:?}",
            cursor_line, cursor_col, src, spans
        ));

    assert_eq!(header.line, expected_header_line);
    assert_eq!(end.line,    expected_end_line);
}

// ───────────────────────── Symmetry ─────────────────────────

#[test]
fn test_highlight_is_symmetric() {
    // Clicking header or end highlights the same pair
    let src = "fn foo():\n    if true:\n        pass\n    end\nend";
    let spans = collect(src);

    let from_header = find_highlight(&spans, 1, 4); // click on if
    let from_end    = find_highlight(&spans, 3, 4); // click on end

    let (h1, e1) = from_header.expect("Expected highlight from header");
    let (h2, e2) = from_end.expect("Expected highlight from end");

    assert_eq!(h1.line, h2.line);
    assert_eq!(e1.line, e2.line);
}
