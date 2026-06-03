use tml_parser::tml_actions::*;
use crate::type_inference::infer_type;
use crate::visitor::{AstVisitor, opt_iter};

// ───────────────────────── Types ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleTypeKind {
    Int,
    Uint,
    Real,
    Bool,
    Str,
    Char,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Simple(SimpleTypeKind),
    Tensor(Box<SymbolType>, Vec<String>), // type + dimensions as strings
    Derived(String),                      // "t.in", "a", "t.in_operand[]"
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Global,
    Function(String),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: SymbolType,
    pub scope: Scope,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<(SymbolType, String)>,
    pub ret_type: Option<SymbolType>,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub functions: Vec<FunctionSignature>,
}

// ───────────────────────── Errors ─────────────────────────

#[derive(Debug, Clone)]
pub struct SymbolError {
    pub message: String,
    pub symbol_name: String,
}

impl SymbolError {
    fn new(symbol_name: &str, message: &str) -> Self {
        SymbolError {
            symbol_name: symbol_name.to_string(),
            message: message.to_string(),
        }
    }
}

// ───────────────────────── Builder ─────────────────────────

pub struct SymbolTableBuilder {
    table: SymbolTable,
    errors: Vec<SymbolError>,
    scope_stack: Vec<Scope>,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::default(),
            errors: vec![],
            scope_stack: vec![],
        }
    }

    pub fn build(mut self, unit: &TranslationUnit) -> (SymbolTable, Vec<SymbolError>) {
        self.prescan_functions(unit);
        unit.accept(&mut self);
        (self.table, self.errors)
    }

    fn prescan_functions(&mut self, unit: &TranslationUnit) {
        for decl in &unit.ext_decls {
            if let ExternalDeclaration::FunctionDefinition(f) = decl {
                self.table.functions.push(self.build_function_signature(f));
            }
        }
    }

    fn build_function_signature(&self, f: &FunctionDefinition) -> FunctionSignature {
        let params = opt_iter(&f.parameters_list)
            .map(|p| (convert_type_spec(&p._type), p.id.value.clone()))
            .collect();
        let ret_type = f.ret_type.as_ref().map(convert_type_spec);
        FunctionSignature { name: f.id.value.clone(), params, ret_type }
    }

    fn current_scope(&self) -> Scope {
        self.scope_stack.last().cloned().unwrap_or(Scope::Global)
    }

    // ── Symbol helpers ──

    fn handle_declaration(&mut self, d: &DeclarationStatement) {
        let name = dot_access_to_string(&d.id);
        let ty = convert_type_spec(&d._type);
        self.add_symbol(&name, ty);
    }

    fn handle_assignment(&mut self, stmt: &AssignmentStatement) {
        if let AssignmentStatement::VarAssignmentStatement(v) = stmt {
            let name = dot_access_to_string(&v.var);
            let scope = self.current_scope();
            if self.table.lookup(&name, &scope).is_none() {
                if let Some(ty) = infer_type(&v.rvalue, &self.table, &scope) {
                    self.add_symbol(&name, ty);
                }
            }
        }
    }

    fn add_symbol(&mut self, name: &str, ty: SymbolType) {
        let scope = self.current_scope();
        let duplicate = self.table.symbols.iter().any(|s| s.name == name && s.scope == scope);
        if duplicate {
            self.errors.push(SymbolError::new(
                name,
                &format!("'{}' is already defined in this scope", name),
            ));
        } else {
            self.table.symbols.push(Symbol { name: name.to_string(), ty, scope });
        }
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for SymbolTableBuilder {
    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        match decl {
            ExternalDeclaration::DeclarationStatement(d) => self.handle_declaration(d),
            ExternalDeclaration::AssignmentStatement(a)  => self.handle_assignment(a),
            _ => {}
        }
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.scope_stack.push(Scope::Function(f.id.value.clone()));
        for p in opt_iter(&f.parameters_list) {
            self.add_symbol(&p.id.value, convert_type_spec(&p._type));
        }
    }

    fn leave_function_definition(&mut self, _f: &FunctionDefinition) {
        self.scope_stack.pop();
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::DeclarationStatement(d) => self.handle_declaration(d),
            Statement::AssignmentStatement(a)  => self.handle_assignment(a),
            _ => {}
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        self.add_symbol(&f.header.idx.value, SymbolType::Simple(SimpleTypeKind::Int));
    }
}

