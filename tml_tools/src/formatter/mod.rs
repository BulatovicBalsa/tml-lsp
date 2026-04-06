pub mod expressions;
pub mod statements;
pub mod types;

pub const INDENT: &str = "    "; // 4 spaces

pub trait Format {
    fn format(&self, indent: usize) -> String;
}

pub fn indent_str(indent: usize) -> String {
    INDENT.repeat(indent)
}