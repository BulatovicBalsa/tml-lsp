use crate::diagnostics::{Diagnostic, DiagnosticSource};
use crate::constants::INDENT;
use crate::position::SourcePosition;
use crate::symbol_table::SymbolTable;
use crate::visitor::AstVisitor;
use tml_parser::tml_actions::*;

// ───────────────────────── Helper ─────────────────────────

/// A block is empty if it has no statements at all.
/// A block containing `pass` (NoopStatement) is NOT considered empty —
/// `pass` is the explicit way to mark an intentionally empty body.
fn is_empty_block(block: &StatementBlock) -> bool {
    match &block.statements {
        None => true,
        Some(stmts) => stmts.is_empty(),
    }
}

// ───────────────────────── Checker ─────────────────────────

pub struct EmptyBodyChecker {
    errors: Vec<EmptyBodyError>,
}

#[derive(Debug, Clone)]
pub struct EmptyBodyError {
    pub message: String,
    pub keyword_position: SourcePosition,
    pub length: usize,

    /// Line where 'pass' should be inserted (from header_colon.position, 0-based).
    pub insert_line: u32,
    pub indent: String,
}

impl EmptyBodyChecker {
    pub fn new() -> Self {
        EmptyBodyChecker { errors: vec![] }
    }

    pub fn check(mut self, unit: &TranslationUnit) -> Vec<EmptyBodyError> {
        unit.accept(&mut self);
        self.errors
    }

    fn check_block(
        &mut self,
        block: &StatementBlock,
        message: &str,
        keyword_pos: SourcePosition,
        length: usize,
        header_colon: &HeaderColon,
    ) {
        if !is_empty_block(block) {
            return;
        }
        let indent = " ".repeat(keyword_pos.column + INDENT.len());
        let insert_line = SourcePosition::from_rustemo(&header_colon.position).line as u32;
        self.errors.push(EmptyBodyError {
            message: message.to_string(),
            keyword_position: keyword_pos,
            length,
            insert_line,
            indent,
        });
    }

    fn check_block_fn(
        &mut self,
        block: &StatementBlock,
        message: &str,
        function_name_position: SourcePosition,
        length: usize,
        header_colon: &HeaderColon,
        func_t_position: SourcePosition,
    ) {
        if !is_empty_block(block) {
            return;
        }
        let indent = " ".repeat(func_t_position.column + INDENT.len());
        let insert_line = SourcePosition::from_rustemo(&header_colon.position).line as u32;
        self.errors.push(EmptyBodyError {
            message: message.to_string(),
            keyword_position: function_name_position,
            length,
            insert_line,
            indent,
        });
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for EmptyBodyChecker {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let pos = SourcePosition::from_rustemo(&f.id.position);
        self.check_block_fn(
            &f.statement_block,
            &format!("Function '{}' has an empty body — add 'pass' if intentional", f.id.value),
            pos, f.id.value.len(), &f.header_colon, SourcePosition::from_rustemo(&f.func_t.position)
        );
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let pos = SourcePosition::from_rustemo(&s.if_t.position);
        self.check_block(
            &s.if_statement_block,
            "Empty 'if' body — add 'pass' if intentional",
            pos, s.if_t.value.len(), &s.header_colon,
        );
        if let Some(elseifs) = &s.elseif_clause {
            for clause in elseifs {
                let pos = SourcePosition::from_rustemo(&clause.else_if_t.position);
                self.check_block(
                    &clause.elseif_statement_block,
                    "Empty 'elseif' body — add 'pass' if intentional",
                    pos, clause.else_if_t.value.len(), &clause.header_colon,
                );
            }
        }
        if let Some(else_c) = &s.else_clause {
            let pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.check_block(
                &else_c.else_statement_block,
                "Empty 'else' body — add 'pass' if intentional",
                pos, else_c.else_t.value.len(), &else_c.header_colon,
            );
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let pos = SourcePosition::from_rustemo(&f.for_t.position);
        self.check_block(
            &f.body.statement_block,
            &format!("Empty 'for {}' body — add 'pass' if intentional", f.header.idx.value),
            pos, f.for_t.value.len(), &f.header_colon,
        );
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let pos = SourcePosition::from_rustemo(&w.while_t.position);
        self.check_block(
            &w.statement_block,
            "Empty 'while' body — add 'pass' if intentional",
            pos, w.while_t.value.len(), &w.header_colon,
        );
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let pos = SourcePosition::from_rustemo(&e.exists_t.position);
        self.check_block(
            &e.statement_block,
            "Empty 'exists' body — add 'pass' if intentional",
            pos, e.exists_t.value.len(), &e.header_colon,
        );
        if let Some(else_c) = &e.else_clause {
            let pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.check_block(
                &else_c.else_statement_block,
                "Empty 'exists else' body — add 'pass' if intentional",
                pos, else_c.else_t.value.len(), &else_c.header_colon,
            );
        }
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let span = e.not_t.value.len() + 1 + e.exists_t.value.len();
        self.check_block(
            &e.statement_block,
            "Empty 'not exists' body — add 'pass' if intentional",
            not_pos, span, &e.header_colon,
        );
        if let Some(else_c) = &e.else_clause {
            let pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.check_block(
                &else_c.else_statement_block,
                "Empty 'not exists else' body — add 'pass' if intentional",
                pos, else_c.else_t.value.len(), &else_c.header_colon,
            );
        }
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        self.check_block(
            &e.statement_block,
            "Empty 'feedthrough' body — add 'pass' if intentional",
            pos, e.feedthrough_t.value.len(), &e.header_colon,
        );
        if let Some(else_c) = &e.else_clause {
            let pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.check_block(
                &else_c.else_statement_block,
                "Empty 'feedthrough else' body — add 'pass' if intentional",
                pos, else_c.else_t.value.len(), &else_c.header_colon,
            );
        }
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
        let span = e.not_t.value.len() + 1 + e.feedthrough_t.value.len();
        self.check_block(
            &e.statement_block,
            "Empty 'not feedthrough' body — add 'pass' if intentional",
            not_pos, span, &e.header_colon,
        );
        if let Some(else_c) = &e.else_clause {
            let pos = SourcePosition::from_rustemo(&else_c.else_t.position);
            self.check_block(
                &else_c.else_statement_block,
                "Empty 'not feedthrough else' body — add 'pass' if intentional",
                pos, else_c.else_t.value.len(), &else_c.header_colon,
            );
        }
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        let pos = SourcePosition::from_rustemo(&m.macro_t.position);
        self.check_block(
            &m.body.body.statement_block,
            &format!("Empty 'macro for {}' body — add 'pass' if intentional", m.body.header.idx.value),
            pos, m.macro_t.value.len(), &m.body.header_colon,
        );
    }

    fn visit_macro_if(&mut self, m: &MacroIf) {
        let pos = SourcePosition::from_rustemo(&m.macro_t.position);
        self.check_block(
            &m.body.if_statement_block,
            "Empty 'macro if' body — add 'pass' if intentional",
            pos, m.macro_t.value.len(), &m.body.header_colon,
        );
    }
}

// ───────────────────────── DiagnosticSource impl ─────────────────────────

pub struct EmptyBodyDiagnosticSource;

impl DiagnosticSource for EmptyBodyDiagnosticSource {
    fn diagnostics(&self, ast: &TranslationUnit, _table: &SymbolTable) -> Vec<Diagnostic> {
        EmptyBodyChecker::new()
            .check(ast)
            .into_iter()
            .map(|e| Diagnostic::error(
                e.message,
                e.keyword_position.line as u32,
                e.keyword_position.column as u32,
                e.length,
            ))
            .collect()
    }
}
