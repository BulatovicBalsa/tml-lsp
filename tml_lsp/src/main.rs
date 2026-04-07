use std::collections::HashMap;
use tokio::sync::RwLock;
use rustemo::Parser;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tml_parser::tml::TmlParser;
use tml_tools::symbol_table::{
    Scope, SimpleTypeKind, SymbolTable, SymbolTableBuilder, SymbolType,
};
use tml_tools::rvalue_collector::{RValueCollector, RValueRef};

// ───────────────────────── Backend ─────────────────────────

struct Backend {
    client: Client,
    symbol_tables: RwLock<HashMap<String, SymbolTable>>,
    documents: RwLock<HashMap<String, String>>,
    rvalue_refs: RwLock<HashMap<String, Vec<RValueRef>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Backend {
            client,
            symbol_tables: RwLock::new(HashMap::new()),
            documents: RwLock::new(HashMap::new()),
            rvalue_refs: RwLock::new(HashMap::new()),
        }
    }

    async fn update_document(&self, uri: Url, text: String) {
        let key = uri.to_string();
        self.documents.write().await.insert(key.clone(), text.clone());

        let text_clone = text.replace("\r\n", "\n").replace('\r', "\n");
        let parse_result = tokio::task::spawn_blocking(move || {
            let parser = TmlParser::new();
            parser.parse(&text_clone).map(|ast| {
                let (table, errors) = SymbolTableBuilder::new().build(&ast);
                let refs = RValueCollector::new().collect(&ast);
                (table, errors, refs)
            })
        })
            .await;

        match parse_result {
            Ok(Ok((table, errors, refs))) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "Parsed OK — {} symbol(s), {} error(s), {} rvalue ref(s)",
                            table.symbols.len(),
                            errors.len(),
                            refs.len(),
                        ),
                    )
                    .await;
                for e in &errors {
                    self.client
                        .log_message(
                            MessageType::WARNING,
                            format!("Symbol error: {}", e.message),
                        )
                        .await;
                }
                self.symbol_tables.write().await.insert(key.clone(), table);
                self.rvalue_refs.write().await.insert(key, refs);
            }
            Ok(Err(e)) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Parse error: {:?}", e),
                    )
                    .await;
                self.symbol_tables.write().await.remove(&key);
                self.rvalue_refs.write().await.remove(&key);
            }
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Internal error: {:?}", e),
                    )
                    .await;
            }
        }
    }

    fn hover_for_symbol(word: &str, table: &SymbolTable) -> Option<String> {
        let symbol = table.symbols.iter().find(|s| s.name == word)?;

        let type_str = format_symbol_type(&symbol.ty);
        let scope_str = match &symbol.scope {
            Scope::Global => "global".to_string(),
            Scope::Function(name) => format!("fn {}", name),
        };

        Some(format!(
            "```tml\n{}: {}\n```\n*scope: {}*",
            symbol.name, type_str, scope_str
        ))
    }

    fn hover_for_function(word: &str, table: &SymbolTable) -> Option<String> {
        let func = table.lookup_function(word)?;

        let params = func
            .params
            .iter()
            .map(|(ty, name)| format!("{} {}", format_symbol_type(ty), name))
            .collect::<Vec<_>>()
            .join(", ");

        let ret = match &func.ret_type {
            None => String::new(),
            Some(ty) => format!(" {}", format_symbol_type(ty)),
        };

        Some(format!(
            "```tml\nfn {}({}){}\n```",
            func.name, params, ret
        ))
    }

    /// Find RValue Ref at cursor position
    fn find_rvalue_at(
        refs: &[RValueRef],
        cursor_line: u32,
        cursor_col: u32,
    ) -> Option<&RValueRef> {
        refs.iter().find(|r| {
            r.position.contains_cursor(cursor_line, cursor_col, r.name.len())
        })
    }
}

// ───────────────────────── Type formatting ─────────────────────────

fn format_symbol_type(ty: &SymbolType) -> String {
    match ty {
        SymbolType::Simple(s) => match s {
            SimpleTypeKind::Int  => "int".to_string(),
            SimpleTypeKind::Uint => "uint".to_string(),
            SimpleTypeKind::Real => "real".to_string(),
            SimpleTypeKind::Bool => "bool".to_string(),
            SimpleTypeKind::Str  => "str".to_string(),
            SimpleTypeKind::Char => "char".to_string(),
        },
        SymbolType::Tensor(inner, dims) => {
            format!("tensor<{}, {}>", format_symbol_type(inner), dims.join(", "))
        }
        SymbolType::Derived(name) => format!("{}.type", name),
    }
}

