use tml_parser::tml_actions::*;
use crate::position::SourcePosition;
use crate::symbol_table::{convert_type_spec, Scope, SymbolTable, SymbolType};
use crate::visitor::{AstVisitor, opt_iter, default_visit_external_declaration, default_visit_statement, default_visit_postfix};

// ───────────────────────── HoverableKind ─────────────────────────

#[derive(Debug, Clone)]
pub enum HoverableKind {
    /// Reference to an expression variable
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
    pub fn name(&self) -> &str {
        match &self.kind {
            HoverableKind::VariableRef { name }      => name,
            HoverableKind::VariableDecl { name, .. } => name,
            HoverableKind::FunctionCall { name }     => name,
            HoverableKind::FunctionDef { name }      => name,
        }
    }

    /// Build hover Markdown content based on symbol table
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
            HoverableKind::FunctionCall { name } => hover_for_function(name, table),
            HoverableKind::FunctionDef { name }  => hover_for_function(name, table),
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
        SymbolType::Simple(s)         => format!("{:?}", s).to_lowercase(),
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
        self.visit_translation_unit(unit);
        self.nodes
    }

    /// Find node at cursor position
    pub fn find_at(nodes: &[HoverableNode], line: u32, col: u32) -> Option<&HoverableNode> {
        nodes.iter().find(|n| {
            n.position.contains_cursor(line, col, n.name().len())
        })
    }

    fn push_variable_ref(&mut self, id: &Id, scope: &Scope) {
        self.nodes.push(HoverableNode {
            kind: HoverableKind::VariableRef { name: id.value.clone() },
            position: SourcePosition::from_rustemo(&id.position),
            scope: scope.clone(),
        });
    }

    fn push_decl(&mut self, d: &DeclarationStatement, scope: &Scope) {
        let first = d.id.names.first().unwrap();
        self.nodes.push(HoverableNode {
            kind: HoverableKind::VariableDecl {
                name: d.id.names.iter().map(|id| id.value.as_str()).collect::<Vec<_>>().join("."),
                ty: convert_type_spec(&d._type),
            },
            position: SourcePosition::from_rustemo(&first.position),
            scope: scope.clone(),
        });
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for HoverableCollector {
    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        // Record top-level declaration nodes before delegating
        if let ExternalDeclaration::DeclarationStatement(d) = decl {
            self.push_decl(d, &Scope::Global);
        }
        default_visit_external_declaration(self, decl);
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        // Record function definition node
        self.nodes.push(HoverableNode {
            kind: HoverableKind::FunctionDef { name: f.id.value.clone() },
            position: SourcePosition::from_rustemo(&f.id.position),
            scope: Scope::Global,
        });

        let scope = Scope::Function(f.id.value.clone());

        // Record parameter declarations
        for p in opt_iter(&f.parameters_list) {
            self.nodes.push(HoverableNode {
                kind: HoverableKind::VariableDecl {
                    name: p.id.value.clone(),
                    ty: convert_type_spec(&p._type),
                },
                position: SourcePosition::from_rustemo(&p.id.position),
                scope: scope.clone(),
            });
        }

        self.visit_statement_block(&f.statement_block, &scope);
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::DeclarationStatement(d) => {
                self.push_decl(d, scope);
                self.visit_expression(&d.rvalue, scope);
            }
            Statement::AssignmentStatement(AssignmentStatement::VarAssignmentStatement(v)) => {
                if let Some(first_id) = v.var.names.first() {
                    self.push_variable_ref(first_id, scope);
                }
                self.visit_expression(&v.rvalue, scope);
            }
            other => default_visit_statement(self, other, scope),
        }
    }

    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        match e {
            PostfixExpression::RValue(r) => {
                if let Some(first_id) = r._ref.names.first() {
                    self.push_variable_ref(first_id, scope);
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
            other => default_visit_postfix(self, other, scope),
        }
    }
}
