use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tml_tools::collectors::block_span::find_indent;
use tml_tools::formatter::indent_str;
use crate::backend::Backend;

// ───────────────────────── Snippet definition ─────────────────────────

struct SnippetDef {
    label: &'static str,
    detail: &'static str,
    documentation: &'static str,
    insert_text: &'static str,
    min_level: Option<usize>,
    max_level: Option<usize>,
}

impl SnippetDef {
    fn is_available_at(&self, level: usize) -> bool {
        let min_ok = self.min_level.map_or(true, |min| level >= min);
        let max_ok = self.max_level.map_or(true, |max| level <= max);
        min_ok && max_ok
    }

    fn to_completion_item(&self, indent: &str) -> CompletionItem {
        let insert_text = self.insert_text.replace("{indent}", indent);
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
        insert_text: "fn ${1:name}(${2:}):\n{outer_indent}{inner_indent}${3:pass}\n{outer_indent}end$0",
        min_level: None,
        max_level: Some(0), // fn is only valid at global scope
    },
    SnippetDef {
        label: "if",
        detail: "If statement",
        documentation: "Insert an if/end block",
        insert_text: "if ${1:condition}:\n{outer_indent}{inner_indent}${2:pass}\n{outer_indent}end$0",
        min_level: Some(1), // if requires being inside a block
        max_level: None,
    },
    SnippetDef {
        label: "for",
        detail: "For loop",
        documentation: "Insert a for loop",
        insert_text: "for ${1:i} = ${2:start}:${3:end}:\n{outer_indent}{inner_indent}${4:pass}\n{outer_indent}end$0",
        min_level: Some(1),
        max_level: None,
    },
    SnippetDef {
        label: "while",
        detail: "While loop",
        documentation: "Insert a while loop",
        insert_text: "while ${1:condition}:\n{outer_indent}{inner_indent}${2:pass}\n{outer_indent}end$0",
        min_level: Some(1),
        max_level: None,
    },
];

// ───────────────────────── Handler ─────────────────────────

pub async fn completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri.to_string();
    let line = params.text_document_position.position.line;

    let spans = backend.block_spans.read().await
        .get(&uri)
        .cloned()
        .unwrap_or_default();

    let level = find_indent(&spans, line);
    let indent = indent_str(level);

    let items: Vec<CompletionItem> = SNIPPETS
        .iter()
        .filter(|s| s.is_available_at(level))
        .map(|s| s.to_completion_item(&indent))
        .collect();

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(CompletionResponse::Array(items)))
    }
}
