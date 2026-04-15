use tml_parser::tml_actions::TranslationUnit;
use crate::symbol_table::SymbolTable;

// ───────────────────────── Diagnostic types ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Hint,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub line: u32,
    pub column: u32,
    pub length: usize,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, line: u32, column: u32, length: usize) -> Self {
        Diagnostic {
            message: message.into(),
            severity: DiagnosticSeverity::Error,
            line,
            column,
            length,
        }
    }

    pub fn warning(message: impl Into<String>, line: u32, column: u32, length: usize) -> Self {
        Diagnostic {
            message: message.into(),
            severity: DiagnosticSeverity::Warning,
            line,
            column,
            length,
        }
    }

    pub fn hint(message: impl Into<String>, line: u32, column: u32, length: usize) -> Self {
        Diagnostic {
            message: message.into(),
            severity: DiagnosticSeverity::Hint,
            line,
            column,
            length,
        }
    }
}

// ───────────────────────── DiagnosticSource trait ─────────────────────────

/// Any checker that can produce diagnostics implements this trait.
/// Adding a new checker only requires implementing this trait and
/// registering it in DiagnosticsRunner — no other changes needed.
pub trait DiagnosticSource {
    fn diagnostics(&self, ast: &TranslationUnit, table: &SymbolTable) -> Vec<Diagnostic>;
}

// ───────────────────────── DiagnosticsRunner ─────────────────────────

/// Collects diagnostics from all registered sources.
/// Open for extension (add new sources), closed for modification.
#[derive(Default)]
pub struct DiagnosticsRunner {
    sources: Vec<Box<dyn DiagnosticSource>>,
}

impl DiagnosticsRunner {
    pub fn new() -> Self {
        DiagnosticsRunner { sources: vec![] }
    }

    pub fn add_source(mut self, source: impl DiagnosticSource + 'static) -> Self {
        self.sources.push(Box::new(source));
        self
    }

    pub fn run(&self, ast: &TranslationUnit, table: &SymbolTable) -> Vec<Diagnostic> {
        self.sources
            .iter()
            .flat_map(|s| s.diagnostics(ast, table))
            .collect()
    }
}