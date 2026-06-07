use tml_parser::tml_actions::*;
use crate::position::SourcePosition;
use crate::symbol_table::{convert_type_spec, Scope, SymbolTable, SymbolType};
use crate::visitor::{AstVisitor, opt_iter};

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
        Scope::Function { name: fn_name, .. } => format!("fn {}", fn_name),
        Scope::Block(_) => "".to_string()
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
    scope_stack: Vec<Scope>,
    function_counter: u32,
}

impl HoverableCollector {
    pub fn new() -> Self {
        HoverableCollector { nodes: vec![], scope_stack: vec![], function_counter: 0 }
    }

    pub fn current_scope(&self) -> Scope {
        self.scope_stack.last().cloned().unwrap_or(Scope::Global)
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<HoverableNode> {
        unit.accept(&mut self);
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
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.nodes.push(HoverableNode {
            kind: HoverableKind::FunctionDef { name: f.id.value.clone() },
            position: SourcePosition::from_rustemo(&f.id.position),
            scope: Scope::Global,
        });

        let fn_id = { self.function_counter += 1; self.function_counter };
        let scope = Scope::Function { name: f.id.value.clone(), id: fn_id };

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

        self.scope_stack.push(scope);
    }

    fn leave_function_definition(&mut self, _f: &FunctionDefinition) {
        self.scope_stack.pop();
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        let scope = self.current_scope();
        match stmt {
            Statement::DeclarationStatement(d) => {
                self.push_decl(d, &scope);
            }
            Statement::AssignmentStatement(AssignmentStatement::VarAssignmentStatement(v)) => {
                if let Some(first_id) = v.var.names.first() {
                    self.push_variable_ref(first_id, &scope);
                }
            }
            _ => {}
        }
    }

    fn visit_postfix(&mut self, e: &PostfixExpression) {
        let scope = self.current_scope();
        match e {
            PostfixExpression::RValue(r) => {
                if let Some(first_id) = r._ref.names.first() {
                    self.push_variable_ref(first_id, &scope);
                }
            }
            _ => {}
        }
    }

    fn visit_function_call(&mut self, f: &FunctionCall) {
        let scope = self.current_scope();
        self.nodes.push(HoverableNode {
            kind: HoverableKind::FunctionCall {name: f.id.value.clone()},
            position: SourcePosition::from_rustemo(&f.id.position),
            scope
        })
    }
}
