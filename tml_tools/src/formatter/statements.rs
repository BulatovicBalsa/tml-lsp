use tml_parser::tml_actions::*;
use super::{Format, indent_str};

// ───────────────────────── Top level ─────────────────────────

impl Format for TranslationUnit {
    fn format(&self, indent: usize) -> String {
        self.ext_decls
            .iter()
            .map(|d| d.format(indent))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Format for ExternalDeclaration {
    fn format(&self, indent: usize) -> String {
        match self {
            ExternalDeclaration::FunctionDefinition(s)    => s.format(indent),
            ExternalDeclaration::DeclarationStatement(s)  => s.format(indent),
            ExternalDeclaration::AssignmentStatement(s)   => s.format(indent),
            ExternalDeclaration::IoDeclarationStatement(s) => s.format(indent),
            ExternalDeclaration::IoWriteStatement(s)      => s.format(indent),
            ExternalDeclaration::MacroFor(s)              => s.format(indent),
            ExternalDeclaration::MacroIf(s)               => s.format(indent),
        }
    }
}

// ───────────────────────── Function ─────────────────────────

impl Format for FunctionDefinition {
    fn format(&self, indent: usize) -> String {
        let params = match &self.parameters_list {
            None => String::new(),
            Some(params) => params
                .iter()
                .map(|p| p.format(indent))
                .collect::<Vec<_>>()
                .join(", "),
        };
        let ret = match &self.ret_type {
            None => String::new(),
            Some(t) => format!(" {}", t.format(indent)),
        };
        let body = self.statement_block.format(indent + 1);
        format!(
            "{}fn {}({}){}:\n{}{}end",
            indent_str(indent),
            self.id,
            params,
            ret,
            body,
            indent_str(indent)
        )
    }
}

impl Format for Parameter {
    fn format(&self, indent: usize) -> String {
        let default = match &self.default {
            None => String::new(),
            Some(d) => format!(" = {}", d.value.format(indent)),
        };
        format!("{} {}{}", self._type.format(indent), self.id, default)
    }
}

// ───────────────────────── Statement block ─────────────────────────

impl Format for StatementBlock {
    fn format(&self, indent: usize) -> String {
        self.statements
            .iter()
            .map(|s| format!("{}\n", s.format(indent)))
            .collect()
    }
}

impl Format for Statement {
    fn format(&self, indent: usize) -> String {
        match self {
            Statement::FunctionCallStatement(s)    => s.format(indent),
            Statement::SelectionStatement(s)       => s.format(indent),
            Statement::IterationStatement(s)       => s.format(indent),
            Statement::JumpStatement(s)            => s.format(indent),
            Statement::ExistsStatement(s)          => s.format(indent),
            Statement::NotExistsStatement(s)       => s.format(indent),
            Statement::FeedthroughStatement(s)     => s.format(indent),
            Statement::NotFeedthroughStatement(s)  => s.format(indent),
            Statement::AssignmentStatement(s)      => s.format(indent),
            Statement::DeclarationStatement(s)     => s.format(indent),
            Statement::IoDeclarationStatement(s)   => s.format(indent),
            Statement::IoWriteStatement(s)         => s.format(indent),
            Statement::NoopStatement(s)            => s.format(indent),
            Statement::MacroFor(s)                 => s.format(indent),
            Statement::MacroIf(s)                  => s.format(indent),
        }
    }
}

// ───────────────────────── Declarations ─────────────────────────

impl Format for DeclarationStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} {} = {}",
            indent_str(indent),
            self._type.format(indent),
            self.id.format(indent),
            self.rvalue.format(indent)
        )
    }
}

impl Format for IoDeclarationStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} {}",
            indent_str(indent),
            self.io_type.format(indent),
            self.id.format(indent)
        )
    }
}

// ───────────────────────── Assignments ─────────────────────────

impl Format for AssignmentStatement {
    fn format(&self, indent: usize) -> String {
        match self {
            AssignmentStatement::VarAssignmentStatement(s)    => s.format(indent),
            AssignmentStatement::TensorAssignmentStatement(s) => s.format(indent),
        }
    }
}

impl Format for VarAssignmentStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} {} {}",
            indent_str(indent),
            self.var.format(indent),
            self.op.format(indent),
            self.rvalue.format(indent)
        )
    }
}

impl Format for TensorAssignmentStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} {} {}",
            indent_str(indent),
            self.tensor.format(indent),
            self.op.format(indent),
            self.rvalue.format(indent)
        )
    }
}

impl Format for AssignmentOperator {
    fn format(&self, _indent: usize) -> String {
        match self {
            AssignmentOperator::Assign           => "=",
            AssignmentOperator::MulAssignT       => "*=",
            AssignmentOperator::DivAssignT       => "/=",
            AssignmentOperator::ModAssignT       => "%=",
            AssignmentOperator::AddAssignT       => "+=",
            AssignmentOperator::SubAssignT       => "-=",
            AssignmentOperator::AndAssignT       => "&=",
            AssignmentOperator::KapAssignT       => "^=",
            AssignmentOperator::PipeAssignT      => "|=",
            AssignmentOperator::LeftShiftAssignT  => "<<=",
            AssignmentOperator::RightShiftAssignT => ">>=",
        }.to_string()
    }
}

// ───────────────────────── IO Write ─────────────────────────

impl Format for IoWriteStatement {
    fn format(&self, indent: usize) -> String {
        match self {
            IoWriteStatement::VarIoWriteStatement(s)    => s.format(indent),
            IoWriteStatement::TensorIoWriteStatement(s) => s.format(indent),
        }
    }
}

