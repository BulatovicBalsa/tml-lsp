use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::folding_collector::FoldingCollector;

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
    assert!(folds.contains(&(0, 2)), "Expected fold (0, 2), got {:?}", folds);
}

#[test]
fn test_function_no_fold_end_inline() {
    // 'end' is not on its own line — no fold
    let src = "fn test(): x = 1 end\n";
    let folds = collect_folds(src);
    assert!(folds.is_empty(), "Expected no folds, got {:?}", folds);
}

#[test]
fn test_multiple_functions() {
    let src = "fn foo():\n    x = 1\nend\nfn bar():\n    y = 2\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 2)), "Expected fold for foo");
    assert!(folds.contains(&(3, 5)), "Expected fold for bar");
}

// ───────────────────────── If statement ─────────────────────────

#[test]
fn test_if_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for if (1, 3), got {:?}", folds);
    assert!(folds.contains(&(0, 4)), "Expected fold for fn (0, 4), got {:?}", folds);
}

#[test]
fn test_if_else_fold() {
    let src = "fn test():\n    if x > 0:\n        y = 1\n    else:\n        y = 0\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 6)), "Expected fold for fn");
    assert!(folds.contains(&(1, 5)), "Expected fold for if");
}

#[test]
fn test_nested_if_fold() {
    let src = "fn test():\n    if x > 0:\n        if y > 0:\n            z = 1\n        end\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 6)), "Expected fold for fn");
    assert!(folds.contains(&(1, 5)), "Expected fold for outer if");
    assert!(folds.contains(&(2, 4)), "Expected fold for inner if");
}

// ───────────────────────── For loop ─────────────────────────

#[test]
fn test_for_fold() {
    let src = "fn test():\n    for i = 0:10:\n        x = i\n    end\nend";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for for loop (1, 3), got {:?}", folds);
    assert!(folds.contains(&(0, 4)), "Expected fold for fn (0, 4), got {:?}", folds);
}

#[test]
fn test_nested_for_fold() {
    let src = "fn test():\n    for i = 0:10:\n        for j = 0:10:\n            x = i\n        end\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(0, 6)), "Expected fold for fn");
    assert!(folds.contains(&(1, 5)), "Expected fold for outer for");
    assert!(folds.contains(&(2, 4)), "Expected fold for inner for");
}

// ───────────────────────── While loop ─────────────────────────

#[test]
fn test_while_fold() {
    let src = "fn test():\n    while x > 0:\n        x = x - 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for while (1, 3), got {:?}", folds);
    assert!(folds.contains(&(0, 4)), "Expected fold for fn (0, 4), got {:?}", folds);
}

// ───────────────────────── Exists ─────────────────────────

#[test]
fn test_exists_fold() {
    let src = "fn test():\n    exists x:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for exists (1, 3), got {:?}", folds);
}

#[test]
fn test_not_exists_fold() {
    let src = "fn test():\n    not exists x:\n        y = 1\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Expected fold for not exists");
}

// ───────────────────────── Indented end ─────────────────────────

#[test]
fn test_indented_end_valid() {
    // 'end' is indented but still on its own line — fold is valid
    let src = "fn test():\n    for i = 0:10:\n        x = i\n    end\nend\n";
    let folds = collect_folds(src);
    assert!(folds.contains(&(1, 3)), "Indented end should still produce fold");
}