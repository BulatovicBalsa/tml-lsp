use tml_parser::tml_actions::*;
use crate::position::SourcePosition;
use crate::symbol_table::{convert_type_spec, Scope, SymbolTable, SymbolType};
use crate::visitor::AstVisitor;

// ───────────────────────── HoverableKind ─────────────────────────

#[derive(Debug, Clone)]
pub enum HoverableKind {
    /// Reference to expression variable
    VariableRef { name: String },
    /// Declaration: `int x = 5`
    VariableDecl { name: String, ty: SymbolType },
    /// Function call: `foo(1, 2)`
    FunctionCall { name: String },
    /// Function definition: `fn foo(...)`
    FunctionDef { name: String },
}

// ───────────────────────── HoverableNode ─────────────────────────

#[derive(Debug, Clone)]
pub struct HoverableNode {
    pub kind: HoverableKind,
    pub position: SourcePosition,
    pub scope: Scope,
}

impl HoverableNode {
    fn name(&self) -> &str {
        match &self.kind {
            HoverableKind::VariableRef { name }  => name,
            HoverableKind::VariableDecl { name, .. } => name,
            HoverableKind::FunctionCall { name } => name,
            HoverableKind::FunctionDef { name }  => name,
        }
    }

    /// Make hover Markdown content based on symbol table
    pub fn hover_content(&self, table: &SymbolTable) -> Option<String> {
        match &self.kind {
            HoverableKind::VariableRef { name } => {
                hover_for_variable(name, &self.scope, table)
            }
            HoverableKind::VariableDecl { name, ty } => {
                Some(format!(
                    "```tml\n{} {}\n```\n*declaration*",
                    format_type(ty),
                    name
                ))
            }
            HoverableKind::FunctionCall { name } => {
                hover_for_function(name, table)
            }
            HoverableKind::FunctionDef { name } => {
                hover_for_function(name, table)
            }
        }
    }
}

// ───────────────────────── Hover helpers ─────────────────────────

fn hover_for_variable(name: &str, scope: &Scope, table: &SymbolTable) -> Option<String> {
    let symbol = table.lookup(name, scope)?;
    let scope_str = match &symbol.scope {
        Scope::Global => "global".to_string(),
        Scope::Function(fn_name) => format!("fn {}", fn_name),
    };
    Some(format!(
        "```tml\n{}: {}\n```\n*scope: {}*",
        symbol.name,
        format_type(&symbol.ty),
        scope_str
    ))
}

fn hover_for_function(name: &str, table: &SymbolTable) -> Option<String> {
    let func = table.lookup_function(name)?;
    let params = func
        .params
        .iter()
        .map(|(ty, name)| format!("{} {}", format_type(ty), name))
        .collect::<Vec<_>>()
        .join(", ");
    let ret = match &func.ret_type {
        None => String::new(),
        Some(ty) => format!(" {}", format_type(ty)),
    };
    Some(format!("```tml\nfn {}({}){}\n```", func.name, params, ret))
}

pub fn format_type(ty: &SymbolType) -> String {
    match ty {
        SymbolType::Simple(s) => format!("{:?}", s).to_lowercase(),
        SymbolType::Tensor(inner, dims) => {
            format!("tensor<{}, {}>", format_type(inner), dims.join(", "))
        }
        SymbolType::Derived(name) => format!("{}.type", name),
    }
}

// ───────────────────────── Collector ─────────────────────────

pub struct HoverableCollector {
    pub nodes: Vec<HoverableNode>,
}

impl HoverableCollector {
    pub fn new() -> Self {
        HoverableCollector { nodes: vec![] }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<HoverableNode> {
        for decl in &unit.ext_decls {
            match decl {
                ExternalDeclaration::FunctionDefinition(f) => {
                    // Add function definition
                    self.nodes.push(HoverableNode {
                        kind: HoverableKind::FunctionDef { name: f.id.value.clone() },
                        position: SourcePosition::from_rustemo(&f.id.position),
                        scope: Scope::Global,
                    });
                    let scope = Scope::Function(f.id.value.clone());

                    if let Some(params) = &f.parameters_list {
                        for p in params {
                            self.nodes.push(HoverableNode {
                                kind: HoverableKind::VariableDecl {
                                    name: p.id.value.clone(),
                                    ty: convert_type_spec(&p._type),
                                },
                                position: SourcePosition::from_rustemo(&p.id.position),
                                scope: scope.clone(),
                            });
                        }
                    }
                    self.visit_statement_block(&f.statement_block, &scope);
                }
                ExternalDeclaration::DeclarationStatement(d) => {
                    self.add_decl(d, &Scope::Global);
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
        self.nodes
    }

    fn add_decl(&mut self, d: &DeclarationStatement, scope: &Scope) {
        self.nodes.push(HoverableNode {
            kind: HoverableKind::VariableDecl {
                name: d.id.names.iter().map(|id| id.value.as_str()).collect::<Vec<_>>().join("."),
                ty: convert_type_spec(&d._type),
            },
            position: SourcePosition::from_rustemo(&d.id.names.first().unwrap().position),
            scope: scope.clone(),
        });
    }

    /// Find node at cursor position
    pub fn find_at(nodes: &[HoverableNode], line: u32, col: u32) -> Option<&HoverableNode> {
        nodes.iter().find(|n| {
            n.position.contains_cursor(line, col, n.name().len())
        })
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for HoverableCollector {
    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::DeclarationStatement(d) => {
                self.add_decl(d, scope);
                self.visit_expression(&d.rvalue, scope);
            }
            other => {
                match other {
                    Statement::AssignmentStatement(s) => {
                        match s {
                            AssignmentStatement::VarAssignmentStatement(v) => {
                                if let Some(first_id) = v.var.names.first() {
                                    self.nodes.push(HoverableNode {
                                        kind: HoverableKind::VariableRef {
                                            name: first_id.value.clone()
                                        },
                                        position: SourcePosition::from_rustemo(&first_id.position),
                                        scope: scope.clone(),
                                    });
                                }
                                self.visit_expression(&v.rvalue, scope);
                            }
                            AssignmentStatement::TensorAssignmentStatement(s) => {
                                self.visit_lvalue_indices(&s.tensor, scope);
                                self.visit_expression(&s.rvalue, scope);
                            }
                        }
                    }
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

    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        match e {
            PostfixExpression::RValue(r) => {
                if let Some(first_id) = r._ref.names.first() {
                    self.nodes.push(HoverableNode {
                        kind: HoverableKind::VariableRef {
                            name: first_id.value.clone()
                        },
                        position: SourcePosition::from_rustemo(&first_id.position),
                        scope: scope.clone(),
                    });
                }
            }
            PostfixExpression::FunctionCall(f) => {
                self.nodes.push(HoverableNode {
                    kind: HoverableKind::FunctionCall { name: f.id.value.clone() },
                    position: SourcePosition::from_rustemo(&f.id.position),
                    scope: scope.clone(),
                });
                self.visit_function_call(f, scope);
            }
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