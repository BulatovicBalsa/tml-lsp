// AstVisitor trait lives in tml_parser so that tml_actions.rs (which defines
// the AST types) can reference it for accept() methods without creating a
// circular dependency.
//
// Re-export here so all tml_tools code can import from one place.
pub use tml_parser::visitor::AstVisitor;

// Helper: iterate over Option<Vec<T>> without repetition
pub fn opt_iter<T>(opt: &Option<Vec<T>>) -> impl Iterator<Item = &T> {
    opt.iter().flat_map(|v| v.iter())
}