impl Format for VarIoWriteStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} <- {}",
            indent_str(indent),
            self.io_var.format(indent),
            self.rvalue.format(indent)
        )
    }
}

impl Format for TensorIoWriteStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}{} <- {}",
            indent_str(indent),
            self.io_tensor.format(indent),
            self.rvalue.format(indent)
        )
    }
}

// ───────────────────────── Selection ─────────────────────────

impl Format for SelectionStatement {
    fn format(&self, indent: usize) -> String {
        let mut out = format!(
            "{}if {}:\n{}",
            indent_str(indent),
            self.condition.format(indent),
            self.if_statement_block.format(indent + 1)
        );
        if let Some(elseifs) = &self.elseif_clause {
            for clause in elseifs {
                out.push_str(&clause.format(indent));
            }
        }
        if let Some(else_clause) = &self.else_clause {
            out.push_str(&else_clause.format(indent));
        }
        out.push_str(&format!("{}end", indent_str(indent)));
        out
    }
}

impl Format for ElseIfClause {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}elseif {}:\n{}",
            indent_str(indent),
            self.condition.format(indent),
            self.elseif_statement_block.format(indent + 1)
        )
    }
}

impl Format for ElseClause {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}else:\n{}",
            indent_str(indent),
            self.else_statement_block.format(indent + 1)
        )
    }
}

// ───────────────────────── Iteration ─────────────────────────

impl Format for IterationStatement {
    fn format(&self, indent: usize) -> String {
        match self {
            IterationStatement::ForIterationStatement(s)   => s.format(indent),
            IterationStatement::WhileIterationStatement(s) => s.format(indent),
        }
    }
}

impl Format for ForIterationStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}for {} = {}:\n{}{}end",
            indent_str(indent),
            self.header.idx,
            self.header.iterator_expression.format(indent),
            self.body.statement_block.format(indent + 1),
            indent_str(indent)
        )
    }
}

impl Format for WhileIterationStatement {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}while {}:\n{}{}end",
            indent_str(indent),
            self.condition.format(indent),
            self.statement_block.format(indent + 1),
            indent_str(indent)
        )
    }
}

// ───────────────────────── Jump ─────────────────────────

impl Format for JumpStatement {
    fn format(&self, indent: usize) -> String {
        match self {
            JumpStatement::BreakStatement(s)    => s.format(indent),
            JumpStatement::ReturnStatement(s)   => s.format(indent),
            JumpStatement::ContinueStatement(s) => s.format(indent),
        }
    }
}

impl Format for BreakStatement {
    fn format(&self, indent: usize) -> String {
        format!("{}break", indent_str(indent))
    }
}

impl Format for ContinueStatement {
    fn format(&self, indent: usize) -> String {
        format!("{}continue", indent_str(indent))
    }
}

impl Format for ReturnStatement {
    fn format(&self, indent: usize) -> String {
        match self {
            ReturnStatement::EmptyReturn(_)  => format!("{}return", indent_str(indent)),
            ReturnStatement::ReturnValue(r)  => format!(
                "{}return {}",
                indent_str(indent),
                r.ret_val.format(indent)
            ),
        }
    }
}

// ───────────────────────── Exists / Feedthrough ─────────────────────────

fn format_guarded_block(
    keyword: &str,
    guarded: &[DotAccessExpression],
    block: &StatementBlock,
    else_clause: &Option<ElseClause>,
    indent: usize,
) -> String {
    let vars = guarded
        .iter()
        .map(|g| g.format(indent))
        .collect::<Vec<_>>()
        .join(", ");
    let mut out = format!(
        "{}{} {}:\n{}",
        indent_str(indent),
        keyword,
        vars,
        block.format(indent + 1)
    );
    if let Some(else_c) = else_clause {
        out.push_str(&else_c.format(indent));
    }
    out.push_str(&format!("{}end", indent_str(indent)));
    out
}

impl Format for ExistsStatement {
    fn format(&self, indent: usize) -> String {
        format_guarded_block("exists", &self.guarded, &self.statement_block, &self.else_clause, indent)
    }
}

impl Format for NotExistsStatement {
    fn format(&self, indent: usize) -> String {
        format_guarded_block("not exists", &self.guarded, &self.statement_block, &self.else_clause, indent)
    }
}

impl Format for FeedthroughStatement {
    fn format(&self, indent: usize) -> String {
        format_guarded_block("feedthrough", &self.guarded, &self.statement_block, &self.else_clause, indent)
    }
}

impl Format for NotFeedthroughStatement {
    fn format(&self, indent: usize) -> String {
        format_guarded_block("not feedthrough", &self.guarded, &self.statement_block, &self.else_clause, indent)
    }
}

// ───────────────────────── Misc ─────────────────────────

impl Format for NoopStatement {
    fn format(&self, indent: usize) -> String {
        format!("{}pass", indent_str(indent))
    }
}

impl Format for FunctionCallStatement {
    fn format(&self, indent: usize) -> String {
        format!("{}{}", indent_str(indent), self.call.format(indent))
    }
}

impl Format for MacroFor {
    fn format(&self, indent: usize) -> String {
        format!("{}macro {}", indent_str(indent), self.body.format(indent))
    }
}

impl Format for MacroIf {
    fn format(&self, indent: usize) -> String {
        format!("{}macro {}", indent_str(indent), self.body.format(indent))
    }
}