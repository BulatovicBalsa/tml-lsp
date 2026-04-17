use tml_parser::tml_actions::*;
use crate::diagnostics::{Diagnostic, DiagnosticSource};
use crate::position::SourcePosition;
use crate::symbol_table::{Scope, SymbolTable, dot_access_to_string};
use crate::visitor::{AstVisitor, default_visit_external_declaration, default_visit_statement, default_visit_postfix};

const RESERVED_NAMESPACES: &[&str] = &["t", "p", "n"];

fn is_namespace_root(name: &str) -> bool {
    RESERVED_NAMESPACES.contains(&name)
}

// ───────────────────────── Errors ─────────────────────────

#[derive(Debug, Clone)]
pub enum CheckError {
    UndefinedVariable { name: String, scope: Scope, position: SourcePosition },
    RedeclaredNamespace { name: String, position: SourcePosition },
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::UndefinedVariable { name, scope, .. } => match scope {
                Scope::Global => write!(f, "Undefined variable '{}'", name),
                Scope::Function(fn_name) => {
                    write!(f, "Undefined variable '{}' in function '{}'", name, fn_name)
                }
            },
            CheckError::RedeclaredNamespace { name, .. } => {
                write!(f, "Cannot redeclare reserved namespace variable '{}'", name)
            }
        }
    }
}

impl CheckError {
    pub fn position(&self) -> &SourcePosition {
        match self {
            CheckError::UndefinedVariable { position, .. } => position,
            CheckError::RedeclaredNamespace { position, .. } => position,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CheckError::UndefinedVariable { name, .. } => name,
            CheckError::RedeclaredNamespace { name, .. } => name,
        }
    }
}

// ───────────────────────── Checker ─────────────────────────

pub struct UndefinedVariableChecker<'a> {
    table: &'a SymbolTable,
    errors: Vec<CheckError>,
}

impl<'a> UndefinedVariableChecker<'a> {
    pub fn new(table: &'a SymbolTable) -> Self {
        UndefinedVariableChecker { table, errors: vec![] }
    }

    pub fn check(mut self, unit: &TranslationUnit) -> Vec<CheckError> {
        self.visit_translation_unit(unit);
        self.errors
    }

    fn check_rvalue(&mut self, dot: &DotAccessExpression, scope: &Scope) {
        let first_id = match dot.names.first() {
            Some(id) => id,
            None => return,
        };
        let root = first_id.value.as_str();

        if is_namespace_root(root) {
            return;
        }
        if self.table.lookup(root, scope).is_none() {
            self.errors.push(CheckError::UndefinedVariable {
                name: root.to_string(),
                scope: scope.clone(),
                position: SourcePosition::from_rustemo(&first_id.position),
            });
        }
    }

    fn check_namespace_redeclaration(&mut self, dot: &DotAccessExpression) {
        let first_id = match dot.names.first() {
            Some(id) => id,
            None => return,
        };
        let root = first_id.value.as_str();
        if is_namespace_root(root) {
            self.errors.push(CheckError::RedeclaredNamespace {
                name: dot_access_to_string(dot),
                position: SourcePosition::from_rustemo(&first_id.position),
            });
        }
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl<'a> AstVisitor for UndefinedVariableChecker<'a> {
    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        // Check for namespace re-declarations at the top level
        if let ExternalDeclaration::DeclarationStatement(d) = decl {
            self.check_namespace_redeclaration(&d.id);
        }
        default_visit_external_declaration(self, decl);
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        if let Statement::DeclarationStatement(d) = stmt {
            self.check_namespace_redeclaration(&d.id);
        }
        default_visit_statement(self, stmt, scope);
    }

    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        if let PostfixExpression::RValue(r) = e {
            self.check_rvalue(&r._ref, scope);
            return; // RValue has no children to traverse
        }
        default_visit_postfix(self, e, scope);
    }
}

// ───────────────────────── DiagnosticSource impl ─────────────────────────

pub struct UndefinedVariableDiagnosticSource;

impl DiagnosticSource for UndefinedVariableDiagnosticSource {
    fn diagnostics(&self, ast: &TranslationUnit, table: &SymbolTable) -> Vec<Diagnostic> {
        UndefinedVariableChecker::new(table)
            .check(ast)
            .into_iter()
            .map(|e| Diagnostic::error(
                e.to_string(),
                e.position().line as u32,
                e.position().column as u32,
                e.name().len(),
            ))
            .collect()
    }
}
