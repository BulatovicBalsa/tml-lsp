/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::{Context as RustemoContext, LineColumn, Position, Token as RustemoToken};
use crate::keyword_token;
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
pub type TrueConst = String;
pub fn true_const(_ctx: &Ctx, token: Token) -> TrueConst {
    token.value.into()
}
pub type FalseConst = String;
pub fn false_const(_ctx: &Ctx, token: Token) -> FalseConst {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct HeaderColon {
    pub value: String,
    pub position: Position,
}
pub fn header_colon(_ctx: &Ctx, token: Token) -> HeaderColon {
    HeaderColon {
        value: token.value.into(),
        position: _ctx.position(),
    }
}
pub type TypeKw = String;
pub fn type_kw(_ctx: &Ctx, token: Token) -> TypeKw {
    token.value.into()
}
pub type LenKw = String;
pub fn len_kw(_ctx: &Ctx, token: Token) -> LenKw {
    token.value.into()
}
pub type SizeKw = String;
pub fn size_kw(_ctx: &Ctx, token: Token) -> SizeKw {
    token.value.into()
}
pub type NumelKw = String;
pub fn numel_kw(_ctx: &Ctx, token: Token) -> NumelKw {
    token.value.into()
}
pub type RowsKw = String;
pub fn rows_kw(_ctx: &Ctx, token: Token) -> RowsKw {
    token.value.into()
}
pub type ColsKw = String;
pub fn cols_kw(_ctx: &Ctx, token: Token) -> ColsKw {
    token.value.into()
}
pub type StringConst = String;
pub fn string_const(_ctx: &Ctx, token: Token) -> StringConst {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Id {
    pub value: String,
    pub position: Position,
}
pub fn id(_ctx: &Ctx, token: Token) -> Id {
    let value: String = token.value.into();
    let pos_start = _ctx.position().line_col.unwrap().column - value.len() + 1;
    Id {
        value,
        position: Position {
            pos: _ctx.position().pos,
            line_col: Some(LineColumn {
                line: _ctx.position().line_col.unwrap().line,
                column: pos_start,
            }),
        },
    }
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
    IoDeclarationStatement(IoDeclarationStatement),
    IoWriteStatement(IoWriteStatement),
    MacroFor(MacroFor),
    MacroIf(MacroIf),
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
pub fn external_declaration_io_declaration_statement(
    _ctx: &Ctx,
    io_declaration_statement: IoDeclarationStatement,
) -> ExternalDeclaration {
    ExternalDeclaration::IoDeclarationStatement(io_declaration_statement)
}
pub fn external_declaration_io_write_statement(
    _ctx: &Ctx,
    io_write_statement: IoWriteStatement,
) -> ExternalDeclaration {
    ExternalDeclaration::IoWriteStatement(io_write_statement)
}
pub fn external_declaration_macro_for(
    _ctx: &Ctx,
    macro_for: MacroFor,
) -> ExternalDeclaration {
    ExternalDeclaration::MacroFor(macro_for)
}
pub fn external_declaration_macro_if(
    _ctx: &Ctx,
    macro_if: MacroIf,
) -> ExternalDeclaration {
    ExternalDeclaration::MacroIf(macro_if)
}
#[derive(Debug, Clone)]
pub struct MacroFor {
    pub macro_t: MacroT,
    pub body: ForIterationStatement,
}
pub fn macro_for_c1(
    _ctx: &Ctx,
    macro_t: MacroT,
    body: ForIterationStatement,
) -> MacroFor {
    MacroFor { macro_t, body }
}
#[derive(Debug, Clone)]
pub struct MacroIf {
    pub macro_t: MacroT,
    pub body: SelectionStatement,
}
pub fn macro_if_c1(_ctx: &Ctx, macro_t: MacroT, body: SelectionStatement) -> MacroIf {
    MacroIf { macro_t, body }
}
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub func_t: FuncT,
    pub id: Id,
    pub parameters_list: Parameter0,
    pub ret_type: TypeSpecOpt,
    pub header_colon: HeaderColon,
    pub statement_block: StatementBlock,
    pub end_t: EndT,
}
pub fn function_definition_c1(
    _ctx: &Ctx,
    func_t: FuncT,
    id: Id,
    parameters_list: Parameter0,
    ret_type: TypeSpecOpt,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    end_t: EndT,
) -> FunctionDefinition {
    FunctionDefinition {
        func_t,
        id,
        parameters_list,
        ret_type,
        header_colon,
        statement_block,
        end_t,
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
pub struct NarrowExpression {
    pub expr: Box<Expression>,
}
pub fn narrow_expression_c1(_ctx: &Ctx, expr: Expression) -> NarrowExpression {
    NarrowExpression {
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
    pub type_kw: TypeKw,
}
pub fn derived_type_c1(
    _ctx: &Ctx,
    name: DotAccessExpression,
    brackets: SquareBrackets0,
    type_kw: TypeKw,
) -> DerivedType {
    DerivedType {
        name,
        brackets,
        type_kw,
    }
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
    IntT(IntT),
    UintT(UintT),
    RealT(RealT),
    BoolT(BoolT),
    StrT(StrT),
    CharT(CharT),
}
pub fn simple_type_spec_int_t(_ctx: &Ctx, int_t: IntT) -> SimpleTypeSpec {
    SimpleTypeSpec::IntT(int_t)
}
pub fn simple_type_spec_uint_t(_ctx: &Ctx, uint_t: UintT) -> SimpleTypeSpec {
    SimpleTypeSpec::UintT(uint_t)
}
pub fn simple_type_spec_real_t(_ctx: &Ctx, real_t: RealT) -> SimpleTypeSpec {
    SimpleTypeSpec::RealT(real_t)
}
pub fn simple_type_spec_bool_t(_ctx: &Ctx, bool_t: BoolT) -> SimpleTypeSpec {
    SimpleTypeSpec::BoolT(bool_t)
}
pub fn simple_type_spec_str_t(_ctx: &Ctx, str_t: StrT) -> SimpleTypeSpec {
    SimpleTypeSpec::StrT(str_t)
}
pub fn simple_type_spec_char_t(_ctx: &Ctx, char_t: CharT) -> SimpleTypeSpec {
    SimpleTypeSpec::CharT(char_t)
}
#[derive(Debug, Clone)]
pub enum IoDirection {
    InT(InT),
    OutT(OutT),
}
pub fn io_direction_in_t(_ctx: &Ctx, in_t: InT) -> IoDirection {
    IoDirection::InT(in_t)
}
pub fn io_direction_out_t(_ctx: &Ctx, out_t: OutT) -> IoDirection {
    IoDirection::OutT(out_t)
}
#[derive(Debug, Clone)]
pub struct TensorConstructor {
    pub tensor_t: TensorT,
    pub _type: Box<TypeSpec>,
    pub dimensions: Expression1,
}
pub fn tensor_constructor_c1(
    _ctx: &Ctx,
    tensor_t: TensorT,
    _type: TypeSpec,
    dimensions: Expression1,
) -> TensorConstructor {
    TensorConstructor {
        tensor_t,
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
pub struct IoConstructor {
    pub direction: IoDirection,
    pub _type: TypeSpec,
    pub address: Expression,
    pub io_flags: IoFlagsSpecOpt,
}
pub fn io_constructor_c1(
    _ctx: &Ctx,
    direction: IoDirection,
    _type: TypeSpec,
    address: Expression,
    io_flags: IoFlagsSpecOpt,
) -> IoConstructor {
    IoConstructor {
        direction,
        _type,
        address,
        io_flags,
    }
}
pub type IoFlagsSpecOpt = Option<IoFlagsSpec>;
pub fn io_flags_spec_opt_io_flags_spec(
    _ctx: &Ctx,
    io_flags_spec: IoFlagsSpec,
) -> IoFlagsSpecOpt {
    Some(io_flags_spec)
}
pub fn io_flags_spec_opt_empty(_ctx: &Ctx) -> IoFlagsSpecOpt {
    None
}
#[derive(Debug, Clone)]
pub struct IoFlagsSpecC2 {
    pub io_range: IoRangeSpec,
}
#[derive(Debug, Clone)]
pub struct IoFlagsSpecC3 {
    pub io_range: IoRangeSpec,
}
#[derive(Debug, Clone)]
pub struct IoFlagsSpecC4 {
    pub io_range: IoRangeSpec,
}
#[derive(Debug, Clone)]
pub enum IoFlagsSpec {
    C1,
    C2(IoFlagsSpecC2),
    C3(IoFlagsSpecC3),
    C4(IoFlagsSpecC4),
}
pub fn io_flags_spec_c1(_ctx: &Ctx) -> IoFlagsSpec {
    IoFlagsSpec::C1
}
pub fn io_flags_spec_c2(_ctx: &Ctx, io_range: IoRangeSpec) -> IoFlagsSpec {
    IoFlagsSpec::C2(IoFlagsSpecC2 { io_range })
}
pub fn io_flags_spec_c3(_ctx: &Ctx, io_range: IoRangeSpec) -> IoFlagsSpec {
    IoFlagsSpec::C3(IoFlagsSpecC3 { io_range })
}
pub fn io_flags_spec_c4(_ctx: &Ctx, io_range: IoRangeSpec) -> IoFlagsSpec {
    IoFlagsSpec::C4(IoFlagsSpecC4 { io_range })
}
#[derive(Debug, Clone)]
pub enum IoRangeSpec {
    Hil,
    Ao,
    Abs,
    Shared,
    Ext,
    Sink,
}
pub fn io_range_spec_hil(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Hil
}
pub fn io_range_spec_ao(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Ao
}
pub fn io_range_spec_abs(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Abs
}
pub fn io_range_spec_shared(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Shared
}
pub fn io_range_spec_ext(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Ext
}
pub fn io_range_spec_sink(_ctx: &Ctx) -> IoRangeSpec {
    IoRangeSpec::Sink
}
#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub statements: Statement0,
}
pub fn statement_block_c1(_ctx: &Ctx, statements: Statement0) -> StatementBlock {
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
pub type Statement0 = Option<Statement1>;
pub fn statement0_statement1(_ctx: &Ctx, statement1: Statement1) -> Statement0 {
    Some(statement1)
}
pub fn statement0_empty(_ctx: &Ctx) -> Statement0 {
    None
}
#[derive(Debug, Clone)]
pub enum Statement {
    FunctionCallStatement(FunctionCallStatement),
    SelectionStatement(SelectionStatement),
    IterationStatement(IterationStatement),
    JumpStatement(JumpStatement),
    ExistsStatement(ExistsStatement),
    NotExistsStatement(NotExistsStatement),
    FeedthroughStatement(FeedthroughStatement),
    NotFeedthroughStatement(NotFeedthroughStatement),
    AssignmentStatement(AssignmentStatement),
    DeclarationStatement(DeclarationStatement),
    IoDeclarationStatement(IoDeclarationStatement),
    IoWriteStatement(IoWriteStatement),
    NoopStatement(NoopStatement),
    MacroFor(MacroFor),
    MacroIf(MacroIf),
}
pub fn statement_function_call_statement(
    _ctx: &Ctx,
    function_call_statement: FunctionCallStatement,
) -> Statement {
    Statement::FunctionCallStatement(function_call_statement)
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
pub fn statement_feedthrough_statement(
    _ctx: &Ctx,
    feedthrough_statement: FeedthroughStatement,
) -> Statement {
    Statement::FeedthroughStatement(feedthrough_statement)
}
pub fn statement_not_feedthrough_statement(
    _ctx: &Ctx,
    not_feedthrough_statement: NotFeedthroughStatement,
) -> Statement {
    Statement::NotFeedthroughStatement(not_feedthrough_statement)
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
pub fn statement_io_declaration_statement(
    _ctx: &Ctx,
    io_declaration_statement: IoDeclarationStatement,
) -> Statement {
    Statement::IoDeclarationStatement(io_declaration_statement)
}
pub fn statement_io_write_statement(
    _ctx: &Ctx,
    io_write_statement: IoWriteStatement,
) -> Statement {
    Statement::IoWriteStatement(io_write_statement)
}
pub fn statement_noop_statement(_ctx: &Ctx, noop_statement: NoopStatement) -> Statement {
    Statement::NoopStatement(noop_statement)
}
pub fn statement_macro_for(_ctx: &Ctx, macro_for: MacroFor) -> Statement {
    Statement::MacroFor(macro_for)
}
pub fn statement_macro_if(_ctx: &Ctx, macro_if: MacroIf) -> Statement {
    Statement::MacroIf(macro_if)
}
#[derive(Debug, Clone)]
pub struct FunctionCallStatement {
    pub call: FunctionCall,
}
pub fn function_call_statement_c1(
    _ctx: &Ctx,
    call: FunctionCall,
) -> FunctionCallStatement {
    FunctionCallStatement { call }
}
#[derive(Debug, Clone)]
pub struct SelectionStatement {
    pub if_t: IfT,
    pub condition: Expression,
    pub header_colon: HeaderColon,
    pub if_statement_block: Box<StatementBlock>,
    pub elseif_clause: ElseIfClause0,
    pub else_clause: ElseClauseOpt,
    pub end_t: EndT,
}
pub fn selection_statement_c1(
    _ctx: &Ctx,
    if_t: IfT,
    condition: Expression,
    header_colon: HeaderColon,
    if_statement_block: StatementBlock,
    elseif_clause: ElseIfClause0,
    else_clause: ElseClauseOpt,
    end_t: EndT,
) -> SelectionStatement {
    SelectionStatement {
        if_t,
        condition,
        header_colon,
        if_statement_block: Box::new(if_statement_block),
        elseif_clause,
        else_clause,
        end_t,
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
    pub else_t: ElseT,
    pub header_colon: HeaderColon,
    pub else_statement_block: Box<StatementBlock>,
}
pub fn else_clause_c1(
    _ctx: &Ctx,
    else_t: ElseT,
    header_colon: HeaderColon,
    else_statement_block: StatementBlock,
) -> ElseClause {
    ElseClause {
        else_t,
        header_colon,
        else_statement_block: Box::new(else_statement_block),
    }
}
#[derive(Debug, Clone)]
pub struct ElseIfClause {
    pub else_if_t: ElseifT,
    pub condition: Expression,
    pub header_colon: HeaderColon,
    pub elseif_statement_block: Box<StatementBlock>,
}
pub fn else_if_clause_c1(
    _ctx: &Ctx,
    else_if_t: ElseifT,
    condition: Expression,
    header_colon: HeaderColon,
    elseif_statement_block: StatementBlock,
) -> ElseIfClause {
    ElseIfClause {
        else_if_t,
        condition,
        header_colon,
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
    pub for_t: ForT,
    pub header: ForIterationHeader,
    pub header_colon: HeaderColon,
    pub body: ForIterationBody,
    pub end_t: EndT,
}
pub fn for_iteration_statement_c1(
    _ctx: &Ctx,
    for_t: ForT,
    header: ForIterationHeader,
    header_colon: HeaderColon,
    body: ForIterationBody,
    end_t: EndT,
) -> ForIterationStatement {
    ForIterationStatement {
        for_t,
        header,
        header_colon,
        body,
        end_t,
    }
}
#[derive(Debug, Clone)]
pub struct ForIterationHeader {
    pub idx: Id,
    pub iterator_expression: IteratorExpression,
}
pub fn for_iteration_header_c1(
    _ctx: &Ctx,
    idx: Id,
    iterator_expression: IteratorExpression,
) -> ForIterationHeader {
    ForIterationHeader {
        idx,
        iterator_expression,
    }
}
#[derive(Debug, Clone)]
pub struct ForIterationBody {
    pub statement_block: Box<StatementBlock>,
}
pub fn for_iteration_body_c1(
    _ctx: &Ctx,
    statement_block: StatementBlock,
) -> ForIterationBody {
    ForIterationBody {
        statement_block: Box::new(statement_block),
    }
}
#[derive(Debug, Clone)]
pub enum IteratorExpression {
    Expression(Expression),
    RangeFromStepTo(RangeFromStepTo),
    RangeFromTo(RangeFromTo),
}
pub fn iterator_expression_expression(
    _ctx: &Ctx,
    expression: Expression,
) -> IteratorExpression {
    IteratorExpression::Expression(expression)
}
pub fn iterator_expression_range_from_step_to(
    _ctx: &Ctx,
    range_from_step_to: RangeFromStepTo,
) -> IteratorExpression {
    IteratorExpression::RangeFromStepTo(range_from_step_to)
}
pub fn iterator_expression_range_from_to(
    _ctx: &Ctx,
    range_from_to: RangeFromTo,
) -> IteratorExpression {
    IteratorExpression::RangeFromTo(range_from_to)
}
#[derive(Debug, Clone)]
pub struct WhileIterationStatement {
    pub while_t: WhileT,
    pub condition: Expression,
    pub header_colon: HeaderColon,
    pub statement_block: Box<StatementBlock>,
    pub end_t: EndT,
}
pub fn while_iteration_statement_c1(
    _ctx: &Ctx,
    while_t: WhileT,
    condition: Expression,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    end_t: EndT,
) -> WhileIterationStatement {
    WhileIterationStatement {
        while_t,
        condition,
        header_colon,
        statement_block: Box::new(statement_block),
        end_t,
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
    BreakT(BreakT),
}
pub fn break_statement_break_t(_ctx: &Ctx, break_t: BreakT) -> BreakStatement {
    BreakStatement::BreakT(break_t)
}
#[derive(Debug, Clone)]
pub enum ContinueStatement {
    ContinueT(ContinueT),
}
pub fn continue_statement_continue_t(
    _ctx: &Ctx,
    continue_t: ContinueT,
) -> ContinueStatement {
    ContinueStatement::ContinueT(continue_t)
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
    ReturnT(ReturnT),
}
pub fn empty_return_return_t(_ctx: &Ctx, return_t: ReturnT) -> EmptyReturn {
    EmptyReturn::ReturnT(return_t)
}
#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub return_t: ReturnT,
    pub ret_val: Expression,
}
pub fn return_value_c1(
    _ctx: &Ctx,
    return_t: ReturnT,
    ret_val: Expression,
) -> ReturnValue {
    ReturnValue { return_t, ret_val }
}
#[derive(Debug, Clone)]
pub struct ExistsStatement {
    pub exists_t: ExistsT,
    pub guarded: DotAccessExpression1,
    pub header_colon: HeaderColon,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
    pub end_t: EndT,
}
pub fn exists_statement_c1(
    _ctx: &Ctx,
    exists_t: ExistsT,
    guarded: DotAccessExpression1,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
    end_t: EndT,
) -> ExistsStatement {
    ExistsStatement {
        exists_t,
        guarded,
        header_colon,
        statement_block: Box::new(statement_block),
        else_clause,
        end_t,
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
    pub not_t: NotT,
    pub exists_t: ExistsT,
    pub guarded: DotAccessExpression1,
    pub header_colon: HeaderColon,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
    pub end_t: EndT,
}
pub fn not_exists_statement_c1(
    _ctx: &Ctx,
    not_t: NotT,
    exists_t: ExistsT,
    guarded: DotAccessExpression1,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
    end_t: EndT,
) -> NotExistsStatement {
    NotExistsStatement {
        not_t,
        exists_t,
        guarded,
        header_colon,
        statement_block: Box::new(statement_block),
        else_clause,
        end_t,
    }
}
#[derive(Debug, Clone)]
pub struct FeedthroughStatement {
    pub feedthrough_t: FeedthroughT,
    pub guarded: DotAccessExpression1,
    pub header_colon: HeaderColon,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
    pub end_t: EndT,
}
pub fn feedthrough_statement_c1(
    _ctx: &Ctx,
    feedthrough_t: FeedthroughT,
    guarded: DotAccessExpression1,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
    end_t: EndT,
) -> FeedthroughStatement {
    FeedthroughStatement {
        feedthrough_t,
        guarded,
        header_colon,
        statement_block: Box::new(statement_block),
        else_clause,
        end_t,
    }
}
#[derive(Debug, Clone)]
pub struct NotFeedthroughStatement {
    pub not_t: NotT,
    pub feedthrough_t: FeedthroughT,
    pub guarded: DotAccessExpression1,
    pub header_colon: HeaderColon,
    pub statement_block: Box<StatementBlock>,
    pub else_clause: ElseClauseOpt,
    pub end_t: EndT,
}
pub fn not_feedthrough_statement_c1(
    _ctx: &Ctx,
    not_t: NotT,
    feedthrough_t: FeedthroughT,
    guarded: DotAccessExpression1,
    header_colon: HeaderColon,
    statement_block: StatementBlock,
    else_clause: ElseClauseOpt,
    end_t: EndT,
) -> NotFeedthroughStatement {
    NotFeedthroughStatement {
        not_t,
        feedthrough_t,
        guarded,
        header_colon,
        statement_block: Box::new(statement_block),
        else_clause,
        end_t,
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
pub enum IoWriteStatement {
    VarIoWriteStatement(VarIoWriteStatement),
    TensorIoWriteStatement(TensorIoWriteStatement),
}
pub fn io_write_statement_var_io_write_statement(
    _ctx: &Ctx,
    var_io_write_statement: VarIoWriteStatement,
) -> IoWriteStatement {
    IoWriteStatement::VarIoWriteStatement(var_io_write_statement)
}
pub fn io_write_statement_tensor_io_write_statement(
    _ctx: &Ctx,
    tensor_io_write_statement: TensorIoWriteStatement,
) -> IoWriteStatement {
    IoWriteStatement::TensorIoWriteStatement(tensor_io_write_statement)
}
#[derive(Debug, Clone)]
pub struct VarIoWriteStatement {
    pub io_var: DotAccessExpression,
    pub rvalue: Expression,
}
pub fn var_io_write_statement_c1(
    _ctx: &Ctx,
    io_var: DotAccessExpression,
    rvalue: Expression,
) -> VarIoWriteStatement {
    VarIoWriteStatement {
        io_var,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct TensorIoWriteStatement {
    pub io_tensor: LValueTensor,
    pub rvalue: Expression,
}
pub fn tensor_io_write_statement_c1(
    _ctx: &Ctx,
    io_tensor: LValueTensor,
    rvalue: Expression,
) -> TensorIoWriteStatement {
    TensorIoWriteStatement {
        io_tensor,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationStatement {
    pub _type: TypeSpec,
    pub id: DotAccessExpression,
    pub rvalue: Expression,
}
pub fn declaration_statement_c1(
    _ctx: &Ctx,
    _type: TypeSpec,
    id: DotAccessExpression,
    rvalue: Expression,
) -> DeclarationStatement {
    DeclarationStatement {
        _type,
        id,
        rvalue,
    }
}
#[derive(Debug, Clone)]
pub struct IoDeclarationStatement {
    pub io_type: IoConstructor,
    pub id: DotAccessExpression,
}
pub fn io_declaration_statement_c1(
    _ctx: &Ctx,
    io_type: IoConstructor,
    id: DotAccessExpression,
) -> IoDeclarationStatement {
    IoDeclarationStatement {
        io_type,
        id,
    }
}
#[derive(Debug, Clone)]
pub struct NoopStatement {
    pub noop: PassT,
}
pub fn noop_statement_c1(_ctx: &Ctx, noop: PassT) -> NoopStatement {
    NoopStatement { noop }
}
#[derive(Debug, Clone)]
pub enum Expression {
    LogicalExpression(LogicalExpression),
    TypeCastExpression(TypeCastExpression),
    NarrowExpression(NarrowExpression),
    MathExpression(MathExpression),
    BitwiseExpression(BitwiseExpression),
    IoReadExpression(IoReadExpression),
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
pub fn expression_narrow_expression(
    _ctx: &Ctx,
    narrow_expression: NarrowExpression,
) -> Expression {
    Expression::NarrowExpression(narrow_expression)
}
pub fn expression_math_expression(
    _ctx: &Ctx,
    math_expression: MathExpression,
) -> Expression {
    Expression::MathExpression(math_expression)
}
pub fn expression_bitwise_expression(
    _ctx: &Ctx,
    bitwise_expression: BitwiseExpression,
) -> Expression {
    Expression::BitwiseExpression(bitwise_expression)
}
pub fn expression_io_read_expression(
    _ctx: &Ctx,
    io_read_expression: IoReadExpression,
) -> Expression {
    Expression::IoReadExpression(io_read_expression)
}
#[derive(Debug, Clone)]
pub struct AttributeAccess {
    pub expr: Box<Expression>,
    pub attr: Attribute,
}
pub fn attribute_access_c1(
    _ctx: &Ctx,
    expr: Expression,
    attr: Attribute,
) -> AttributeAccess {
    AttributeAccess {
        expr: Box::new(expr),
        attr,
    }
}
#[derive(Debug, Clone)]
pub enum Attribute {
    LenKw(LenKw),
    SizeKw(SizeKw),
    NumelKw(NumelKw),
    RowsKw(RowsKw),
    ColsKw(ColsKw),
}
pub fn attribute_len_kw(_ctx: &Ctx, len_kw: LenKw) -> Attribute {
    Attribute::LenKw(len_kw)
}
pub fn attribute_size_kw(_ctx: &Ctx, size_kw: SizeKw) -> Attribute {
    Attribute::SizeKw(size_kw)
}
pub fn attribute_numel_kw(_ctx: &Ctx, numel_kw: NumelKw) -> Attribute {
    Attribute::NumelKw(numel_kw)
}
pub fn attribute_rows_kw(_ctx: &Ctx, rows_kw: RowsKw) -> Attribute {
    Attribute::RowsKw(rows_kw)
}
pub fn attribute_cols_kw(_ctx: &Ctx, cols_kw: ColsKw) -> Attribute {
    Attribute::ColsKw(cols_kw)
}
#[derive(Debug, Clone)]
pub enum IoReadExpression {
    VarIoReadExpression(VarIoReadExpression),
    TensorIoReadExpression(TensorIoReadExpression),
}
pub fn io_read_expression_var_io_read_expression(
    _ctx: &Ctx,
    var_io_read_expression: VarIoReadExpression,
) -> IoReadExpression {
    IoReadExpression::VarIoReadExpression(var_io_read_expression)
}
pub fn io_read_expression_tensor_io_read_expression(
    _ctx: &Ctx,
    tensor_io_read_expression: TensorIoReadExpression,
) -> IoReadExpression {
    IoReadExpression::TensorIoReadExpression(tensor_io_read_expression)
}
#[derive(Debug, Clone)]
pub struct VarIoReadExpression {
    pub io_var: Box<DotAccessExpression>,
}
pub fn var_io_read_expression_c1(
    _ctx: &Ctx,
    io_var: DotAccessExpression,
) -> VarIoReadExpression {
    VarIoReadExpression {
        io_var: Box::new(io_var),
    }
}
#[derive(Debug, Clone)]
pub struct TensorIoReadExpression {
    pub io_tensor: LValueTensor,
}
pub fn tensor_io_read_expression_c1(
    _ctx: &Ctx,
    io_tensor: LValueTensor,
) -> TensorIoReadExpression {
    TensorIoReadExpression {
        io_tensor,
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
    AndAssignT,
    KapAssignT,
    PipeAssignT,
    LeftShiftAssignT,
    RightShiftAssignT,
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
pub fn assignment_operator_and_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::AndAssignT
}
pub fn assignment_operator_kap_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::KapAssignT
}
pub fn assignment_operator_pipe_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::PipeAssignT
}
pub fn assignment_operator_left_shift_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::LeftShiftAssignT
}
pub fn assignment_operator_right_shift_assign_t(_ctx: &Ctx) -> AssignmentOperator {
    AssignmentOperator::RightShiftAssignT
}
#[derive(Debug, Clone)]
pub enum LogicalExpression {
    BinaryRelationalExpression(BinaryRelationalExpression),
    BinaryLogicalExpression(BinaryLogicalExpression),
    UnaryLogicalExpression(UnaryLogicalExpression),
}
pub fn logical_expression_binary_relational_expression(
    _ctx: &Ctx,
    binary_relational_expression: BinaryRelationalExpression,
) -> LogicalExpression {
    LogicalExpression::BinaryRelationalExpression(binary_relational_expression)
}
pub fn logical_expression_binary_logical_expression(
    _ctx: &Ctx,
    binary_logical_expression: BinaryLogicalExpression,
) -> LogicalExpression {
    LogicalExpression::BinaryLogicalExpression(binary_logical_expression)
}
pub fn logical_expression_unary_logical_expression(
    _ctx: &Ctx,
    unary_logical_expression: UnaryLogicalExpression,
) -> LogicalExpression {
    LogicalExpression::UnaryLogicalExpression(unary_logical_expression)
}
#[derive(Debug, Clone)]
pub struct BinaryLogicalExpressionC1 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryLogicalExpressionC2 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum BinaryLogicalExpression {
    C1(BinaryLogicalExpressionC1),
    C2(BinaryLogicalExpressionC2),
}
pub fn binary_logical_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryLogicalExpression {
    BinaryLogicalExpression::C1(BinaryLogicalExpressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_logical_expression_c2(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryLogicalExpression {
    BinaryLogicalExpression::C2(BinaryLogicalExpressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct UnaryLogicalExpression {
    pub expr: Box<Expression>,
}
pub fn unary_logical_expression_c1(
    _ctx: &Ctx,
    expr: Expression,
) -> UnaryLogicalExpression {
    UnaryLogicalExpression {
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
pub struct BinaryMathExpressionC8 {
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
    C8(BinaryMathExpressionC8),
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
pub fn binary_math_expression_c8(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryMathExpression {
    BinaryMathExpression::C8(BinaryMathExpressionC8 {
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
pub enum BitwiseExpression {
    BinaryBitwiseExpression(BinaryBitwiseExpression),
    UnaryBitwiseExpression(UnaryBitwiseExpression),
}
pub fn bitwise_expression_binary_bitwise_expression(
    _ctx: &Ctx,
    binary_bitwise_expression: BinaryBitwiseExpression,
) -> BitwiseExpression {
    BitwiseExpression::BinaryBitwiseExpression(binary_bitwise_expression)
}
pub fn bitwise_expression_unary_bitwise_expression(
    _ctx: &Ctx,
    unary_bitwise_expression: UnaryBitwiseExpression,
) -> BitwiseExpression {
    BitwiseExpression::UnaryBitwiseExpression(unary_bitwise_expression)
}
#[derive(Debug, Clone)]
pub struct BinaryBitwiseExpressionC1 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryBitwiseExpressionC2 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryBitwiseExpressionC3 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryBitwiseExpressionC4 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct BinaryBitwiseExpressionC5 {
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub enum BinaryBitwiseExpression {
    C1(BinaryBitwiseExpressionC1),
    C2(BinaryBitwiseExpressionC2),
    C3(BinaryBitwiseExpressionC3),
    C4(BinaryBitwiseExpressionC4),
    C5(BinaryBitwiseExpressionC5),
}
pub fn binary_bitwise_expression_c1(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryBitwiseExpression {
    BinaryBitwiseExpression::C1(BinaryBitwiseExpressionC1 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_bitwise_expression_c2(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryBitwiseExpression {
    BinaryBitwiseExpression::C2(BinaryBitwiseExpressionC2 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_bitwise_expression_c3(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryBitwiseExpression {
    BinaryBitwiseExpression::C3(BinaryBitwiseExpressionC3 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_bitwise_expression_c4(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryBitwiseExpression {
    BinaryBitwiseExpression::C4(BinaryBitwiseExpressionC4 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
pub fn binary_bitwise_expression_c5(
    _ctx: &Ctx,
    left_expr: Expression,
    right_expr: Expression,
) -> BinaryBitwiseExpression {
    BinaryBitwiseExpression::C5(BinaryBitwiseExpressionC5 {
        left_expr: Box::new(left_expr),
        right_expr: Box::new(right_expr),
    })
}
#[derive(Debug, Clone)]
pub struct UnaryBitwiseExpression {
    pub expr: Box<Expression>,
}
pub fn unary_bitwise_expression_c1(
    _ctx: &Ctx,
    expr: Expression,
) -> UnaryBitwiseExpression {
    UnaryBitwiseExpression {
        expr: Box::new(expr),
    }
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
    InputExpression(InputExpression),
    AttributeAccess(AttributeAccess),
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
pub fn postfix_expression_input_expression(
    _ctx: &Ctx,
    input_expression: InputExpression,
) -> PostfixExpression {
    PostfixExpression::InputExpression(input_expression)
}
pub fn postfix_expression_attribute_access(
    _ctx: &Ctx,
    attribute_access: AttributeAccess,
) -> PostfixExpression {
    PostfixExpression::AttributeAccess(attribute_access)
}
#[derive(Debug, Clone)]
pub struct RValue {
    pub _ref: Box<DotAccessExpression>,
}
pub fn rvalue_c1(_ctx: &Ctx, _ref: DotAccessExpression) -> RValue {
    RValue { _ref: Box::new(_ref) }
}
#[derive(Debug, Clone)]
pub enum RangeExpression {
    RangeFromTo(RangeFromTo),
    RangeFrom(RangeFrom),
    RangeTo(RangeTo),
    RangeFromStepTo(RangeFromStepTo),
    RangeAll(RangeAll),
}
pub fn range_expression_range_from_to(
    _ctx: &Ctx,
    range_from_to: RangeFromTo,
) -> RangeExpression {
    RangeExpression::RangeFromTo(range_from_to)
}
pub fn range_expression_range_from(
    _ctx: &Ctx,
    range_from: RangeFrom,
) -> RangeExpression {
    RangeExpression::RangeFrom(range_from)
}
pub fn range_expression_range_to(_ctx: &Ctx, range_to: RangeTo) -> RangeExpression {
    RangeExpression::RangeTo(range_to)
}
pub fn range_expression_range_from_step_to(
    _ctx: &Ctx,
    range_from_step_to: RangeFromStepTo,
) -> RangeExpression {
    RangeExpression::RangeFromStepTo(range_from_step_to)
}
pub fn range_expression_range_all(_ctx: &Ctx, range_all: RangeAll) -> RangeExpression {
    RangeExpression::RangeAll(range_all)
}
#[derive(Debug, Clone)]
pub struct RangeFromTo {
    pub start: Box<Expression>,
    pub stop: Box<Expression>,
}
pub fn range_from_to_c1(_ctx: &Ctx, start: Expression, stop: Expression) -> RangeFromTo {
    RangeFromTo {
        start: Box::new(start),
        stop: Box::new(stop),
    }
}
#[derive(Debug, Clone)]
pub struct RangeFrom {
    pub start: Box<Expression>,
}
pub fn range_from_c1(_ctx: &Ctx, start: Expression) -> RangeFrom {
    RangeFrom {
        start: Box::new(start),
    }
}
#[derive(Debug, Clone)]
pub struct RangeTo {
    pub stop: Box<Expression>,
}
pub fn range_to_c1(_ctx: &Ctx, stop: Expression) -> RangeTo {
    RangeTo { stop: Box::new(stop) }
}
#[derive(Debug, Clone)]
pub struct RangeFromStepTo {
    pub start: Box<Expression>,
    pub stop: Box<Expression>,
    pub step: Box<Expression>,
}
pub fn range_from_step_to_c1(
    _ctx: &Ctx,
    start: Expression,
    stop: Expression,
    step: Expression,
) -> RangeFromStepTo {
    RangeFromStepTo {
        start: Box::new(start),
        stop: Box::new(stop),
        step: Box::new(step),
    }
}
#[derive(Debug, Clone)]
pub enum RangeAll {
    Colon,
}
pub fn range_all_colon(_ctx: &Ctx) -> RangeAll {
    RangeAll::Colon
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
#[derive(Debug, Clone)]
pub struct TransposeExpression {
    pub expr: Box<PostfixExpression>,
}
pub fn transpose_expression_c1(
    _ctx: &Ctx,
    expr: PostfixExpression,
) -> TransposeExpression {
    TransposeExpression {
        expr: Box::new(expr),
    }
}
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub id: Id,
    pub alternative: AlternativeOpt,
    pub arguments_list: Argument0,
}
pub fn function_call_c1(
    _ctx: &Ctx,
    id: Id,
    alternative: AlternativeOpt,
    arguments_list: Argument0,
) -> FunctionCall {
    FunctionCall {
        id,
        alternative,
        arguments_list,
    }
}
pub type AlternativeOpt = Option<AlternativeOptNoO>;
#[derive(Debug, Clone)]
pub enum AlternativeOptNoO {
    Alternative,
}
pub fn alternative_opt_alternative(_ctx: &Ctx) -> AlternativeOpt {
    Some(AlternativeOptNoO::Alternative)
}
pub fn alternative_opt_empty(_ctx: &Ctx) -> AlternativeOpt {
    None
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
pub struct InputExpression {
    pub input_t: InputT,
    pub _type: Box<TypeSpec>,
}
pub fn input_expression_c1(_ctx: &Ctx, input_t: InputT, _type: TypeSpec) -> InputExpression {
    InputExpression {
        input_t,
        _type: Box::new(_type),
    }
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
pub struct IndexExpressionC1 {
    pub expr: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct IndexExpressionC2 {
    pub expr: RangeExpression,
}
#[derive(Debug, Clone)]
pub enum IndexExpression {
    C1(IndexExpressionC1),
    C2(IndexExpressionC2),
}
pub fn index_expression_c1(_ctx: &Ctx, expr: Expression) -> IndexExpression {
    IndexExpression::C1(IndexExpressionC1 {
        expr: Box::new(expr),
    })
}
pub fn index_expression_c2(_ctx: &Ctx, expr: RangeExpression) -> IndexExpression {
    IndexExpression::C2(IndexExpressionC2 { expr })
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
    pub expr: Box<DotAccessExpression>,
    pub indices: Index1,
}
pub fn lvalue_tensor_c1(
    _ctx: &Ctx,
    expr: DotAccessExpression,
    indices: Index1,
) -> LValueTensor {
    LValueTensor {
        expr: Box::new(expr),
        indices,
    }
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
pub struct Index {
    pub index: IndexExpressionList,
}
pub fn index_c1(_ctx: &Ctx, index: IndexExpressionList) -> Index {
    Index { index }
}
#[derive(Debug, Clone)]
pub struct DotAccessExpression {
    pub names: Id1,
    pub macro_index: MacroIndexOpt,
    pub optional: QuestionOpt,
}
pub fn dot_access_expression_c1(
    _ctx: &Ctx,
    names: Id1,
    macro_index: MacroIndexOpt,
    optional: QuestionOpt,
) -> DotAccessExpression {
    DotAccessExpression {
        names,
        macro_index,
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
pub type MacroIndexOpt = Option<MacroIndex>;
pub fn macro_index_opt_macro_index(
    _ctx: &Ctx,
    macro_index: MacroIndex,
) -> MacroIndexOpt {
    Some(macro_index)
}
pub fn macro_index_opt_empty(_ctx: &Ctx) -> MacroIndexOpt {
    None
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
pub struct MacroIndex {
    pub index: Expression1,
}
pub fn macro_index_c1(_ctx: &Ctx, index: Expression1) -> MacroIndex {
    MacroIndex { index }
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
pub struct BooleanC1 {
    pub value: TrueConst,
}
#[derive(Debug, Clone)]
pub struct BooleanC2 {
    pub value: FalseConst,
}
#[derive(Debug, Clone)]
pub enum Boolean {
    C1(BooleanC1),
    C2(BooleanC2),
}
pub fn boolean_c1(_ctx: &Ctx, value: TrueConst) -> Boolean {
    Boolean::C1(BooleanC1 { value })
}
pub fn boolean_c2(_ctx: &Ctx, value: FalseConst) -> Boolean {
    Boolean::C2(BooleanC2 { value })
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
#[derive(Debug, Clone)]
pub struct EndT {
    pub value: String,
    pub position: Position,
}
pub fn end_t(_ctx: &Ctx, token: Token) -> EndT {
    EndT {
        value: token.value.into(),
        position: _ctx.position(),
    }
}
pub type InT = String;
pub fn in_t(_ctx: &Ctx, token: Token) -> InT {
    token.value.into()
}
pub type OutT = String;
pub fn out_t(_ctx: &Ctx, token: Token) -> OutT {
    token.value.into()
}
keyword_token!(FuncT, func_t);
keyword_token!(IfT, if_t);
keyword_token!(ElseT, else_t);
keyword_token!(ElseifT, elseif_t);
keyword_token!(ForT, for_t);
keyword_token!(WhileT, while_t);
keyword_token!(ExistsT, exists_t);
keyword_token!(Feedthrough, feedthrough);
keyword_token!(FeedthroughT, feedthrough_t);
keyword_token!(NotT, not_t);
keyword_token!(IntT, int_t);
keyword_token!(UintT, uint_t);
keyword_token!(RealT, real_t);
keyword_token!(BoolT, bool_t);
keyword_token!(StrT, str_t);
keyword_token!(CharT, char_t);
keyword_token!(TensorT, tensor_t);
keyword_token!(ContinueT, continue_t);
keyword_token!(ReturnT, return_t);
keyword_token!(PassT, pass_t);
keyword_token!(MacroT, macro_t);
keyword_token!(BreakT, break_t);
keyword_token!(InputT, input_t);
