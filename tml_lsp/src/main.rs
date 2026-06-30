mod backend;
mod document;
mod handlers;

use backend::Backend;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::SemanticTokensFullOptions::Bool;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};

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
                    },
                )),
                document_formatting_provider: Some(OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: "\n".to_string(),
                    more_trigger_character: Some(vec!["\r".to_string()]),
                }),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                document_highlight_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend{
                                token_types: vec![
                                    SemanticTokenType::KEYWORD,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::FUNCTION,
                                    SemanticTokenType::PARAMETER,
                                    SemanticTokenType::PROPERTY,
                                    SemanticTokenType::TYPE,
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::NAMESPACE
                                ],
                                token_modifiers: vec![
                                    SemanticTokenModifier::DECLARATION
                                ]
                            },
                            full: Some(Bool(true)),
                            ..Default::default()
                        }
                    )
                ),
                ..Default::default()

            },
            server_info: Some(ServerInfo {
                name: "tml-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
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
        self.client.log_message(
            MessageType::INFO,
            format!("Document opened: {}", params.text_document.uri),
        ).await;
        document::update_document(self, params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client.log_message(
            MessageType::INFO,
            format!("Document changed: {}", params.text_document.uri),
        ).await;
        if let Some(change) = params.content_changes.into_iter().last() {
            document::update_document(self, params.text_document.uri, change.text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client.log_message(
            MessageType::INFO,
            format!("Document saved: {}", params.text_document.uri),
        ).await;
        let text = self.documents.read().await
            .get(&params.text_document.uri.to_string())
            .cloned();
        if let Some(text) = text {
            document::update_document(self, params.text_document.uri, text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client.log_message(
            MessageType::INFO,
            format!("Document closed: {}", params.text_document.uri),
        ).await;        
        let key = params.text_document.uri.to_string();
        self.documents.write().await.remove(&key);
        self.hoverable.write().await.remove(&key);
        self.symbol_tables.write().await.remove(&key);
        self.folding_ranges.write().await.remove(&key);
        self.quick_fixes.write().await.remove(&key);
        self.block_spans.write().await.remove(&key);
        self.client.publish_diagnostics(params.text_document.uri, vec![], None).await;
    }

    // ── Handlers ──

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        handlers::hover::goto_definition(self, params).await
    }

    async fn document_highlight(&self, params: DocumentHighlightParams) -> Result<Option<Vec<DocumentHighlight>>> {
        handlers::highlight::document_highlight(self, params).await
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        handlers::hover::hover(self, params).await
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        handlers::folding::folding_range(self, params).await
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        handlers::semantic_tokens::semantic_tokens_full(self, params).await
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        handlers::completion::completion(self, params).await
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        handlers::code_action::code_action(self, params).await
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        handlers::formatting::formatting(self, params).await
    }

    async fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        handlers::formatting::on_type_formatting(self, params).await
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
