use crate::backend::Backend;
use tml_tools::collectors::semantic_tokens::RawToken;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokens, SemanticTokensParams, SemanticTokensResult};

pub async fn semantic_tokens_full(
    backend: &Backend,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri.to_string();

    let tokens = match backend.last_valid.read().await.get(&uri).cloned() {
        Some(cached) => encode_tokens(cached.semantic_tokens),
        None => return Ok(None),
    };

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
    })))
}

// Converts absolute token positions to LSP delta encoding.
// Tokens must be sorted by line then column before calling this.
fn encode_tokens(mut tokens: Vec<RawToken>) -> Vec<SemanticToken> {
    tokens.sort_by(|a, b| a.line.cmp(&b.line).then(a.col.cmp(&b.col)));

    let mut result = Vec::with_capacity(tokens.len());
    let mut prev_line = 0u32;
    let mut prev_col  = 0u32;

    for token in &tokens {
        let delta_line  = token.line - prev_line;
        let delta_start = if delta_line == 0 {
            token.col - prev_col
        } else {
            token.col
        };

        result.push(SemanticToken {
            delta_line,
            delta_start,
            length: token.len as u32,
            token_type: token.token_type.clone() as u32,
            token_modifiers_bitset: token.modifiers,
        });

        prev_line = token.line;
        prev_col  = token.col;
    }

    result
}