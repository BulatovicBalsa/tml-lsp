use tml_parser::tml_actions::*;
use super::Format;

impl Format for Expression {
    fn format(&self, indent: usize) -> String {
        match self {
            Expression::LogicalExpression(e)  => e.format(indent),
            Expression::TypeCastExpression(e) => e.format(indent),
            Expression::NarrowExpression(e)   => e.format(indent),
            Expression::MathExpression(e)     => e.format(indent),
            Expression::BitwiseExpression(e)  => e.format(indent),
            Expression::IoReadExpression(e)   => e.format(indent),
        }
    }
}

// ───────────────────────── Logical ─────────────────────────

impl Format for LogicalExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            LogicalExpression::BinaryRelationalExpression(e) => e.format(indent),
            LogicalExpression::BinaryLogicalExpression(e)    => e.format(indent),
            LogicalExpression::UnaryLogicalExpression(e)     => e.format(indent),
        }
    }
}

impl Format for BinaryLogicalExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            BinaryLogicalExpression::C1(c) => format!(
                "{} or {}", c.left_expr.format(indent), c.right_expr.format(indent)
            ),
            BinaryLogicalExpression::C2(c) => format!(
                "{} and {}", c.left_expr.format(indent), c.right_expr.format(indent)
            ),
        }
    }
}

impl Format for UnaryLogicalExpression {
    fn format(&self, indent: usize) -> String {
        format!("not {}", self.expr.format(indent))
    }
}

impl Format for BinaryRelationalExpression {
    fn format(&self, indent: usize) -> String {
        let (left, op, right) = match self {
            BinaryRelationalExpression::C1(c) => (&c.left_expr, "==", &c.right_expr),
            BinaryRelationalExpression::C2(c) => (&c.left_expr, "!=", &c.right_expr),
            BinaryRelationalExpression::C3(c) => (&c.left_expr, ">",  &c.right_expr),
            BinaryRelationalExpression::C4(c) => (&c.left_expr, "<",  &c.right_expr),
            BinaryRelationalExpression::C5(c) => (&c.left_expr, ">=", &c.right_expr),
            BinaryRelationalExpression::C6(c) => (&c.left_expr, "<=", &c.right_expr),
        };
        format!("{} {} {}", left.format(indent), op, right.format(indent))
    }
}

// ───────────────────────── Math ─────────────────────────

impl Format for MathExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            MathExpression::PostfixExpression(e) => e.format(indent),
            MathExpression::BinaryMathExpression(e) => e.format(indent),
            MathExpression::UnaryMathExpression(e)  => e.format(indent),
            MathExpression::ElvisExpression(e)      => e.format(indent),
        }
    }
}

impl Format for BinaryMathExpression {
    fn format(&self, indent: usize) -> String {
        let (left, op, right) = match self {
            BinaryMathExpression::C1(c) => (&c.left_expr, "+",  &c.right_expr),
            BinaryMathExpression::C2(c) => (&c.left_expr, "-",  &c.right_expr),
            BinaryMathExpression::C3(c) => (&c.left_expr, "%",  &c.right_expr),
            BinaryMathExpression::C4(c) => (&c.left_expr, "*",  &c.right_expr),
            BinaryMathExpression::C5(c) => (&c.left_expr, ".*", &c.right_expr),
            BinaryMathExpression::C6(c) => (&c.left_expr, "/",  &c.right_expr),
            BinaryMathExpression::C7(c) => (&c.left_expr, "\\", &c.right_expr),
            BinaryMathExpression::C8(c) => (&c.left_expr, "**", &c.right_expr),
        };
        format!("{} {} {}", left.format(indent), op, right.format(indent))
    }
}

impl Format for UnaryMathExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            UnaryMathExpression::C1(c) => format!("-{}", c.expr.format(indent)),
            UnaryMathExpression::C2(c) => format!("+{}", c.expr.format(indent)),
        }
    }
}

impl Format for ElvisExpression {
    fn format(&self, indent: usize) -> String {
        format!(
            "{} ?: {}",
            self.left_expr.format(indent),
            self.right_expr.format(indent)
        )
    }
}

// ───────────────────────── Postfix ─────────────────────────

impl Format for PostfixExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            PostfixExpression::RValue(e)            => e.format(indent),
            PostfixExpression::Constant(e)          => e.format(indent),
            PostfixExpression::ExprInParenthesis(e) => e.format(indent),
            PostfixExpression::TransposeExpression(e) => e.format(indent),
            PostfixExpression::TensorLiteral(e)     => e.format(indent),
            PostfixExpression::TensorExpression(e)  => e.format(indent),
            PostfixExpression::FunctionCall(e)      => e.format(indent),
            PostfixExpression::InputExpression(e)   => e.format(indent),
            PostfixExpression::AttributeAccess(e)   => e.format(indent),
        }
    }
}

impl Format for RValue {
    fn format(&self, indent: usize) -> String {
        self._ref.format(indent)
    }
}

impl Format for ExprInParenthesis {
    fn format(&self, indent: usize) -> String {
        format!("({})", self.expr.format(indent))
    }
}

