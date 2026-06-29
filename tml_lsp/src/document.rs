use crate::backend::{Backend, CachedDocumentState, EmptyBodyQuickFix};
use rustemo::{Error, Parser};
use tml_parser::tml::TmlParser;
use tml_tools::checkers::empty_body::{EmptyBodyChecker, EmptyBodyDiagnosticSource};
use tml_tools::checkers::function_call::FunctionCallDiagnosticSource;
use tml_tools::checkers::undefined_variable::UndefinedVariableDiagnosticSource;
use tml_tools::collectors::block_span::BlockSpanCollector;
use tml_tools::collectors::folding::FoldingCollector;
use tml_tools::collectors::hoverable::HoverableCollector;
use tml_tools::diagnostics::{DiagnosticSeverity as ParserDiagnosticSeverity, DiagnosticsRunner};
use tml_tools::symbol_table::SymbolTableBuilder;
use tower_lsp::lsp_types::*;
use tml_tools::collectors::semantic_tokens::SemanticTokenCollector;

pub async fn update_document(backend: &Backend, uri: Url, text: String) {
    let key = uri.to_string();
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    backend.documents.write().await.insert(key.clone(), normalized.clone());

    let parse_result = tokio::task::spawn_blocking(move || {
        TmlParser::new().parse(&normalized).map(|ast| {
            let (table, sym_errors) = SymbolTableBuilder::new().build(&ast);
            let nodes = HoverableCollector::new().collect(&ast);
            let folds = FoldingCollector::new(&normalized).collect(&ast);
            let spans = BlockSpanCollector::new().collect(&ast);
            let semantic_tokens = SemanticTokenCollector::new().collect(&ast);

            let diagnostics = DiagnosticsRunner::new()
                .add_source(UndefinedVariableDiagnosticSource)
                .add_source(FunctionCallDiagnosticSource)
                .add_source(EmptyBodyDiagnosticSource)
                .run(&ast, &table);

            let empty_body_fixes = EmptyBodyChecker::new().check(&ast);
            (table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, spans, semantic_tokens)
        })
    }).await;

    match parse_result {
        Ok(Ok((table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, block_spans, semantic_tokens))) => {
            backend.client.log_message(
                MessageType::INFO,
                format!(
                    "Parsed OK - {} symbol(s), {} hoverable node(s), {} fold(s), {} error(s), {} block span(s)",
                    table.symbols.len(), nodes.len(), folds.len(), sym_errors.len(), block_spans.len(),
                ),
            ).await;

            backend.symbol_tables.write().await.insert(key.clone(), table);
            backend.hoverable.write().await.insert(key.clone(), nodes);
            backend.folding_ranges.write().await.insert(key.clone(), folds);
            backend.block_spans.write().await.insert(key.clone(), block_spans);

            let fixes: Vec<EmptyBodyQuickFix> = empty_body_fixes
                .into_iter()
                .map(|e| EmptyBodyQuickFix {
                    diag_line: e.keyword_position.line as u32,
                    insert_line: e.insert_line,
                    indent: e.indent,
                })
                .collect();
            backend.quick_fixes.write().await.insert(key.clone(), fixes);

            let mut lsp_diagnostics: Vec<Diagnostic> = diagnostics
                .iter()
                .map(|d| {
                    let severity = match d.severity {
                        ParserDiagnosticSeverity::Error   => DiagnosticSeverity::ERROR,
                        ParserDiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
                        ParserDiagnosticSeverity::Hint    => DiagnosticSeverity::HINT,
                    };
                    let start = Position { line: d.line, character: d.column };
                    let end   = Position { line: d.line, character: d.column + d.length as u32 };
                    Diagnostic {
                        range: Range { start, end },
                        severity: Some(severity),
                        message: d.message.clone(),
                        ..Default::default()
                    }
                })
                .collect();

            for e in &sym_errors {
                if let Some(pos) = &e.position {
                    lsp_diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: pos.line as u32, character: pos.column as u32 },
                            end:   Position { line: pos.line as u32, character: pos.column as u32 + e.symbol_name.len() as u32 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: e.message.clone(),
                        ..Default::default()
                    });
                }
            }

            backend.last_valid.write().await.insert(key.clone(), CachedDocumentState {
                table: backend.symbol_tables.read().await.get(&key).unwrap().clone(),
                nodes: backend.hoverable.read().await.get(&key).unwrap().clone(),
                folds: backend.folding_ranges.read().await.get(&key).unwrap().clone(),
                spans: backend.block_spans.read().await.get(&key).unwrap().clone(),
                quick_fixes: backend.quick_fixes.read().await.get(&key).unwrap().clone(),
                diagnostics: lsp_diagnostics.clone(),
                semantic_tokens
            });

            backend.client.log_message(
                MessageType::INFO,
                format!("Publishing diagnostics for {}", uri)
            ).await;
            backend.client.publish_diagnostics(uri, lsp_diagnostics, None).await;
        }
        Ok(Err(e)) => {
            let (line, col) = extract_parse_error_position(&e);

            let parse_diag = Diagnostic {
                range: Range {
                    start: Position { line, character: col },
                    end:   Position { line, character: col + 1 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: "Syntax error".to_string(),
                ..Default::default()
            };

            if let Some(cached) = backend.last_valid.read().await.get(&key).cloned() {
                // Restore all cached state
                backend.hoverable.write().await.insert(key.clone(), cached.nodes);
                backend.folding_ranges.write().await.insert(key.clone(), cached.folds);
                backend.block_spans.write().await.insert(key.clone(), cached.spans);
                backend.quick_fixes.write().await.insert(key.clone(), cached.quick_fixes);
                backend.symbol_tables.write().await.insert(key.clone(), cached.table);

                let mut diagnostics = cached.diagnostics.clone();
                diagnostics.push(parse_diag);
                backend.client.publish_diagnostics(uri, diagnostics, None).await;
            } else {
                // No cache — just show parse error
                backend.client.publish_diagnostics(uri, vec![parse_diag], None).await;
            }
        }
        Err(e) => {
            backend.client.log_message(MessageType::ERROR, format!("Internal error: {:?}", e)).await;
        }
    }
}

fn extract_parse_error_position(error: &Error) -> (u32, u32) {
    match error {
        Error::ParseError(parse_error) => {
            let position = tml_tools::position::SourcePosition::from_rustemo(&parse_error.span.unwrap().start);
            (position.line as u32, position.column as u32)
        }
        Error::IOError(_) => {
            (0, 0) // Default position for IO errors
        }
    }
}
