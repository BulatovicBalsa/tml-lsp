use rustemo::Parser;
use std::collections::HashMap;
use tml_parser::tml::TmlParser;
use tml_tools::folding_collector::{FoldingCollector, TmlFoldingRange};
use tml_tools::hoverable_collector::{HoverableCollector, HoverableKind, HoverableNode};
use tml_tools::symbol_table::{Scope, SymbolTable, SymbolTableBuilder};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// ───────────────────────── Backend ─────────────────────────

struct Backend {
    client: Client,
    documents: RwLock<HashMap<String, String>>,
    hoverable: RwLock<HashMap<String, Vec<HoverableNode>>>,
    symbol_tables: RwLock<HashMap<String, SymbolTable>>,
    folding_ranges: RwLock<HashMap<String, Vec<TmlFoldingRange>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Backend {
            client,
            documents: RwLock::new(HashMap::new()),
            hoverable: RwLock::new(HashMap::new()),
            symbol_tables: RwLock::new(HashMap::new()),
            folding_ranges: RwLock::new(HashMap::new()),
        }
    }

    async fn update_document(&self, uri: Url, text: String) {
        let key = uri.to_string();
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        self.documents.write().await.insert(key.clone(), normalized.clone());

        let parse_result = tokio::task::spawn_blocking(move || {
            let parser = TmlParser::new();
            parser.parse(&normalized).map(|ast| {
                let (table, sym_errors) = SymbolTableBuilder::new().build(&ast);
                let nodes = HoverableCollector::new().collect(&ast);
                let folds = FoldingCollector::new(&normalized).collect(&ast);
                (table, sym_errors, nodes, folds)
            })
        })
            .await;

        match parse_result {
            Ok(Ok((table, sym_errors, nodes, folds))) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "Parsed OK — {} symbol(s), {} hoverable node(s), {} fold(s), {} error(s)",
                            table.symbols.len(),
                            nodes.len(),
                            folds.len(),
                            sym_errors.len(),
                        ),
                    )
                    .await;
                self.symbol_tables.write().await.insert(key.clone(), table);
                self.hoverable.write().await.insert(key.clone(), nodes);
                self.folding_ranges.write().await.insert(key.clone(), folds);

                let hov_str = self.hoverable.read().await
                    .get(&key)
                    .map(|v| {
                        v.iter()
                            .map(|n| format!("  - {:?} at {}:{}", n.kind, n.position.line, n.position.column))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_default();
                if !hov_str.is_empty() {
                    self.client
                        .log_message(MessageType::INFO, format!("Hoverable variables:\n{}", hov_str))
                        .await;
                }
            }
            Ok(Err(e)) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Parse error: {:?}", e))
                    .await;
                self.hoverable.write().await.remove(&key);
                self.folding_ranges.write().await.remove(&key);
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Internal error: {:?}", e))
                    .await;
            }
        }
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
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
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
        self.update_document(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.update_document(params.text_document.uri, change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let key = params.text_document.uri.to_string();
        self.documents.write().await.remove(&key);
        self.hoverable.write().await.remove(&key);
        self.symbol_tables.write().await.remove(&key);
        self.folding_ranges.write().await.remove(&key);
    }

    // ── Hover ──

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;

        let hoverable = self.hoverable.read().await;
        let nodes = match hoverable.get(&uri) {
            Some(n) => n,
            None => return Ok(None),
        };

        let node = match HoverableCollector::find_at(nodes, position.line, position.character) {
            Some(n) => n,
            None => return Ok(None),
        };

        let tables = self.symbol_tables.read().await;
        let table = tables.get(&uri);

        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Hover: {:?} at {}:{}",
                    node.kind, node.position.line, node.position.column
                ),
            )
            .await;

        let content = table.and_then(|t| node.hover_content(t))
            .unwrap_or_else(|| format!("```tml\n{}\n```", node.name()));

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: content,
            }),
            range: None,
        }))
    }

    // ── Formatting ──

    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri.to_string();

        let ranges = self.folding_ranges.read().await;
        let result = ranges.get(&uri).map(|folds| {
            folds.iter().map(|f| FoldingRange {
                start_line: f.start_line,
                end_line: f.end_line,
                kind: Some(FoldingRangeKind::Region),
                ..Default::default()
            })
                .collect()
        });

        Ok(result)
    }

    // ── Folding ──

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

        let format_result = tokio::task::spawn_blocking(move || {
            let parser = TmlParser::new();
            parser.parse(&text).ok().map(|ast| {
                use tml_tools::formatter::Format;
                ast.format(0)
            })
        })
            .await;

        match format_result {
            Ok(Some(formatted)) => {
                let (lines, last_line_len) = {
                    let docs = self.documents.read().await;
                    match docs.get(&uri) {
                        Some(t) => (
                            t.lines().count(),
                            t.lines().last().map(|l| l.len()).unwrap_or(0),
                        ),
                        None => return Ok(None),
                    }
                };

                self.client
                    .log_message(MessageType::INFO, "Formatting OK")
                    .await;

                Ok(Some(vec![TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position {
                            line: lines as u32,
                            character: last_line_len as u32,
                        },
                    },
                    new_text: formatted,
                }]))
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

    // ── Definition ──

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.clone();
        let position = params.text_document_position_params.position;
        let uri_str = uri.to_string();

        let hoverable = self.hoverable.read().await;
        let nodes = match hoverable.get(&uri_str) {
            Some(n) => n,
            None => return Ok(None),
        };

        // Find what the user clicked on
        let node = match HoverableCollector::find_at(nodes, position.line, position.character) {
            Some(n) => n,
            None => return Ok(None),
        };

        // Determine what name and scope we are looking for
        let (target_name, target_scope) = match &node.kind {
            HoverableKind::VariableRef { name } => (name.clone(), Some(node.scope.clone())),
            HoverableKind::FunctionCall { name } => (name.clone(), None),
            // Already on a definition — nothing to jump to
            HoverableKind::VariableDecl { .. } | HoverableKind::FunctionDef { .. } => {
                return Ok(None)
            }
        };

        // Find the declaration node
        let decl_node = nodes.iter().find(|n| match &n.kind {
            HoverableKind::VariableDecl { name, .. } => {
                *name == target_name && match &target_scope {
                    // Check current scope first, then global
                    Some(scope) => &n.scope == scope || n.scope == Scope::Global,
                    None => true,
                }
            }
            HoverableKind::FunctionDef { name } => *name == target_name,
            _ => false,
        });

        match decl_node {
            None => Ok(None),
            Some(decl) => {
                let start = Position {
                    line: decl.position.line as u32,
                    character: decl.position.column as u32,
                };
                Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri,
                    range: Range { start, end: start },
                })))
            }
        }
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