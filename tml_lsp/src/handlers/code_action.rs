use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use crate::backend::Backend;

pub async fn code_action(
    backend: &Backend,
    params: CodeActionParams,
) -> Result<Option<CodeActionResponse>> {
    let uri = params.text_document.uri.clone();
    let uri_str = uri.to_string();
    let range = params.range;

    let fixes = backend.quick_fixes.read().await;
    let fixes = match fixes.get(&uri_str) {
        Some(f) => f,
        None => return Ok(None),
    };

    let matching: Vec<CodeActionOrCommand> = fixes
        .iter()
        .filter(|fix| fix.diag_line >= range.start.line && fix.diag_line <= range.end.line)
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
                edit: Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }),
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
