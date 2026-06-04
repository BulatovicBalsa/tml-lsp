use tml_parser::tml_actions::{AssignmentStatement, Boolean, Constant, DeclarationStatement, ElseClause, ElseIfClause, ExistsStatement, ExternalDeclaration, FeedthroughStatement, ForIterationStatement, FunctionCall, FunctionDefinition, Id, Integer, IoDeclarationStatement, IoDirection, IoWriteStatement, MacroFor, MacroIf, NotExistsStatement, NotFeedthroughStatement, PostfixExpression, SelectionStatement, SimpleTypeSpec, Statement, TranslationUnit, TypeSpec, UnsignedInteger, WhileIterationStatement};
use tml_parser::visitor::AstVisitor;
use crate::constants::is_reserved_namespace;
use crate::position::SourcePosition;

macro_rules! const_pos_len {
    ($val:expr, $($variant:path),+) => {
        match $val {
            $($variant(c) => (SourcePosition::from_rustemo(&c.value.position), c.value.value.len()),)+
        }
    };
}

// Token type indices — must match the order in SemanticTokensLegend in main.rs
#[repr(u32)]
#[derive(Clone, Debug)]
pub enum TokenType {
    Keyword = 0,
    Variable = 1,
    Function = 2,
    Parameter = 3,
    Property = 4,
    Type = 5,
    Number = 6,
    String = 7,
    Namespace = 8,
}

// Token modifier bitmask — must match SemanticTokensLegend modifiers
pub struct TokenModifiers;
impl TokenModifiers {
    pub const NONE: u32        = 0;
    pub const DECLARATION: u32 = 1 << 0;
}

// A single token with absolute position
#[derive(Debug, Clone)]
pub struct RawToken {
    pub line: u32,
    pub col: u32,
    pub len: usize,
    pub token_type: TokenType,
    pub modifiers: u32,
}

impl RawToken {
    pub fn new(line: u32, col: u32, len: usize, token_type: TokenType, modifiers: u32) -> Self {
        RawToken { line, col, len, token_type, modifiers }
    }
}

pub struct SemanticTokenCollector {
    tokens: Vec<RawToken>
}

