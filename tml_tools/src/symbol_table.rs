use std::collections::HashSet;
use tml_parser::tml_actions::*;
use crate::constants::{is_predefined_literal, is_reserved_namespace};
use crate::position::SourcePosition;
use crate::type_inference::infer_type;
use crate::types::{FunctionSignature, Scope, SimpleTypeKind, Symbol, SymbolError, SymbolType};
use crate::visitor::{opt_iter, AstVisitor};

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub functions: Vec<FunctionSignature>,
}

pub struct SymbolTableBuilder {
    table: SymbolTable,
    errors: HashSet<SymbolError>,
    scope_stack: Vec<Scope>,
    function_counter: u32,
    /// Keyword positions waiting to be consumed by visit_statement_block.
    pending_block_pos: Vec<(u32, u32)>,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::default(),
            errors: HashSet::new(),
            scope_stack: vec![],
            function_counter: 0,
            pending_block_pos: vec![],
        }
    }

    pub fn build(mut self, unit: &TranslationUnit) -> (SymbolTable, Vec<SymbolError>) {
        self.prescan_functions(unit);
        unit.accept(&mut self);
        (self.table, self.errors.into_iter().collect())
    }

    fn prescan_functions(&mut self, unit: &TranslationUnit) {
        for decl in &unit.ext_decls {
            if let ExternalDeclaration::FunctionDefinition(f) = decl {
                let name = &f.id.value;
                if let Some(existing) = self.table.functions.iter().find(|sig| &sig.name == name) {
                    // error for the original function
                    self.errors.insert(SymbolError::new(
                        name,
                        &format!("Function '{}' is already defined", name),
                        Some(existing.position.clone())
                    ));

                    // error for the duplicate function
                    self.errors.insert(SymbolError::new(
                        name,
                        &format!("Function '{}' is already defined", name),
                        Some(SourcePosition::from_rustemo(&f.id.position)),
                    ));
                } else {
                    self.table.functions.push(self.build_function_signature(f));
                }
            }
        }
    }

    fn build_function_signature(&self, f: &FunctionDefinition) -> FunctionSignature {
        let params = opt_iter(&f.parameters_list)
            .map(|p| (convert_type_spec(&p._type), p.id.value.clone()))
            .collect();
        let ret_type = f.ret_type.as_ref().map(convert_type_spec);
        FunctionSignature {
            name: f.id.value.clone(),
            params,
            ret_type,
            position: SourcePosition::from_rustemo(&f.id.position)
        }
    }

    fn current_scope(&self) -> Scope {
        self.scope_stack.iter().rev()
            .find(|s| !matches!(s, Scope::TransparentBlock | Scope::MacroIndexBlock { .. }))
            .cloned()
            .unwrap_or(Scope::Global)
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

    // ── Symbol helpers ──

    fn handle_io_declaration(&mut self, d: &IoDeclarationStatement) {
        let name = dot_access_to_string(&d.id);
        let position = dot_access_position(&d.id);
        let ty = convert_type_spec(&d.io_type._type);
        self.add_symbol(&name, ty, Some(position));
    }

    fn handle_declaration(&mut self, d: &DeclarationStatement) {
        let name = dot_access_to_string(&d.id);
        let position = dot_access_position(&d.id);
        let ty = convert_type_spec(&d._type);
        self.add_symbol(&name, ty, Some(position));
    }

    fn handle_assignment(&mut self, stmt: &AssignmentStatement) {
        if let AssignmentStatement::VarAssignmentStatement(v) = stmt {
            let name = dot_access_to_string(&v.var);
            let position = dot_access_position(&v.var);
            if self.table.lookup_in_stack(&name, &self.scope_stack).is_none() {
                if let Some(ty) = infer_type(&v.rvalue, &self.table, &self.scope_stack) {
                    self.add_symbol(&name, ty, Some(position));
                }
            }
        }
    }

    fn add_symbol_in_scope(&mut self, name: &str, ty: SymbolType, position: Option<SourcePosition>, scope: Scope) {
        let is_duplicate = self.table.symbols.iter().any(|s| s.name == name && s.scope == scope);

        if is_duplicate {
            self.errors.insert(SymbolError::new(name, &format!("'{}' is already defined in this scope", name), position));
            return;
        }

        // Don't add reserved namespaces or predefined literals —
        // UndefinedVariableChecker reports these as semantic errors.
        if is_reserved_namespace(name) || is_predefined_literal(name) {
            return;
        }

        self.table.symbols.push(Symbol { name: name.to_string(), ty, scope });
    }

    fn add_symbol(&mut self, name: &str, ty: SymbolType, position: Option<SourcePosition>) {
        let scope = self.current_scope();
        let is_duplicate = self.table.symbols.iter().any(|s| s.name == name && s.scope == scope);

        if is_duplicate {
            self.errors.insert(SymbolError::new(
                name,
                &format!("'{}' is already defined in this scope", name),
                position,
            ));
            return;
        }

        // Don't add reserved namespaces or predefined literals to the symbol table —
        // UndefinedVariableChecker reports these as semantic errors.
        if is_reserved_namespace(name) || is_predefined_literal(name) {
            return;
        }

        self.table.symbols.push(Symbol { name: name.to_string(), ty, scope });
    }

}

