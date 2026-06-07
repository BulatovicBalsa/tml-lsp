use crate::tml_actions::*;

// ── AstVisitor trait ─────────────────────────────────────────────────────────
//
// All methods have empty default implementations — implementors override only
// what they care about. Traversal is driven by accept() methods on AST nodes,
// so implementors never need to manually recurse into children.

pub trait AstVisitor {
    fn visit_translation_unit(&mut self, _node: &TranslationUnit) {}
    fn leave_translation_unit(&mut self, _node: &TranslationUnit) {}
    fn visit_external_declaration(&mut self, _node: &ExternalDeclaration) {}
    fn leave_external_declaration(&mut self, _node: &ExternalDeclaration) {}
    fn visit_function_definition(&mut self, _node: &FunctionDefinition) {}
    fn leave_function_definition(&mut self, _node: &FunctionDefinition) {}
    fn visit_statement_block(&mut self, _node: &StatementBlock) {}
    fn leave_statement_block(&mut self, _node: &StatementBlock) {}
    fn visit_statement(&mut self, _node: &Statement) {}
    fn visit_selection(&mut self, _node: &SelectionStatement) {}
    fn leave_selection(&mut self, _node: &SelectionStatement) {}
    fn visit_else_if_clause(&mut self, _node: &ElseIfClause) {}
    fn leave_else_if_clause(&mut self, _node: &ElseIfClause) {}
    fn visit_else_clause(&mut self, _node: &ElseClause) {}
    fn leave_else_clause(&mut self, _node: &ElseClause) {}
    fn visit_iteration(&mut self, _node: &IterationStatement) {}
    fn visit_for(&mut self, _node: &ForIterationStatement) {}
    fn leave_for(&mut self, _node: &ForIterationStatement) {}
    fn visit_while(&mut self, _node: &WhileIterationStatement) {}
    fn leave_while(&mut self, _node: &WhileIterationStatement) {}
    fn visit_exists(&mut self, _node: &ExistsStatement) {}
    fn leave_exists(&mut self, _node: &ExistsStatement) {}
    fn visit_not_exists(&mut self, _node: &NotExistsStatement) {}
    fn leave_not_exists(&mut self, _node: &NotExistsStatement) {}
    fn visit_feedthrough(&mut self, _node: &FeedthroughStatement) {}
    fn leave_feedthrough(&mut self, _node: &FeedthroughStatement) {}
    fn visit_not_feedthrough(&mut self, _node: &NotFeedthroughStatement) {}
    fn leave_not_feedthrough(&mut self, _node: &NotFeedthroughStatement) {}
    fn visit_assignment(&mut self, _node: &AssignmentStatement) {}
    fn visit_io_write(&mut self, _node: &IoWriteStatement) {}
    fn visit_jump(&mut self, _node: &JumpStatement) {}
    fn visit_expression(&mut self, _node: &Expression) {}
    fn visit_postfix(&mut self, _node: &PostfixExpression) {}
    fn visit_function_call(&mut self, _node: &FunctionCall) {}
    fn visit_macro_for(&mut self, _node: &MacroFor) {}
    fn leave_macro_for(&mut self, _node: &MacroFor) {}
    fn visit_macro_if(&mut self, _node: &MacroIf) {}
    fn leave_macro_if(&mut self, _node: &MacroIf) {}
}
