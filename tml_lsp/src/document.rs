use crate::backend::{Backend, EmptyBodyQuickFix};
use rustemo::Parser;
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

            let diagnostics = DiagnosticsRunner::new()
                .add_source(UndefinedVariableDiagnosticSource)
                .add_source(FunctionCallDiagnosticSource)
                .add_source(EmptyBodyDiagnosticSource)
                .run(&ast, &table);

            let empty_body_fixes = EmptyBodyChecker::new().check(&ast);
            (table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, spans)
        })
    }).await;

    match parse_result {
        Ok(Ok((table, sym_errors, nodes, folds, diagnostics, empty_body_fixes, block_spans))) => {
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

            backend.client.log_message(
                MessageType::INFO,
                format!("Publishing diagnostics for {}", uri)
            ).await;
            backend.client.publish_diagnostics(uri, lsp_diagnostics, None).await;
        }
        Ok(Err(e)) => {
            backend.client.log_message(MessageType::ERROR, format!("Parse error: {:?}", e)).await;
            backend.client.publish_diagnostics(uri, vec![], None).await;
            backend.hoverable.write().await.remove(&key);
            backend.folding_ranges.write().await.remove(&key);
        }
        Err(e) => {
            backend.client.log_message(MessageType::ERROR, format!("Internal error: {:?}", e)).await;
        }
    }
}
