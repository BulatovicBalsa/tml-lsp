use crate::checkers::function_call::{infer_builtin_return_type, lookup_builtin};
use crate::symbol_table::SymbolTable;
use tml_parser::tml_actions::*;
use crate::constants::RESERVED_NAMESPACES;
use crate::types::{Scope, SimpleTypeKind, SymbolType};
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

pub fn infer_type(expr: &Expression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    match expr {
        Expression::MathExpression(e)     => infer_math(e, table, stack),
        Expression::LogicalExpression(_)  => Some(SymbolType::Simple(SimpleTypeKind::Bool)),
        Expression::BitwiseExpression(e)  => infer_bitwise(e, table, stack),
        Expression::TypeCastExpression(e) => Some(crate::symbol_table::convert_type_spec(&e._type)),
        Expression::NarrowExpression(e)   => infer_type(&e.expr, table, stack),
        Expression::IoReadExpression(_)   => None,
    }
}

fn infer_math(e: &MathExpression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    match e {
        MathExpression::PostfixExpression(p) => infer_postfix(p, table, stack),
        MathExpression::BinaryMathExpression(b) => infer_binary_math(b, table, stack),
        MathExpression::UnaryMathExpression(u) => {
            let expr = match u {
                UnaryMathExpression::C1(c) => &c.expr,
                UnaryMathExpression::C2(c) => &c.expr,
            };
            infer_type(expr, table, stack)
        }
        MathExpression::ElvisExpression(e) => infer_type(&e.right_expr, table, stack),
    }
}

fn infer_binary_math(b: &BinaryMathExpression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    let (left, right) = binary_math_exprs(b);
    let left_ty = infer_type(left, table, stack)?;
    let right_ty = infer_type(right, table, stack)?;
    Some(promote(&left_ty, &right_ty))
}

fn infer_postfix(e: &PostfixExpression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    match e {
        PostfixExpression::Constant(c)            => infer_constant(c),
        PostfixExpression::RValue(r)              => infer_rvalue(r, table, stack),
        PostfixExpression::TensorExpression(t)    => infer_tensor_index(t, table, stack),
        PostfixExpression::FunctionCall(f)        => infer_function_call(f, table, stack),
        PostfixExpression::ExprInParenthesis(e)   => infer_type(&e.expr, table, stack),
        PostfixExpression::TransposeExpression(t) => infer_postfix(&t.expr, table, stack),
        PostfixExpression::TensorLiteral(_)       => None,
        PostfixExpression::AttributeAccess(_)     => Some(SymbolType::Simple(SimpleTypeKind::Int)),
        PostfixExpression::InputExpression(i)     => Some(crate::symbol_table::convert_type_spec(&i._type)),
    }
}

fn infer_constant(c: &Constant) -> Option<SymbolType> {
    match c {
        Constant::Integer(_)         => Some(SymbolType::Simple(SimpleTypeKind::Int)),
        Constant::UnsignedInteger(_) => Some(SymbolType::Simple(SimpleTypeKind::Uint)),
        Constant::TmlFloat(_)        => Some(SymbolType::Simple(SimpleTypeKind::Real)),
        Constant::TmlString(_)       => Some(SymbolType::Simple(SimpleTypeKind::Str)),
        Constant::Boolean(_)         => Some(SymbolType::Simple(SimpleTypeKind::Bool)),
    }
}

fn infer_rvalue(r: &RValue, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    let dot = &r._ref;
    let root = dot.names.first()?.value.as_str();

    if RESERVED_NAMESPACES.contains(&root) && dot.names.len() > 1 {
        let full = crate::symbol_table::dot_access_to_string(dot);
        return Some(SymbolType::Derived(full));
    }

    let symbol = table.lookup_in_stack(root, stack)?;
    Some(symbol.ty.clone())
}

fn infer_tensor_index(t: &TensorExpression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    let tensor_ty = infer_type(&t.expr, table, stack)?;
    match tensor_ty {
        SymbolType::Tensor(inner, _) => Some(*inner),
        other => Some(other),
    }
}

fn infer_function_call(f: &FunctionCall, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    let name = &f.id.value;
    let args: Vec<&Argument> = match &f.arguments_list {
        None => vec![],
        Some(args) => args.iter().collect(),
    };

    if lookup_builtin(name).is_some() {
        return infer_builtin_return_type(name, &args, table, stack);
    }

    let func = table.lookup_function(name)?;
    func.ret_type.clone()
}

fn infer_bitwise(e: &BitwiseExpression, table: &SymbolTable, stack: &[Scope]) -> Option<SymbolType> {
    match e {
        BitwiseExpression::UnaryBitwiseExpression(u) => infer_type(&u.expr, table, stack),
        BitwiseExpression::BinaryBitwiseExpression(b) => {
            let (left, right) = binary_bitwise_exprs(b);
            let left_ty = infer_type(left, table, stack)?;
            let right_ty = infer_type(right, table, stack)?;
            Some(promote(&left_ty, &right_ty))
        }
    }
}