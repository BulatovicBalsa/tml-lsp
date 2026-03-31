pub mod tml;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    docs: Arc<Mutex<HashMap<Url, String>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            docs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "mini-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "mini-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        let uri = params.text_document.uri;

        self.docs.lock().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params
            .content_changes
            .last()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        self.docs.lock().await.insert(uri, text);
    }

    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>> {
        let docs = self.docs.lock().await;
        let Some(text) = docs.get(&params.text_document.uri) else {
            return Ok(Some(vec![]));
        };

        Ok(Some(compute_brace_folds(text)))
    }
}

fn compute_brace_folds(text: &str) -> Vec<FoldingRange> {
    let mut folds = Vec::new();
    let mut stack: Vec<u32> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let line_idx = line_idx as u32;

        for ch in line.chars() {
            match ch {
                '{' => stack.push(line_idx),
                '}' => {
                    if let Some(start_line) = stack.pop() {
                        // Only fold multi-line blocks.
                        if start_line < line_idx {
                            folds.push(FoldingRange {
                                start_line,
                                start_character: None,
                                end_line: line_idx,
                                end_character: None,
                                kind: Some(FoldingRangeKind::Region),
                                collapsed_text: None,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    folds
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}