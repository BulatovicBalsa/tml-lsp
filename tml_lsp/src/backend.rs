use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::Client;
use tml_tools::collectors::block_span::BlockSpan;
use tml_tools::collectors::folding::TmlFoldingRange;
use tml_tools::collectors::hoverable::HoverableNode;
use tml_tools::symbol_table::SymbolTable;

// ───────────────────────── Quick fix types ─────────────────────────

#[derive(Debug, Clone)]
pub struct EmptyBodyQuickFix {
    /// Diagnostic line this fix applies to
    pub diag_line: u32,
    /// Line where 'pass' should be inserted (from header_colon position)
    pub insert_line: u32,
    /// Indentation string computed via formatter::INDENT
    pub indent: String,
}

// ───────────────────────── Backend ─────────────────────────

pub struct Backend {
    pub client: Client,
    pub documents: RwLock<HashMap<String, String>>,
    pub hoverable: RwLock<HashMap<String, Vec<HoverableNode>>>,
    pub symbol_tables: RwLock<HashMap<String, SymbolTable>>,
    pub folding_ranges: RwLock<HashMap<String, Vec<TmlFoldingRange>>>,
    pub quick_fixes: RwLock<HashMap<String, Vec<EmptyBodyQuickFix>>>,
    pub block_spans: RwLock<HashMap<String, Vec<BlockSpan>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Backend {
            client,
            documents: RwLock::new(HashMap::new()),
            hoverable: RwLock::new(HashMap::new()),
            symbol_tables: RwLock::new(HashMap::new()),
            folding_ranges: RwLock::new(HashMap::new()),
            quick_fixes: RwLock::new(HashMap::new()),
            block_spans: RwLock::new(HashMap::new()),
        }
    }
}
