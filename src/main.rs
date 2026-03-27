use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
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

        let diagnostics = fake_diagnostics(&text);

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        let text = params
            .content_changes
            .last()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let diagnostics = fake_diagnostics(&text);

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "Hello from mini-lsp".to_string(),
            )),
            range: None,
        }))
    }

    async fn document_symbol(
        &self,
        _: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        Ok(Some(DocumentSymbolResponse::Nested(vec![
            DocumentSymbol {
                name: "example_symbol".into(),
                detail: Some("demo".into()),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                range: Range::new(Position::new(0, 0), Position::new(0, 10)),
                selection_range: Range::new(Position::new(0, 0), Position::new(0, 10)),
                children: None,
            }
        ])))
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("fn".into(), "keyword".into()),
            CompletionItem::new_simple("if".into(), "keyword".into()),
            CompletionItem::new_simple("return".into(), "keyword".into()),
        ])))
    }
}

fn fake_diagnostics(text: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        if line.contains("todo_error") {
            out.push(Diagnostic {
                range: Range::new(
                    Position::new(line_idx as u32, 0),
                    Position::new(line_idx as u32, line.len() as u32),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("mini-lsp".into()),
                message: "found todo_error".into(),
                related_information: None,
                tags: None,
                data: None,
            });
        }
    }

    out
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}