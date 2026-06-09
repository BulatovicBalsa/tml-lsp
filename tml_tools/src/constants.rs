use crate::types::SimpleTypeKind::Real;
use crate::types::SymbolType;

pub const RESERVED_NAMESPACES: &[&str] = &["t", "p", "n"];
pub fn is_reserved_namespace(ns: &str) -> bool {
    RESERVED_NAMESPACES.contains(&ns)
}
pub const INDENT: &str = "    ";
pub const ENTRY_FUNCTIONS: &[&str] = &["init_fnc", "output_fnc", "update_fnc"];
pub fn is_entry_function(name: &str) -> bool {
    ENTRY_FUNCTIONS.contains(&name)
}
pub const PREDEFINED_LITERALS: &[&str] = &["M_PI", "M_E", "inf"];
pub fn is_predefined_literal(name: &str) -> bool {
    PREDEFINED_LITERALS.contains(&name)
}
pub const PREDEFINED_LITERAL_TYPES: &[(&str, SymbolType)] = &[
    ("M_PI", SymbolType::Simple(Real)),
    ("M_E", SymbolType::Simple(Real)),
    ("inf", SymbolType::Simple(Real)),
];

pub fn get_predefined_literal_type(name: &str) -> Option<SymbolType> {
    PREDEFINED_LITERAL_TYPES
        .iter()
        .find(|(lit_name, _)| *lit_name == name)
        .map(|(_, ty)| ty.clone())
}