// ───────────────────────── Conversions ─────────────────────────

pub fn convert_type_spec(ts: &TypeSpec) -> SymbolType {
    match ts {
        TypeSpec::SimpleType(s)       => SymbolType::Simple(convert_simple(&s._type)),
        TypeSpec::DerivedType(d)      => SymbolType::Derived(derived_type_to_string(d)),
        TypeSpec::TensorConstructor(t) => {
            let inner = convert_type_spec(&t._type);
            let dims = t.dimensions.iter().map(expr_to_string).collect();
            SymbolType::Tensor(Box::new(inner), dims)
        }
    }
}

fn convert_simple(s: &SimpleTypeSpec) -> SimpleTypeKind {
    match s {
        SimpleTypeSpec::IntT(_)  => SimpleTypeKind::Int,
        SimpleTypeSpec::UintT(_) => SimpleTypeKind::Uint,
        SimpleTypeSpec::RealT(_) => SimpleTypeKind::Real,
        SimpleTypeSpec::BoolT(_) => SimpleTypeKind::Bool,
        SimpleTypeSpec::StrT(_)  => SimpleTypeKind::Str,
        SimpleTypeSpec::CharT(_) => SimpleTypeKind::Char,
    }
}

fn derived_type_to_string(d: &DerivedType) -> String {
    let base = dot_access_to_string(&d.name);
    let brackets = count_brackets(&d.brackets);
    format!("{}{}", base, "[]".repeat(brackets))
}

pub fn dot_access_to_string(d: &DotAccessExpression) -> String {
    let base = d.names.iter().map(|id| id.value.clone()).collect::<Vec<_>>().join(".");
    let optional = if d.optional.is_some() { "?" } else { "" };
    format!("{}{}", base, optional)
}

fn expr_to_string(e: &Expression) -> String {
    // Simplified for tensor dimension expressions only
    match e {
        Expression::MathExpression(MathExpression::PostfixExpression(
            PostfixExpression::Constant(Constant::Integer(i))
        )) => match i {
            Integer::C1(c) => c.value.clone().value,
            Integer::C2(c) => c.value.clone().value,
            Integer::C3(c) => c.value.clone().value,
        },
        Expression::MathExpression(MathExpression::PostfixExpression(
            PostfixExpression::RValue(r)
        )) => dot_access_to_string(&r._ref),
        _ => "<expr>".to_string(),
    }
}

fn count_brackets(b: &SquareBrackets0) -> usize {
    match b {
        None => 0,
        Some(inner) => 1 + count_brackets_inner(inner),
    }
}

fn count_brackets_inner(b: &SquareBrackets1) -> usize {
    match b {
        SquareBrackets1::SquareBrackets => 0,
        SquareBrackets1::SquareBrackets1(inner) => 1 + count_brackets_inner(inner),
    }
}

// ───────────────────────── Lookup ─────────────────────────

impl SymbolTable {
    /// Look for the symbol by name, Function Scope -> Global Scope order
    pub fn lookup(&self, name: &str, scope: &Scope) -> Option<&Symbol> {
        // Current scope first
        if let Some(s) = self.symbols.iter().find(|s| s.name == name && &s.scope == scope) {
            return Some(s);
        }
        // Global scope next
        self.symbols.iter().find(|s| s.name == name && s.scope == Scope::Global)
    }

    /// Look for function signature by name
    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get all symbols in the given scope
    pub fn symbols_in_scope(&self, scope: &Scope) -> Vec<&Symbol> {
        self.symbols.iter().filter(|s| &s.scope == scope).collect()
    }
}