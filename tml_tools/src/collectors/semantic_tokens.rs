use tml_parser::tml_actions::{FunctionDefinition, TranslationUnit};
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
}

impl AstVisitor for SemanticTokenCollector {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let func_t = SourcePosition::from_rustemo(&f.func_t.position);
        self.push(func_t.line as u32, func_t.column as u32, f.func_t.value.len(), TokenType::Keyword, TokenModifiers::NONE);

        let id = SourcePosition::from_rustemo(&f.id.position);
        self.push(id.line as u32, id.column as u32, f.id.value.len(), TokenType::Function, TokenModifiers::DECLARATION);


    }
}