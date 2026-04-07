use rustemo::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct SourcePosition {
    pub byte_offset: usize,
    pub line: usize,   // 0-based
    pub column: usize, // 0-based
}

impl SourcePosition {
    pub fn from_rustemo(pos: &Position) -> Self {
        let (line, column) = pos.line_col
            .map(|lc| (lc.line.saturating_sub(1), lc.column.saturating_sub(1)))
            .unwrap_or((0, 0));
        SourcePosition {
            byte_offset: pos.pos,
            line,
            column,
        }
    }

    pub fn contains_cursor(&self, cursor_line: u32, cursor_col: u32, word_len: usize) -> bool {
        self.line == cursor_line as usize
            && cursor_col as usize >= self.column
            && (cursor_col as usize) < self.column + word_len
    }
}