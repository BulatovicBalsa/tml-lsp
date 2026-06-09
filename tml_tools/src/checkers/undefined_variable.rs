use crate::constants::{is_predefined_literal, is_reserved_namespace};
use crate::diagnostics::{Diagnostic, DiagnosticSource};
use crate::position::SourcePosition;
use crate::symbol_table::{dot_access_to_string, Scope, SymbolTable};
use crate::visitor::AstVisitor;
use tml_parser::tml_actions::*;

#[derive(Debug, Clone)]
pub enum CheckError {
    UndefinedVariable { name: String, scope: Scope, position: SourcePosition },
    RedeclaredNamespace { name: String, position: SourcePosition },
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::UndefinedVariable { name, scope, .. } => match scope {
                Scope::Block { .. } |
                Scope::MacroIndexBlock { .. } |
                Scope::TransparentBlock |
                Scope::Global => write!(f, "Undefined variable '{}'", name),
                Scope::Function { name: fn_name, .. } => {
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
    scope_stack: Vec<Scope>,
    function_counter: u32,
    pending_block_pos: Vec<(u32, u32)>,
}

impl<'a> UndefinedVariableChecker<'a> {
    pub fn new(table: &'a SymbolTable) -> Self {
        UndefinedVariableChecker {
            table,
            errors: vec![],
            scope_stack: vec![],
            function_counter: 0,
            pending_block_pos: vec![],
        }
    }

    pub fn current_scope(&self) -> Scope {
        self.scope_stack.iter().rev()
            .find(|s| !matches!(s, Scope::TransparentBlock | Scope::MacroIndexBlock { .. }))
            .cloned()
            .unwrap_or(Scope::Global)
    }

    pub fn check(mut self, unit: &TranslationUnit) -> Vec<CheckError> {
        unit.accept(&mut self);
        self.errors
    }

    fn enter_block(&mut self, line: u32, col: u32) {
        self.scope_stack.push(Scope::Block { line, col });
    }

    fn exit_block(&mut self) {
        self.scope_stack.pop();
    }

    fn is_directly_in_transparent_block(&self) -> bool {
        matches!(self.scope_stack.last(), Some(Scope::TransparentBlock))
    }

    fn check_rvalue(&mut self, dot: &DotAccessExpression) {
        let first_id = match dot.names.first() {
            Some(id) => id,
            None => return,
        };
        let root = first_id.value.as_str();

        if is_reserved_namespace(root) && dot.names.len() > 1 {
            return;
        }
        if is_predefined_literal(root) && dot.names.len() == 1 {
            return;
        }
        if self.table.lookup_in_stack(root, &self.scope_stack).is_none() {
            let scope = self.current_scope();
            self.errors.push(CheckError::UndefinedVariable {
                name: root.to_string(),
                scope,
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
        if is_reserved_namespace(root) && dot.names.len() == 1 {
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
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.function_counter += 1;
        self.scope_stack.push(Scope::Function {
            name: f.id.value.clone(),
            id: self.function_counter,
        });
    }

    fn leave_function_definition(&mut self, _f: &FunctionDefinition) {
        self.scope_stack.pop();
    }

    fn visit_statement_block(&mut self, _b: &StatementBlock) {
        if let Some((line, col)) = self.pending_block_pos.pop() {
            self.enter_block(line, col);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        if let Statement::DeclarationStatement(d) = stmt {
            self.check_namespace_redeclaration(&d.id);
        }
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let pos = SourcePosition::from_rustemo(&s.if_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_selection(&mut self, _s: &SelectionStatement) {
        self.exit_block();
    }

    fn visit_else_if_clause(&mut self, c: &ElseIfClause) {
        // Only exit block if we're not inside a macro_if
        // (macro_if branches are transparent and share the same scope)
        if !self.is_directly_in_transparent_block() {
            self.exit_block();
            let pos = SourcePosition::from_rustemo(&c.else_if_t.position);
            self.pending_block_pos.push((pos.line as u32, pos.column as u32));
        }
    }

    fn visit_else_clause(&mut self, c: &ElseClause) {
        // Only exit block if we're not inside a macro_if
        // (macro_if branches are transparent and share the same scope)
        if !self.is_directly_in_transparent_block() {
            self.exit_block();
            let pos = SourcePosition::from_rustemo(&c.else_t.position);
            self.pending_block_pos.push((pos.line as u32, pos.column as u32));
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let pos = SourcePosition::from_rustemo(&f.for_t.position);
        self.enter_block(pos.line as u32, pos.column as u32);

        let index_pos = SourcePosition::from_rustemo(&f.header.idx.position);
        self.pending_block_pos.push((index_pos.line as u32, index_pos.column as u32));
    }

    fn leave_for(&mut self, _f: &ForIterationStatement) {
        self.exit_block(); // body block
        self.exit_block(); // for index block
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let pos = SourcePosition::from_rustemo(&w.while_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_while(&mut self, _w: &WhileIterationStatement) {
        self.exit_block();
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let pos = SourcePosition::from_rustemo(&e.exists_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_exists(&mut self, _e: &ExistsStatement) {
        self.exit_block();
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let pos = SourcePosition::from_rustemo(&e.not_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_not_exists(&mut self, _e: &NotExistsStatement) {
        self.exit_block();
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_feedthrough(&mut self, _e: &FeedthroughStatement) {
        self.exit_block();
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let pos = SourcePosition::from_rustemo(&e.not_t.position);
        self.pending_block_pos.push((pos.line as u32, pos.column as u32));
    }

    fn leave_not_feedthrough(&mut self, _e: &NotFeedthroughStatement) {
        self.exit_block();
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        self.scope_stack.push(Scope::TransparentBlock);
        let pos = SourcePosition::from_rustemo(&m.macro_t.position);
        self.scope_stack.push(Scope::MacroIndexBlock { line: pos.line as u32, col: pos.column as u32 });
    }

    fn leave_macro_for(&mut self, _m: &MacroFor) {
        self.scope_stack.pop(); // pop MacroIndexBlock
        self.scope_stack.pop(); // pop TransparentBlock
    }

    fn visit_macro_if(&mut self, _node: &MacroIf) {
        self.scope_stack.push(Scope::TransparentBlock);
    }

    fn leave_macro_if(&mut self, _node: &MacroIf) {
        self.scope_stack.pop();
    }

    fn visit_assignment(&mut self, a: &AssignmentStatement) {
        match a {
            AssignmentStatement::VarAssignmentStatement(v) => {
                self.check_namespace_redeclaration(&v.var)
            }
            AssignmentStatement::TensorAssignmentStatement(t) => {
                self.check_namespace_redeclaration(&t.tensor.expr)
            }
        }
    }

    fn visit_postfix(&mut self, e: &PostfixExpression) {
        if let PostfixExpression::RValue(r) = e {
            self.check_rvalue(&r._ref);
        }
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
