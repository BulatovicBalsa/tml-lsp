use crate::formatter::INDENT;
use crate::position::SourcePosition;
use crate::visitor::AstVisitor;
use tml_parser::tml_actions::{
    ExistsStatement, FeedthroughStatement, ForIterationStatement, FunctionDefinition,
    MacroFor, MacroIf,
    NotExistsStatement, NotFeedthroughStatement, SelectionStatement, TranslationUnit,
    WhileIterationStatement,
};

// ───────────────────────── BlockKind ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    Function,
    If,
    Elseif,
    Else,
    For,
    While,
    Exists,
    ExistsElse,
    NotExists,
    NotExistsElse,
    Feedthrough,
    FeedthroughElse,
    NotFeedthrough,
    NotFeedthroughElse,
    MacroIf,
    MacroFor,
}

// ───────────────────────── BlockSpan ─────────────────────────

#[derive(Debug, Clone)]
pub struct BlockSpan {
    pub header_line: u32,
    pub end_line: u32,
    pub body_indent_level: usize,
    pub kind: BlockKind,
}

// ───────────────────────── Query functions ─────────────────────────

pub fn find_indent(spans: &[BlockSpan], line: u32) -> usize {
    spans
        .iter()
        .filter(|span| span.header_line < line && line < span.end_line)
        .map(|span| span.body_indent_level)
        .max()
        .unwrap_or(0)
}

/// Returns the deepest block that contains the given line.
pub fn find_enclosing_block(spans: &[BlockSpan], line: u32) -> Option<&BlockSpan> {
    spans
        .iter()
        .filter(|s| s.header_line < line && line < s.end_line)
        .max_by_key(|s| s.body_indent_level)
}

pub struct BlockSpanCollector {
    spans: Vec<BlockSpan>,
}

impl BlockSpanCollector {
    pub fn new() -> Self {
        BlockSpanCollector { spans: vec![] }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<BlockSpan> {
        unit.accept(&mut self);
        self.spans
    }

    fn register(
        &mut self,
        header_pos: &SourcePosition,
        end_pos: &SourcePosition,
        keyword_col: usize,
        kind: BlockKind,
    ) {
        self.spans.push(BlockSpan {
            header_line: header_pos.line as u32,
            end_line: end_pos.line as u32,
            body_indent_level: keyword_col / INDENT.len() + 1,
            kind,
        });
    }
}

impl AstVisitor for BlockSpanCollector {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let header_pos = SourcePosition::from_rustemo(&f.func_t.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        self.register(&header_pos, &end_pos, 0, BlockKind::Function);
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let if_pos = SourcePosition::from_rustemo(&s.if_t.position);
        let end_pos = SourcePosition::from_rustemo(&s.end_t.position);

        self.register(&if_pos, &end_pos, if_pos.column, BlockKind::If);

        let else_pos_opt = s.else_clause.as_ref().map(|else_clause| {
            SourcePosition::from_rustemo(&else_clause.else_t.position)
        });

        let elif_pos_opts = s.elseif_clause.as_ref().map(|else_ifs| {
            else_ifs.iter().map(|else_if| {
                SourcePosition::from_rustemo(&else_if.else_if_t.position)
            }).collect::<Vec<_>>()
        });

        if let Some(else_pos) = else_pos_opt {
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::Else);
        }

        if let Some(else_if_cols) = elif_pos_opts {
            for index in 0..else_if_cols.len() {
                let else_if_pos = &else_if_cols[index];
                let next_pos = else_if_cols.get(index + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                self.register(else_if_pos, &next_pos, else_if_pos.column, BlockKind::Elseif);
            }
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&f.for_t.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::For);
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&w.while_t.position);
        let end_pos = SourcePosition::from_rustemo(&w.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::While);
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.exists_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::Exists);
        if let Some(else_clause) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::ExistsElse);
        }
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::NotExists);
        if let Some(else_clause) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::NotExistsElse);
        }
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::Feedthrough);
        if let Some(else_clause) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::FeedthroughElse);
        }
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::NotFeedthrough);
        if let Some(else_clause) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::NotFeedthroughElse);
        }
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::MacroFor);
    }

    fn visit_macro_if(&mut self, m: &MacroIf) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        self.register(&header_pos, &end_pos, header_pos.column, BlockKind::MacroIf);

        if let Some(else_c) = &m.body.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, BlockKind::Else);
        }

        if let Some(elseifs) = &m.body.elseif_clause {
            let elif_positions: Vec<SourcePosition> = elseifs
                .iter()
                .map(|clause| SourcePosition::from_rustemo(&clause.else_if_t.position))
                .collect();

            for (i, elif_pos) in elif_positions.iter().enumerate() {
                let next_pos = elif_positions.get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                self.register(elif_pos, &next_pos, elif_pos.column, BlockKind::Elseif);
            }
        }
    }
}