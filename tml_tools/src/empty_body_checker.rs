use tml_parser::tml_actions::*;
use crate::diagnostics::{Diagnostic, DiagnosticSource};
use crate::position::SourcePosition;
use crate::symbol_table::{Scope, SymbolTable};
use crate::visitor::{AstVisitor, default_visit_statement};

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
    pub position: SourcePosition,
    pub length: usize,
}

impl EmptyBodyChecker {
    pub fn new() -> Self {
        EmptyBodyChecker { errors: vec![] }
    }

    pub fn check(mut self, unit: &TranslationUnit) -> Vec<EmptyBodyError> {
        self.visit_translation_unit(unit);
        self.errors
    }

    fn report_error(&mut self, message: impl Into<String>, position: SourcePosition, length: usize) {
        self.errors.push(EmptyBodyError {
            message: message.into(),
            position,
            length,
        });
    }

    fn check_block(
        &mut self,
        block: &StatementBlock,
        message: &str,
        position: SourcePosition,
        length: usize,
    ) {
        if is_empty_block(block) {
            self.report_error(message, position, length);
        }
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for EmptyBodyChecker {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.check_block(
            &f.statement_block,
            &format!("Function '{}' has an empty body — add 'pass' if intentional", f.id.value),
            SourcePosition::from_rustemo(&f.id.position),
            f.id.value.len(),
        );
        // Continue traversal so nested constructs inside the function are also checked
        let scope = Scope::Function(f.id.value.clone());
        self.visit_statement_block(&f.statement_block, &scope);
    }

    // Skip all expression traversal — irrelevant for this check
    fn visit_expression(&mut self, _expr: &Expression, _scope: &Scope) {}

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::SelectionStatement(s) => {
                self.check_block(
                    &s.if_statement_block,
                    "Empty 'if' body — add 'pass' if intentional",
                    SourcePosition::from_rustemo(&s.if_t.position),
                    s.if_t.value.len(),
                );
                if let Some(elseifs) = &s.elseif_clause {
                    for clause in elseifs {
                        self.check_block(
                            &clause.elseif_statement_block,
                            "Empty 'elseif' body — add 'pass' if intentional",
                            SourcePosition::from_rustemo(&clause.else_if_t.position),
                            clause.else_if_t.value.len(),
                        );
                    }
                }
                if let Some(else_c) = &s.else_clause {
                    self.check_block(
                        &else_c.else_statement_block,
                        "Empty 'else' body — add 'pass' if intentional",
                        SourcePosition::from_rustemo(&else_c.else_t.position),
                        else_c.else_t.value.len(),
                    );
                }
                self.visit_selection(s, scope);
            }
            Statement::IterationStatement(i) => {
                match i {
                    IterationStatement::ForIterationStatement(f) => {
                        self.check_block(
                            &f.body.statement_block,
                            &format!("Empty 'for {}' body — add 'pass' if intentional", f.header.idx.value),
                            SourcePosition::from_rustemo(&f.for_t.position),
                            f.for_t.value.len(),
                        );
                    }
                    IterationStatement::WhileIterationStatement(w) => {
                        self.check_block(
                            &w.statement_block,
                            "Empty 'while' body — add 'pass' if intentional",
                            SourcePosition::from_rustemo(&w.while_t.position),
                            w.while_t.value.len(),
                        );
                    }
                }
                self.visit_iteration(i, scope);
            }
            Statement::ExistsStatement(e) => {
                self.check_block(
                    &e.statement_block,
                    "Empty 'exists' body — add 'pass' if intentional",
                    SourcePosition::from_rustemo(&e.exists_t.position),
                    e.exists_t.value.len(),
                );
                if let Some(else_c) = &e.else_clause {
                    self.check_block(
                        &else_c.else_statement_block,
                        "Empty 'exists else' body — add 'pass' if intentional",
                        SourcePosition::from_rustemo(&else_c.else_t.position),
                        else_c.else_t.value.len(),
                    );
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            }
            Statement::NotExistsStatement(e) => {
                // Underline from 'not' through 'exists' as one span
                let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
                let span = e.not_t.value.len() + 1 + e.exists_t.value.len();
                self.check_block(
                    &e.statement_block,
                    "Empty 'not exists' body — add 'pass' if intentional",
                    not_pos, span,
                );
                if let Some(else_c) = &e.else_clause {
                    self.check_block(
                        &else_c.else_statement_block,
                        "Empty 'not exists else' body — add 'pass' if intentional",
                        SourcePosition::from_rustemo(&else_c.else_t.position),
                        else_c.else_t.value.len(),
                    );
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            }
            Statement::FeedthroughStatement(e) => {
                self.check_block(
                    &e.statement_block,
                    "Empty 'feedthrough' body — add 'pass' if intentional",
                    SourcePosition::from_rustemo(&e.feedthrough_t.position),
                    e.feedthrough_t.value.len(),
                );
                if let Some(else_c) = &e.else_clause {
                    self.check_block(
                        &else_c.else_statement_block,
                        "Empty 'feedthrough else' body — add 'pass' if intentional",
                        SourcePosition::from_rustemo(&else_c.else_t.position),
                        else_c.else_t.value.len(),
                    );
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            }
            Statement::NotFeedthroughStatement(e) => {
                // Underline from 'not' through 'feedthrough' as one span
                let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
                let span = e.not_t.value.len() + 1 + e.feedthrough_t.value.len();
                self.check_block(
                    &e.statement_block,
                    "Empty 'not feedthrough' body — add 'pass' if intentional",
                    not_pos, span,
                );
                if let Some(else_c) = &e.else_clause {
                    self.check_block(
                        &else_c.else_statement_block,
                        "Empty 'not feedthrough else' body — add 'pass' if intentional",
                        SourcePosition::from_rustemo(&else_c.else_t.position),
                        else_c.else_t.value.len(),
                    );
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            }
            Statement::MacroFor(m) => {
                self.check_block(
                    &m.body.body.statement_block,
                    &format!("Empty 'for {}' body — add 'pass' if intentional", m.body.header.idx.value),
                    SourcePosition::from_rustemo(&m.body.for_t.position),
                    m.body.for_t.value.len(),
                );
                self.visit_for(&m.body, scope);
            }
            Statement::MacroIf(m) => {
                self.visit_statement(&Statement::SelectionStatement(m.body.clone()), scope);
            }
            other => default_visit_statement(self, other, scope),
        }
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
                e.position.line as u32,
                e.position.column as u32,
                e.length,
            ))
            .collect()
    }
}
