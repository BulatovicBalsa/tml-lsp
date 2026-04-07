use tml_parser::tml_actions::*;
use crate::symbol_table::{Scope, SymbolTable, dot_access_to_string};
use crate::visitor::AstVisitor;

const RESERVED_NAMESPACES: &[&str] = &["t", "p", "n"];

fn is_namespace_root(name: &str) -> bool {
    RESERVED_NAMESPACES.contains(&name)
}

// ───────────────────────── Errors ─────────────────────────

#[derive(Debug, Clone)]
pub enum CheckError {
    UndefinedVariable { name: String, scope: Scope },
    RedeclaredNamespace { name: String },
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::UndefinedVariable { name, scope } => match scope {
                Scope::Global => write!(f, "Undefined variable '{}'", name),
                Scope::Function(fn_name) => {
                    write!(f, "Undefined variable '{}' in function '{}'", name, fn_name)
                }
            },
            CheckError::RedeclaredNamespace { name } => {
                write!(f, "Cannot redeclare reserved namespace variable '{}'", name)
            }
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
        for decl in &unit.ext_decls {
            match decl {
                ExternalDeclaration::FunctionDefinition(f) => {
                    let scope = Scope::Function(f.id.value.clone());
                    self.visit_statement_block(&f.statement_block, &scope);
                }
                ExternalDeclaration::DeclarationStatement(d) => {
                    self.check_namespace_redeclaration(&d.id);
                    self.visit_expression(&d.rvalue, &Scope::Global);
                }
                ExternalDeclaration::AssignmentStatement(s) => {
                    self.visit_assignment(s, &Scope::Global);
                }
                ExternalDeclaration::IoWriteStatement(s) => {
                    self.visit_io_write(s, &Scope::Global);
                }
                ExternalDeclaration::MacroFor(m) => {
                    self.visit_for(&m.body, &Scope::Global);
                }
                ExternalDeclaration::MacroIf(m) => {
                    self.visit_selection(&m.body, &Scope::Global);
                }
                ExternalDeclaration::IoDeclarationStatement(_) => {}
            }
        }
        self.errors
    }

    fn check_rvalue(&mut self, dot: &DotAccessExpression, scope: &Scope) {
        let root = dot.names.first().map(|s| s.value.as_str()).unwrap_or("");
        if is_namespace_root(root) {
            return;
        }
        if self.table.lookup(root, scope).is_none() {
            self.errors.push(CheckError::UndefinedVariable {
                name: root.to_string(),
                scope: scope.clone(),
            });
        }
    }

    fn check_namespace_redeclaration(&mut self, dot: &DotAccessExpression) {
        let root = dot.names.first().map(|s| s.value.as_str()).unwrap_or("");
        if is_namespace_root(root) {
            self.errors.push(CheckError::RedeclaredNamespace {
                name: dot_access_to_string(dot),
            });
        }
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl<'a> AstVisitor for UndefinedVariableChecker<'a> {
    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::DeclarationStatement(d) => {
                self.check_namespace_redeclaration(&d.id);
                self.visit_expression(&d.rvalue, scope);
            }
            other => {
                match other {
                    Statement::AssignmentStatement(s)     => self.visit_assignment(s, scope),
                    Statement::IoWriteStatement(s)        => self.visit_io_write(s, scope),
                    Statement::FunctionCallStatement(s)   => self.visit_function_call(&s.call, scope),
                    Statement::SelectionStatement(s)      => self.visit_selection(s, scope),
                    Statement::IterationStatement(i)      => self.visit_iteration(i, scope),
                    Statement::JumpStatement(j)           => self.visit_jump(j, scope),
                    Statement::ExistsStatement(e)         => {
                        self.visit_statement_block(&e.statement_block, scope);
                        if let Some(else_c) = &e.else_clause {
                            self.visit_statement_block(&else_c.else_statement_block, scope);
                        }
                    }
                    Statement::NotExistsStatement(e)      => {
                        self.visit_statement_block(&e.statement_block, scope);
                        if let Some(else_c) = &e.else_clause {
                            self.visit_statement_block(&else_c.else_statement_block, scope);
                        }
                    }
                    Statement::FeedthroughStatement(e)    => {
                        self.visit_statement_block(&e.statement_block, scope);
                        if let Some(else_c) = &e.else_clause {
                            self.visit_statement_block(&else_c.else_statement_block, scope);
                        }
                    }
                    Statement::NotFeedthroughStatement(e) => {
                        self.visit_statement_block(&e.statement_block, scope);
                        if let Some(else_c) = &e.else_clause {
                            self.visit_statement_block(&else_c.else_statement_block, scope);
                        }
                    }
                    Statement::MacroFor(m)                => self.visit_for(&m.body, scope),
                    Statement::MacroIf(m)                 => self.visit_selection(&m.body, scope),
                    Statement::IoDeclarationStatement(_)  => {}
                    Statement::NoopStatement(_)           => {}
                    Statement::DeclarationStatement(_)    => unreachable!(),
                }
            }
        }
    }

    // Override visit_postfix to check rvalues
    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        match e {
            PostfixExpression::RValue(r) => self.check_rvalue(&r._ref, scope),
            PostfixExpression::FunctionCall(f)        => self.visit_function_call(f, scope),
            PostfixExpression::TensorExpression(t)    => {
                self.visit_expression(&t.expr, scope);
                self.visit_index_expression_list(&t.index, scope);
            }
            PostfixExpression::TransposeExpression(t) => self.visit_postfix(&t.expr, scope),
            PostfixExpression::ExprInParenthesis(e)   => self.visit_expression(&e.expr, scope),
            PostfixExpression::AttributeAccess(a)     => self.visit_expression(&a.expr, scope),
            PostfixExpression::TensorLiteral(t)       => self.visit_cube(&t.expr, scope),
            PostfixExpression::Constant(_)            => {}
            PostfixExpression::InputExpression(_)     => {}
        }
    }
}