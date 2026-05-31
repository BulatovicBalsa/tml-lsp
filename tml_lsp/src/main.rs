use rustemo::Parser;
use std::collections::HashMap;
use tml_parser::tml::TmlParser;
use tml_tools::diagnostics::DiagnosticSeverity as ParserDiagnosticSeverity;
use tml_tools::diagnostics::DiagnosticsRunner;
use tml_tools::checkers::empty_body_checker::{EmptyBodyChecker, EmptyBodyDiagnosticSource};
use tml_tools::collectors::folding_collector::{FoldingCollector, TmlFoldingRange};
use tml_tools::checkers::function_call_checker::FunctionCallDiagnosticSource;
use tml_tools::collectors::hoverable_collector::{HoverableCollector, HoverableKind, HoverableNode};
use tml_tools::symbol_table::{Scope, SymbolTable, SymbolTableBuilder};
use tml_tools::checkers::undefined_variable_checker::UndefinedVariableDiagnosticSource;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tml_tools::collectors::block_span_collector::{find_indent, BlockSpan, BlockSpanCollector};
use tml_tools::formatter::indent_str;

struct Backend {
    client: Client,
    documents: RwLock<HashMap<String, String>>,
    hoverable: RwLock<HashMap<String, Vec<HoverableNode>>>,
    symbol_tables: RwLock<HashMap<String, SymbolTable>>,
    folding_ranges: RwLock<HashMap<String, Vec<TmlFoldingRange>>>,
    quick_fixes: RwLock<HashMap<String, Vec<EmptyBodyQuickFix>>>,
    block_spans: RwLock<HashMap<String, Vec<BlockSpan>>>,
}

#[derive(Debug, Clone)]
struct EmptyBodyQuickFix {
    /// Diagnostic line this fix applies to
    diag_line: u32,
    /// Line where 'pass' should be inserted (from header_colon position)
    insert_line: u32,
    /// Indentation string computed via formatter::INDENT
    indent: String,
}

