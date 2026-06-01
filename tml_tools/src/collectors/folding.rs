use crate::visitor::AstVisitor;
use tml_parser::tml_actions::*;

macro_rules! start_line {
    ($e:expr) => {
        $e.header_colon.position.line_col
            .map(|lc| lc.line.saturating_sub(2))
            .unwrap_or(0) as u32
    };
}

// ───────────────────────── FoldingRange ─────────────────────────

#[derive(Debug, Clone)]
pub struct TmlFoldingRange {
    pub start_line: u32,
    pub end_line: u32,
}

// ───────────────────────── Collector ─────────────────────────

pub struct FoldingCollector<'a> {
    ranges: Vec<TmlFoldingRange>,
    _text: &'a str,
}

impl<'a> FoldingCollector<'a> {
    pub fn new(text: &'a str) -> Self {
        FoldingCollector {
            ranges: vec![],
            _text: text,
        }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<TmlFoldingRange> {
        unit.accept(&mut self);
        self.ranges
    }

    /// Check if line contains only whitespace chars and 'end'
    fn _is_end_on_own_line(&self, line: usize) -> bool {
        self._text
            .lines()
            .nth(line)
            .map(|l| l.trim() == "end")
            .unwrap_or(false)
    }

    fn try_add_range(&mut self, start_line: u32, end_position: &rustemo::Position) {
        let end_line = end_position.line_col
            .map(|lc| lc.line.saturating_sub(2))
            .unwrap_or(0);

        self.try_add_range_end_line(start_line, end_line as u32);
    }

    fn try_add_range_end_line(&mut self, start_line: u32, end_line: u32) {
        if start_line < end_line {
            self.ranges.push(TmlFoldingRange {
                start_line,
                end_line,
            });
        }
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl<'a> AstVisitor for FoldingCollector<'a> {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.try_add_range(start_line!(f), &f.end_t.position);
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let mut if_end_line = 0;
        if let Some(elseifs) = &s.elseif_clause {
            for index in 0..elseifs.len() {
                let clause = &elseifs[index];
                if index == 0 {
                    if_end_line = start_line!(clause);
                }
                if index == &elseifs.len() - 1 {
                    if let Some(else_c) = &s.else_clause {
                        self.try_add_range_end_line(start_line!(clause), start_line!(else_c) - 1);
                    } else {
                        self.try_add_range(start_line!(clause), &s.end_t.position);
                    }
                } else {
                    let next_clause = &elseifs[index + 1];
                    self.try_add_range_end_line(start_line!(clause), start_line!(next_clause) - 1);
                }
            }
        }
        if let Some(else_c) = &s.else_clause {
            if if_end_line == 0 {
                if_end_line = start_line!(else_c);
            }
            self.try_add_range(start_line!(else_c), &s.end_t.position);
        }

        if if_end_line == 0 {
            self.try_add_range(start_line!(s), &s.end_t.position);
        } else {
            self.try_add_range_end_line(start_line!(s), if_end_line - 1);
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        self.try_add_range(start_line!(f), &f.end_t.position);
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        self.try_add_range(start_line!(w), &w.end_t.position);
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        self.try_add_range(start_line!(e), &e.end_t.position);
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        self.try_add_range(start_line!(e), &e.end_t.position);
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        self.try_add_range(start_line!(e), &e.end_t.position);
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        self.try_add_range(start_line!(e), &e.end_t.position);
    }
}