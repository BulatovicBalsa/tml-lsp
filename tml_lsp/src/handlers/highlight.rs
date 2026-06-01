use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
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

    // Find the span whose header or end keyword contains the cursor.
    let span = spans.iter().find(|s| {
        let on_header = s.header.line == line
            && character >= s.header.col
            && character < s.header.col + s.header.len as u32;
        let on_end = s.end.line == line
            && character >= s.end.col
            && character < s.end.col + s.end.len as u32;
        on_header || on_end
    });

    match span {
        None => Ok(None),
        Some(s) => Ok(Some(vec![
            make_highlight(s.header.line, s.header.col, s.header.len),
            make_highlight(s.end.line, s.end.col, s.end.len),
        ])),
    }
}