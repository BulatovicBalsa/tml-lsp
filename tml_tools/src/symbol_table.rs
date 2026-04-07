use tml_parser::tml_actions::*;

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
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::default(),
            errors: vec![],
        }
    }

    pub fn build(mut self, unit: &TranslationUnit) -> (SymbolTable, Vec<SymbolError>) {
        for decl in &unit.ext_decls {
            self.visit_external_declaration(decl);
        }
        (self.table, self.errors)
    }

    // ── External declarations ──

    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        match decl {
            ExternalDeclaration::FunctionDefinition(f)     => self.visit_function(f),
            ExternalDeclaration::DeclarationStatement(d)   => self.visit_declaration(d, Scope::Global),
            ExternalDeclaration::AssignmentStatement(_)    => {} // no new symbols
            ExternalDeclaration::IoDeclarationStatement(_) => {},
            ExternalDeclaration::IoWriteStatement(_)       => {}
            ExternalDeclaration::MacroFor(m)               => self.visit_for(&m.body, Scope::Global),
            ExternalDeclaration::MacroIf(m)                => self.visit_selection(&m.body, Scope::Global),
        }
    }

    // ── Function ──

    fn visit_function(&mut self, f: &FunctionDefinition) {
        let scope = Scope::Function(f.id.value.clone());

        let params: Vec<(SymbolType, String)> = match &f.parameters_list {
            None => vec![],
            Some(params) => params
                .iter()
                .map(|p| (convert_type_spec(&p._type), p.id.value.clone()))
                .collect(),
        };

        for (ty, name) in &params {
            self.add_symbol(&name, ty.clone(), scope.clone());
        }

        let ret_type = f.ret_type.as_ref().map(convert_type_spec);

        // Register function
        self.table.functions.push(FunctionSignature {
            name: f.id.value.clone(),
            params,
            ret_type,
        });

        // Function body
        self.visit_statement_block(&f.statement_block, &scope);
    }

    // ── Statement block ──

    fn visit_statement_block(&mut self, block: &StatementBlock, scope: &Scope) {
        for stmt in &block.statements {
            self.visit_statement(stmt, scope);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::DeclarationStatement(d)   => self.visit_declaration(d, scope.clone()),
            Statement::IoDeclarationStatement(_) => {},
            Statement::SelectionStatement(s)     => self.visit_selection(s, scope.clone()),
            Statement::IterationStatement(i)     => self.visit_iteration(i, scope),
            Statement::ExistsStatement(e)        => self.visit_statement_block(&e.statement_block, scope),
            Statement::NotExistsStatement(e)     => self.visit_statement_block(&e.statement_block, scope),
            Statement::FeedthroughStatement(e)   => self.visit_statement_block(&e.statement_block, scope),
            Statement::NotFeedthroughStatement(e) => self.visit_statement_block(&e.statement_block, scope),
            Statement::MacroFor(m)               => self.visit_for(&m.body, scope.clone()),
            Statement::MacroIf(m)                => self.visit_selection(&m.body, scope.clone()),
            Statement::AssignmentStatement(_)    => {},
            Statement::JumpStatement(_)          => {},
            Statement::FunctionCallStatement(_)  => {},
            Statement::IoWriteStatement(_)       => {},
            Statement::NoopStatement(_)          => {},
        }
    }

    // ── Declaration ──

    fn visit_declaration(&mut self, d: &DeclarationStatement, scope: Scope) {
        let name = dot_access_to_string(&d.id);
        let ty = convert_type_spec(&d._type);
        self.add_symbol(&name, ty, scope);
    }

    // ── Selection ──

    fn visit_selection(&mut self, s: &SelectionStatement, scope: Scope) {
        self.visit_statement_block(&s.if_statement_block, &scope);

        if let Some(elseifs) = &s.elseif_clause {
            for clause in elseifs {
                self.visit_statement_block(&clause.elseif_statement_block, &scope);
            }
        }
        if let Some(else_clause) = &s.else_clause {
            self.visit_statement_block(&else_clause.else_statement_block, &scope);
        }
    }

    // ── Iteration ──

    fn visit_iteration(&mut self, i: &IterationStatement, scope: &Scope) {
        match i {
            IterationStatement::ForIterationStatement(f)   => self.visit_for(f, scope.clone()),
            IterationStatement::WhileIterationStatement(w) => {
                self.visit_statement_block(&w.statement_block, scope)
            }
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement, scope: Scope) {
        // For loop index type is int
        self.add_symbol(
            &f.header.idx.value,
            SymbolType::Simple(SimpleTypeKind::Int),
            scope.clone(),
        );
        self.visit_statement_block(&f.body.statement_block, &scope);
    }

    // ── Helper ──

    fn add_symbol(&mut self, name: &str, ty: SymbolType, scope: Scope) {
        // Check for duplicates in the same scope
        let duplicate = self.table.symbols.iter().any(|s| {
            s.name == name && s.scope == scope
        });
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
        SimpleTypeSpec::IntT  => SimpleTypeKind::Int,
        SimpleTypeSpec::UintT => SimpleTypeKind::Uint,
        SimpleTypeSpec::RealT => SimpleTypeKind::Real,
        SimpleTypeSpec::BoolT => SimpleTypeKind::Bool,
        SimpleTypeSpec::StrT  => SimpleTypeKind::Str,
        SimpleTypeSpec::CharT => SimpleTypeKind::Char,
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
    // Simplified for tensor dimension only
    match e {
        Expression::MathExpression(MathExpression::PostfixExpression(
                                       PostfixExpression::Constant(Constant::Integer(i))
                                   )) => match i {
            Integer::C1(c) => c.value.clone(),
            Integer::C2(c) => c.value.clone(),
            Integer::C3(c) => c.value.clone(),
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
    /// Look for the symbol by name, Function Scope -> Global Scope Order
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