impl SemanticTokenCollector {
    pub fn new() -> Self {
        SemanticTokenCollector { tokens: vec![] }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<RawToken> {
        unit.accept(&mut self);
        self.tokens
    }

    fn push(&mut self, line: u32, col: u32, len: usize, token_type: TokenType, modifiers: u32) {
        self.tokens.push(RawToken::new(line, col, len, token_type, modifiers));
    }

    fn push_declaration(&mut self, d: &DeclarationStatement) {
        self.push_type_spec(&d._type);
        let names = &d.id.names;
        let first = &names[0];
        let pos = SourcePosition::from_rustemo(&first.position);
        let token_type = if is_reserved_namespace(&first.value) {TokenType::Namespace} else {TokenType::Variable};
        self.push(pos.line as u32, pos.column as u32, first.value.len(), token_type, TokenModifiers::DECLARATION);
        for id in &names[1..] {
            let pos = SourcePosition::from_rustemo(&id.position);
            self.push(pos.line as u32, pos.column as u32, id.value.len(), TokenType::Property, TokenModifiers::NONE);
        }
    }

    fn push_io_declaration(&mut self, d: &IoDeclarationStatement) {
        // in/out keyword
        let (dir_pos, dir_len) = match &d.io_type.direction {
            IoDirection::InT(t)  => (SourcePosition::from_rustemo(&t.position), t.value.len()),
            IoDirection::OutT(t) => (SourcePosition::from_rustemo(&t.position), t.value.len()),
        };
        self.push(dir_pos.line as u32, dir_pos.column as u32, dir_len, TokenType::Keyword, TokenModifiers::NONE);
        // type
        self.push_type_spec(&d.io_type._type);
        // variable name — declaration
        let first = &d.id.names[0];
        let pos = SourcePosition::from_rustemo(&first.position);
        let token_type = if is_reserved_namespace(&first.value) {TokenType::Namespace} else {TokenType::Variable};
        self.push(pos.line as u32, pos.column as u32, first.value.len(), token_type, TokenModifiers::DECLARATION);
        for id in &d.id.names[1..] {
            let pos = SourcePosition::from_rustemo(&id.position);
            self.push(pos.line as u32, pos.column as u32, id.value.len(), TokenType::Property, TokenModifiers::NONE);
        }
    }

    fn push_dot_access_lhs(&mut self, names: &[Id]) {
        let first = &names[0];
        let pos = SourcePosition::from_rustemo(&first.position);
        let token_type = if is_reserved_namespace(&first.value) {TokenType::Namespace} else {TokenType::Variable};
        self.push(pos.line as u32, pos.column as u32, first.value.len(), token_type, TokenModifiers::NONE);
        for id in &names[1..] {
            let pos = SourcePosition::from_rustemo(&id.position);
            self.push(pos.line as u32, pos.column as u32, id.value.len(), TokenType::Property, TokenModifiers::NONE);
        }
    }

    fn push_type_spec(&mut self, type_spec: &TypeSpec) {
        match type_spec {
            TypeSpec::SimpleType(st) => {
                let (pos, len) = match &st._type {
                    SimpleTypeSpec::IntT(t)  => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                    SimpleTypeSpec::UintT(t) => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                    SimpleTypeSpec::RealT(t) => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                    SimpleTypeSpec::BoolT(t) => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                    SimpleTypeSpec::StrT(t)  => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                    SimpleTypeSpec::CharT(t) => (SourcePosition::from_rustemo(&t.position), t.value.len()),
                };
                self.push(pos.line as u32, pos.column as u32, len, TokenType::Type, TokenModifiers::NONE);
            }
            TypeSpec::DerivedType(dt) => {
                for id in &dt.name.names {
                    let pos = SourcePosition::from_rustemo(&id.position);
                    self.push(pos.line as u32, pos.column as u32, id.value.len(), TokenType::Parameter, TokenModifiers::NONE);
                }
                let type_kw_pos = SourcePosition::from_rustemo(&dt.type_kw.position);
                self.push(type_kw_pos.line as u32, type_kw_pos.column as u32, dt.type_kw.value.len(), TokenType::Type, TokenModifiers::NONE)
            }
            TypeSpec::TensorConstructor(tc) => {
                let pos = SourcePosition::from_rustemo(&tc.tensor_t.position);
                self.push(pos.line as u32, pos.column as u32,
                          tc.tensor_t.value.len(), TokenType::Type, TokenModifiers::NONE);
                self.push_type_spec(&tc._type); // recursion for tensor<tensor<int, 2>, 3>
            }
        }
    }
}

impl AstVisitor for SemanticTokenCollector {
    fn visit_external_declaration(&mut self, decl: &ExternalDeclaration) {
        match decl {
            ExternalDeclaration::DeclarationStatement(d)   => self.push_declaration(d),
            ExternalDeclaration::IoDeclarationStatement(d) => self.push_io_declaration(d),
            _ => {}
        }
    }

    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let func_t = SourcePosition::from_rustemo(&f.func_t.position);
        self.push(func_t.line as u32, func_t.column as u32, f.func_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let id = SourcePosition::from_rustemo(&f.id.position);
        self.push(id.line as u32, id.column as u32, f.id.value.len(), TokenType::Function, TokenModifiers::DECLARATION);

        // parameters — type + name for each
        if let Some(params) = &f.parameters_list {
            for p in params {
                self.push_type_spec(&p._type);

                // parameter name
                let param_pos = SourcePosition::from_rustemo(&p.id.position);
                self.push(param_pos.line as u32, param_pos.column as u32, p.id.value.len(), TokenType::Parameter, TokenModifiers::DECLARATION);
            }
        }

        if let Some(return_type) = &f.ret_type {
            self.push_type_spec(return_type)
        }

        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, f.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::DeclarationStatement(d)   => self.push_declaration(d),
            Statement::IoDeclarationStatement(d) => self.push_io_declaration(d),
            _ => {}
        }
    }

