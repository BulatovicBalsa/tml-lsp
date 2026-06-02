use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tml_tools::collectors::block_span::find_highlight;
use crate::backend::Backend;

fn make_highlight(line: u32, col: u32, len: usize) -> DocumentHighlight {
    DocumentHighlight {
        range: Range {
            start: Position { line, character: col },
            end:   Position { line, character: col + len as u32 },
        },
        kind: Some(DocumentHighlightKind::TEXT),
    }
}

pub async fn document_highlight(
    backend: &Backend,
    params: DocumentHighlightParams,
) -> Result<Option<Vec<DocumentHighlight>>> {
    let uri = params.text_document_position_params.text_document.uri.to_string();
    let line = params.text_document_position_params.position.line;
    let character = params.text_document_position_params.position.character;

    let spans = backend.block_spans.read().await;
    let spans = match spans.get(&uri) {
        Some(s) => s,
        None => return Ok(None),
    };

    match find_highlight(spans, line, character) {
        None => Ok(None),
        Some((header, end)) => Ok(Some(vec![
            make_highlight(header.line, header.col, header.len),
            make_highlight(end.line,    end.col,    end.len),
        ])),
    }
}
