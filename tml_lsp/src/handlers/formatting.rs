use crate::backend::Backend;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::collectors::block_span::find_body_col;
use tml_tools::formatter::Format;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub async fn formatting(
    backend: &Backend,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let uri = params.text_document.uri.to_string();

    let text = match backend.documents.read().await.get(&uri) {
        Some(t) => t.clone(),
        None => return Ok(None),
    };

    let format_result = tokio::task::spawn_blocking(move || {
        TmlParser::new().parse(&text).ok().map(|ast| ast.format(0))
    }).await;

    match format_result {
        Ok(Some(formatted)) => {
            let (lines, last_line_len) = {
                let docs = backend.documents.read().await;
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
                    end:   Position { line: lines as u32, character: last_line_len as u32 },
                },
                new_text: formatted,
            }]))
        }
        Ok(None) => {
            backend.client.log_message(MessageType::WARNING, "Formatting skipped — parse error").await;
            Ok(None)
        }
        Err(e) => {
            backend.client.log_message(MessageType::ERROR, format!("Formatting error: {:?}", e)).await;
            Ok(None)
        }
    }
}

pub async fn on_type_formatting(
    backend: &Backend,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    backend.client.log_message(MessageType::INFO, "On-type formatting triggered").await;

    let uri = params.text_document_position.text_document.uri.to_string();
    let cursor_line = params.text_document_position.position.line;

    let spans = backend.block_spans.read().await.get(&uri).cloned().unwrap_or_default();

    let col = find_body_col(&spans, cursor_line);

    backend.client.log_message(
        MessageType::INFO,
        format!("Cursor at line {}, computed body_col: {}", cursor_line, col),
    ).await;

    if col == 0 {
        return Ok(None);
    }

    let target_indent = " ".repeat(col);

    let current_indent = backend.documents.read().await
        .get(&uri)
        .and_then(|text| text.lines().nth(cursor_line as usize).map(|l| l.to_string()))
        .map(|line| {
            let trimmed_len = line.trim_start().len();
            line[..line.len() - trimmed_len].to_string()
        })
        .unwrap_or_default();

    if current_indent == target_indent {
        return Ok(None);
    }

    Ok(Some(vec![TextEdit {
        range: Range {
            start: Position { line: cursor_line, character: 0 },
            end:   Position { line: cursor_line, character: current_indent.len() as u32 },
        },
        new_text: target_indent,
    }]))
}
