#[macro_export]
macro_rules! keyword_token {
    ($name:ident, $fn_name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub value: String,
            pub position: Position,
        }
        pub fn $fn_name(_ctx: &Ctx, token: Token) -> $name {
            let value: String = token.value.into();
            let col_start = _ctx
                .position()
                .line_col
                .unwrap()
                .column
                .saturating_sub(value.len() - 1);
            $name {
                position: Position {
                    pos: _ctx.position().pos.saturating_sub(value.len() - 1),
                    line_col: Some(LineColumn {
                        line: _ctx.position().line_col.unwrap().line,
                        column: col_start,
                    }),
                },
                value,
            }
        }
    };
}
