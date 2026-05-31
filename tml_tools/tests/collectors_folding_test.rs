use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::folding::FoldingCollector;

fn collect_folds(src: &str) -> Vec<(u32, u32)> {
    let normalized = src.replace("\r\n", "\n").replace('\r', "\n");
    let parser = TmlParser::new();
    let ast = parser.parse(&normalized).expect("Parse failed");
    FoldingCollector::new(&normalized)
        .collect(&ast)
        .into_iter()
        .map(|r| (r.start_line, r.end_line))
        .collect()
}

// ───────────────────────── Function ─────────────────────────

#[test]
fn test_function_fold() {
    let src = "fn test():\n    x = 1\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) for function, got {:?}", folds);
}

#[test]
fn test_function_fold_end_inline_no_comment() {
    // 'end' is not on its own line — still folds
    let src = "fn test():\n    x = 1\n    y = 1 end\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) with inline end, got {:?}", folds);
}

#[test]
fn test_function_fold_end_inline_with_comment() {
    // 'end' is not on its own line and has a comment — still folds
    let src = "fn test():\n    x = 1\n    x = 2 end # End of function\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) with inline end and comment, got {:?}", folds);
}

#[test]
fn test_multiple_functions() {
    let src = "fn foo():\n    x = 1\nend\nfn bar():\n    y = 2\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold for foo");
    assert!(folds.contains(&(3, 4)), "Expected fold for bar");
}

// ───────────────────────── If statement ─────────────────────────

#[test]
fn test_if_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for if (1, 2), got {:?}", folds);
    assert!(folds.contains(&(0, 3)), "Expected fold for fn (0, 3), got {:?}", folds);
}

#[test]
fn test_if_else_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    else:\n        y = 0\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 5)), "Expected fold for fn (0, 5), got {:?}", folds);
    assert!(folds.contains(&(1, 2)), "Expected fold for if (1, 2), got {:?}", folds);
    assert!(folds.contains(&(3, 4)), "Expected fold for else (3, 4), got {:?}", folds);
}

#[test]
fn test_nested_if_fold() {
    let src = "fn test():\n    if x > 0:\n        if y > 0:\n            z = 1\n        end\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 5)), "Expected fold for fn");
    assert!(folds.contains(&(1, 4)), "Expected fold for outer if");
    assert!(folds.contains(&(2, 3)), "Expected fold for inner if");
}

// ───────────────────────── For loop ─────────────────────────

#[test]
fn test_for_fold() {
    let src = "fn test():\n    for i = 0:10:\n        x = i\n    end\nend";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for for loop (1, 3), got {:?}", folds);
    assert!(folds.contains(&(0, 3)), "Expected fold for fn (0, 4), got {:?}", folds);
}

#[test]
fn test_nested_for_fold() {
    let src = "fn test():\n    for i = 0:10:\n        for j = 0:10:\n            x = i\n        end\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 5)), "Expected fold for fn");
    assert!(folds.contains(&(1, 4)), "Expected fold for outer for");
    assert!(folds.contains(&(2, 3)), "Expected fold for inner for");
}

// ───────────────────────── While loop ─────────────────────────

#[test]
fn test_while_fold() {
    let src = "fn test():\n    while x > 0:\n        x = x - 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for while (1, 2), got {:?}", folds);
    assert!(folds.contains(&(0, 3)), "Expected fold for fn (0, 3), got {:?}", folds);
}

// ───────────────────────── Exists ─────────────────────────

#[test]
fn test_exists_fold() {
    let src = "fn test():\n    exists x:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for exists (1, 2), got {:?}", folds);
}

#[test]
fn test_not_exists_fold() {
    let src = "fn test():\n    not exists x:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for not exists");
}

// ───────────────────────── Indented end ─────────────────────────

#[test]
fn test_indented_end_valid() {
    // 'end' is indented but still on its own line — fold is valid
    let src = "fn test():\n    for i = 0:10:\n        x = i\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Indented end should still produce fold");
}

// ───────────────────────── Comments on header colon line ─────────────────────────

#[test]
fn test_comment_on_fn_header() {
    // Comment on the same line as the header colon
    let src = "fn test(): # This is a function\n    x = 1\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) with comment on header, got {:?}", folds);
}

