use crate::formatter::INDENT;
use crate::position::SourcePosition;
use crate::symbol_table::Scope;
use crate::visitor::{default_visit_statement, AstVisitor};
use tml_parser::tml_actions::{FunctionDefinition, IterationStatement, Statement, TranslationUnit};

#[derive(Debug, Clone)]
pub struct BlockSpan {
    pub header_line: u32,
    pub end_line: u32,
    pub body_indent_level: usize,
}

pub fn find_indent(spans: &[BlockSpan], line: u32) -> usize {
    spans
        .iter()
        .filter(|span| span.header_line < line && line <= span.end_line)
        .map(|span| span.body_indent_level)
        .max()
        .unwrap_or(0)
}

pub struct BlockSpanCollector {
    spans: Vec<BlockSpan>,
}

impl BlockSpanCollector {
    pub fn new() -> Self {
        BlockSpanCollector { spans: vec![] }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<BlockSpan> {
        self.visit_translation_unit(unit);
        self.spans
    }

    fn register(
        &mut self,
        header_pos: &SourcePosition,
        end_pos: &SourcePosition,
        keyword_col: usize,
    ) {
        self.spans.push(BlockSpan {
            header_line: header_pos.line as u32,
            end_line: end_pos.line as u32,
            body_indent_level: keyword_col / INDENT.len() + 1,
        });
    }
}

impl AstVisitor for BlockSpanCollector {
    fn visit_function_definition(&mut self, f: &FunctionDefinition) {
        let header_pos = SourcePosition::from_rustemo(&f.header_colon.position);
        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
        self.register(&header_pos, &end_pos, 0);
        let scope = Scope::Function(f.id.value.clone());
        self.visit_statement_block(&f.statement_block, &scope);
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::SelectionStatement(s) => {
                let if_pos = SourcePosition::from_rustemo(&s.if_t.position);
                let end_pos = SourcePosition::from_rustemo(&s.end_t.position);

                self.register(&if_pos, &end_pos, if_pos.column);

                let else_pos_opt = s.else_clause.as_ref().map(|else_clause| {
                    SourcePosition::from_rustemo(&else_clause.else_t.position)
                });

                let elif_pos_opts = s.elseif_clause.as_ref().map(|else_ifs| {
                    else_ifs.iter().map(|else_if| {
                        SourcePosition::from_rustemo(&else_if.else_if_t.position)
                    }).collect::<Vec<_>>()
                });

                if let Some(else_pos) = else_pos_opt {
                    self.register(&else_pos, &end_pos, else_pos.column);
                }

                if let Some(else_if_cols) = elif_pos_opts {
                    for index in 0..else_if_cols.len() {
                        let else_if_pos = &else_if_cols[index];
                        let next_else_if_pos_opt = else_if_cols.get(index + 1).cloned();

                        if let Some(next_else_if_pos) = next_else_if_pos_opt {
                            self.register(&else_if_pos, &next_else_if_pos, else_if_pos.column);
                        }
                    }
                }

                self.visit_selection(s, scope);
            },
            Statement::IterationStatement(i) => {
                match i {
                    IterationStatement::ForIterationStatement(f) => {
                        let header_pos = SourcePosition::from_rustemo(&f.header_colon.position);
                        let end_pos = SourcePosition::from_rustemo(&f.end_t.position);
                        self.register(&header_pos, &end_pos, header_pos.column);
                    },
                    IterationStatement::WhileIterationStatement(w) => {
                        let header_pos = SourcePosition::from_rustemo(&w.while_t.position);
                        let end_pos = SourcePosition::from_rustemo(&w.end_t.position);
                        self.register(&header_pos, &end_pos, header_pos.column);
                    }
                }
                self.visit_iteration(i, scope);
            },
            Statement::ExistsStatement(e) => {
                let header_pos = SourcePosition::from_rustemo(&e.exists_t.position);
                let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
                self.register(&header_pos, &end_pos, header_pos.column);
                if let Some(else_clause) = &e.else_clause {
                    let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
                    self.register(&else_pos, &end_pos, else_pos.column);
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            },
            Statement::NotExistsStatement(e) => {
                let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
                let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
                self.register(&header_pos, &end_pos, header_pos.column);
                if let Some(else_clause) = &e.else_clause {
                    let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
                    self.register(&else_pos, &end_pos, else_pos.column);
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            },
            Statement::FeedthroughStatement(e) => {
                let header_pos = SourcePosition::from_rustemo(&e.feedthrough_t.position);
                let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
                self.register(&header_pos, &end_pos, header_pos.column);
                if let Some(else_clause) = &e.else_clause {
                    let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
                    self.register(&else_pos, &end_pos, else_pos.column);
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            },
            Statement::NotFeedthroughStatement(e) => {
                let header_pos = SourcePosition::from_rustemo(&e.not_t.position);
                let end_pos = SourcePosition::from_rustemo(&e.end_t.position);
                self.register(&header_pos, &end_pos, header_pos.column);
                if let Some(else_clause) = &e.else_clause {
                    let else_pos = SourcePosition::from_rustemo(&else_clause.else_t.position);
                    self.register(&else_pos, &end_pos, else_pos.column);
                }
                self.visit_exists_body(&e.statement_block, &e.else_clause, scope);
            },
            Statement::MacroFor(m) => {
                let header_pos = SourcePosition::from_rustemo(&m.macro_t.position);
                let end_pos = SourcePosition::from_rustemo(&m.body.end_t.position);
                self.register(&header_pos, &end_pos, header_pos.column);
                self.visit_for(&m.body, scope);
            },
            Statement::MacroIf(m) => {
                self.visit_statement(&Statement::SelectionStatement(m.body.clone()), scope);
            },
            other => default_visit_statement(self, other, scope),
        }
    }
}