    fn visit_selection(&mut self, s: &SelectionStatement) {
        let if_pos = SourcePosition::from_rustemo(&s.if_t.position);
        self.push(if_pos.line as u32, if_pos.column as u32, s.if_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&s.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, s.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_else_if_clause(&mut self, c: &ElseIfClause) {
        let pos = SourcePosition::from_rustemo(&c.else_if_t.position);
        self.push(pos.line as u32, pos.column as u32, c.else_if_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_else_clause(&mut self, c: &ElseClause) {
        let pos = SourcePosition::from_rustemo(&c.else_t.position);
        self.push(pos.line as u32, pos.column as u32, c.else_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_for(&mut self, f: &ForIterationStatement) {
        let pos = SourcePosition::from_rustemo(&f.for_t.position);
        self.push(pos.line as u32, pos.column as u32, f.for_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        // for index variable
        let idx_pos = SourcePosition::from_rustemo(&f.header.idx.position);
        self.push(idx_pos.line as u32, idx_pos.column as u32, f.header.idx.value.len(), TokenType::Variable, TokenModifiers::DECLARATION);
        // end keyword
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, f.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_while(&mut self, w: &WhileIterationStatement) {
        let pos = SourcePosition::from_rustemo(&w.while_t.position);
        self.push(pos.line as u32, pos.column as u32, w.while_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&w.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, w.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_exists(&mut self, e: &ExistsStatement) {
        let pos = SourcePosition::from_rustemo(&e.exists_t.position);
        self.push(pos.line as u32, pos.column as u32, e.exists_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, e.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_not_exists(&mut self, e: &NotExistsStatement) {
        let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
        self.push(not_pos.line as u32, not_pos.column as u32, e.not_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let exists_pos = SourcePosition::from_rustemo(&e.exists_t.position);
        self.push(exists_pos.line as u32, exists_pos.column as u32, e.exists_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, e.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_feedthrough(&mut self, e: &FeedthroughStatement) {
        let pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        self.push(pos.line as u32, pos.column as u32, e.feedthrough_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, e.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_not_feedthrough(&mut self, e: &NotFeedthroughStatement) {
        let not_pos = SourcePosition::from_rustemo(&e.not_t.position);
        self.push(not_pos.line as u32, not_pos.column as u32, e.not_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let ft_pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
        self.push(ft_pos.line as u32, ft_pos.column as u32, e.feedthrough_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
        let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, e.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }

    fn visit_assignment(&mut self, a: &AssignmentStatement) {
        match a {
            AssignmentStatement::VarAssignmentStatement(v) => {
                self.push_dot_access_lhs(&v.var.names);
            }
            AssignmentStatement::TensorAssignmentStatement(t) => {
                self.push_dot_access_lhs(&t.tensor.expr.names);
            }
        }
    }

    fn visit_io_write(&mut self, s: &IoWriteStatement) {
        match s {
            IoWriteStatement::VarIoWriteStatement(v)    => self.push_dot_access_lhs(&v.io_var.names),
            IoWriteStatement::TensorIoWriteStatement(t) => self.push_dot_access_lhs(&t.io_tensor.expr.names),
        }
    }

    fn visit_postfix(&mut self, e: &PostfixExpression) {
        if let PostfixExpression::RValue(r) = e {
            let names = &r._ref.names;

            // First identifier is always Variable (simple var or namespace root)
            let first = &names[0];
            let pos = SourcePosition::from_rustemo(&first.position);
            let token_type = if is_reserved_namespace(&first.value) {TokenType::Namespace} else {TokenType::Variable};
            self.push(pos.line as u32, pos.column as u32,
                      first.value.len(), token_type, TokenModifiers::NONE);

            // Remaining identifiers in dot access are Property (p.x, p.gain.value)
            for id in &names[1..] {
                let pos = SourcePosition::from_rustemo(&id.position);
                self.push(pos.line as u32, pos.column as u32,
                          id.value.len(), TokenType::Property, TokenModifiers::NONE);
            }
        }
        if let PostfixExpression::Constant(c) = e {
            match c {
                Constant::Integer(i) => {
                    let (pos, len) = const_pos_len!(i, Integer::C1, Integer::C2, Integer::C3);
                    self.push(pos.line as u32, pos.column as u32, len, TokenType::Number, TokenModifiers::NONE);
                }
                Constant::UnsignedInteger(u) => {
                    let (pos, len) = const_pos_len!(u, UnsignedInteger::C1, UnsignedInteger::C2, UnsignedInteger::C3);
                    self.push(pos.line as u32, pos.column as u32, len, TokenType::Number, TokenModifiers::NONE);
                }
                Constant::TmlFloat(f) => {
                    let pos = SourcePosition::from_rustemo(&f.value.position);
                    self.push(pos.line as u32, pos.column as u32, f.value.value.len(), TokenType::Number, TokenModifiers::NONE);
                }
                Constant::TmlString(s) => {
                    let pos = SourcePosition::from_rustemo(&s.value.position);
                    self.push(pos.line as u32, pos.column as u32, s.value.value.len(), TokenType::String, TokenModifiers::NONE);
                }
                Constant::Boolean(b) => {
                    let (pos, len) = const_pos_len!(b, Boolean::C1, Boolean::C2);
                    self.push(pos.line as u32, pos.column as u32, len, TokenType::Type, TokenModifiers::DECLARATION);
                }
            }
        }
    }

    fn visit_function_call(&mut self, f: &FunctionCall) {
        let id = SourcePosition::from_rustemo(&f.id.position);
        self.push(id.line as u32, id.column as u32, f.id.value.len(), TokenType::Function, TokenModifiers::NONE);
    }

    fn visit_macro_for(&mut self, m: &MacroFor) {
        let macro_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        self.push(macro_pos.line as u32, macro_pos.column as u32, m.macro_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let for_pos = SourcePosition::from_rustemo(&m.body.for_t.position);
        self.push(for_pos.line as u32, for_pos.column as u32, m.body.for_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let idx_pos = SourcePosition::from_rustemo(&m.body.header.idx.position);
        self.push(idx_pos.line as u32, idx_pos.column as u32, m.body.header.idx.value.len(), TokenType::Variable, TokenModifiers::DECLARATION);

        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, m.body.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }
    fn visit_macro_if(&mut self, m: &MacroIf) {
        let macro_pos = SourcePosition::from_rustemo(&m.macro_t.position);
        self.push(macro_pos.line as u32, macro_pos.column as u32, m.macro_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let if_pos = SourcePosition::from_rustemo(&m.body.if_t.position);
        self.push(if_pos.line as u32, if_pos.column as u32, m.body.if_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
        self.push(end_pos.line as u32, end_pos.column as u32, m.body.end_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);
    }}