/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::tml::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type IntegerConst = String;
pub fn integer_const(_ctx: &Ctx, token: Token) -> IntegerConst {
    token.value.into()
}
pub type UnsignedIntegerConst = String;
pub fn unsigned_integer_const(_ctx: &Ctx, token: Token) -> UnsignedIntegerConst {
    token.value.into()
}
pub type HexadecimalConst = String;
pub fn hexadecimal_const(_ctx: &Ctx, token: Token) -> HexadecimalConst {
    token.value.into()
}
pub type UnsignedHexadecimalConst = String;
pub fn unsigned_hexadecimal_const(_ctx: &Ctx, token: Token) -> UnsignedHexadecimalConst {
    token.value.into()
}
pub type BinaryConst = String;
pub fn binary_const(_ctx: &Ctx, token: Token) -> BinaryConst {
    token.value.into()
}
pub type UnsignedBinaryConst = String;
pub fn unsigned_binary_const(_ctx: &Ctx, token: Token) -> UnsignedBinaryConst {
    token.value.into()
}
pub type FloatConst = String;
pub fn float_const(_ctx: &Ctx, token: Token) -> FloatConst {
    token.value.into()
}
pub type BooleanConst = String;
pub fn boolean_const(_ctx: &Ctx, token: Token) -> BooleanConst {
    token.value.into()
}
pub type StringConst = String;
pub fn string_const(_ctx: &Ctx, token: Token) -> StringConst {
    token.value.into()
}
pub type Id = String;
pub fn id(_ctx: &Ctx, token: Token) -> Id {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct TranslationUnit {
    pub ext_decls: ExternalDeclaration1,
}
pub fn translation_unit_c1(
    _ctx: &Ctx,
    ext_decls: ExternalDeclaration1,
) -> TranslationUnit {
    TranslationUnit { ext_decls }
}
pub type ExternalDeclaration1 = Vec<ExternalDeclaration>;
pub fn external_declaration1_c1(
    _ctx: &Ctx,
    mut external_declaration1: ExternalDeclaration1,
    external_declaration: ExternalDeclaration,
) -> ExternalDeclaration1 {
    external_declaration1.push(external_declaration);
    external_declaration1
}
pub fn external_declaration1_external_declaration(
    _ctx: &Ctx,
    external_declaration: ExternalDeclaration,
) -> ExternalDeclaration1 {
    vec![external_declaration]
}
#[derive(Debug, Clone)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    DeclarationStatement(DeclarationStatement),
    AssignmentStatement(AssignmentStatement),
}
pub fn external_declaration_function_definition(
    _ctx: &Ctx,
    function_definition: FunctionDefinition,
) -> ExternalDeclaration {
    ExternalDeclaration::FunctionDefinition(function_definition)
}
pub fn external_declaration_declaration_statement(
    _ctx: &Ctx,
    declaration_statement: DeclarationStatement,
) -> ExternalDeclaration {
    ExternalDeclaration::DeclarationStatement(declaration_statement)
}
pub fn external_declaration_assignment_statement(
    _ctx: &Ctx,
    assignment_statement: AssignmentStatement,
) -> ExternalDeclaration {
    ExternalDeclaration::AssignmentStatement(assignment_statement)
}
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub id: Id,
    pub parameters_list: Parameter0,
    pub ret_type: TypeSpecOpt,
    pub statement_block: StatementBlock,
}
pub fn function_definition_c1(
    _ctx: &Ctx,
    id: Id,
    parameters_list: Parameter0,
    ret_type: TypeSpecOpt,
    statement_block: StatementBlock,
) -> FunctionDefinition {
    FunctionDefinition {
        id,
        parameters_list,
        ret_type,
        statement_block,
    }
}
pub type Parameter1 = Vec<Parameter>;
pub fn parameter1_c1(
    _ctx: &Ctx,
    mut parameter1: Parameter1,
    parameter: Parameter,
) -> Parameter1 {
    parameter1.push(parameter);
    parameter1
}
pub fn parameter1_parameter(_ctx: &Ctx, parameter: Parameter) -> Parameter1 {
    vec![parameter]
}
pub type Parameter0 = Option<Parameter1>;
pub fn parameter0_parameter1(_ctx: &Ctx, parameter1: Parameter1) -> Parameter0 {
    Some(parameter1)
}
pub fn parameter0_empty(_ctx: &Ctx) -> Parameter0 {
    None
}
pub type TypeSpecOpt = Option<TypeSpec>;
pub fn type_spec_opt_type_spec(_ctx: &Ctx, type_spec: TypeSpec) -> TypeSpecOpt {
    Some(type_spec)
}
pub fn type_spec_opt_empty(_ctx: &Ctx) -> TypeSpecOpt {
    None
}
#[derive(Debug, Clone)]
pub struct Parameter {
    pub _type: TypeSpec,
    pub id: Id,
    pub default: DefaultParamValueOpt,
}
pub fn parameter_c1(
    _ctx: &Ctx,
    _type: TypeSpec,
    id: Id,
    default: DefaultParamValueOpt,
) -> Parameter {
    Parameter { _type, id, default }
}
pub type DefaultParamValueOpt = Option<DefaultParamValue>;
pub fn default_param_value_opt_default_param_value(
    _ctx: &Ctx,
    default_param_value: DefaultParamValue,
) -> DefaultParamValueOpt {
    Some(default_param_value)
}
pub fn default_param_value_opt_empty(_ctx: &Ctx) -> DefaultParamValueOpt {
    None
}
#[derive(Debug, Clone)]
pub struct DefaultParamValue {
    pub value: Expression,
}
pub fn default_param_value_c1(_ctx: &Ctx, value: Expression) -> DefaultParamValue {
    DefaultParamValue { value }
}
#[derive(Debug, Clone)]
pub struct TypeCastExpression {
    pub _type: Box<TypeSpec>,
    pub expr: Box<Expression>,
}
pub fn type_cast_expression_c1(
    _ctx: &Ctx,
    _type: TypeSpec,
    expr: Expression,
) -> TypeCastExpression {
    TypeCastExpression {
        _type: Box::new(_type),
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub enum TypeSpec {
    SimpleType(SimpleType),
    DerivedType(DerivedType),
    TensorConstructor(TensorConstructor),
}
pub fn type_spec_simple_type(_ctx: &Ctx, simple_type: SimpleType) -> TypeSpec {
    TypeSpec::SimpleType(simple_type)
}
pub fn type_spec_derived_type(_ctx: &Ctx, derived_type: DerivedType) -> TypeSpec {
    TypeSpec::DerivedType(derived_type)
}
pub fn type_spec_tensor_constructor(
    _ctx: &Ctx,
    tensor_constructor: TensorConstructor,
) -> TypeSpec {
    TypeSpec::TensorConstructor(tensor_constructor)
}
#[derive(Debug, Clone)]
pub struct SimpleType {
    pub _type: SimpleTypeSpec,
}
pub fn simple_type_c1(_ctx: &Ctx, _type: SimpleTypeSpec) -> SimpleType {
    SimpleType { _type }
}
#[derive(Debug, Clone)]
pub struct DerivedType {
    pub name: DotAccessExpression,
    pub brackets: SquareBrackets0,
}
pub fn derived_type_c1(
    _ctx: &Ctx,
    name: DotAccessExpression,
    brackets: SquareBrackets0,
) -> DerivedType {
    DerivedType { name, brackets }
}
#[derive(Debug, Clone)]
pub enum SquareBrackets1 {
    SquareBrackets1(Box<SquareBrackets1>),
    SquareBrackets,
}
pub fn square_brackets1_square_brackets1(
    _ctx: &Ctx,
    square_brackets1: SquareBrackets1,
) -> SquareBrackets1 {
    SquareBrackets1::SquareBrackets1(Box::new(square_brackets1))
}
pub fn square_brackets1_square_brackets(_ctx: &Ctx) -> SquareBrackets1 {
    SquareBrackets1::SquareBrackets
}
pub type SquareBrackets0 = Option<SquareBrackets1>;
pub fn square_brackets0_square_brackets1(
    _ctx: &Ctx,
    square_brackets1: SquareBrackets1,
) -> SquareBrackets0 {
    Some(square_brackets1)
}
pub fn square_brackets0_empty(_ctx: &Ctx) -> SquareBrackets0 {
    None
}
#[derive(Debug, Clone)]
pub enum SimpleTypeSpec {
    IntT,
    UintT,
    RealT,
    BoolT,
    StrT,
}
pub fn simple_type_spec_int_t(_ctx: &Ctx) -> SimpleTypeSpec {
    SimpleTypeSpec::IntT
}
pub fn simple_type_spec_uint_t(_ctx: &Ctx) -> SimpleTypeSpec {
    SimpleTypeSpec::UintT
}
pub fn simple_type_spec_real_t(_ctx: &Ctx) -> SimpleTypeSpec {
    SimpleTypeSpec::RealT
}
pub fn simple_type_spec_bool_t(_ctx: &Ctx) -> SimpleTypeSpec {
    SimpleTypeSpec::BoolT
}
pub fn simple_type_spec_str_t(_ctx: &Ctx) -> SimpleTypeSpec {
    SimpleTypeSpec::StrT
}
#[derive(Debug, Clone)]
pub struct TensorConstructor {
    pub _type: Box<TypeSpec>,
    pub dimensions: Expression1,
}
pub fn tensor_constructor_c1(
    _ctx: &Ctx,
    _type: TypeSpec,
    dimensions: Expression1,
) -> TensorConstructor {
    TensorConstructor {
        _type: Box::new(_type),
        dimensions,
    }
}
pub type Expression1 = Vec<Expression>;
pub fn expression1_c1(
    _ctx: &Ctx,
    mut expression1: Expression1,
    expression: Expression,
) -> Expression1 {
    expression1.push(expression);
    expression1
}
pub fn expression1_expression(_ctx: &Ctx, expression: Expression) -> Expression1 {
    vec![expression]
}
#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub statements: Statement1,
}
pub fn statement_block_c1(_ctx: &Ctx, statements: Statement1) -> StatementBlock {
    StatementBlock { statements }
}
pub type Statement1 = Vec<Statement>;
pub fn statement1_c1(
    _ctx: &Ctx,
    mut statement1: Statement1,
    statement: Statement,
) -> Statement1 {
    statement1.push(statement);
    statement1
}
pub fn statement1_statement(_ctx: &Ctx, statement: Statement) -> Statement1 {
    vec![statement]
}
#[derive(Debug, Clone)]
pub enum Statement {
    FunctionCall(FunctionCall),
    SelectionStatement(SelectionStatement),
    IterationStatement(IterationStatement),
    JumpStatement(JumpStatement),
    ExistsStatement(ExistsStatement),
    NotExistsStatement(NotExistsStatement),
    AssignmentStatement(AssignmentStatement),
    DeclarationStatement(DeclarationStatement),
}
pub fn statement_function_call(_ctx: &Ctx, function_call: FunctionCall) -> Statement {
    Statement::FunctionCall(function_call)
}
pub fn statement_selection_statement(
    _ctx: &Ctx,
    selection_statement: SelectionStatement,
) -> Statement {
    Statement::SelectionStatement(selection_statement)
}
pub fn statement_iteration_statement(
    _ctx: &Ctx,
    iteration_statement: IterationStatement,
) -> Statement {
    Statement::IterationStatement(iteration_statement)
}
pub fn statement_jump_statement(_ctx: &Ctx, jump_statement: JumpStatement) -> Statement {
    Statement::JumpStatement(jump_statement)
}
pub fn statement_exists_statement(
    _ctx: &Ctx,
    exists_statement: ExistsStatement,
) -> Statement {
    Statement::ExistsStatement(exists_statement)
}
pub fn statement_not_exists_statement(
    _ctx: &Ctx,
    not_exists_statement: NotExistsStatement,
) -> Statement {
    Statement::NotExistsStatement(not_exists_statement)
}
pub fn statement_assignment_statement(
    _ctx: &Ctx,
    assignment_statement: AssignmentStatement,
) -> Statement {
    Statement::AssignmentStatement(assignment_statement)
}
pub fn statement_declaration_statement(
    _ctx: &Ctx,
    declaration_statement: DeclarationStatement,
) -> Statement {
    Statement::DeclarationStatement(declaration_statement)
}
#[derive(Debug, Clone)]
pub struct SelectionStatement {
    pub condition: Expression,
    pub if_statement_block: Box<StatementBlock>,
    pub elseif_clause: ElseIfClause0,
    pub else_clause: ElseClauseOpt,
}
pub fn selection_statement_c1(
    _ctx: &Ctx,
    condition: Expression,
    if_statement_block: StatementBlock,
    elseif_clause: ElseIfClause0,
    else_clause: ElseClauseOpt,
) -> SelectionStatement {
    SelectionStatement {
        condition,
        if_statement_block: Box::new(if_statement_block),
        elseif_clause,
        else_clause,
    }
}
pub type ElseIfClause1 = Vec<ElseIfClause>;
pub fn else_if_clause1_c1(
    _ctx: &Ctx,
    mut else_if_clause1: ElseIfClause1,
    else_if_clause: ElseIfClause,
) -> ElseIfClause1 {
    else_if_clause1.push(else_if_clause);
    else_if_clause1
}
pub fn else_if_clause1_else_if_clause(
    _ctx: &Ctx,
    else_if_clause: ElseIfClause,
) -> ElseIfClause1 {
    vec![else_if_clause]
}
pub type ElseIfClause0 = Option<ElseIfClause1>;
pub fn else_if_clause0_else_if_clause1(
    _ctx: &Ctx,
    else_if_clause1: ElseIfClause1,
) -> ElseIfClause0 {
    Some(else_if_clause1)
}
pub fn else_if_clause0_empty(_ctx: &Ctx) -> ElseIfClause0 {
    None
}
pub type ElseClauseOpt = Option<ElseClause>;
pub fn else_clause_opt_else_clause(
    _ctx: &Ctx,
    else_clause: ElseClause,
) -> ElseClauseOpt {
    Some(else_clause)
}
pub fn else_clause_opt_empty(_ctx: &Ctx) -> ElseClauseOpt {
    None
}
#[derive(Debug, Clone)]
pub struct ElseClause {
    pub else_statement_block: Box<StatementBlock>,
}
pub fn else_clause_c1(_ctx: &Ctx, else_statement_block: StatementBlock) -> ElseClause {
    ElseClause {
        else_statement_block: Box::new(else_statement_block),
    }
}
#[derive(Debug, Clone)]
pub struct ElseIfClause {
    pub condition: Expression,
    pub elseif_statement_block: Box<StatementBlock>,
}
pub fn else_if_clause_c1(
    _ctx: &Ctx,
    condition: Expression,
    elseif_statement_block: StatementBlock,
) -> ElseIfClause {
    ElseIfClause {
        condition,
        elseif_statement_block: Box::new(elseif_statement_block),
    }
}
#[derive(Debug, Clone)]
pub enum IterationStatement {
    ForIterationStatement(ForIterationStatement),
    WhileIterationStatement(WhileIterationStatement),
}
pub fn iteration_statement_for_iteration_statement(
    _ctx: &Ctx,
    for_iteration_statement: ForIterationStatement,
) -> IterationStatement {
    IterationStatement::ForIterationStatement(for_iteration_statement)
}
pub fn iteration_statement_while_iteration_statement(
    _ctx: &Ctx,
    while_iteration_statement: WhileIterationStatement,
) -> IterationStatement {
    IterationStatement::WhileIterationStatement(while_iteration_statement)
}
#[derive(Debug, Clone)]
pub struct ForIterationStatement {
    pub idx: Id,
    pub iterator_expression: Expression,
    pub statement_block: Box<StatementBlock>,
}
pub fn for_iteration_statement_c1(
    _ctx: &Ctx,
    idx: Id,
    iterator_expression: Expression,
    statement_block: StatementBlock,
) -> ForIterationStatement {
    ForIterationStatement {
        idx,
        iterator_expression,
        statement_block: Box::new(statement_block),
    }
}
#[derive(Debug, Clone)]
pub struct WhileIterationStatement {
    pub condition: Expression,
    pub statement_block: Box<StatementBlock>,
}
pub fn while_iteration_statement_c1(
    _ctx: &Ctx,
    condition: Expression,
    statement_block: StatementBlock,
) -> WhileIterationStatement {
    WhileIterationStatement {
        condition,
        statement_block: Box::new(statement_block),
    }
}
#[derive(Debug, Clone)]
pub enum JumpStatement {
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ContinueStatement(ContinueStatement),
}
pub fn jump_statement_break_statement(
    _ctx: &Ctx,
    break_statement: BreakStatement,
) -> JumpStatement {
    JumpStatement::BreakStatement(break_statement)
}
pub fn jump_statement_return_statement(
    _ctx: &Ctx,
    return_statement: ReturnStatement,
) -> JumpStatement {
    JumpStatement::ReturnStatement(return_statement)
}
pub fn jump_statement_continue_statement(
    _ctx: &Ctx,
    continue_statement: ContinueStatement,
) -> JumpStatement {
    JumpStatement::ContinueStatement(continue_statement)
}
#[derive(Debug, Clone)]
pub enum BreakStatement {
    BreakT,
}
pub fn break_statement_break_t(_ctx: &Ctx) -> BreakStatement {
    BreakStatement::BreakT
}
#[derive(Debug, Clone)]
pub enum ContinueStatement {
    ContinueT,
}
pub fn continue_statement_continue_t(_ctx: &Ctx) -> ContinueStatement {
    ContinueStatement::ContinueT
}
#[derive(Debug, Clone)]
pub enum ReturnStatement {
    EmptyReturn(EmptyReturn),
    ReturnValue(ReturnValue),
}
pub fn return_statement_empty_return(
    _ctx: &Ctx,
    empty_return: EmptyReturn,
) -> ReturnStatement {
    ReturnStatement::EmptyReturn(empty_return)
}
pub fn return_statement_return_value(
    _ctx: &Ctx,
    return_value: ReturnValue,
) -> ReturnStatement {
    ReturnStatement::ReturnValue(return_value)
}
#[derive(Debug, Clone)]
pub enum EmptyReturn {
    ReturnT,
}
pub fn empty_return_return_t(_ctx: &Ctx) -> EmptyReturn {
    EmptyReturn::ReturnT
}
#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub ret_val: Expression,
}
pub fn return_value_c1(_ctx: &Ctx, ret_val: Expression) -> ReturnValue {
    ReturnValue { ret_val }
}
#[derive(Debug, Clone)]
pub struct ExistsStatement {
    pub guarded: DotAccessExpression1,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
}
pub fn exists_statement_c1(
    _ctx: &Ctx,
    guarded: DotAccessExpression1,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
) -> ExistsStatement {
    ExistsStatement {
        guarded,
        statement_block: Box::new(statement_block),
        else_clause,
    }
}
pub type DotAccessExpression1 = Vec<DotAccessExpression>;
pub fn dot_access_expression1_c1(
    _ctx: &Ctx,
    mut dot_access_expression1: DotAccessExpression1,
    dot_access_expression: DotAccessExpression,
) -> DotAccessExpression1 {
    dot_access_expression1.push(dot_access_expression);
    dot_access_expression1
}
pub fn dot_access_expression1_dot_access_expression(
    _ctx: &Ctx,
    dot_access_expression: DotAccessExpression,
) -> DotAccessExpression1 {
    vec![dot_access_expression]
}
#[derive(Debug, Clone)]
pub struct NotExistsStatement {
    pub guarded: DotAccessExpression1,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
}
pub fn not_exists_statement_c1(
    _ctx: &Ctx,
    guarded: DotAccessExpression1,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
) -> NotExistsStatement {
    NotExistsStatement {
        guarded,
        statement_block: Box::new(statement_block),
        else_clause,
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentStatement {
    VarAssignmentStatement(VarAssignmentStatement),
    TensorAssignmentStatement(TensorAssignmentStatement),
}
pub fn assignment_statement_var_assignment_statement(
    _ctx: &Ctx,
    var_assignment_statement: VarAssignmentStatement,
) -> AssignmentStatement {
    AssignmentStatement::VarAssignmentStatement(var_assignment_statement)
}
pub fn assignment_statement_tensor_assignment_statement(
    _ctx: &Ctx,
    tensor_assignment_statement: TensorAssignmentStatement,
) -> AssignmentStatement {
    AssignmentStatement::TensorAssignmentStatement(tensor_assignment_statement)
}
#[derive(Debug, Clone)]
pub struct VarAssignmentStatement {
    pub var: DotAccessExpression,
    pub op: AssignmentOperator,
    pub rvalue: Expression,
}
pub fn var_assignment_statement_c1(
    _ctx: &Ctx,
    var: DotAccessExpression,
    op: AssignmentOperator,
    rvalue: Expression,
) -> VarAssignmentStatement {
    VarAssignmentStatement {
        var,
        op,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct TensorAssignmentStatement {
    pub tensor: LValueTensor,
    pub op: AssignmentOperator,
    pub rvalue: Expression,
}
pub fn tensor_assignment_statement_c1(
    _ctx: &Ctx,
    tensor: LValueTensor,
    op: AssignmentOperator,
    rvalue: Expression,
) -> TensorAssignmentStatement {
    TensorAssignmentStatement {
        tensor,
        op,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationStatement {
    pub _type: TypeSpec,
    pub id: Id,
    pub rvalue: Expression,
}
pub fn declaration_statement_c1(
    _ctx: &Ctx,
    _type: TypeSpec,
    id: Id,
    rvalue: Expression,
) -> DeclarationStatement {
    DeclarationStatement {
        _type,
        id,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub enum Expression {
    LogicalExpression(LogicalExpression),
    TypeCastExpression(TypeCastExpression),
    MathExpression(MathExpression),
}
pub fn expression_logical_expression(
    _ctx: &Ctx,
    logical_expression: LogicalExpression,
) -> Expression {
    Expression::LogicalExpression(logical_expression)
}
pub fn expression_type_cast_expression(
    _ctx: &Ctx,
    type_cast_expression: TypeCastExpression,
) -> Expression {
    Expression::TypeCastExpression(type_cast_expression)
}
pub fn expression_math_expression(
    _ctx: &Ctx,
    math_expression: MathExpression,
) -> Expression {
    Expression::MathExpression(math_expression)
}
#[derive(Debug, Clone)]
pub struct AttributeAccess {
    pub expr: Box<Expression>,
    pub attr: Id,
}
pub fn attribute_access_c1(_ctx: &Ctx, expr: Expression, attr: Id) -> AttributeAccess {
    AttributeAccess {
        expr: Box::new(expr),
        attr,
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,
    MulAssignT,
    DivAssignT,
    ModAssignT,
    AddAssignT,
    SubAssignT,
}
pub fn assignment_operator_assign(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::Assign
}
pub fn assignment_operator_mul_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::MulAssignT
}
pub fn assignment_operator_div_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::DivAssignT
}
pub fn assignment_operator_mod_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::ModAssignT
}
pub fn assignment_operator_add_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::AddAssignT
}
pub fn assignment_operator_sub_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::SubAssignT
}
#[derive(Debug, Clone)]
pub enum LogicalExpression {
    BinaryRelationalExpression(BinaryRelationalExpression),
    UnaryLogicalExpression(UnaryLogicalExpression),
    UnaryRelationalExpression(UnaryRelationalExpression),
}
pub fn logical_expression_binary_relational_expression(
    _ctx: &Ctx,
    binary_relational_expression: BinaryRelationalExpression,
) -> LogicalExpression {
    LogicalExpression::BinaryRelationalExpression(binary_relational_expression)
}
pub fn logical_expression_unary_logical_expression(
    _ctx: &Ctx,
    unary_logical_expression: UnaryLogicalExpression,
) -> LogicalExpression {
    LogicalExpression::UnaryLogicalExpression(unary_logical_expression)
}
pub fn logical_expression_unary_relational_expression(
    _ctx: &Ctx,
    unary_relational_expression: UnaryRelationalExpression,
) -> LogicalExpression {
    LogicalExpression::UnaryRelationalExpression(unary_relational_expression)
}
#[derive(Debug, Clone)]
pub struct UnaryLogicalExpressionC1 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct UnaryLogicalExpressionC2 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum UnaryLogicalExpression {
    C1(UnaryLogicalExpressionC1),
    C2(UnaryLogicalExpressionC2),
}
pub fn unary_logical_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> UnaryLogicalExpression {
    UnaryLogicalExpression::C1(UnaryLogicalExpressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn unary_logical_expression_c2(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> UnaryLogicalExpression {
    UnaryLogicalExpression::C2(UnaryLogicalExpressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct UnaryRelationalExpression {
    pub expr: Box<Expression>,
}
pub fn unary_relational_expression_c1(
    _ctx: &Ctx,
    expr: Expression,
) -> UnaryRelationalExpression {
    UnaryRelationalExpression {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC1 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC2 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC3 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC4 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC5 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryRelationalExpressionC6 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum BinaryRelationalExpression {
    C1(BinaryRelationalExpressionC1),
    C2(BinaryRelationalExpressionC2),
    C3(BinaryRelationalExpressionC3),
    C4(BinaryRelationalExpressionC4),
    C5(BinaryRelationalExpressionC5),
    C6(BinaryRelationalExpressionC6),
}
pub fn binary_relational_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C1(BinaryRelationalExpressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c2(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C2(BinaryRelationalExpressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c3(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C3(BinaryRelationalExpressionC3 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c4(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C4(BinaryRelationalExpressionC4 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c5(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C5(BinaryRelationalExpressionC5 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_relational_expression_c6(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryRelationalExpression {
    BinaryRelationalExpression::C6(BinaryRelationalExpressionC6 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub enum MathExpression {
    PostfixExpression(PostfixExpression),
    BinaryMathExpression(BinaryMathExpression),
    UnaryMathExpression(UnaryMathExpression),
    ElvisExpression(ElvisExpression),
}
pub fn math_expression_postfix_expression(
    _ctx: &Ctx,
    postfix_expression: PostfixExpression,
) -> MathExpression {
    MathExpression::PostfixExpression(postfix_expression)
}
pub fn math_expression_binary_math_expression(
    _ctx: &Ctx,
    binary_math_expression: BinaryMathExpression,
) -> MathExpression {
    MathExpression::BinaryMathExpression(binary_math_expression)
}
pub fn math_expression_unary_math_expression(
    _ctx: &Ctx,
    unary_math_expression: UnaryMathExpression,
) -> MathExpression {
    MathExpression::UnaryMathExpression(unary_math_expression)
}
pub fn math_expression_elvis_expression(
    _ctx: &Ctx,
    elvis_expression: ElvisExpression,
) -> MathExpression {
    MathExpression::ElvisExpression(elvis_expression)
}
#[derive(Debug, Clone)]
pub struct ElvisExpression {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
pub fn elvis_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> ElvisExpression {
    ElvisExpression {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    }
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC1 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC2 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC3 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC4 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC5 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC6 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryMathExpressionC7 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum BinaryMathExpression {
    C1(BinaryMathExpressionC1),
    C2(BinaryMathExpressionC2),
    C3(BinaryMathExpressionC3),
    C4(BinaryMathExpressionC4),
    C5(BinaryMathExpressionC5),
    C6(BinaryMathExpressionC6),
    C7(BinaryMathExpressionC7),
}
pub fn binary_math_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C1(BinaryMathExpressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c2(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C2(BinaryMathExpressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c3(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C3(BinaryMathExpressionC3 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c4(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C4(BinaryMathExpressionC4 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c5(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C5(BinaryMathExpressionC5 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c6(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C6(BinaryMathExpressionC6 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_math_expression_c7(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C7(BinaryMathExpressionC7 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct UnaryMathExpressionC1 {
    pub expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct UnaryMathExpressionC2 {
    pub expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum UnaryMathExpression {
    C1(UnaryMathExpressionC1),
    C2(UnaryMathExpressionC2),
}
pub fn unary_math_expression_c1(_ctx: &Ctx, expr: Expression) -> UnaryMathExpression {
    UnaryMathExpression::C1(UnaryMathExpressionC1 {
        expr: Box::new(expr),
    })
}
pub fn unary_math_expression_c2(_ctx: &Ctx, expr: Expression) -> UnaryMathExpression {
    UnaryMathExpression::C2(UnaryMathExpressionC2 {
        expr: Box::new(expr),
    })
}
#[derive(Debug, Clone)]
pub enum PostfixExpression {
    RValue(RValue),
    Constant(Constant),
    ExprInParenthesis(ExprInParenthesis),
    TransposeExpression(TransposeExpression),
    TensorLiteral(TensorLiteral),
    TensorExpression(TensorExpression),
    FunctionCall(FunctionCall),
    AttributeAccess(AttributeAccess),
    RangeExpression(RangeExpression),
}
pub fn postfix_expression_rvalue(_ctx: &Ctx, rvalue: RValue) -> PostfixExpression {
    PostfixExpression::RValue(rvalue)
}
pub fn postfix_expression_constant(_ctx: &Ctx, constant: Constant) -> PostfixExpression {
    PostfixExpression::Constant(constant)
}
pub fn postfix_expression_expr_in_parenthesis(
    _ctx: &Ctx,
    expr_in_parenthesis: ExprInParenthesis,
) -> PostfixExpression {
    PostfixExpression::ExprInParenthesis(expr_in_parenthesis)
}
pub fn postfix_expression_transpose_expression(
    _ctx: &Ctx,
    transpose_expression: TransposeExpression,
) -> PostfixExpression {
    PostfixExpression::TransposeExpression(transpose_expression)
}
pub fn postfix_expression_tensor_literal(
    _ctx: &Ctx,
    tensor_literal: TensorLiteral,
) -> PostfixExpression {
    PostfixExpression::TensorLiteral(tensor_literal)
}
pub fn postfix_expression_tensor_expression(
    _ctx: &Ctx,
    tensor_expression: TensorExpression,
) -> PostfixExpression {
    PostfixExpression::TensorExpression(tensor_expression)
}
pub fn postfix_expression_function_call(
    _ctx: &Ctx,
    function_call: FunctionCall,
) -> PostfixExpression {
    PostfixExpression::FunctionCall(function_call)
}
pub fn postfix_expression_attribute_access(
    _ctx: &Ctx,
    attribute_access: AttributeAccess,
) -> PostfixExpression {
    PostfixExpression::AttributeAccess(attribute_access)
}
pub fn postfix_expression_range_expression(
    _ctx: &Ctx,
    range_expression: RangeExpression,
) -> PostfixExpression {
    PostfixExpression::RangeExpression(range_expression)
}
#[derive(Debug, Clone)]
pub struct RValue {
    pub _ref: DotAccessExpression,
}
pub fn rvalue_c1(_ctx: &Ctx, _ref: DotAccessExpression) -> RValue {
    RValue { _ref }
}
#[derive(Debug, Clone)]
pub enum RangeExpression {
    RangeWithoutStepExpression(RangeWithoutStepExpression),
    RangeWithStepExpression(RangeWithStepExpression),
}
pub fn range_expression_range_without_step_expression(
    _ctx: &Ctx,
    range_without_step_expression: RangeWithoutStepExpression,
) -> RangeExpression {
    RangeExpression::RangeWithoutStepExpression(range_without_step_expression)
}
pub fn range_expression_range_with_step_expression(
    _ctx: &Ctx,
    range_with_step_expression: RangeWithStepExpression,
) -> RangeExpression {
    RangeExpression::RangeWithStepExpression(range_with_step_expression)
}
#[derive(Debug, Clone)]
pub enum RangeOperand {
    RValue(RValue),
    Constant(Constant),
    ExprInParenthesis(ExprInParenthesis),
    FunctionCall(FunctionCall),
}
pub fn range_operand_rvalue(_ctx: &Ctx, rvalue: RValue) -> RangeOperand {
    RangeOperand::RValue(rvalue)
}
pub fn range_operand_constant(_ctx: &Ctx, constant: Constant) -> RangeOperand {
    RangeOperand::Constant(constant)
}
pub fn range_operand_expr_in_parenthesis(
    _ctx: &Ctx,
    expr_in_parenthesis: ExprInParenthesis,
) -> RangeOperand {
    RangeOperand::ExprInParenthesis(expr_in_parenthesis)
}
pub fn range_operand_function_call(
    _ctx: &Ctx,
    function_call: FunctionCall,
) -> RangeOperand {
    RangeOperand::FunctionCall(function_call)
}
#[derive(Debug, Clone)]
pub struct RangeWithoutStepExpression {
    pub start: RangeOperand,
    pub stop: RangeOperand,
}
pub fn range_without_step_expression_c1(
    _ctx: &Ctx,
    start: RangeOperand,
    stop: RangeOperand,
) -> RangeWithoutStepExpression {
    RangeWithoutStepExpression {
        start,
        stop,
    }
}
#[derive(Debug, Clone)]
pub struct RangeWithStepExpression {
    pub start: RangeOperand,
    pub stop: RangeOperand,
    pub step: RangeOperand,
}
pub fn range_with_step_expression_c1(
    _ctx: &Ctx,
    start: RangeOperand,
    stop: RangeOperand,
    step: RangeOperand,
) -> RangeWithStepExpression {
    RangeWithStepExpression {
        start,
        stop,
        step,
    }
}
#[derive(Debug, Clone)]
pub struct ExprInParenthesis {
    pub expr: Box<Expression>,
}
pub fn expr_in_parenthesis_c1(_ctx: &Ctx, expr: Expression) -> ExprInParenthesis {
    ExprInParenthesis {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct TensorLiteral {
    pub expr: Cube,
}
pub fn tensor_literal_c1(_ctx: &Ctx, expr: Cube) -> TensorLiteral {
    TensorLiteral { expr }
}
pub type TransposeExpression = Box<PostfixExpression>;
pub fn transpose_expression_postfix_expression(
    _ctx: &Ctx,
    postfix_expression: PostfixExpression,
) -> TransposeExpression {
    Box::new(postfix_expression)
}
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub id: Id,
    pub arguments_list: Argument0,
}
pub fn function_call_c1(_ctx: &Ctx, id: Id, arguments_list: Argument0) -> FunctionCall {
    FunctionCall { id, arguments_list }
}
pub type Argument1 = Vec<Argument>;
pub fn argument1_c1(
    _ctx: &Ctx,
    mut argument1: Argument1,
    argument: Argument,
) -> Argument1 {
    argument1.push(argument);
    argument1
}
pub fn argument1_argument(_ctx: &Ctx, argument: Argument) -> Argument1 {
    vec![argument]
}
pub type Argument0 = Option<Argument1>;
pub fn argument0_argument1(_ctx: &Ctx, argument1: Argument1) -> Argument0 {
    Some(argument1)
}
pub fn argument0_empty(_ctx: &Ctx) -> Argument0 {
    None
}
#[derive(Debug, Clone)]
pub struct IndexExpressionList {
    pub index_expression_list: IndexExpression1,
}
pub fn index_expression_list_c1(
    _ctx: &Ctx,
    index_expression_list: IndexExpression1,
) -> IndexExpressionList {
    IndexExpressionList {
        index_expression_list,
    }
}
pub type IndexExpression1 = Vec<IndexExpression>;
pub fn index_expression1_c1(
    _ctx: &Ctx,
    mut index_expression1: IndexExpression1,
    index_expression: IndexExpression,
) -> IndexExpression1 {
    index_expression1.push(index_expression);
    index_expression1
}
pub fn index_expression1_index_expression(
    _ctx: &Ctx,
    index_expression: IndexExpression,
) -> IndexExpression1 {
    vec![index_expression]
}
#[derive(Debug, Clone)]
pub enum IndexExpression {
    IndexCopyExpr(IndexCopyExpr),
    IndexFromPosition(IndexFromPosition),
    IndexBounds(IndexBounds),
    IndexBoundsStep(IndexBoundsStep),
    IndexUpperBound(IndexUpperBound),
    IndexLowerBound(IndexLowerBound),
}
pub fn index_expression_index_copy_expr(
    _ctx: &Ctx,
    index_copy_expr: IndexCopyExpr,
) -> IndexExpression {
    IndexExpression::IndexCopyExpr(index_copy_expr)
}
pub fn index_expression_index_from_position(
    _ctx: &Ctx,
    index_from_position: IndexFromPosition,
) -> IndexExpression {
    IndexExpression::IndexFromPosition(index_from_position)
}
pub fn index_expression_index_bounds(
    _ctx: &Ctx,
    index_bounds: IndexBounds,
) -> IndexExpression {
    IndexExpression::IndexBounds(index_bounds)
}
pub fn index_expression_index_bounds_step(
    _ctx: &Ctx,
    index_bounds_step: IndexBoundsStep,
) -> IndexExpression {
    IndexExpression::IndexBoundsStep(index_bounds_step)
}
pub fn index_expression_index_upper_bound(
    _ctx: &Ctx,
    index_upper_bound: IndexUpperBound,
) -> IndexExpression {
    IndexExpression::IndexUpperBound(index_upper_bound)
}
pub fn index_expression_index_lower_bound(
    _ctx: &Ctx,
    index_lower_bound: IndexLowerBound,
) -> IndexExpression {
    IndexExpression::IndexLowerBound(index_lower_bound)
}
#[derive(Debug, Clone)]
pub enum IndexCopyExpr {
    Colon,
}
pub fn index_copy_expr_colon(_ctx: &Ctx) -> IndexCopyExpr {
    IndexCopyExpr::Colon
}
#[derive(Debug, Clone)]
pub struct IndexFromPosition {
    pub expr: Box<Expression>,
}
pub fn index_from_position_c1(_ctx: &Ctx, expr: Expression) -> IndexFromPosition {
    IndexFromPosition {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct IndexBounds {
    pub lower_bound: Box<Expression>,
    pub upper_bound: Box<Expression>,
}
pub fn index_bounds_c1(
    _ctx: &Ctx,
    lower_bound: Expression,
    upper_bound: Expression,
) -> IndexBounds {
    IndexBounds {
        lower_bound: Box::new(lower_bound),
        upper_bound: Box::new(upper_bound),
    }
}
#[derive(Debug, Clone)]
pub struct IndexBoundsStep {
    pub lower_bound: Box<Expression>,
    pub upper_bound: Box<Expression>,
    pub step: Box<Expression>,
}
pub fn index_bounds_step_c1(
    _ctx: &Ctx,
    lower_bound: Expression,
    upper_bound: Expression,
    step: Expression,
) -> IndexBoundsStep {
    IndexBoundsStep {
        lower_bound: Box::new(lower_bound),
        upper_bound: Box::new(upper_bound),
        step: Box::new(step),
    }
}
#[derive(Debug, Clone)]
pub struct IndexUpperBound {
    pub upper_bound: Box<Expression>,
}
pub fn index_upper_bound_c1(_ctx: &Ctx, upper_bound: Expression) -> IndexUpperBound {
    IndexUpperBound {
        upper_bound: Box::new(upper_bound),
    }
}
#[derive(Debug, Clone)]
pub struct IndexLowerBound {
    pub lower_bound: Box<Expression>,
}
pub fn index_lower_bound_c1(_ctx: &Ctx, lower_bound: Expression) -> IndexLowerBound {
    IndexLowerBound {
        lower_bound: Box::new(lower_bound),
    }
}
#[derive(Debug, Clone)]
pub struct ArgumentC1 {
    pub id: Id,
    pub value: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct ArgumentC2 {
    pub value: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum Argument {
    C1(ArgumentC1),
    C2(ArgumentC2),
}
pub fn argument_c1(_ctx: &Ctx, id: Id, value: Expression) -> Argument {
    Argument::C1(ArgumentC1 {
        id,
        value: Box::new(value),
    })
}
pub fn argument_c2(_ctx: &Ctx, value: Expression) -> Argument {
    Argument::C2(ArgumentC2 {
        value: Box::new(value),
    })
}
#[derive(Debug, Clone)]
pub struct LValueTensor {
    pub expr: DotAccessExpression,
    pub indices: Index1,
}
pub fn lvalue_tensor_c1(
    _ctx: &Ctx,
    expr: DotAccessExpression,
    indices: Index1,
) -> LValueTensor {
    LValueTensor { expr, indices }
}
pub type Index1 = Vec<Index>;
pub fn index1_c1(_ctx: &Ctx, mut index1: Index1, index: Index) -> Index1 {
    index1.push(index);
    index1
}
pub fn index1_index(_ctx: &Ctx, index: Index) -> Index1 {
    vec![index]
}
#[derive(Debug, Clone)]
pub struct Index {
    pub index: IndexExpressionList,
}
pub fn index_c1(_ctx: &Ctx, index: IndexExpressionList) -> Index {
    Index { index }
}
#[derive(Debug, Clone)]
pub struct TensorExpression {
    pub expr: Box<Expression>,
    pub index: IndexExpressionList,
}
pub fn tensor_expression_c1(
    _ctx: &Ctx,
    expr: Expression,
    index: IndexExpressionList,
) -> TensorExpression {
    TensorExpression {
        expr: Box::new(expr),
        index,
    }
}
#[derive(Debug, Clone)]
pub struct DotAccessExpression {
    pub names: Id1,
    pub optional: QuestionOpt,
}
pub fn dot_access_expression_c1(
    _ctx: &Ctx,
    names: Id1,
    optional: QuestionOpt,
) -> DotAccessExpression {
    DotAccessExpression {
        names,
        optional,
    }
}
pub type Id1 = Vec<Id>;
pub fn id1_c1(_ctx: &Ctx, mut id1: Id1, id: Id) -> Id1 {
    id1.push(id);
    id1
}
pub fn id1_id(_ctx: &Ctx, id: Id) -> Id1 {
    vec![id]
}
pub type QuestionOpt = Option<QuestionOptNoO>;
#[derive(Debug, Clone)]
pub enum QuestionOptNoO {
    Question,
}
pub fn question_opt_question(_ctx: &Ctx) -> QuestionOpt {
    Some(QuestionOptNoO::Question)
}
pub fn question_opt_empty(_ctx: &Ctx) -> QuestionOpt {
    None
}
#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Box<Expression1>,
}
pub fn array_c1(_ctx: &Ctx, elements: Expression1) -> Array {
    Array {
        elements: Box::new(elements),
    }
}
#[derive(Debug, Clone)]
pub struct Matrix {
    pub elements: Array1,
}
pub fn matrix_c1(_ctx: &Ctx, elements: Array1) -> Matrix {
    Matrix { elements }
}
pub type Array1 = Vec<Array>;
pub fn array1_c1(_ctx: &Ctx, mut array1: Array1, array: Array) -> Array1 {
    array1.push(array);
    array1
}
pub fn array1_array(_ctx: &Ctx, array: Array) -> Array1 {
    vec![array]
}
#[derive(Debug, Clone)]
pub struct Cube {
    pub elements: Matrix1,
}
pub fn cube_c1(_ctx: &Ctx, elements: Matrix1) -> Cube {
    Cube { elements }
}
pub type Matrix1 = Vec<Matrix>;
pub fn matrix1_c1(_ctx: &Ctx, mut matrix1: Matrix1, matrix: Matrix) -> Matrix1 {
    matrix1.push(matrix);
    matrix1
}
pub fn matrix1_matrix(_ctx: &Ctx, matrix: Matrix) -> Matrix1 {
    vec![matrix]
}
#[derive(Debug, Clone)]
pub enum Constant {
    Integer(Integer),
    UnsignedInteger(UnsignedInteger),
    TmlFloat(TmlFloat),
    TmlString(TmlString),
    Boolean(Boolean),
}
pub fn constant_integer(_ctx: &Ctx, integer: Integer) -> Constant {
    Constant::Integer(integer)
}
pub fn constant_unsigned_integer(
    _ctx: &Ctx,
    unsigned_integer: UnsignedInteger,
) -> Constant {
    Constant::UnsignedInteger(unsigned_integer)
}
pub fn constant_tml_float(_ctx: &Ctx, tml_float: TmlFloat) -> Constant {
    Constant::TmlFloat(tml_float)
}
pub fn constant_tml_string(_ctx: &Ctx, tml_string: TmlString) -> Constant {
    Constant::TmlString(tml_string)
}
pub fn constant_boolean(_ctx: &Ctx, boolean: Boolean) -> Constant {
    Constant::Boolean(boolean)
}
#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: BooleanConst,
}
pub fn boolean_c1(_ctx: &Ctx, value: BooleanConst) -> Boolean {
    Boolean { value }
}
#[derive(Debug, Clone)]
pub struct IntegerC1 {
    pub value: IntegerConst,
}
#[derive(Debug, Clone)]
pub struct IntegerC2 {
    pub value: HexadecimalConst,
}
#[derive(Debug, Clone)]
pub struct IntegerC3 {
    pub value: BinaryConst,
}
#[derive(Debug, Clone)]
pub enum Integer {
    C1(IntegerC1),
    C2(IntegerC2),
    C3(IntegerC3),
}
pub fn integer_c1(_ctx: &Ctx, value: IntegerConst) -> Integer {
    Integer::C1(IntegerC1 { value })
}
pub fn integer_c2(_ctx: &Ctx, value: HexadecimalConst) -> Integer {
    Integer::C2(IntegerC2 { value })
}
pub fn integer_c3(_ctx: &Ctx, value: BinaryConst) -> Integer {
    Integer::C3(IntegerC3 { value })
}
#[derive(Debug, Clone)]
pub struct UnsignedIntegerC1 {
    pub value: UnsignedIntegerConst,
}
#[derive(Debug, Clone)]
pub struct UnsignedIntegerC2 {
    pub value: UnsignedHexadecimalConst,
}
#[derive(Debug, Clone)]
pub struct UnsignedIntegerC3 {
    pub value: UnsignedBinaryConst,
}
#[derive(Debug, Clone)]
pub enum UnsignedInteger {
    C1(UnsignedIntegerC1),
    C2(UnsignedIntegerC2),
    C3(UnsignedIntegerC3),
}
pub fn unsigned_integer_c1(_ctx: &Ctx, value: UnsignedIntegerConst) -> UnsignedInteger {
    UnsignedInteger::C1(UnsignedIntegerC1 { value })
}
pub fn unsigned_integer_c2(
    _ctx: &Ctx,
    value: UnsignedHexadecimalConst,
) -> UnsignedInteger {
    UnsignedInteger::C2(UnsignedIntegerC2 { value })
}
pub fn unsigned_integer_c3(_ctx: &Ctx, value: UnsignedBinaryConst) -> UnsignedInteger {
    UnsignedInteger::C3(UnsignedIntegerC3 { value })
}
#[derive(Debug, Clone)]
pub struct TmlFloat {
    pub value: FloatConst,
}
pub fn tml_float_c1(_ctx: &Ctx, value: FloatConst) -> TmlFloat {
    TmlFloat { value }
}
#[derive(Debug, Clone)]
pub struct TmlString {
    pub value: StringConst,
}
pub fn tml_string_c1(_ctx: &Ctx, value: StringConst) -> TmlString {
    TmlString { value }
}