impl Format for TransposeExpression {
    fn format(&self, indent: usize) -> String {
        format!("{}'", self.expr.format(indent))
    }
}

const MAX_LINE_WIDTH: usize = 80;

impl Format for TensorLiteral {
    fn format(&self, indent: usize) -> String {
        let cube = &self.expr;
        let is_3d = cube.elements.len() > 1;
        let is_2d = !is_3d && cube.elements[0].elements.len() > 1;

        if is_3d {
            // always multiline
            format_cube_multiline(cube, indent)
        } else if is_2d {
            // inline if it fits, otherwise multiline
            let single = format!("[{}]", format_cube_inline(cube));
            if indent * 4 + single.len() <= MAX_LINE_WIDTH {
                single
            } else {
                format_cube_multiline(cube, indent)
            }
        } else {
            // 1D tensor, always inline
            format!("[{}]", format_cube_inline(cube))
        }
    }
}

// Inline format: [1, 2; 3, 4]
fn format_cube_inline(cube: &Cube) -> String {
    cube.elements
        .iter()
        .map(|matrix| {
            matrix.elements
                .iter()
                .map(|array| {
                    array.elements
                        .iter()
                        .map(|e| e.format(0))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .collect::<Vec<_>>()
                .join("; ")
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn format_cube_multiline(cube: &Cube, indent: usize) -> String {
    use super::indent_str;
    let inner_indent = indent_str(indent + 1);
    let close_indent = indent_str(indent);

    let last_matrix = cube.elements.len() - 1;
    let body = cube.elements
        .iter()
        .enumerate()
        .map(|(mi, matrix)| {
            let last_row = matrix.elements.len() - 1;
            matrix.elements
                .iter()
                .enumerate()
                .map(|(ri, array)| {
                    let row = array.elements
                        .iter()
                        .map(|e| e.format(indent + 1))
                        .collect::<Vec<_>>()
                        .join(", ");

                    // separators: ';' for the line end, '|' for the matrix end
                    let sep = if ri < last_row {
                        ";"
                    } else if mi < last_matrix {
                        " |"
                    } else {
                        ""
                    };
                    format!("{}{}{}", inner_indent, row, sep)
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("[\n{}\n{}]", body, close_indent)
}

impl Format for Cube {
    fn format(&self, _indent: usize) -> String {
        format_cube_inline(self)
    }
}

impl Format for Matrix {
    fn format(&self, indent: usize) -> String {
        self.elements
            .iter()
            .map(|a| a.format(indent))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

impl Format for Array {
    fn format(&self, indent: usize) -> String {
        self.elements
            .iter()
            .map(|e| e.format(indent))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Format for TensorExpression {
    fn format(&self, indent: usize) -> String {
        format!("{}[{}]", self.expr.format(indent), self.index.format(indent))
    }
}

impl Format for IndexExpressionList {
    fn format(&self, indent: usize) -> String {
        self.index_expression_list
            .iter()
            .map(|e| e.format(indent))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Format for IndexExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            IndexExpression::C1(c) => c.expr.format(indent),
            IndexExpression::C2(c) => c.expr.format(indent),
        }
    }
}

impl Format for RangeExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            RangeExpression::RangeFromTo(r)     => r.format(indent),
            RangeExpression::RangeFrom(r)       => r.format(indent),
            RangeExpression::RangeTo(r)         => r.format(indent),
            RangeExpression::RangeFromStepTo(r) => r.format(indent),
            RangeExpression::RangeAll(_)        => ":".to_string(),
        }
    }
}

impl Format for RangeFromTo {
    fn format(&self, indent: usize) -> String {
        format!("{}:{}", self.start.format(indent), self.stop.format(indent))
    }
}

impl Format for RangeFrom {
    fn format(&self, indent: usize) -> String {
        format!("{}:", self.start.format(indent))
    }
}

impl Format for RangeTo {
    fn format(&self, indent: usize) -> String {
        format!(":{}", self.stop.format(indent))
    }
}

impl Format for RangeFromStepTo {
    fn format(&self, indent: usize) -> String {
        format!(
            "{}:{}:{}",
            self.start.format(indent),
            self.stop.format(indent),
            self.step.format(indent)
        )
    }
}

impl Format for FunctionCall {
    fn format(&self, indent: usize) -> String {
        let alt = if self.alternative.is_some() { "!" } else { "" };
        let args = match &self.arguments_list {
            None => String::new(),
            Some(arg_list) => arg_list
                .iter()
                .map(|a| a.format(indent))
                .collect::<Vec<_>>()
                .join(", "),
        };
        format!("{}{}({})", self.id.value, alt, args)
    }
}

impl Format for Argument {
    fn format(&self, indent: usize) -> String {
        match self {
            Argument::C1(c) => format!("{} = {}", c.id.value, c.value.format(indent)),
            Argument::C2(c) => c.value.format(indent),
        }
    }
}

impl Format for InputExpression {
    fn format(&self, indent: usize) -> String {
        format!("input({})", self._type.format(indent))
    }
}

impl Format for AttributeAccess {
    fn format(&self, indent: usize) -> String {
        let attr = match &self.attr {
            Attribute::LenKw(s)   => s.clone().value,
            Attribute::SizeKw(s)  => s.clone().value,
            Attribute::NumelKw(s) => s.clone().value,
            Attribute::RowsKw(s)  => s.clone().value,
            Attribute::ColsKw(s)  => s.clone().value,
        };
        format!("{}{}", self.expr.format(indent), attr)
    }
}

// ───────────────────────── Bitwise ─────────────────────────

impl Format for BitwiseExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            BitwiseExpression::BinaryBitwiseExpression(e) => e.format(indent),
            BitwiseExpression::UnaryBitwiseExpression(e)  => e.format(indent),
        }
    }
}

impl Format for BinaryBitwiseExpression {
    fn format(&self, indent: usize) -> String {
        let (left, op, right) = match self {
            BinaryBitwiseExpression::C1(c) => (&c.left_expr, "^",  &c.right_expr),
            BinaryBitwiseExpression::C2(c) => (&c.left_expr, "&",  &c.right_expr),
            BinaryBitwiseExpression::C3(c) => (&c.left_expr, "|",  &c.right_expr),
            BinaryBitwiseExpression::C4(c) => (&c.left_expr, "<<", &c.right_expr),
            BinaryBitwiseExpression::C5(c) => (&c.left_expr, ">>", &c.right_expr),
        };
        format!("{} {} {}", left.format(indent), op, right.format(indent))
    }
}

impl Format for UnaryBitwiseExpression {
    fn format(&self, indent: usize) -> String {
        format!("~{}", self.expr.format(indent))
    }
}

// ───────────────────────── IO Read ─────────────────────────

impl Format for IoReadExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            IoReadExpression::VarIoReadExpression(e)    => e.format(indent),
            IoReadExpression::TensorIoReadExpression(e) => e.format(indent),
        }
    }
}

impl Format for VarIoReadExpression {
    fn format(&self, indent: usize) -> String {
        format!("<- {}", self.io_var.format(indent))
    }
}

impl Format for TensorIoReadExpression {
    fn format(&self, indent: usize) -> String {
        format!("<- {}", self.io_tensor.format(indent))
    }
}

// ───────────────────────── Cast / Narrow ─────────────────────────

impl Format for TypeCastExpression {
    fn format(&self, indent: usize) -> String {
        format!("{}({})", self._type.format(indent), self.expr.format(indent))
    }
}

impl Format for NarrowExpression {
    fn format(&self, indent: usize) -> String {
        format!("narrow({})", self.expr.format(indent))
    }
}

// ───────────────────────── Constants ─────────────────────────

impl Format for Constant {
    fn format(&self, _indent: usize) -> String {
        match self {
            Constant::Integer(c)         => c.format(0),
            Constant::UnsignedInteger(c) => c.format(0),
            Constant::TmlFloat(c)        => c.value.clone(),
            Constant::TmlString(c)       => c.value.clone(),
            Constant::Boolean(c)         => c.format(0),
        }
    }
}

impl Format for Integer {
    fn format(&self, _indent: usize) -> String {
        match self {
            Integer::C1(c) => c.value.clone(),
            Integer::C2(c) => c.value.clone(),
            Integer::C3(c) => c.value.clone(),
        }
    }
}

impl Format for UnsignedInteger {
    fn format(&self, _indent: usize) -> String {
        match self {
            UnsignedInteger::C1(c) => c.value.clone(),
            UnsignedInteger::C2(c) => c.value.clone(),
            UnsignedInteger::C3(c) => c.value.clone(),
        }
    }
}

impl Format for Boolean {
    fn format(&self, _indent: usize) -> String {
        match self {
            Boolean::C1(_) => "true".to_string(),
            Boolean::C2(_) => "false".to_string(),
        }
    }
}

// ───────────────────────── DotAccess / LValue ─────────────────────────

impl Format for DotAccessExpression {
    fn format(&self, indent: usize) -> String {
        let base = self.names.iter().map(|id| id.value.as_str()).collect::<Vec<_>>().join(".");
        let macro_idx = match &self.macro_index {
            None => String::new(),
            Some(m) => {
                let indices = m.index
                    .iter()
                    .map(|e| e.format(indent))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", indices)
            }
        };
        let optional = if self.optional.is_some() { "?" } else { "" };
        format!("{}{}{}", base, macro_idx, optional)
    }
}

impl Format for LValueTensor {
    fn format(&self, indent: usize) -> String {
        let indices = self.indices
            .iter()
            .map(|i| format!("[{}]", i.index.format(indent)))
            .collect::<Vec<_>>()
            .join("");
        format!("{}{}", self.expr.format(indent), indices)
    }
}

impl Format for IteratorExpression {
    fn format(&self, indent: usize) -> String {
        match self {
            IteratorExpression::Expression(e)     => e.format(indent),
            IteratorExpression::RangeFromStepTo(r) => r.format(indent),
            IteratorExpression::RangeFromTo(r)    => r.format(indent),
        }
    }
}