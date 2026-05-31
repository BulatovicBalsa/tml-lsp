use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tml_tools::collectors::folding::TmlFoldingRange;
use crate::backend::Backend;

pub async fn folding_range(
    backend: &Backend,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    let uri = params.text_document.uri.to_string();

    let ranges = backend.folding_ranges.read().await;
    let result = ranges.get(&uri).map(|folds| {
        folds.iter().map(|f: &TmlFoldingRange| FoldingRange {
            start_line: f.start_line,
            end_line: f.end_line,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        }).collect()
    });

    Ok(result)
}
