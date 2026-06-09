use std::hash::{Hash, Hasher};
use crate::position::SourcePosition;

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
    Function { name: String, id: u32 },
    /// Block scope identified by the position of its opening keyword.
    Block { line: u32, col: u32 },
    /// Compile-time macro body — variables go into parent scope.
    TransparentBlock,
    /// Block scope for macro for index variable — skipped in current_scope() so
    /// variables defined in the macro body go into the parent scope.
    MacroIndexBlock { line: u32, col: u32 },
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
    pub position: SourcePosition
}

#[derive(Debug, Clone)]
pub struct SymbolError {
    pub message: String,
    pub symbol_name: String,
    pub position: Option<SourcePosition>,
}

impl SymbolError {
    pub fn new(symbol_name: &str, message: &str, position: Option<SourcePosition>) -> Self {
        SymbolError {
            symbol_name: symbol_name.to_string(),
            message: message.to_string(),
            position
        }
    }
}

impl Hash for SymbolError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol_name.hash(state);
        self.position.as_ref().map(|p| p.line).hash(state);
        self.position.as_ref().map(|p| p.column).hash(state);
    }
}

impl PartialEq for SymbolError {
    fn eq(&self, other: &Self) -> bool {
        self.symbol_name == other.symbol_name
            && self.position.as_ref().map(|p| p.line) == other.position.as_ref().map(|p| p.line)
            && self.position.as_ref().map(|p| p.column) == other.position.as_ref().map(|p| p.column)
    }
}

impl Eq for SymbolError {}