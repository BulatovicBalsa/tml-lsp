use tml_parser::tml_actions::{FunctionDefinition, SimpleTypeSpec, TranslationUnit, TypeSpec};
use tml_parser::visitor::AstVisitor;
use crate::position::SourcePosition;

// Token type indices — must match the order in SemanticTokensLegend in main.rs
#[repr(u32)]
#[derive(Clone, Debug)]
pub enum TokenType {
    Keyword   = 0,
    Variable  = 1,
    Function  = 2,
    Parameter = 3,
    Property  = 4,
    Type      = 5,
    Number    = 6,
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
}