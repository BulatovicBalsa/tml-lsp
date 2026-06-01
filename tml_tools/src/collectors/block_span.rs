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
    /// Nesting level of the body (0 = global, 1 = inside fn, 2 = inside fn+if, ...)
    pub body_indent_level: usize,
    /// Exact column where body content should start (keyword_col + INDENT.len())
    pub body_col: usize,
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

/// Returns the exact column where body content should start for the deepest
/// enclosing block. Use this for indentation in on_type_formatting and completion.
pub fn find_body_col(spans: &[BlockSpan], line: u32) -> usize {
    spans
        .iter()
        .filter(|s| s.header_line < line && line < s.end_line)
        .max_by_key(|s| s.body_col)
        .map(|s| s.body_col)
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
        parent_level: usize,
        kind: BlockKind,
    ) {
        self.spans.push(BlockSpan {
            header_line: header_pos.line as u32,
            end_line: end_pos.line as u32,
            body_indent_level: parent_level + 1,
            body_col: keyword_col + INDENT.len(),
            kind,
        });
    }
}

impl AstVisitor for BlockSpanCollector {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let header_pos = SourcePosition::from_rustemo(&f.func_t.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        // fn is always at global scope (level 0)
        self.register(&header_pos, &end_pos, 0, 0, BlockKind::Function);
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let if_pos = SourcePosition::from_rustemo(&s.if_t.position);
        let end_pos = SourcePosition::from_rustemo(&s.end_t.position);
        // parent_level is the level at the if keyword line
        let if_parent_level = find_indent(&self.spans, if_pos.line as u32);

        self.register(&if_pos, &end_pos, if_pos.column, if_parent_level, BlockKind::If);

        // elseif and else are siblings of if — same parent_level
        if let Some(elseifs) = &s.elseif_clause {
            let elif_positions: Vec<SourcePosition> = elseifs
                .iter()
                .map(|c| SourcePosition::from_rustemo(&c.else_if_t.position))
                .collect();
            for (i, elif_pos) in elif_positions.iter().enumerate() {
                let next_pos = elif_positions.get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                self.register(elif_pos, &next_pos, elif_pos.column, if_parent_level, BlockKind::Elseif);
            }
        }

        if let Some(else_c) = &s.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, if_parent_level, BlockKind::Else);
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&f.for_t.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::For);
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&w.while_t.position);
        let end_pos = SourcePosition::from_rustemo(&w.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::While);
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.exists_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::Exists);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, parent_level, BlockKind::ExistsElse);
        }
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::NotExists);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, parent_level, BlockKind::NotExistsElse);
        }
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::Feedthrough);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, parent_level, BlockKind::FeedthroughElse);
        }
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::NotFeedthrough);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, parent_level, BlockKind::NotFeedthroughElse);
        }
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, parent_level, BlockKind::MacroFor);
    }

    fn visit_macro_if(&mut self, m: &MacroIf) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        let macro_parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, &end_pos, header_pos.column, macro_parent_level, BlockKind::MacroIf);

        // elseif and else are siblings of macro if — same parent_level
        if let Some(elseifs) = &m.body.elseif_clause {
            let elif_positions: Vec<SourcePosition> = elseifs
                .iter()
                .map(|c| SourcePosition::from_rustemo(&c.else_if_t.position))
                .collect();
            for (i, elif_pos) in elif_positions.iter().enumerate() {
                let next_pos = elif_positions.get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                self.register(elif_pos, &next_pos, elif_pos.column, macro_parent_level, BlockKind::Elseif);
            }
        }

        if let Some(else_c) = &m.body.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, &end_pos, else_pos.column, macro_parent_level, BlockKind::Else);
        }
    }
}