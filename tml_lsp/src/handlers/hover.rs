use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tml_tools::collectors::hoverable::{HoverableCollector, HoverableKind};
use tml_tools::symbol_table::Scope;
use crate::backend::Backend;

pub async fn hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri.to_string();
    let position = params.text_document_position_params.position;

    let hoverable = backend.hoverable.read().await;
    let nodes = match hoverable.get(&uri) {
        Some(n) => n,
        None => return Ok(None),
    };

    let node = match HoverableCollector::find_at(nodes, position.line, position.character) {
        Some(n) => n,
        None => return Ok(None),
    };

    let tables = backend.symbol_tables.read().await;
    let table = tables.get(&uri);

    let content = table
        .and_then(|t| node.hover_content(t))
        .unwrap_or_else(|| format!("```tml\n{}\n```", node.name()));

    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    }))
}

pub async fn goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri.clone();
    let position = params.text_document_position_params.position;
    let uri_str = uri.to_string();

    let hoverable = backend.hoverable.read().await;
    let nodes = match hoverable.get(&uri_str) {
        Some(n) => n,
        None => return Ok(None),
    };

    let node = match HoverableCollector::find_at(nodes, position.line, position.character) {
        Some(n) => n,
        None => return Ok(None),
    };

    let (target_name, target_scope) = match &node.kind {
        HoverableKind::VariableRef { name }  => (name.clone(), Some(node.scope.clone())),
        HoverableKind::FunctionCall { name } => (name.clone(), None),
        HoverableKind::VariableDecl { .. } | HoverableKind::FunctionDef { .. } => {
            return Ok(None)
        }
    };

    let decl_node = nodes.iter().find(|n| match &n.kind {
        HoverableKind::VariableDecl { name, .. } => {
            *name == target_name && match &target_scope {
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
