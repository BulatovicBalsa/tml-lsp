use crate::symbol_table::Scope::Global;
use crate::visitor::AstVisitor;
use tml_parser::tml_actions::*;

macro_rules! start_line {
    ($e:expr) => {
        $e.header_colon.position.line_col
            .map(|lc| lc.line.saturating_sub(2))
            .unwrap_or(0) as u32
    };
}

macro_rules! handle_stmt {
    ($self:ident, $scope:ident, $e:ident) => {
        {
            let start_line = start_line!($e);

            $self.try_add_range(start_line, &$e.end_t.position);
            $self.visit_statement_block(&$e.statement_block, $scope);

            if let Some(else_c) = &$e.else_clause {
                $self.visit_statement_block(&else_c.else_statement_block, $scope);
            }
        }
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
    text: &'a str,
}

impl<'a> FoldingCollector<'a> {
    pub fn new(text: &'a str) -> Self {
        FoldingCollector {
            ranges: vec![],
            text,
        }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<TmlFoldingRange> {
        for decl in &unit.ext_decls {
            match decl {
                ExternalDeclaration::FunctionDefinition(f) => self.visit_function(f),
                ExternalDeclaration::MacroFor(m)           => self.visit_for(&m.body, &Global),
                ExternalDeclaration::MacroIf(m)            => self.visit_selection(&m.body, &Global),
                _ => {}
            }
        }
        self.ranges
    }

    /// Proverava da li linija sadrzi samo whitespace i 'end'
    fn is_end_on_own_line(&self, line: usize) -> bool {
        self.text
            .lines()
            .nth(line)
            .map(|l| l.trim() == "end")
            .unwrap_or(false)
    }

    fn try_add_range(&mut self, start_line: u32, end_position: &rustemo::Position) {
        let end_line = end_position.line_col
            .map(|lc| lc.line.saturating_sub(1))
            .unwrap_or(0);

        if self.is_end_on_own_line(end_line) && start_line < end_line as u32 {
            self.ranges.push(TmlFoldingRange {
                start_line,
                end_line: end_line as u32,
            });
        }
    }

    fn visit_function(&mut self, f: &FunctionDefinition) {
        let start_line = start_line!(f);

        self.try_add_range(start_line, &f.end_t.position);

        let scope = crate::symbol_table::Scope::Function(f.id.value.clone());
        self.visit_statement_block(&f.statement_block, &scope);
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl<'a> AstVisitor for FoldingCollector<'a> {
    fn visit_expression(&mut self, _expr: &Expression, _scope: &crate::symbol_table::Scope) {
        // Expressions ne generisu folding range-ove
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &crate::symbol_table::Scope) {

        match stmt {
            Statement::SelectionStatement(s)  => self.visit_selection(s, scope),
            Statement::IterationStatement(i)  => self.visit_iteration(i, scope),
            Statement::ExistsStatement(e) => handle_stmt!(self, scope, e),
            Statement::NotExistsStatement(e) => handle_stmt!(self, scope, e),
            Statement::FeedthroughStatement(e) => handle_stmt!(self, scope, e),
            Statement::NotFeedthroughStatement(e) => handle_stmt!(self, scope, e),
            Statement::MacroFor(m) => self.visit_for(&m.body, scope),
            Statement::MacroIf(m)  => self.visit_selection(&m.body, scope),
            // Ostali statement-i ne generisu folding
            _ => {}
        }
    }

    fn visit_selection(&mut self, s: &SelectionStatement, scope: &crate::symbol_table::Scope) {
        let start_line = start_line!(s);

        self.try_add_range(start_line, &s.end_t.position);
        self.visit_statement_block(&s.if_statement_block, scope);

        // elseif i else blokovi
        if let Some(elseifs) = &s.elseif_clause {
            for clause in elseifs {
                self.visit_statement_block(&clause.elseif_statement_block, scope);
            }
        }
        if let Some(else_c) = &s.else_clause {
            self.visit_statement_block(&else_c.else_statement_block, scope);
        }
    }

    fn visit_iteration(&mut self, i: &IterationStatement, scope: &crate::symbol_table::Scope) {
        match i {
            IterationStatement::ForIterationStatement(f) => self.visit_for(f, scope),
            IterationStatement::WhileIterationStatement(w) => {
                let start_line = start_line!(w);
                self.try_add_range(start_line, &w.end_t.position);
                self.visit_statement_block(&w.statement_block, scope);
            }
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement, scope: &crate::symbol_table::Scope) {
        let start_line = start_line!(f);

        self.try_add_range(start_line, &f.end_t.position);
        self.visit_statement_block(&f.body.statement_block, scope);
    }
}