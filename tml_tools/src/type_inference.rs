use tml_parser::tml_actions::*;
use crate::checkers::function_call::{infer_builtin_return_type, lookup_builtin};
use crate::symbol_table::{Scope, SimpleTypeKind, SymbolTable, SymbolType};
use crate::visitor::{unpack_binary_bitwise_expressions, unpack_binary_math_expression};
// ───────────────────────── Type promotion ─────────────────────────

/// Returns the more general of two numeric types.
/// Promotion order: uint < int < real
/// Derived types are treated as "unknown" — if one side is Derived,
/// the other side wins (its concrete type is more informative).
pub fn promote(a: &SymbolType, b: &SymbolType) -> SymbolType {
    // If either side is Derived, defer to the other side
    if matches!(a, SymbolType::Derived(_)) { return b.clone(); }
    if matches!(b, SymbolType::Derived(_)) { return a.clone(); }

    match (a, b) {
        // If either side is real, result is real
        (SymbolType::Simple(SimpleTypeKind::Real), _) => SymbolType::Simple(SimpleTypeKind::Real),
        (_, SymbolType::Simple(SimpleTypeKind::Real)) => SymbolType::Simple(SimpleTypeKind::Real),
        // int + uint -> int
        (SymbolType::Simple(SimpleTypeKind::Int), _) => SymbolType::Simple(SimpleTypeKind::Int),
        (_, SymbolType::Simple(SimpleTypeKind::Int)) => SymbolType::Simple(SimpleTypeKind::Int),
        // uint + uint -> uint
        (SymbolType::Simple(SimpleTypeKind::Uint), SymbolType::Simple(SimpleTypeKind::Uint)) => {
            SymbolType::Simple(SimpleTypeKind::Uint)
        }
        // Fallback — return left side
        _ => a.clone(),
    }
}

// ───────────────────────── Main inference entry point ─────────────────────────

pub fn infer_type(expr: &Expression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    match expr {
        Expression::MathExpression(e)     => infer_math(e, table, scope),
        Expression::LogicalExpression(_)  => Some(SymbolType::Simple(SimpleTypeKind::Bool)),
        Expression::BitwiseExpression(e)  => infer_bitwise(e, table, scope),
        Expression::TypeCastExpression(e) => Some(crate::symbol_table::convert_type_spec(&e._type)),
        Expression::NarrowExpression(e)   => infer_type(&e.expr, table, scope),
        Expression::IoReadExpression(_)   => None, // IO type depends on declaration
    }
}

// ───────────────────────── Math expressions ─────────────────────────

fn infer_math(e: &MathExpression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    match e {
        MathExpression::PostfixExpression(p) => infer_postfix(p, table, scope),
        MathExpression::BinaryMathExpression(b) => infer_binary_math(b, table, scope),
        MathExpression::UnaryMathExpression(u) => {
            // Unary +/- preserves type of operand
            let expr = match u {
                UnaryMathExpression::C1(c) => &c.expr,
                UnaryMathExpression::C2(c) => &c.expr,
            };
            infer_type(expr, table, scope)
        }
        MathExpression::ElvisExpression(e) => {
            // Elvis ?: returns type of right side (fallback value)
            infer_type(&e.right_expr, table, scope)
        }
    }
}

fn infer_binary_math(b: &BinaryMathExpression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    let (left, right) = unpack_binary_math_expression(b);
    let left_ty = infer_type(left, table, scope)?;
    let right_ty = infer_type(right, table, scope)?;
    Some(promote(&left_ty, &right_ty))
}

// ───────────────────────── Postfix expressions ─────────────────────────

fn infer_postfix(e: &PostfixExpression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    match e {
        PostfixExpression::Constant(c)          => infer_constant(c),
        PostfixExpression::RValue(r)            => infer_rvalue(r, table, scope),
        PostfixExpression::TensorExpression(t)  => infer_tensor_index(t, table, scope),
        PostfixExpression::FunctionCall(f)      => infer_function_call(f, table, scope),
        PostfixExpression::ExprInParenthesis(e) => infer_type(&e.expr, table, scope),
        PostfixExpression::TransposeExpression(t) => {
            // Transpose preserves tensor type
            infer_postfix(&t.expr, table, scope)
        }
        PostfixExpression::TensorLiteral(_)     => None, // would need element inference
        PostfixExpression::AttributeAccess(_)   => Some(SymbolType::Simple(SimpleTypeKind::Int)), // .len, .size etc. are always int
        PostfixExpression::InputExpression(i)   => Some(crate::symbol_table::convert_type_spec(&i._type)),
    }
}

// ───────────────────────── Constants ─────────────────────────

fn infer_constant(c: &Constant) -> Option<SymbolType> {
    match c {
        Constant::Integer(_)         => Some(SymbolType::Simple(SimpleTypeKind::Int)),
        Constant::UnsignedInteger(_) => Some(SymbolType::Simple(SimpleTypeKind::Uint)),
        Constant::TmlFloat(_)        => Some(SymbolType::Simple(SimpleTypeKind::Real)),
        Constant::TmlString(_)       => Some(SymbolType::Simple(SimpleTypeKind::Str)),
        Constant::Boolean(_)         => Some(SymbolType::Simple(SimpleTypeKind::Bool)),
    }
}

// ───────────────────────── RValue (variable reference) ─────────────────────────

const RESERVED_NAMESPACES: &[&str] = &["t", "p", "n"];

fn infer_rvalue(r: &RValue, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    let dot = &r._ref;
    let root = dot.names.first()?.value.as_str();

    // Namespace references (p.x, t.y, n.z, v.w) are always valid —
    // they come from the runtime environment, not user declarations.
    // We treat them as Derived so downstream inference can continue.
    if RESERVED_NAMESPACES.contains(&root) && dot.names.len() > 1 {
        let full = crate::symbol_table::dot_access_to_string(dot);
        return Some(SymbolType::Derived(full));
    }

    let symbol = table.lookup(root, scope)?;
    Some(symbol.ty.clone())
}

// ───────────────────────── Tensor indexing ─────────────────────────

fn infer_tensor_index(t: &TensorExpression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    let tensor_ty = infer_type(&t.expr, table, scope)?;
    match tensor_ty {
        // tensor<int, N>[i] → int
        SymbolType::Tensor(inner, _) => Some(*inner),
        // Non-tensor indexing — return same type
        other => Some(other),
    }
}

// ───────────────────────── Function call ─────────────────────────

fn infer_function_call(f: &FunctionCall, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    let name = &f.id.value;

    // Check built-ins first
    let args: Vec<&Argument> = match &f.arguments_list {
        None => vec![],
        Some(args) => args.iter().collect(),
    };

    if lookup_builtin(name).is_some() {
        return infer_builtin_return_type(name, &args, table, scope);
    }

    // User-defined function
    let func = table.lookup_function(name)?;
    func.ret_type.clone()
}

// ───────────────────────── Bitwise expressions ─────────────────────────

fn infer_bitwise(e: &BitwiseExpression, table: &SymbolTable, scope: &Scope) -> Option<SymbolType> {
    match e {
        BitwiseExpression::UnaryBitwiseExpression(u) => {
            // ~x preserves type of x
            infer_type(&u.expr, table, scope)
        }
        BitwiseExpression::BinaryBitwiseExpression(b) => {
            let (left, right) = unpack_binary_bitwise_expressions(b);
            let left_ty = infer_type(left, table, scope)?;
            let right_ty = infer_type(right, table, scope)?;
            Some(promote(&left_ty, &right_ty))
        }
    }
}