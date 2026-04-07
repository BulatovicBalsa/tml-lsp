use tml_parser::tml_actions::*;
use crate::position::SourcePosition;
use crate::symbol_table::Scope;
use crate::visitor::AstVisitor;

// ───────────────────────── RValueRef ─────────────────────────

#[derive(Debug, Clone)]
pub struct RValueRef {
    pub name: String,
    pub position: SourcePosition,
    pub scope: Scope,
}

// ───────────────────────── Collector ─────────────────────────

pub struct RValueCollector {
    pub refs: Vec<RValueRef>,
}

impl RValueCollector {
    pub fn new() -> Self {
        RValueCollector { refs: vec![] }
    }

    pub fn collect(mut self, unit: &TranslationUnit) -> Vec<RValueRef> {
        for decl in &unit.ext_decls {
            match decl {
                ExternalDeclaration::FunctionDefinition(f) => {
                    let scope = Scope::Function(f.id.clone());
                    self.visit_statement_block(&f.statement_block, &scope);
                }
                ExternalDeclaration::DeclarationStatement(d) => {
                    self.visit_expression(&d.rvalue, &Scope::Global);
                }
                ExternalDeclaration::AssignmentStatement(s) => {
                    self.visit_assignment(s, &Scope::Global);
                }
                ExternalDeclaration::IoWriteStatement(s) => {
                    self.visit_io_write(s, &Scope::Global);
                }
                ExternalDeclaration::MacroFor(m) => {
                    self.visit_for(&m.body, &Scope::Global);
                }
                ExternalDeclaration::MacroIf(m) => {
                    self.visit_selection(&m.body, &Scope::Global);
                }
                ExternalDeclaration::IoDeclarationStatement(_) => {}
            }
        }
        self.refs
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl AstVisitor for RValueCollector {
    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        match e {
            PostfixExpression::RValue(r) => {
                let root = r._ref.names.first().cloned().unwrap_or_default();
                let position = SourcePosition::from_rustemo(&r.position);
                self.refs.push(RValueRef {
                    name: root,
                    position,
                    scope: scope.clone(),
                });
            }
            PostfixExpression::FunctionCall(f)        => self.visit_function_call(f, scope),
            PostfixExpression::TensorExpression(t)    => {
                self.visit_expression(&t.expr, scope);
                self.visit_index_expression_list(&t.index, scope);
            }
            PostfixExpression::TransposeExpression(t) => self.visit_postfix(&t.expr, scope),
            PostfixExpression::ExprInParenthesis(e)   => self.visit_expression(&e.expr, scope),
            PostfixExpression::AttributeAccess(a)     => self.visit_expression(&a.expr, scope),
            PostfixExpression::TensorLiteral(t)       => self.visit_cube(&t.expr, scope),
            PostfixExpression::Constant(_)            => {}
            PostfixExpression::InputExpression(_)     => {}
        }
    }
}