#[test]
fn test_comment_on_if_header() {
    let src = "fn test():\n    if x > 0: # Check condition\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for if with comment on header, got {:?}", folds);
    assert!(folds.contains(&(0, 3)), "Expected fold for fn, got {:?}", folds);
}

#[test]
fn test_comment_on_for_header() {
    let src = "fn test():\n    for i = 0:10: # Loop over range\n        x = i\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for for with comment on header");
}

#[test]
fn test_comment_on_while_header() {
    let src = "fn test():\n    while x > 0: # Keep looping\n        x = x - 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for while with comment on header");
}

// ───────────────────────── Comments before block ─────────────────────────

#[test]
fn test_comment_before_function() {
    // Comment line before the function should not affect folding
    let src = "# This is a comment\nfn test():\n    x = 1\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold (1, 2) with comment before fn, got {:?}", folds);
}

#[test]
fn test_comment_before_if() {
    let src = "fn test():\n    # Check if positive\n    if x > 0:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(2, 3)), "Expected fold for if after comment");
    assert!(folds.contains(&(0, 4)), "Expected fold for fn");
}

#[test]
fn test_comment_before_for() {
    let src = "fn test():\n    # Iterate over range\n    for i = 0:10:\n        x = i\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(2, 3)), "Expected fold for for after comment");
}

// ───────────────────────── Comments inside block ─────────────────────────

#[test]
fn test_comment_inside_function() {
    let src = "fn test():\n    # Initialize\n    x = 1\n    # Update\n    y = 2\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 4)), "Expected fold for fn with comments inside, got {:?}", folds);
}

#[test]
fn test_comment_inside_for() {
    let src = "fn test():\n    for i = 0:10:\n        # Process element\n        x = i\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for for with comment inside");
    assert!(folds.contains(&(0, 4)), "Expected fold for fn");
}

// ───────────────────────── Comments after block ─────────────────────────

#[test]
fn test_comment_after_end() {
    // Comment after 'end' should not affect folding
    let src = "fn test():\n    x = 1\nend\n# Done\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) with comment after end, got {:?}", folds);
}

#[test]
fn test_comment_between_functions() {
    let src = "fn foo():\n    x = 1\nend\n# Separator comment\nfn bar():\n    y = 2\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold for foo");
    assert!(folds.contains(&(4, 5)), "Expected fold for bar after comment");
}

// ───────────────────────── Multiple comments ─────────────────────────

#[test]
fn test_multiple_comments_around_block() {
    let src = "# Pre-comment\nfn test(): # Header comment\n    # Body comment\n    x = 1\nend\n# Post-comment\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold (1, 3) with multiple comments, got {:?}", folds);
}

#[test]
fn test_multiple_fn_folds() {
    let src = r#"
        fn init_fnc():
            pass
        end

        fn output_fnc():
            if p.unit == "Hz":
                unit_conv = 2.0 * M_PI
            else:
                unit_conv = 1.0
            end
        end

        fn update_fnc():
            previous_in = t.in
        end
    "#;
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for init_fnc");
    assert!(folds.contains(&(5, 10)), "Expected fold for output_fnc");
    assert!(folds.contains(&(13, 14)), "Expected fold for update_fnc");
}

#[test]
fn test_fn_no_trailing_newline() {
    let src = "fn test():\n    x = 1\nend"; // No trailing newline
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 1)), "Expected fold (0, 1) without trailing newline, got {:?}", folds);
}

#[test]
fn test_else_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    else:\n        y = 0\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(3, 4)), "Expected fold for else (3, 4), got {:?}", folds);
}

#[test]
fn test_elseif_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    elseif x < 0:\n        y = -1\n    else:\n        y = 0\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(3, 4)), "Expected fold for elseif (3, 4), got {:?}", folds);
}

#[test]
fn test_multiple_elseif_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    elseif x < 0:\n        y = -1\n    elseif x == 0:\n        y = 0\n    else:\n        y = 0\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 2)), "Expected fold for if (1, 2), got {:?}", folds);
    assert!(folds.contains(&(3, 4)), "Expected fold for first elseif (3, 4), got {:?}", folds);
    assert!(folds.contains(&(5, 6)), "Expected fold for second elseif (5, 6), got {:?}", folds);
    assert!(folds.contains(&(7, 8)), "Expected fold for else (7, 8), got {:?}", folds);
}
