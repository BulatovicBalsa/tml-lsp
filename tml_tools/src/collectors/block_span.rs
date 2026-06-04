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

// ───────────────────────── KeywordSpan ─────────────────────────

/// Position and length of a single keyword token.
#[derive(Debug, Clone)]
pub struct KeywordSpan {
    pub line: u32,
    pub col: u32,
    pub len: usize,
}

impl KeywordSpan {
    pub fn from_source(pos: &SourcePosition, len: usize) -> Self {
        KeywordSpan {
            line: pos.line as u32,
            col: pos.column as u32,
            len,
        }
    }
}

// ───────────────────────── BlockSpan ─────────────────────────

#[derive(Debug, Clone)]
pub struct BlockSpan {
    /// Opening keyword (fn, if, elseif, else, for, while, ...)
    pub header: KeywordSpan,
    /// Used for indentation — for elseif/else this points to the next sibling's header
    pub end: KeywordSpan,
    /// Always the real `end` keyword — used for document highlight
    pub block_end: KeywordSpan,
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
        .filter(|s| s.header.line < line && line < s.end.line)
        .map(|s| s.body_indent_level)
        .max()
        .unwrap_or(0)
}

pub fn find_body_col(spans: &[BlockSpan], line: u32) -> usize {
    spans
        .iter()
        .filter(|s| s.header.line < line && line < s.end.line)
        .max_by_key(|s| s.body_col)
        .map(|s| s.body_col)
        .unwrap_or(0)
}

pub fn find_enclosing_block(spans: &[BlockSpan], line: u32) -> Option<&BlockSpan> {
    spans
        .iter()
        .filter(|s| s.header.line < line && line < s.end.line)
        .max_by_key(|s| s.body_indent_level)
}

/// Returns the span whose header or end keyword is on the given line.
/// Used for document highlight — clicking on a keyword highlights its pair.
pub fn find_span_at_keyword(spans: &[BlockSpan], line: u32) -> Option<&BlockSpan> {
    spans
        .iter()
        .find(|s| s.header.line == line || s.end.line == line)
}

/// Returns the (header, end) keyword pair when cursor is within either keyword.
/// When multiple spans share the same end line (if/elseif/else), the one whose
/// header contains the cursor gets priority. If no header contains the cursor,
/// the outermost block (lowest body_indent_level) is returned — this ensures
/// clicking `end` highlights the `if` that owns it, not a sibling elseif.
pub fn find_highlight(spans: &[BlockSpan], line: u32, character: u32) -> Option<(KeywordSpan, KeywordSpan)> {
    spans.iter()
        .filter(|s| {
            let on_header = s.header.line == line
                && character >= s.header.col
                && character < s.header.col + s.header.len as u32;
            let on_end = s.block_end.line == line
                && character >= s.block_end.col
                && character < s.block_end.col + s.block_end.len as u32;
            on_header || on_end
        })
        .min_by_key(|s| {
            let cursor_on_header = s.header.line == line
                && character >= s.header.col
                && character < s.header.col + s.header.len as u32;
            if cursor_on_header {
                (0u32, 0u32)
            } else {
                (s.header.line, s.header.col)
            }
        })
        .map(|s| (s.header.clone(), s.block_end.clone()))
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
        header_len: usize,
        end_pos: &SourcePosition,       // for indentation (next sibling for elseif/else)
        block_end_pos: &SourcePosition, // real end_t - for highlight
        keyword_col: usize,
        parent_level: usize,
        kind: BlockKind,
    ) {
        self.spans.push(BlockSpan {
            header: KeywordSpan::from_source(header_pos, header_len),
            end: KeywordSpan::from_source(end_pos, "end".len()),
            block_end: KeywordSpan::from_source(block_end_pos, "end".len()),
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
        self.register(&header_pos, f.func_t.value.len(), &end_pos, &end_pos, 0, 0, BlockKind::Function);
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let if_pos = SourcePosition::from_rustemo(&s.if_t.position);
        let end_pos = SourcePosition::from_rustemo(&s.end_t.position);
        let if_parent_level = find_indent(&self.spans, if_pos.line as u32);
        self.register(&if_pos, s.if_t.value.len(), &end_pos, &end_pos, if_pos.column, if_parent_level, BlockKind::If);

        if let Some(elseifs) = &s.elseif_clause {
            let elif_positions: Vec<(SourcePosition, usize)> = elseifs
                .iter()
                .map(|c| (SourcePosition::from_rustemo(&c.else_if_t.position), c.else_if_t.value.len()))
                .collect();
            let end_positions: Vec<SourcePosition> = elif_positions.iter().map(|(p, _)| p.clone()).collect();
            for (i, (elif_pos, elif_len)) in elif_positions.iter().enumerate() {
                let next_pos = end_positions.get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                // end_pos is next sibling (for indentation), block_end is real end_t
                self.register(elif_pos, *elif_len, &next_pos, &end_pos, elif_pos.column, if_parent_level, BlockKind::Elseif);
            }
        }

        if let Some(else_c) = &s.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, if_parent_level, BlockKind::Else);
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&f.for_t.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, f.for_t.value.len(), &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::For);
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let header_pos = SourcePosition::from_rustemo(&w.while_t.position);
        let end_pos = SourcePosition::from_rustemo(&w.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, w.while_t.value.len(), &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::While);
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.exists_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, e.exists_t.value.len(), &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::Exists);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, parent_level, BlockKind::ExistsElse);
        }
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        let span = e.not_t.value.len() + 1 + e.exists_t.value.len();
        self.register(&header_pos, span, &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::NotExists);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, parent_level, BlockKind::NotExistsElse);
        }
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, e.feedthrough_t.value.len(), &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::Feedthrough);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, parent_level, BlockKind::FeedthroughElse);
        }
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        let span = e.not_t.value.len() + 1 + e.feedthrough_t.value.len();
        self.register(&header_pos, span, &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::NotFeedthrough);
        if let Some(else_c) = &e.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, parent_level, BlockKind::NotFeedthroughElse);
        }
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        let parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, m.macro_t.value.len(), &end_pos, &end_pos, header_pos.column, parent_level, BlockKind::MacroFor);
    }

    fn visit_macro_if(&mut self, m: &MacroIf) {
        let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        let macro_parent_level = find_indent(&self.spans, header_pos.line as u32);
        self.register(&header_pos, m.macro_t.value.len(), &end_pos, &end_pos, header_pos.column, macro_parent_level, BlockKind::MacroIf);

        if let Some(elseifs) = &m.body.elseif_clause {
            let elif_data: Vec<(SourcePosition, usize)> = elseifs
                .iter()
                .map(|c| (SourcePosition::from_rustemo(&c.else_if_t.position), c.else_if_t.value.len()))
                .collect();
            let elif_positions: Vec<SourcePosition> = elif_data.iter().map(|(p, _)| p.clone()).collect();
            for (i, (elif_pos, elif_len)) in elif_data.iter().enumerate() {
                let next_pos = elif_positions.get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| end_pos.clone());
                // end_pos is next sibling (for indentation), block_end is real end_t
                self.register(elif_pos, *elif_len, &next_pos, &end_pos, elif_pos.column, macro_parent_level, BlockKind::Elseif);
            }
        }

        if let Some(else_c) = &m.body.else_clause {
            let else_pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.register(&else_pos, else_c.else_t.value.len(), &end_pos, &end_pos, else_pos.column, macro_parent_level, BlockKind::Else);
        }
    }
}