impl AstVisitor for SymbolTableBuilder {
    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        match decl {
            ExternalDeclaration::DeclarationStatement(d)   => self.handle_declaration(d),
            ExternalDeclaration::AssignmentStatement(a)    => self.handle_assignment(a),
            ExternalDeclaration::IoDeclarationStatement(d) => self.handle_io_declaration(d),
            _ => {}
        }
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        self.function_counter += 1;
        self.scope_stack.push(Scope::Function {
            name: f.id.value.clone(),
            id: self.function_counter,
        });
        for p in opt_iter(&f.parameters_list) {
            self.add_symbol(&p.id.value, convert_type_spec(&p._type), Some(SourcePosition::from_rustemo(&p.id.position)));
        }
    }

    fn leave_function_definition(&mut self, _f: &FunctionDefinition) {
        self.scope_stack.pop();
    }

    fn visit_statement_block(&mut self, _b: &StatementBlock) {
        if let Some((line, col)) = self.pending_block_pos.pop() {
            self.enter_block(line, col);
        }
        // If no pending pos (e.g. function body), do nothing —
        // function body vars go into Function scope directly.
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::DeclarationStatement(d)   => self.handle_declaration(d),
            Statement::AssignmentStatement(a)    => self.handle_assignment(a),
            Statement::IoDeclarationStatement(d) => self.handle_io_declaration(d),
            _ => {}
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
        // Close previous if/elseif block, open new one for this elseif
        // (unless inside a macro_if, which is transparent)
        if !self.is_directly_in_transparent_block() {
            self.exit_block();
            let pos = SourcePosition::from_rustemo(&c.else_if_t.position);
            self.pending_block_pos.push((pos.line as u32, pos.column as u32));
        }
    }

    fn visit_else_clause(&mut self, c: &ElseClause) {
        // Close previous if/elseif block, open new one for else
        // (unless inside a macro_if, which is transparent)
        if !self.is_directly_in_transparent_block() {
            self.exit_block();
            let pos = SourcePosition::from_rustemo(&c.else_t.position);
            self.pending_block_pos.push((pos.line as u32, pos.column as u32));
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let fot_t_pos = SourcePosition::from_rustemo(&f.for_t.position);
        let index_pos = SourcePosition::from_rustemo(&f.header.idx.position);
        
        self.enter_block(fot_t_pos.line as u32, fot_t_pos.column as u32);
        self.add_symbol(&f.header.idx.value, SymbolType::Simple(SimpleTypeKind::Int), Some(index_pos.clone()));
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
        let index_scope = Scope::MacroIndexBlock { line: pos.line as u32, col: pos.column as u32 };
        // Add index variable into MacroIndexBlock scope (with namespace/predef checks)
        let idx_pos = SourcePosition::from_rustemo(&m.body.header.idx.position);
        self.add_symbol_in_scope(
            &m.body.header.idx.value,
            SymbolType::Simple(SimpleTypeKind::Int),
            Some(idx_pos),
            index_scope.clone(),
        );
        self.scope_stack.push(index_scope);
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

pub fn dot_access_position(d: &DotAccessExpression) -> SourcePosition {
    SourcePosition::from_rustemo(&d.names.first().unwrap().position)
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

    pub fn lookup_in_stack(&self, name: &str, stack: &[Scope]) -> Option<&Symbol> {
        for scope in stack.iter().rev() {
            if let Some(s) = self.symbols.iter().find(|s| s.name == name && &s.scope == scope) {
                return Some(s);
            }
        }
        // Always fall back to global scope
        self.symbols.iter().find(|s| s.name == name && s.scope == Scope::Global)
    }

    /// Get all symbols in the given scope
    pub fn symbols_in_scope(&self, scope: &Scope) -> Vec<&Symbol> {
        self.symbols.iter().filter(|s| &s.scope == scope).collect()
    }
}