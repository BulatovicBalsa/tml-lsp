/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::tml::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type integer_const = String;
pub fn integer_const(_ctx: &Ctx, token: Token) -> integer_const {
    token.value.into()
}
pub type unsigned_integer_const = String;
pub fn unsigned_integer_const(_ctx: &Ctx, token: Token) -> unsigned_integer_const {
    token.value.into()
}
pub type hexadecimal_const = String;
pub fn hexadecimal_const(_ctx: &Ctx, token: Token) -> hexadecimal_const {
    token.value.into()
}
pub type unsigned_hexadecimal_const = String;
pub fn unsigned_hexadecimal_const(
    _ctx: &Ctx,
    token: Token,
) -> unsigned_hexadecimal_const {
    token.value.into()
}
pub type binary_const = String;
pub fn binary_const(_ctx: &Ctx, token: Token) -> binary_const {
    token.value.into()
}
pub type unsigned_binary_const = String;
pub fn unsigned_binary_const(_ctx: &Ctx, token: Token) -> unsigned_binary_const {
    token.value.into()
}
pub type float_const = String;
pub fn float_const(_ctx: &Ctx, token: Token) -> float_const {
    token.value.into()
}
pub type boolean_const = String;
pub fn boolean_const(_ctx: &Ctx, token: Token) -> boolean_const {
    token.value.into()
}
pub type string_const = String;
pub fn string_const(_ctx: &Ctx, token: Token) -> string_const {
    token.value.into()
}
pub type id = String;
pub fn id(_ctx: &Ctx, token: Token) -> id {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct translation_unit {
    pub ext_decls: external_declaration1,
}
pub fn translation_unit_c1(
    _ctx: &Ctx,
    ext_decls: external_declaration1,
) -> translation_unit {
    translation_unit { ext_decls }
}
pub type external_declaration1 = Vec<external_declaration>;
pub fn external_declaration1_c1(
    _ctx: &Ctx,
    mut external_declaration1: external_declaration1,
    external_declaration: external_declaration,
) -> external_declaration1 {
    external_declaration1.push(external_declaration);
    external_declaration1
}
pub fn external_declaration1_external_declaration(
    _ctx: &Ctx,
    external_declaration: external_declaration,
) -> external_declaration1 {
    vec![external_declaration]
}
#[derive(Debug, Clone)]
pub enum external_declaration {
    function_definition(function_definition),
    declaration_statement(declaration_statement),
    assignment_statement(assignment_statement),
}
pub fn external_declaration_function_definition(
    _ctx: &Ctx,
    function_definition: function_definition,
) -> external_declaration {
    external_declaration::function_definition(function_definition)
}
pub fn external_declaration_declaration_statement(
    _ctx: &Ctx,
    declaration_statement: declaration_statement,
) -> external_declaration {
    external_declaration::declaration_statement(declaration_statement)
}
pub fn external_declaration_assignment_statement(
    _ctx: &Ctx,
    assignment_statement: assignment_statement,
) -> external_declaration {
    external_declaration::assignment_statement(assignment_statement)
}
#[derive(Debug, Clone)]
pub struct function_definition {
    pub id: id,
    pub parameters_list: parameter0,
    pub ret_type: type_specOpt,
    pub statement_block: statement_block,
}
pub fn function_definition_c1(
    _ctx: &Ctx,
    id: id,
    parameters_list: parameter0,
    ret_type: type_specOpt,
    statement_block: statement_block,
) -> function_definition {
    function_definition {
        id,
        parameters_list,
        ret_type,
        statement_block,
    }
}
pub type parameter1 = Vec<parameter>;
pub fn parameter1_c1(
    _ctx: &Ctx,
    mut parameter1: parameter1,
    parameter: parameter,
) -> parameter1 {
    parameter1.push(parameter);
    parameter1
}
pub fn parameter1_parameter(_ctx: &Ctx, parameter: parameter) -> parameter1 {
    vec![parameter]
}
pub type parameter0 = Option<parameter1>;
pub fn parameter0_parameter1(_ctx: &Ctx, parameter1: parameter1) -> parameter0 {
    Some(parameter1)
}
pub fn parameter0_empty(_ctx: &Ctx) -> parameter0 {
    None
}
pub type type_specOpt = Option<type_spec>;
pub fn type_spec_opt_type_spec(_ctx: &Ctx, type_spec: type_spec) -> type_specOpt {
    Some(type_spec)
}
pub fn type_spec_opt_empty(_ctx: &Ctx) -> type_specOpt {
    None
}
#[derive(Debug, Clone)]
pub struct parameter {
    pub _type: type_spec,
    pub id: id,
    pub default: default_param_valueOpt,
}
pub fn parameter_c1(
    _ctx: &Ctx,
    _type: type_spec,
    id: id,
    default: default_param_valueOpt,
) -> parameter {
    parameter { _type, id, default }
}
pub type default_param_valueOpt = Option<default_param_value>;
pub fn default_param_value_opt_default_param_value(
    _ctx: &Ctx,
    default_param_value: default_param_value,
) -> default_param_valueOpt {
    Some(default_param_value)
}
pub fn default_param_value_opt_empty(_ctx: &Ctx) -> default_param_valueOpt {
    None
}
#[derive(Debug, Clone)]
pub struct default_param_value {
    pub value: expression,
}
pub fn default_param_value_c1(_ctx: &Ctx, value: expression) -> default_param_value {
    default_param_value { value }
}
#[derive(Debug, Clone)]
pub struct type_cast_expression {
    pub _type: Box<type_spec>,
    pub expr: Box<expression>,
}
pub fn type_cast_expression_c1(
    _ctx: &Ctx,
    _type: type_spec,
    expr: expression,
) -> type_cast_expression {
    type_cast_expression {
        _type: Box::new(_type),
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub enum type_spec {
    simple_type(simple_type),
    derived_type(derived_type),
    tensor_constructor(tensor_constructor),
}
pub fn type_spec_simple_type(_ctx: &Ctx, simple_type: simple_type) -> type_spec {
    type_spec::simple_type(simple_type)
}
pub fn type_spec_derived_type(_ctx: &Ctx, derived_type: derived_type) -> type_spec {
    type_spec::derived_type(derived_type)
}
pub fn type_spec_tensor_constructor(
    _ctx: &Ctx,
    tensor_constructor: tensor_constructor,
) -> type_spec {
    type_spec::tensor_constructor(tensor_constructor)
}
#[derive(Debug, Clone)]
pub struct simple_type {
    pub _type: simple_type_spec,
}
pub fn simple_type_c1(_ctx: &Ctx, _type: simple_type_spec) -> simple_type {
    simple_type { _type }
}
#[derive(Debug, Clone)]
pub struct derived_type {
    pub name: dot_access_expression,
    pub brackets: square_brackets0,
}
pub fn derived_type_c1(
    _ctx: &Ctx,
    name: dot_access_expression,
    brackets: square_brackets0,
) -> derived_type {
    derived_type { name, brackets }
}
#[derive(Debug, Clone)]
pub enum square_brackets1 {
    square_brackets1(Box<square_brackets1>),
    square_brackets,
}
pub fn square_brackets1_square_brackets1(
    _ctx: &Ctx,
    square_brackets1: square_brackets1,
) -> square_brackets1 {
    square_brackets1::square_brackets1(Box::new(square_brackets1))
}
pub fn square_brackets1_square_brackets(_ctx: &Ctx) -> square_brackets1 {
    square_brackets1::square_brackets
}
pub type square_brackets0 = Option<square_brackets1>;
pub fn square_brackets0_square_brackets1(
    _ctx: &Ctx,
    square_brackets1: square_brackets1,
) -> square_brackets0 {
    Some(square_brackets1)
}
pub fn square_brackets0_empty(_ctx: &Ctx) -> square_brackets0 {
    None
}
#[derive(Debug, Clone)]
pub enum simple_type_spec {
    intt,
    uintt,
    realt,
    boolt,
    strt,
}
pub fn simple_type_spec_intt(_ctx: &Ctx) -> simple_type_spec {
    simple_type_spec::intt
}
pub fn simple_type_spec_uintt(_ctx: &Ctx) -> simple_type_spec {
    simple_type_spec::uintt
}
pub fn simple_type_spec_realt(_ctx: &Ctx) -> simple_type_spec {
    simple_type_spec::realt
}
pub fn simple_type_spec_boolt(_ctx: &Ctx) -> simple_type_spec {
    simple_type_spec::boolt
}
pub fn simple_type_spec_strt(_ctx: &Ctx) -> simple_type_spec {
    simple_type_spec::strt
}
#[derive(Debug, Clone)]
pub struct tensor_constructor {
    pub _type: Box<type_spec>,
    pub dimensions: expression1,
}
pub fn tensor_constructor_c1(
    _ctx: &Ctx,
    _type: type_spec,
    dimensions: expression1,
) -> tensor_constructor {
    tensor_constructor {
        _type: Box::new(_type),
        dimensions,
    }
}
pub type expression1 = Vec<expression>;
pub fn expression1_c1(
    _ctx: &Ctx,
    mut expression1: expression1,
    expression: expression,
) -> expression1 {
    expression1.push(expression);
    expression1
}
pub fn expression1_expression(_ctx: &Ctx, expression: expression) -> expression1 {
    vec![expression]
}
#[derive(Debug, Clone)]
pub struct statement_block {
    pub statements: statement1,
}
pub fn statement_block_c1(_ctx: &Ctx, statements: statement1) -> statement_block {
    statement_block { statements }
}
pub type statement1 = Vec<statement>;
pub fn statement1_c1(
    _ctx: &Ctx,
    mut statement1: statement1,
    statement: statement,
) -> statement1 {
    statement1.push(statement);
    statement1
}
pub fn statement1_statement(_ctx: &Ctx, statement: statement) -> statement1 {
    vec![statement]
}
#[derive(Debug, Clone)]
pub enum statement {
    function_call(function_call),
    selection_statement(selection_statement),
    iteration_statement(iteration_statement),
    jump_statement(jump_statement),
    exists_statement(exists_statement),
    not_exists_statement(not_exists_statement),
    assignment_statement(assignment_statement),
    declaration_statement(declaration_statement),
}
pub fn statement_function_call(_ctx: &Ctx, function_call: function_call) -> statement {
    statement::function_call(function_call)
}
pub fn statement_selection_statement(
    _ctx: &Ctx,
    selection_statement: selection_statement,
) -> statement {
    statement::selection_statement(selection_statement)
}
pub fn statement_iteration_statement(
    _ctx: &Ctx,
    iteration_statement: iteration_statement,
) -> statement {
    statement::iteration_statement(iteration_statement)
}
pub fn statement_jump_statement(
    _ctx: &Ctx,
    jump_statement: jump_statement,
) -> statement {
    statement::jump_statement(jump_statement)
}
pub fn statement_exists_statement(
    _ctx: &Ctx,
    exists_statement: exists_statement,
) -> statement {
    statement::exists_statement(exists_statement)
}
pub fn statement_not_exists_statement(
    _ctx: &Ctx,
    not_exists_statement: not_exists_statement,
) -> statement {
    statement::not_exists_statement(not_exists_statement)
}
pub fn statement_assignment_statement(
    _ctx: &Ctx,
    assignment_statement: assignment_statement,
) -> statement {
    statement::assignment_statement(assignment_statement)
}
pub fn statement_declaration_statement(
    _ctx: &Ctx,
    declaration_statement: declaration_statement,
) -> statement {
    statement::declaration_statement(declaration_statement)
}
#[derive(Debug, Clone)]
pub struct selection_statement {
    pub condition: expression,
    pub if_statement_block: Box<statement_block>,
    pub elseif_clause: elseif_clause0,
    pub else_clause: else_clauseOpt,
}
pub fn selection_statement_c1(
    _ctx: &Ctx,
    condition: expression,
    if_statement_block: statement_block,
    elseif_clause: elseif_clause0,
    else_clause: else_clauseOpt,
) -> selection_statement {
    selection_statement {
        condition,
        if_statement_block: Box::new(if_statement_block),
        elseif_clause,
        else_clause,
    }
}
pub type elseif_clause1 = Vec<elseif_clause>;
pub fn elseif_clause1_c1(
    _ctx: &Ctx,
    mut elseif_clause1: elseif_clause1,
    elseif_clause: elseif_clause,
) -> elseif_clause1 {
    elseif_clause1.push(elseif_clause);
    elseif_clause1
}
pub fn elseif_clause1_elseif_clause(
    _ctx: &Ctx,
    elseif_clause: elseif_clause,
) -> elseif_clause1 {
    vec![elseif_clause]
}
pub type elseif_clause0 = Option<elseif_clause1>;
pub fn elseif_clause0_elseif_clause1(
    _ctx: &Ctx,
    elseif_clause1: elseif_clause1,
) -> elseif_clause0 {
    Some(elseif_clause1)
}
pub fn elseif_clause0_empty(_ctx: &Ctx) -> elseif_clause0 {
    None
}
pub type else_clauseOpt = Option<else_clause>;
pub fn else_clause_opt_else_clause(
    _ctx: &Ctx,
    else_clause: else_clause,
) -> else_clauseOpt {
    Some(else_clause)
}
pub fn else_clause_opt_empty(_ctx: &Ctx) -> else_clauseOpt {
    None
}
#[derive(Debug, Clone)]
pub struct else_clause {
    pub else_statement_block: Box<statement_block>,
}
pub fn else_clause_c1(_ctx: &Ctx, else_statement_block: statement_block) -> else_clause {
    else_clause {
        else_statement_block: Box::new(else_statement_block),
    }
}
#[derive(Debug, Clone)]
pub struct elseif_clause {
    pub condition: expression,
    pub elseif_statement_block: Box<statement_block>,
}
pub fn elseif_clause_c1(
    _ctx: &Ctx,
    condition: expression,
    elseif_statement_block: statement_block,
) -> elseif_clause {
    elseif_clause {
        condition,
        elseif_statement_block: Box::new(elseif_statement_block),
    }
}
#[derive(Debug, Clone)]
pub enum iteration_statement {
    for_iteration_statement(for_iteration_statement),
    while_iteration_statement(while_iteration_statement),
}
pub fn iteration_statement_for_iteration_statement(
    _ctx: &Ctx,
    for_iteration_statement: for_iteration_statement,
) -> iteration_statement {
    iteration_statement::for_iteration_statement(for_iteration_statement)
}
pub fn iteration_statement_while_iteration_statement(
    _ctx: &Ctx,
    while_iteration_statement: while_iteration_statement,
) -> iteration_statement {
    iteration_statement::while_iteration_statement(while_iteration_statement)
}
#[derive(Debug, Clone)]
pub struct for_iteration_statement {
    pub idx: id,
    pub iterator_expression: expression,
    pub statement_block: Box<statement_block>,
}
pub fn for_iteration_statement_c1(
    _ctx: &Ctx,
    idx: id,
    iterator_expression: expression,
    statement_block: statement_block,
) -> for_iteration_statement {
    for_iteration_statement {
        idx,
        iterator_expression,
        statement_block: Box::new(statement_block),
    }
}
#[derive(Debug, Clone)]
pub struct while_iteration_statement {
    pub condition: expression,
    pub statement_block: Box<statement_block>,
}
pub fn while_iteration_statement_c1(
    _ctx: &Ctx,
    condition: expression,
    statement_block: statement_block,
) -> while_iteration_statement {
    while_iteration_statement {
        condition,
        statement_block: Box::new(statement_block),
    }
}
#[derive(Debug, Clone)]
pub enum jump_statement {
    break_statement(break_statement),
    return_statement(return_statement),
    continue_statement(continue_statement),
}
pub fn jump_statement_break_statement(
    _ctx: &Ctx,
    break_statement: break_statement,
) -> jump_statement {
    jump_statement::break_statement(break_statement)
}
pub fn jump_statement_return_statement(
    _ctx: &Ctx,
    return_statement: return_statement,
) -> jump_statement {
    jump_statement::return_statement(return_statement)
}
pub fn jump_statement_continue_statement(
    _ctx: &Ctx,
    continue_statement: continue_statement,
) -> jump_statement {
    jump_statement::continue_statement(continue_statement)
}
#[derive(Debug, Clone)]
pub enum break_statement {
    breakt,
}
pub fn break_statement_breakt(_ctx: &Ctx) -> break_statement {
    break_statement::breakt
}
#[derive(Debug, Clone)]
pub enum continue_statement {
    continuet,
}
pub fn continue_statement_continuet(_ctx: &Ctx) -> continue_statement {
    continue_statement::continuet
}
#[derive(Debug, Clone)]
pub enum return_statement {
    empty_return(empty_return),
    return_value(return_value),
}
pub fn return_statement_empty_return(
    _ctx: &Ctx,
    empty_return: empty_return,
) -> return_statement {
    return_statement::empty_return(empty_return)
}
pub fn return_statement_return_value(
    _ctx: &Ctx,
    return_value: return_value,
) -> return_statement {
    return_statement::return_value(return_value)
}
#[derive(Debug, Clone)]
pub enum empty_return {
    returnt,
}
pub fn empty_return_returnt(_ctx: &Ctx) -> empty_return {
    empty_return::returnt
}
#[derive(Debug, Clone)]
pub struct return_value {
    pub ret_val: expression,
}
pub fn return_value_c1(_ctx: &Ctx, ret_val: expression) -> return_value {
    return_value { ret_val }
}
#[derive(Debug, Clone)]
pub struct exists_statement {
    pub guarded: dot_access_expression1,
    pub statement_block: Box<statement_block>,
    pub else_clause: else_clauseOpt,
}
pub fn exists_statement_c1(
    _ctx: &Ctx,
    guarded: dot_access_expression1,
    statement_block: statement_block,
    else_clause: else_clauseOpt,
) -> exists_statement {
    exists_statement {
        guarded,
        statement_block: Box::new(statement_block),
        else_clause,
    }
}
pub type dot_access_expression1 = Vec<dot_access_expression>;
pub fn dot_access_expression1_c1(
    _ctx: &Ctx,
    mut dot_access_expression1: dot_access_expression1,
    dot_access_expression: dot_access_expression,
) -> dot_access_expression1 {
    dot_access_expression1.push(dot_access_expression);
    dot_access_expression1
}
pub fn dot_access_expression1_dot_access_expression(
    _ctx: &Ctx,
    dot_access_expression: dot_access_expression,
) -> dot_access_expression1 {
    vec![dot_access_expression]
}
#[derive(Debug, Clone)]
pub struct not_exists_statement {
    pub guarded: dot_access_expression1,
    pub statement_block: Box<statement_block>,
    pub else_clause: else_clauseOpt,
}
pub fn not_exists_statement_c1(
    _ctx: &Ctx,
    guarded: dot_access_expression1,
    statement_block: statement_block,
    else_clause: else_clauseOpt,
) -> not_exists_statement {
    not_exists_statement {
        guarded,
        statement_block: Box::new(statement_block),
        else_clause,
    }
}
#[derive(Debug, Clone)]
pub enum assignment_statement {
    var_assignment_statement(var_assignment_statement),
    tensor_assignment_statement(tensor_assignment_statement),
}
pub fn assignment_statement_var_assignment_statement(
    _ctx: &Ctx,
    var_assignment_statement: var_assignment_statement,
) -> assignment_statement {
    assignment_statement::var_assignment_statement(var_assignment_statement)
}
pub fn assignment_statement_tensor_assignment_statement(
    _ctx: &Ctx,
    tensor_assignment_statement: tensor_assignment_statement,
) -> assignment_statement {
    assignment_statement::tensor_assignment_statement(tensor_assignment_statement)
}
#[derive(Debug, Clone)]
pub struct var_assignment_statement {
    pub var: dot_access_expression,
    pub op: assignment_operator,
    pub rvalue: expression,
}
pub fn var_assignment_statement_c1(
    _ctx: &Ctx,
    var: dot_access_expression,
    op: assignment_operator,
    rvalue: expression,
) -> var_assignment_statement {
    var_assignment_statement {
        var,
        op,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct tensor_assignment_statement {
    pub tensor: lvalue_tensor,
    pub op: assignment_operator,
    pub rvalue: expression,
}
pub fn tensor_assignment_statement_c1(
    _ctx: &Ctx,
    tensor: lvalue_tensor,
    op: assignment_operator,
    rvalue: expression,
) -> tensor_assignment_statement {
    tensor_assignment_statement {
        tensor,
        op,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct declaration_statement {
    pub _type: type_spec,
    pub id: id,
    pub rvalue: expression,
}
pub fn declaration_statement_c1(
    _ctx: &Ctx,
    _type: type_spec,
    id: id,
    rvalue: expression,
) -> declaration_statement {
    declaration_statement {
        _type,
        id,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub enum expression {
    logical_expression(logical_expression),
    type_cast_expression(type_cast_expression),
    math_expression(math_expression),
}
pub fn expression_logical_expression(
    _ctx: &Ctx,
    logical_expression: logical_expression,
) -> expression {
    expression::logical_expression(logical_expression)
}
pub fn expression_type_cast_expression(
    _ctx: &Ctx,
    type_cast_expression: type_cast_expression,
) -> expression {
    expression::type_cast_expression(type_cast_expression)
}
pub fn expression_math_expression(
    _ctx: &Ctx,
    math_expression: math_expression,
) -> expression {
    expression::math_expression(math_expression)
}
#[derive(Debug, Clone)]
pub struct attribute_access {
    pub expr: Box<expression>,
    pub attr: id,
}
pub fn attribute_access_c1(_ctx: &Ctx, expr: expression, attr: id) -> attribute_access {
    attribute_access {
        expr: Box::new(expr),
        attr,
    }
}
#[derive(Debug, Clone)]
pub enum assignment_operator {
    assign,
    mulassignt,
    divassignt,
    modassignt,
    addassignt,
    subassignt,
}
pub fn assignment_operator_assign(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::assign
}
pub fn assignment_operator_mulassignt(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::mulassignt
}
pub fn assignment_operator_divassignt(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::divassignt
}
pub fn assignment_operator_modassignt(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::modassignt
}
pub fn assignment_operator_addassignt(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::addassignt
}
pub fn assignment_operator_subassignt(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::subassignt
}
#[derive(Debug, Clone)]
pub enum logical_expression {
    binary_relational_expression(binary_relational_expression),
    binary_logical_expression(binary_logical_expression),
    unary_logical_expression(unary_logical_expression),
}
pub fn logical_expression_binary_relational_expression(
    _ctx: &Ctx,
    binary_relational_expression: binary_relational_expression,
) -> logical_expression {
    logical_expression::binary_relational_expression(binary_relational_expression)
}
pub fn logical_expression_binary_logical_expression(
    _ctx: &Ctx,
    binary_logical_expression: binary_logical_expression,
) -> logical_expression {
    logical_expression::binary_logical_expression(binary_logical_expression)
}
pub fn logical_expression_unary_logical_expression(
    _ctx: &Ctx,
    unary_logical_expression: unary_logical_expression,
) -> logical_expression {
    logical_expression::unary_logical_expression(unary_logical_expression)
}
#[derive(Debug, Clone)]
pub struct binary_logical_expressionC1 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_logical_expressionC2 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub enum binary_logical_expression {
    C1(binary_logical_expressionC1),
    C2(binary_logical_expressionC2),
}
pub fn binary_logical_expression_c1(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_logical_expression {
    binary_logical_expression::C1(binary_logical_expressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_logical_expression_c2(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_logical_expression {
    binary_logical_expression::C2(binary_logical_expressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct unary_logical_expression {
    pub expr: Box<expression>,
}
pub fn unary_logical_expression_c1(
    _ctx: &Ctx,
    expr: expression,
) -> unary_logical_expression {
    unary_logical_expression {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC1 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC2 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC3 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC4 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC5 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_relational_expressionC6 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub enum binary_relational_expression {
    C1(binary_relational_expressionC1),
    C2(binary_relational_expressionC2),
    C3(binary_relational_expressionC3),
    C4(binary_relational_expressionC4),
    C5(binary_relational_expressionC5),
    C6(binary_relational_expressionC6),
}
pub fn binary_relational_expression_c1(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C1(binary_relational_expressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c2(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C2(binary_relational_expressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c3(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C3(binary_relational_expressionC3 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c4(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C4(binary_relational_expressionC4 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c5(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C5(binary_relational_expressionC5 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c6(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_relational_expression {
    binary_relational_expression::C6(binary_relational_expressionC6 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub enum math_expression {
    postfix_expression(postfix_expression),
    binary_math_expression(binary_math_expression),
    unary_math_expression(unary_math_expression),
    elvis_expression(elvis_expression),
}
pub fn math_expression_postfix_expression(
    _ctx: &Ctx,
    postfix_expression: postfix_expression,
) -> math_expression {
    math_expression::postfix_expression(postfix_expression)
}
pub fn math_expression_binary_math_expression(
    _ctx: &Ctx,
    binary_math_expression: binary_math_expression,
) -> math_expression {
    math_expression::binary_math_expression(binary_math_expression)
}
pub fn math_expression_unary_math_expression(
    _ctx: &Ctx,
    unary_math_expression: unary_math_expression,
) -> math_expression {
    math_expression::unary_math_expression(unary_math_expression)
}
pub fn math_expression_elvis_expression(
    _ctx: &Ctx,
    elvis_expression: elvis_expression,
) -> math_expression {
    math_expression::elvis_expression(elvis_expression)
}
#[derive(Debug, Clone)]
pub struct elvis_expression {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
pub fn elvis_expression_c1(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> elvis_expression {
    elvis_expression {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    }
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC1 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC2 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC3 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC4 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC5 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC6 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct binary_math_expressionC7 {
    pub left_expr: Box<expression>,
    pub right_expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub enum binary_math_expression {
    C1(binary_math_expressionC1),
    C2(binary_math_expressionC2),
    C3(binary_math_expressionC3),
    C4(binary_math_expressionC4),
    C5(binary_math_expressionC5),
    C6(binary_math_expressionC6),
    C7(binary_math_expressionC7),
}
pub fn binary_math_expression_c1(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C1(binary_math_expressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c2(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C2(binary_math_expressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c3(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C3(binary_math_expressionC3 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c4(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C4(binary_math_expressionC4 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c5(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C5(binary_math_expressionC5 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c6(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C6(binary_math_expressionC6 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c7(
    _ctx: &Ctx,
    left_expr: expression,
    right_expr: expression,
) -> binary_math_expression {
    binary_math_expression::C7(binary_math_expressionC7 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct unary_math_expressionC1 {
    pub expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct unary_math_expressionC2 {
    pub expr: Box<expression>,
}
#[derive(Debug, Clone)]
pub enum unary_math_expression {
    C1(unary_math_expressionC1),
    C2(unary_math_expressionC2),
}
pub fn unary_math_expression_c1(_ctx: &Ctx, expr: expression) -> unary_math_expression {
    unary_math_expression::C1(unary_math_expressionC1 {
        expr: Box::new(expr),
    })
}
pub fn unary_math_expression_c2(_ctx: &Ctx, expr: expression) -> unary_math_expression {
    unary_math_expression::C2(unary_math_expressionC2 {
        expr: Box::new(expr),
    })
}
#[derive(Debug, Clone)]
pub enum postfix_expression {
    rvalue(rvalue),
    constant(constant),
    expr_in_parenthesis(expr_in_parenthesis),
    transpose_expression(transpose_expression),
    tensor_literal(tensor_literal),
    tensor_expression(tensor_expression),
    function_call(function_call),
    attribute_access(attribute_access),
    range_expression(range_expression),
}
pub fn postfix_expression_rvalue(_ctx: &Ctx, rvalue: rvalue) -> postfix_expression {
    postfix_expression::rvalue(rvalue)
}
pub fn postfix_expression_constant(
    _ctx: &Ctx,
    constant: constant,
) -> postfix_expression {
    postfix_expression::constant(constant)
}
pub fn postfix_expression_expr_in_parenthesis(
    _ctx: &Ctx,
    expr_in_parenthesis: expr_in_parenthesis,
) -> postfix_expression {
    postfix_expression::expr_in_parenthesis(expr_in_parenthesis)
}
pub fn postfix_expression_transpose_expression(
    _ctx: &Ctx,
    transpose_expression: transpose_expression,
) -> postfix_expression {
    postfix_expression::transpose_expression(transpose_expression)
}
pub fn postfix_expression_tensor_literal(
    _ctx: &Ctx,
    tensor_literal: tensor_literal,
) -> postfix_expression {
    postfix_expression::tensor_literal(tensor_literal)
}
pub fn postfix_expression_tensor_expression(
    _ctx: &Ctx,
    tensor_expression: tensor_expression,
) -> postfix_expression {
    postfix_expression::tensor_expression(tensor_expression)
}
pub fn postfix_expression_function_call(
    _ctx: &Ctx,
    function_call: function_call,
) -> postfix_expression {
    postfix_expression::function_call(function_call)
}
pub fn postfix_expression_attribute_access(
    _ctx: &Ctx,
    attribute_access: attribute_access,
) -> postfix_expression {
    postfix_expression::attribute_access(attribute_access)
}
pub fn postfix_expression_range_expression(
    _ctx: &Ctx,
    range_expression: range_expression,
) -> postfix_expression {
    postfix_expression::range_expression(range_expression)
}
#[derive(Debug, Clone)]
pub struct rvalue {
    pub _ref: dot_access_expression,
}
pub fn rvalue_c1(_ctx: &Ctx, _ref: dot_access_expression) -> rvalue {
    rvalue { _ref }
}
#[derive(Debug, Clone)]
pub enum range_expression {
    range_without_step_expression(range_without_step_expression),
    range_with_step_expression(range_with_step_expression),
}
pub fn range_expression_range_without_step_expression(
    _ctx: &Ctx,
    range_without_step_expression: range_without_step_expression,
) -> range_expression {
    range_expression::range_without_step_expression(range_without_step_expression)
}
pub fn range_expression_range_with_step_expression(
    _ctx: &Ctx,
    range_with_step_expression: range_with_step_expression,
) -> range_expression {
    range_expression::range_with_step_expression(range_with_step_expression)
}
#[derive(Debug, Clone)]
pub enum range_operand {
    rvalue(rvalue),
    constant(constant),
    expr_in_parenthesis(expr_in_parenthesis),
    function_call(function_call),
}
pub fn range_operand_rvalue(_ctx: &Ctx, rvalue: rvalue) -> range_operand {
    range_operand::rvalue(rvalue)
}
pub fn range_operand_constant(_ctx: &Ctx, constant: constant) -> range_operand {
    range_operand::constant(constant)
}
pub fn range_operand_expr_in_parenthesis(
    _ctx: &Ctx,
    expr_in_parenthesis: expr_in_parenthesis,
) -> range_operand {
    range_operand::expr_in_parenthesis(expr_in_parenthesis)
}
pub fn range_operand_function_call(
    _ctx: &Ctx,
    function_call: function_call,
) -> range_operand {
    range_operand::function_call(function_call)
}
#[derive(Debug, Clone)]
pub struct range_without_step_expression {
    pub start: range_operand,
    pub stop: range_operand,
}
pub fn range_without_step_expression_c1(
    _ctx: &Ctx,
    start: range_operand,
    stop: range_operand,
) -> range_without_step_expression {
    range_without_step_expression {
        start,
        stop,
    }
}
#[derive(Debug, Clone)]
pub struct range_with_step_expression {
    pub start: range_operand,
    pub stop: range_operand,
    pub step: range_operand,
}
pub fn range_with_step_expression_c1(
    _ctx: &Ctx,
    start: range_operand,
    stop: range_operand,
    step: range_operand,
) -> range_with_step_expression {
    range_with_step_expression {
        start,
        stop,
        step,
    }
}
#[derive(Debug, Clone)]
pub struct expr_in_parenthesis {
    pub expr: Box<expression>,
}
pub fn expr_in_parenthesis_c1(_ctx: &Ctx, expr: expression) -> expr_in_parenthesis {
    expr_in_parenthesis {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct tensor_literal {
    pub expr: cube,
}
pub fn tensor_literal_c1(_ctx: &Ctx, expr: cube) -> tensor_literal {
    tensor_literal { expr }
}
pub type transpose_expression = Box<postfix_expression>;
pub fn transpose_expression_postfix_expression(
    _ctx: &Ctx,
    postfix_expression: postfix_expression,
) -> transpose_expression {
    Box::new(postfix_expression)
}
#[derive(Debug, Clone)]
pub struct function_call {
    pub id: id,
    pub arguments_list: argument0,
}
pub fn function_call_c1(_ctx: &Ctx, id: id, arguments_list: argument0) -> function_call {
    function_call {
        id,
        arguments_list,
    }
}
pub type argument1 = Vec<argument>;
pub fn argument1_c1(
    _ctx: &Ctx,
    mut argument1: argument1,
    argument: argument,
) -> argument1 {
    argument1.push(argument);
    argument1
}
pub fn argument1_argument(_ctx: &Ctx, argument: argument) -> argument1 {
    vec![argument]
}
pub type argument0 = Option<argument1>;
pub fn argument0_argument1(_ctx: &Ctx, argument1: argument1) -> argument0 {
    Some(argument1)
}
pub fn argument0_empty(_ctx: &Ctx) -> argument0 {
    None
}
#[derive(Debug, Clone)]
pub struct index_expression_list {
    pub index_expression_list: index_expression1,
}
pub fn index_expression_list_c1(
    _ctx: &Ctx,
    index_expression_list: index_expression1,
) -> index_expression_list {
    index_expression_list {
        index_expression_list,
    }
}
pub type index_expression1 = Vec<index_expression>;
pub fn index_expression1_c1(
    _ctx: &Ctx,
    mut index_expression1: index_expression1,
    index_expression: index_expression,
) -> index_expression1 {
    index_expression1.push(index_expression);
    index_expression1
}
pub fn index_expression1_index_expression(
    _ctx: &Ctx,
    index_expression: index_expression,
) -> index_expression1 {
    vec![index_expression]
}
#[derive(Debug, Clone)]
pub enum index_expression {
    index_copy_expr(index_copy_expr),
    index_from_position(index_from_position),
    index_bounds(index_bounds),
    index_bounds_step(index_bounds_step),
    index_upper_bound(index_upper_bound),
    index_lower_bound(index_lower_bound),
}
pub fn index_expression_index_copy_expr(
    _ctx: &Ctx,
    index_copy_expr: index_copy_expr,
) -> index_expression {
    index_expression::index_copy_expr(index_copy_expr)
}
pub fn index_expression_index_from_position(
    _ctx: &Ctx,
    index_from_position: index_from_position,
) -> index_expression {
    index_expression::index_from_position(index_from_position)
}
pub fn index_expression_index_bounds(
    _ctx: &Ctx,
    index_bounds: index_bounds,
) -> index_expression {
    index_expression::index_bounds(index_bounds)
}
pub fn index_expression_index_bounds_step(
    _ctx: &Ctx,
    index_bounds_step: index_bounds_step,
) -> index_expression {
    index_expression::index_bounds_step(index_bounds_step)
}
pub fn index_expression_index_upper_bound(
    _ctx: &Ctx,
    index_upper_bound: index_upper_bound,
) -> index_expression {
    index_expression::index_upper_bound(index_upper_bound)
}
pub fn index_expression_index_lower_bound(
    _ctx: &Ctx,
    index_lower_bound: index_lower_bound,
) -> index_expression {
    index_expression::index_lower_bound(index_lower_bound)
}
#[derive(Debug, Clone)]
pub enum index_copy_expr {
    colon,
}
pub fn index_copy_expr_colon(_ctx: &Ctx) -> index_copy_expr {
    index_copy_expr::colon
}
#[derive(Debug, Clone)]
pub struct index_from_position {
    pub expr: Box<expression>,
}
pub fn index_from_position_c1(_ctx: &Ctx, expr: expression) -> index_from_position {
    index_from_position {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct index_bounds {
    pub lower_bound: Box<expression>,
    pub upper_bound: Box<expression>,
}
pub fn index_bounds_c1(
    _ctx: &Ctx,
    lower_bound: expression,
    upper_bound: expression,
) -> index_bounds {
    index_bounds {
        lower_bound: Box::new(lower_bound),
        upper_bound: Box::new(upper_bound),
    }
}
#[derive(Debug, Clone)]
pub struct index_bounds_step {
    pub lower_bound: Box<expression>,
    pub upper_bound: Box<expression>,
    pub step: Box<expression>,
}
pub fn index_bounds_step_c1(
    _ctx: &Ctx,
    lower_bound: expression,
    upper_bound: expression,
    step: expression,
) -> index_bounds_step {
    index_bounds_step {
        lower_bound: Box::new(lower_bound),
        upper_bound: Box::new(upper_bound),
        step: Box::new(step),
    }
}
#[derive(Debug, Clone)]
pub struct index_upper_bound {
    pub upper_bound: Box<expression>,
}
pub fn index_upper_bound_c1(_ctx: &Ctx, upper_bound: expression) -> index_upper_bound {
    index_upper_bound {
        upper_bound: Box::new(upper_bound),
    }
}
#[derive(Debug, Clone)]
pub struct index_lower_bound {
    pub lower_bound: Box<expression>,
}
pub fn index_lower_bound_c1(_ctx: &Ctx, lower_bound: expression) -> index_lower_bound {
    index_lower_bound {
        lower_bound: Box::new(lower_bound),
    }
}
#[derive(Debug, Clone)]
pub struct argumentC1 {
    pub id: id,
    pub value: Box<expression>,
}
#[derive(Debug, Clone)]
pub struct argumentC2 {
    pub value: Box<expression>,
}
#[derive(Debug, Clone)]
pub enum argument {
    C1(argumentC1),
    C2(argumentC2),
}
pub fn argument_c1(_ctx: &Ctx, id: id, value: expression) -> argument {
    argument::C1(argumentC1 {
        id,
        value: Box::new(value),
    })
}
pub fn argument_c2(_ctx: &Ctx, value: expression) -> argument {
    argument::C2(argumentC2 {
        value: Box::new(value),
    })
}
#[derive(Debug, Clone)]
pub struct lvalue_tensor {
    pub expr: dot_access_expression,
    pub indices: index1,
}
pub fn lvalue_tensor_c1(
    _ctx: &Ctx,
    expr: dot_access_expression,
    indices: index1,
) -> lvalue_tensor {
    lvalue_tensor { expr, indices }
}
pub type index1 = Vec<index>;
pub fn index1_c1(_ctx: &Ctx, mut index1: index1, index: index) -> index1 {
    index1.push(index);
    index1
}
pub fn index1_index(_ctx: &Ctx, index: index) -> index1 {
    vec![index]
}
#[derive(Debug, Clone)]
pub struct index {
    pub index: index_expression_list,
}
pub fn index_c1(_ctx: &Ctx, index: index_expression_list) -> index {
    index { index }
}
#[derive(Debug, Clone)]
pub struct tensor_expression {
    pub expr: Box<expression>,
    pub index: index_expression_list,
}
pub fn tensor_expression_c1(
    _ctx: &Ctx,
    expr: expression,
    index: index_expression_list,
) -> tensor_expression {
    tensor_expression {
        expr: Box::new(expr),
        index,
    }
}
#[derive(Debug, Clone)]
pub struct dot_access_expression {
    pub names: id1,
    pub optional: questionOpt,
}
pub fn dot_access_expression_c1(
    _ctx: &Ctx,
    names: id1,
    optional: questionOpt,
) -> dot_access_expression {
    dot_access_expression {
        names,
        optional,
    }
}
pub type id1 = Vec<id>;
pub fn id1_c1(_ctx: &Ctx, mut id1: id1, id: id) -> id1 {
    id1.push(id);
    id1
}
pub fn id1_id(_ctx: &Ctx, id: id) -> id1 {
    vec![id]
}
pub type questionOpt = Option<QuestionOptNoO>;
#[derive(Debug, Clone)]
pub enum QuestionOptNoO {
    question,
}
pub fn question_opt_question(_ctx: &Ctx) -> questionOpt {
    Some(QuestionOptNoO::question)
}
pub fn question_opt_empty(_ctx: &Ctx) -> questionOpt {
    None
}
#[derive(Debug, Clone)]
pub struct array {
    pub elements: Box<expression1>,
}
pub fn array_c1(_ctx: &Ctx, elements: expression1) -> array {
    array {
        elements: Box::new(elements),
    }
}
#[derive(Debug, Clone)]
pub struct matrix {
    pub elements: array1,
}
pub fn matrix_c1(_ctx: &Ctx, elements: array1) -> matrix {
    matrix { elements }
}
pub type array1 = Vec<array>;
pub fn array1_c1(_ctx: &Ctx, mut array1: array1, array: array) -> array1 {
    array1.push(array);
    array1
}
pub fn array1_array(_ctx: &Ctx, array: array) -> array1 {
    vec![array]
}
#[derive(Debug, Clone)]
pub struct cube {
    pub elements: matrix1,
}
pub fn cube_c1(_ctx: &Ctx, elements: matrix1) -> cube {
    cube { elements }
}
pub type matrix1 = Vec<matrix>;
pub fn matrix1_c1(_ctx: &Ctx, mut matrix1: matrix1, matrix: matrix) -> matrix1 {
    matrix1.push(matrix);
    matrix1
}
pub fn matrix1_matrix(_ctx: &Ctx, matrix: matrix) -> matrix1 {
    vec![matrix]
}
#[derive(Debug, Clone)]
pub enum constant {
    integer(integer),
    unsigned_integer(unsigned_integer),
    float(float),
    string(string),
    boolean(boolean),
}
pub fn constant_integer(_ctx: &Ctx, integer: integer) -> constant {
    constant::integer(integer)
}
pub fn constant_unsigned_integer(
    _ctx: &Ctx,
    unsigned_integer: unsigned_integer,
) -> constant {
    constant::unsigned_integer(unsigned_integer)
}
pub fn constant_float(_ctx: &Ctx, float: float) -> constant {
    constant::float(float)
}
pub fn constant_string(_ctx: &Ctx, string: string) -> constant {
    constant::string(string)
}
pub fn constant_boolean(_ctx: &Ctx, boolean: boolean) -> constant {
    constant::boolean(boolean)
}
#[derive(Debug, Clone)]
pub struct boolean {
    pub value: boolean_const,
}
pub fn boolean_c1(_ctx: &Ctx, value: boolean_const) -> boolean {
    boolean { value }
}
#[derive(Debug, Clone)]
pub struct integerC1 {
    pub value: integer_const,
}
#[derive(Debug, Clone)]
pub struct integerC2 {
    pub value: hexadecimal_const,
}
#[derive(Debug, Clone)]
pub struct integerC3 {
    pub value: binary_const,
}
#[derive(Debug, Clone)]
pub enum integer {
    C1(integerC1),
    C2(integerC2),
    C3(integerC3),
}
pub fn integer_c1(_ctx: &Ctx, value: integer_const) -> integer {
    integer::C1(integerC1 { value })
}
pub fn integer_c2(_ctx: &Ctx, value: hexadecimal_const) -> integer {
    integer::C2(integerC2 { value })
}
pub fn integer_c3(_ctx: &Ctx, value: binary_const) -> integer {
    integer::C3(integerC3 { value })
}
#[derive(Debug, Clone)]
pub struct unsigned_integerC1 {
    pub value: unsigned_integer_const,
}
#[derive(Debug, Clone)]
pub struct unsigned_integerC2 {
    pub value: unsigned_hexadecimal_const,
}
#[derive(Debug, Clone)]
pub struct unsigned_integerC3 {
    pub value: unsigned_binary_const,
}
#[derive(Debug, Clone)]
pub enum unsigned_integer {
    C1(unsigned_integerC1),
    C2(unsigned_integerC2),
    C3(unsigned_integerC3),
}
pub fn unsigned_integer_c1(
    _ctx: &Ctx,
    value: unsigned_integer_const,
) -> unsigned_integer {
    unsigned_integer::C1(unsigned_integerC1 { value })
}
pub fn unsigned_integer_c2(
    _ctx: &Ctx,
    value: unsigned_hexadecimal_const,
) -> unsigned_integer {
    unsigned_integer::C2(unsigned_integerC2 { value })
}
pub fn unsigned_integer_c3(
    _ctx: &Ctx,
    value: unsigned_binary_const,
) -> unsigned_integer {
    unsigned_integer::C3(unsigned_integerC3 { value })
}
#[derive(Debug, Clone)]
pub struct float {
    pub value: float_const,
}
pub fn float_c1(_ctx: &Ctx, value: float_const) -> float {
    float { value }
}
#[derive(Debug, Clone)]
pub struct string {
    pub value: string_const,
}
pub fn string_c1(_ctx: &Ctx, value: string_const) -> string {
    string { value }
}