// ───────────────────────── LanguageServer impl ─────────────────────────

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "tml-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "TML Language Server initialized and ready!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // ── Document sync ──

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.update_document(
            params.text_document.uri,
            params.text_document.text,
        )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.update_document(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let key = params.text_document.uri.to_string();
        self.documents.write().await.remove(&key);
        self.symbol_tables.write().await.remove(&key);
        self.rvalue_refs.write().await.remove(&key);
    }

    // ── Hover ──

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;

        // Try precise hover over RValueRef positions
        let refs = self.rvalue_refs.read().await;
        let word = if let Some(doc_refs) = refs.get(&uri) {
            match Self::find_rvalue_at(doc_refs, position.line, position.character) {
                Some(r) => {
                    self.client
                        .log_message(
                            MessageType::INFO,
                            format!("Hover via RValueRef: '{}' at {}:{}", r.name, r.position.line, r.position.column),
                        )
                        .await;
                    r.name.clone()
                }
                // Fallback to word extraction if there is no RValueRef's at current position
                None => {
                    drop(refs);
                    let text = match self.documents.read().await.get(&uri) {
                        Some(t) => t.clone(),
                        None => return Ok(None),
                    };
                    match Self::word_at_position(&text, position) {
                        Some(w) => {
                            self.client
                                .log_message(
                                    MessageType::INFO,
                                    format!("Hover via word extraction: '{}'", w),
                                )
                                .await;
                            w
                        }
                        None => return Ok(None),
                    }
                }
            }
        } else {
            drop(refs);
            let text = match self.documents.read().await.get(&uri) {
                Some(t) => t.clone(),
                None => return Ok(None),
            };
            match Self::word_at_position(&text, position) {
                Some(w) => w,
                None => return Ok(None),
            }
        };

        let tables = self.symbol_tables.read().await;
        let content = tables
            .get(&uri)
            .and_then(|t| {
                Self::hover_for_function(&word, t)
                    .or_else(|| Self::hover_for_symbol(&word, t))
            });

        match content {
            None => Ok(None),
            Some(markdown) => Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: markdown,
                }),
                range: None,
            })),
        }
    }

    // ── Formatting ──

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        let text = match self.documents.read().await.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };

        self.client
            .log_message(MessageType::INFO, "Formatting requested")
            .await;

        let text_clone = text.replace("\r\n", "\n").replace('\r', "\n");
        let format_result = tokio::task::spawn_blocking(move || {
            let parser = TmlParser::new();
            parser.parse(&text_clone).ok().map(|ast| {
                use tml_tools::formatter::Format;
                ast.format(0)
            })
        })
            .await;

        match format_result {
            Ok(Some(formatted)) => {
                let lines = text.lines().count();
                let last_line_len = text.lines().last().map(|l| l.len()).unwrap_or(0);

                let edit = TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position {
                            line: lines as u32,
                            character: last_line_len as u32,
                        },
                    },
                    new_text: formatted,
                };
                self.client
                    .log_message(MessageType::INFO, "Formatting OK")
                    .await;
                Ok(Some(vec![edit]))
            }
            Ok(None) => {
                self.client
                    .log_message(MessageType::WARNING, "Formatting skipped — parse error")
                    .await;
                Ok(None)
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Formatting error: {:?}", e))
                    .await;
                Ok(None)
            }
        }
    }
}

// ───────────────────────── Helper ─────────────────────────

fn word_at_position(text: &str, position: Position) -> Option<String> {
    let line = text.lines().nth(position.line as usize)?;
    let char_idx = position.character as usize;

    if char_idx > line.len() {
        return None;
    }

    let start = line[..char_idx]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = line[char_idx..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + char_idx)
        .unwrap_or(line.len());

    if start >= end {
        return None;
    }

    let word = &line[start..end];
    if word.is_empty() {
        None
    } else {
        Some(word.to_string())
    }
}

impl Backend {
    fn word_at_position(text: &str, position: Position) -> Option<String> {
        word_at_position(text, position)
    }
}

// ───────────────────────── Main ─────────────────────────

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}