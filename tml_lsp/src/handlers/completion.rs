use crate::backend::Backend;
use tml_tools::collectors::block_span::{find_enclosing_block, find_indent, BlockKind};
use tml_tools::formatter::INDENT;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

// ───────────────────────── Snippet definition ─────────────────────────

struct SnippetDef {
    label: &'static str,
    detail: &'static str,
    documentation: &'static str,
    insert_text: &'static str,
    min_level: Option<usize>,
    max_level: Option<usize>,
    /// If true, only shown when the enclosing block is an if/elseif
    requires_if_context: bool,
}

impl SnippetDef {
    fn is_available_at(&self, level: usize, in_if_context: bool) -> bool {
        let min_ok = self.min_level.map_or(true, |min| level >= min);
        let max_ok = self.max_level.map_or(true, |max| level <= max);
        let context_ok = !self.requires_if_context || in_if_context;
        min_ok && max_ok && context_ok
    }

    fn to_completion_item(&self, inner_indent: &str) -> CompletionItem {
        let insert_text = self.insert_text
            .replace("{inner_indent}", inner_indent);
        CompletionItem {
            label: self.label.to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some(self.detail.to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: self.documentation.to_string(),
            })),
            filter_text: Some(self.label.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(insert_text),
            ..Default::default()
        }
    }
}

// ───────────────────────── Snippet registry ─────────────────────────

const SNIPPETS: &[SnippetDef] = &[
    SnippetDef {
        label: "fn",
        detail: "Function definition",
        documentation: "Insert a function definition",
        insert_text: "fn ${1:name}(${2:}):\n{inner_indent}${3:pass}\nend$0",
        min_level: None,
        max_level: Some(0),
        requires_if_context: false,
    },
    SnippetDef {
        label: "if",
        detail: "If statement",
        documentation: "Insert an if/end block",
        insert_text: "if ${1:condition}:\n{inner_indent}${2:pass}\nend$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: false,
    },
    SnippetDef {
        label: "for_range",
        detail: "For loop",
        documentation: "Insert a for loop",
        insert_text: "for ${1:i} = ${2:start}:${3:end}:\n{inner_indent}${4:pass}\nend$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: false,
    },
    SnippetDef {
        label: "for_step",
        detail: "For loop with step",
        documentation: "Insert a for loop with step",
        insert_text: "for ${1:i} = ${2:start}:${3:end}:${4:step}:\n{inner_indent}${5:pass}\nend$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: false,
    },
    SnippetDef {
        label: "while",
        detail: "While loop",
        documentation: "Insert a while loop",
        insert_text: "while ${1:condition}:\n{inner_indent}${2:pass}\nend$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: false,
    },
    SnippetDef {
        label: "elseif",
        detail: "Elseif clause",
        documentation: "Insert an elseif clause (continuation of if block)",
        insert_text: "elseif ${1:condition}:\n{inner_indent}${2:pass}$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: true,
    },
    SnippetDef {
        label: "else",
        detail: "Else clause",
        documentation: "Insert an else clause (continuation of if block)",
        insert_text: "else:\n{inner_indent}${1:pass}$0",
        min_level: Some(1),
        max_level: None,
        requires_if_context: true,
    },
];

// ───────────────────────── Handler ─────────────────────────

pub async fn completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri.to_string();
    let line = params.text_document_position.position.line;

    let spans = backend.block_spans.read().await;
    let level = spans
        .get(&uri)
        .map(|s| find_indent(s, line))
        .unwrap_or(0);
    let in_if_context = spans
        .get(&uri)
        .and_then(|s| find_enclosing_block(s, line))
        .map(|b| matches!(b.kind, BlockKind::If | BlockKind::Elseif | BlockKind::MacroIf))
        .unwrap_or(false);

    drop(spans);

    let inner_indent = INDENT.to_string();

    let items: Vec<CompletionItem> = SNIPPETS
        .iter()
        .filter(|s| s.is_available_at(level, in_if_context))
        .map(|s| s.to_completion_item(&inner_indent))
        .collect();

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(CompletionResponse::Array(items)))
    }
}
