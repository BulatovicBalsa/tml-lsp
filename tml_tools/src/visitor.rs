use tml_parser::tml_actions::*;
use crate::symbol_table::Scope;

pub trait AstVisitor {
    fn visit_expression(&mut self, expr: &Expression, scope: &Scope) {
        match expr {
            Expression::MathExpression(e)     => self.visit_math(e, scope),
            Expression::LogicalExpression(e)  => self.visit_logical(e, scope),
            Expression::BitwiseExpression(e)  => self.visit_bitwise(e, scope),
            Expression::TypeCastExpression(e) => self.visit_expression(&e.expr, scope),
            Expression::NarrowExpression(e)   => self.visit_expression(&e.expr, scope),
            Expression::IoReadExpression(_)   => {}
        }
    }

    fn visit_statement_block(&mut self, block: &StatementBlock, scope: &Scope) {
        for stmt in &block.statements {
            self.visit_statement(stmt, scope);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement, scope: &Scope) {
        match stmt {
            Statement::DeclarationStatement(d)    => self.visit_expression(&d.rvalue, scope),
            Statement::AssignmentStatement(s)     => self.visit_assignment(s, scope),
            Statement::IoWriteStatement(s)        => self.visit_io_write(s, scope),
            Statement::FunctionCallStatement(s)   => self.visit_postfix(
                &PostfixExpression::FunctionCall(s.call.clone()), scope
            ),
            Statement::SelectionStatement(s)      => self.visit_selection(s, scope),
            Statement::IterationStatement(i)      => self.visit_iteration(i, scope),
            Statement::JumpStatement(j)           => self.visit_jump(j, scope),
            Statement::ExistsStatement(e)         => {
                self.visit_statement_block(&e.statement_block, scope);
                if let Some(else_c) = &e.else_clause {
                    self.visit_statement_block(&else_c.else_statement_block, scope);
                }
            }
            Statement::NotExistsStatement(e)      => {
                self.visit_statement_block(&e.statement_block, scope);
                if let Some(else_c) = &e.else_clause {
                    self.visit_statement_block(&else_c.else_statement_block, scope);
                }
            }
            Statement::FeedthroughStatement(e)    => {
                self.visit_statement_block(&e.statement_block, scope);
                if let Some(else_c) = &e.else_clause {
                    self.visit_statement_block(&else_c.else_statement_block, scope);
                }
            }
            Statement::NotFeedthroughStatement(e) => {
                self.visit_statement_block(&e.statement_block, scope);
                if let Some(else_c) = &e.else_clause {
                    self.visit_statement_block(&else_c.else_statement_block, scope);
                }
            }
            Statement::MacroFor(m)                => self.visit_for(&m.body, scope),
            Statement::MacroIf(m)                 => self.visit_selection(&m.body, scope),
            Statement::IoDeclarationStatement(_)  => {}
            Statement::NoopStatement(_)           => {}
        }
    }

    // ── Assignment ──

    fn visit_assignment(&mut self, stmt: &AssignmentStatement, scope: &Scope) {
        match stmt {
            AssignmentStatement::VarAssignmentStatement(s) => {
                self.visit_expression(&s.rvalue, scope);
            }
            AssignmentStatement::TensorAssignmentStatement(s) => {
                self.visit_lvalue_indices(&s.tensor, scope);
                self.visit_expression(&s.rvalue, scope);
            }
        }
    }

    fn visit_io_write(&mut self, stmt: &IoWriteStatement, scope: &Scope) {
        match stmt {
            IoWriteStatement::VarIoWriteStatement(s) => {
                self.visit_expression(&s.rvalue, scope);
            }
            IoWriteStatement::TensorIoWriteStatement(s) => {
                self.visit_lvalue_indices(&s.io_tensor, scope);
                self.visit_expression(&s.rvalue, scope);
            }
        }
    }

    fn visit_lvalue_indices(&mut self, tensor: &LValueTensor, scope: &Scope) {
        for index in &tensor.indices {
            self.visit_index_expression_list(&index.index, scope);
        }
    }

    // ── Selection ──

    fn visit_selection(&mut self, s: &SelectionStatement, scope: &Scope) {
        self.visit_expression(&s.condition, scope);
        self.visit_statement_block(&s.if_statement_block, scope);
        if let Some(elseifs) = &s.elseif_clause {
            for clause in elseifs {
                self.visit_expression(&clause.condition, scope);
                self.visit_statement_block(&clause.elseif_statement_block, scope);
            }
        }
        if let Some(else_c) = &s.else_clause {
            self.visit_statement_block(&else_c.else_statement_block, scope);
        }
    }

    // ── Iteration ──

    fn visit_iteration(&mut self, i: &IterationStatement, scope: &Scope) {
        match i {
            IterationStatement::ForIterationStatement(f)   => self.visit_for(f, scope),
            IterationStatement::WhileIterationStatement(w) => {
                self.visit_expression(&w.condition, scope);
                self.visit_statement_block(&w.statement_block, scope);
            }
        }
    }

    fn visit_for(&mut self, f: &ForIterationStatement, scope: &Scope) {
        match &f.header.iterator_expression {
            IteratorExpression::Expression(e)      => self.visit_expression(e, scope),
            IteratorExpression::RangeFromTo(r)     => {
                self.visit_expression(&r.start, scope);
                self.visit_expression(&r.stop, scope);
            }
            IteratorExpression::RangeFromStepTo(r) => {
                self.visit_expression(&r.start, scope);
                self.visit_expression(&r.stop, scope);
                self.visit_expression(&r.step, scope);
            }
        }
        self.visit_statement_block(&f.body.statement_block, scope);
    }

    // ── Jump ──

    fn visit_jump(&mut self, j: &JumpStatement, scope: &Scope) {
        if let JumpStatement::ReturnStatement(ReturnStatement::ReturnValue(r)) = j {
            self.visit_expression(&r.ret_val, scope);
        }
    }

    // ── Math ──

    fn visit_math(&mut self, e: &MathExpression, scope: &Scope) {
        match e {
            MathExpression::PostfixExpression(p)    => self.visit_postfix(p, scope),
            MathExpression::BinaryMathExpression(b) => {
                let (left, right) = unpack_binary_math_expression(b);
                self.visit_expression(left, scope);
                self.visit_expression(right, scope);
            }
            MathExpression::UnaryMathExpression(u) => {
                let expr = match u {
                    UnaryMathExpression::C1(c) => &c.expr,
                    UnaryMathExpression::C2(c) => &c.expr,
                };
                self.visit_expression(expr, scope);
            }
            MathExpression::ElvisExpression(e) => {
                self.visit_expression(&e.left_expr, scope);
                self.visit_expression(&e.right_expr, scope);
            }
        }
    }

    // ── Logical ──

    fn visit_logical(&mut self, e: &LogicalExpression, scope: &Scope) {
        match e {
            LogicalExpression::BinaryRelationalExpression(b) => {
                let (left, right) = match b {
                    BinaryRelationalExpression::C1(c) => (&c.left_expr, &c.right_expr),
                    BinaryRelationalExpression::C2(c) => (&c.left_expr, &c.right_expr),
                    BinaryRelationalExpression::C3(c) => (&c.left_expr, &c.right_expr),
                    BinaryRelationalExpression::C4(c) => (&c.left_expr, &c.right_expr),
                    BinaryRelationalExpression::C5(c) => (&c.left_expr, &c.right_expr),
                    BinaryRelationalExpression::C6(c) => (&c.left_expr, &c.right_expr),
                };
                self.visit_expression(left, scope);
                self.visit_expression(right, scope);
            }
            LogicalExpression::BinaryLogicalExpression(b) => {
                let (left, right) = match b {
                    BinaryLogicalExpression::C1(c) => (&c.left_expr, &c.right_expr),
                    BinaryLogicalExpression::C2(c) => (&c.left_expr, &c.right_expr),
                };
                self.visit_expression(left, scope);
                self.visit_expression(right, scope);
            }
            LogicalExpression::UnaryLogicalExpression(u) => {
                self.visit_expression(&u.expr, scope);
            }
        }
    }

    // ── Bitwise ──

    fn visit_bitwise(&mut self, e: &BitwiseExpression, scope: &Scope) {
        match e {
            BitwiseExpression::BinaryBitwiseExpression(b) => {
                let (left, right) = unpack_binary_bitwise_expressions(b);
                self.visit_expression(left, scope);
                self.visit_expression(right, scope);
            }
            BitwiseExpression::UnaryBitwiseExpression(u) => {
                self.visit_expression(&u.expr, scope);
            }
        }
    }

    // ── Postfix ──

    fn visit_postfix(&mut self, e: &PostfixExpression, scope: &Scope) {
        match e {
            PostfixExpression::FunctionCall(f)        => self.visit_function_call(f, scope),
            PostfixExpression::TensorExpression(t)    => {
                self.visit_expression(&t.expr, scope);
                self.visit_index_expression_list(&t.index, scope);
            }
            PostfixExpression::TransposeExpression(t) => self.visit_postfix(&t.expr, scope),
            PostfixExpression::ExprInParenthesis(e)   => self.visit_expression(&e.expr, scope),
            PostfixExpression::AttributeAccess(a)     => self.visit_expression(&a.expr, scope),
            PostfixExpression::TensorLiteral(t)       => self.visit_cube(&t.expr, scope),
            PostfixExpression::RValue(_)              => {}
            PostfixExpression::Constant(_)            => {}
            PostfixExpression::InputExpression(_)     => {}
        }
    }

    fn visit_index_expression_list(&mut self, list: &IndexExpressionList, scope: &Scope) {
        for idx in &list.index_expression_list {
            match idx {
                IndexExpression::C1(e) => self.visit_expression(&e.expr, scope),
                IndexExpression::C2(e) => self.visit_range_expression(&e.expr, scope),
            }
        }
    }
    // ── Function call — checker is implementing its own logic ──

    fn visit_function_call(&mut self, f: &FunctionCall, scope: &Scope) {
        // Default: visit all arguments
        if let Some(args) = &f.arguments_list {
            for arg in args {
                let val = match arg {
                    Argument::C1(a) => &a.value,
                    Argument::C2(a) => &a.value,
                };
                self.visit_expression(val, scope);
            }
        }
    }

    // ── Range ──

    fn visit_range_expression(&mut self, r: &RangeExpression, scope: &Scope) {
        match r {
            RangeExpression::RangeFromTo(r) => {
                self.visit_expression(&r.start, scope);
                self.visit_expression(&r.stop, scope);
            }
            RangeExpression::RangeFrom(r)       => self.visit_expression(&r.start, scope),
            RangeExpression::RangeTo(r)         => self.visit_expression(&r.stop, scope),
            RangeExpression::RangeFromStepTo(r) => {
                self.visit_expression(&r.start, scope);
                self.visit_expression(&r.stop, scope);
                self.visit_expression(&r.step, scope);
            }
            RangeExpression::RangeAll(_) => {}
        }
    }

    // ── Cube ──

    fn visit_cube(&mut self, cube: &Cube, scope: &Scope) {
        for matrix in &cube.elements {
            for array in &matrix.elements {
                for expr in array.elements.iter() {
                    self.visit_expression(expr, scope);
                }
            }
        }
    }
}

pub fn unpack_binary_bitwise_expressions(b: &BinaryBitwiseExpression) -> (&Box<Expression>, &Box<Expression>) {
    let (left, right) = match b {
        BinaryBitwiseExpression::C1(c) => (&c.left_expr, &c.right_expr),
        BinaryBitwiseExpression::C2(c) => (&c.left_expr, &c.right_expr),
        BinaryBitwiseExpression::C3(c) => (&c.left_expr, &c.right_expr),
        BinaryBitwiseExpression::C4(c) => (&c.left_expr, &c.right_expr),
        BinaryBitwiseExpression::C5(c) => (&c.left_expr, &c.right_expr),
    };
    (left, right)
}

pub fn unpack_binary_math_expression(b: &BinaryMathExpression) -> (&Box<Expression>, &Box<Expression>) {
    let (left, right) = match b {
        BinaryMathExpression::C1(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C2(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C3(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C4(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C5(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C6(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C7(c) => (&c.left_expr, &c.right_expr),
        BinaryMathExpression::C8(c) => (&c.left_expr, &c.right_expr),
    };
    (left, right)
}