impl Backend {
    fn new(client: Client) -> Self {
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

    async fn update_document(&self, uri: Url, text: String) {
        self.client.log_message(MessageType::INFO, "Updating document...").await;
        let key = uri.to_string();
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        self.documents.write().await.insert(key.clone(), normalized.clone());

        self.client
            .log_message(MessageType::INFO, "Document updated, starting parse...")
            .await;

        let parse_result = tokio::task::spawn_blocking(move || {
            let parser = TmlParser::new();
            parser.parse(&normalized).map(|ast| {
                let (table, sym_errors) = SymbolTableBuilder::new().build(&ast);
                let nodes = HoverableCollector::new().collect(&ast);
                let folds = FoldingCollector::new(&normalized).collect(&ast);
                let spans = BlockSpanCollector::new().collect(&ast);

                let runner = DiagnosticsRunner::new()
                    .add_source(UndefinedVariableDiagnosticSource)
                    .add_source(FunctionCallDiagnosticSource)
                    .add_source(EmptyBodyDiagnosticSource);
                let diagnostics = runner.run(&ast, &table);

                // Collect empty body fix metadata separately for code actions
                let empty_body_fixes = EmptyBodyChecker::new().check(&ast);
                (table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, spans)
            })
        })
            .await;

        match parse_result {
            Ok(Ok((table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, block_spans))) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "Parsed OK — {} symbol(s), {} hoverable node(s), {} fold(s), {} error(s), {} block span(s)",
                            table.symbols.len(),
                            nodes.len(),
                            folds.len(),
                            sym_errors.len(),
                            block_spans.len(),
                        ),
                    )
                    .await;
                self.symbol_tables.write().await.insert(key.clone(), table);
                self.hoverable.write().await.insert(key.clone(), nodes);
                self.folding_ranges.write().await.insert(key.clone(), folds);
                self.block_spans.write().await.insert(key.clone(), block_spans);

                // Store quick fix metadata indexed by diagnostic position
                let fixes: Vec<EmptyBodyQuickFix> = empty_body_fixes
                    .into_iter()
                    .map(|e| EmptyBodyQuickFix {
                        diag_line: e.keyword_position.line as u32,
                        insert_line: e.insert_line,
                        indent: e.indent,
                    })
                    .collect();
                self.quick_fixes.write().await.insert(key.clone(), fixes);

                let lsp_diagnostics: Vec<Diagnostic> = diagnostics
                    .iter()
                    .map(|d| {
                        let severity = match d.severity {
                            ParserDiagnosticSeverity::Error   => DiagnosticSeverity::ERROR,
                            ParserDiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
                            ParserDiagnosticSeverity::Hint    => DiagnosticSeverity::HINT,
                        };
                        let start = Position { line: d.line, character: d.column };
                        let end = Position { line: d.line, character: d.column + d.length as u32 };
                        Diagnostic {
                            range: Range { start, end },
                            severity: Some(severity),
                            message: d.message.clone(),
                            ..Default::default()
                        }
                    })
                    .collect();

                self.client.publish_diagnostics(uri.clone(), lsp_diagnostics, None).await;
            }
            Ok(Err(e)) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Parse error: {:?}", e))
                    .await;
                self.client.publish_diagnostics(uri.clone(), vec![], None).await;
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
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                        ..Default::default()
                    }
                )),
                document_formatting_provider: Some(OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: "\n".to_string(),
                    more_trigger_character: Some(vec!["\r".to_string()]),
                }),
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

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let text = self.documents.read().await
            .get(&params.text_document.uri.to_string())
            .cloned();
        if let Some(text) = text {
            self.update_document(params.text_document.uri, text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let key = params.text_document.uri.to_string();
        self.documents.write().await.remove(&key);
        self.hoverable.write().await.remove(&key);
        self.symbol_tables.write().await.remove(&key);
        self.folding_ranges.write().await.remove(&key);
        self.quick_fixes.write().await.remove(&key);
        self.block_spans.write().await.remove(&key);
        self.client.publish_diagnostics(params.text_document.uri, vec![], None).await;
    }

    // ── Code actions ──

    // ── Goto definition ──
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

    // ── Folding ──
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

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.clone();
        let uri_str = uri.to_string();
        let range = params.range;

        let fixes = self.quick_fixes.read().await;
        let fixes = match fixes.get(&uri_str) {
            Some(f) => f,
            None => return Ok(None),
        };

        // Find a fix whose diagnostic position overlaps the requested range
        let matching: Vec<CodeActionOrCommand> = fixes
            .iter()
            .filter(|fix| {
                fix.diag_line >= range.start.line && fix.diag_line <= range.end.line
            })
            .map(|fix| {
                let new_text = format!("{}pass\n", fix.indent);
                let edit = TextEdit {
                    range: Range {
                        start: Position { line: fix.insert_line, character: 0 },
                        end:   Position { line: fix.insert_line, character: 0 },
                    },
                    new_text,
                };
                let mut changes = HashMap::new();
                changes.insert(uri.clone(), vec![edit]);

                CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Add 'pass' statement".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: None,
                    edit: Some(WorkspaceEdit {
                        changes: Some(changes),
                        ..Default::default()
                    }),
                    is_preferred: Some(true),
                    ..Default::default()
                })
            })
            .collect();

        if matching.is_empty() {
            Ok(None)
        } else {
            Ok(Some(matching))
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

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        self.client.log_message(MessageType::INFO, "On-type formatting triggered").await;
        let uri = params.text_document_position.text_document.uri.to_string();
        let cursor_line = params.text_document_position.position.line;

        let spans = self.block_spans.read().await.get(&uri).cloned().unwrap_or_default();

        for span in &spans {
            self.client.log_message(
                MessageType::INFO,
                format!(
                    "Block span: header_line={}, end_line={}, indent_level={}",
                    span.header_line, span.end_line, span.body_indent_level
                ),
            ).await;
        }

        let level = find_indent(&spans, cursor_line);

        self.client.log_message(
            MessageType::INFO,
            format!("Cursor at line {}, computed indent level: {}", cursor_line, level),
        ).await;

        if level == 0 {
            return Ok(None);
        }

        let target_indent = indent_str(level);

        // Check how much whitespace VS Code already inserted on cursor_line
        let current_indent = self.documents.read().await
            .get(&uri)
            .and_then(|text| text.lines().nth(cursor_line as usize))
            .map(|line| {
                let trimmed = line.trim_start();
                &line[..line.len() - trimmed.len()]
            })
            .unwrap_or("")
            .to_string();

        // Already correct — nothing to do
        if current_indent == target_indent {
            return Ok(None);
        }

        // Replace whatever whitespace is already there with the correct indent
        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position { line: cursor_line, character: 0 },
                end:   Position { line: cursor_line, character: current_indent.len() as u32 },
            },
            new_text: target_indent,
        }]))    }
}

// ───────────────────────── Main ─────────────────────────

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}