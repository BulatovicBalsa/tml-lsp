use rstest::rstest;
use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_parser::tml_actions::{AssignmentStatement, ExternalDeclaration, Statement};

#[test]
fn test_fn_prefix_identifier() {
    let parser = TmlParser::new();
    let input = "fn_x = 0\nfn test_fn():\n    real if_x = 0\nend\n";
    assert!(parser.parse(input).is_ok(), "Failed to parse: {}", input);
}

#[rstest]
#[case::if_kw("if_x")]
#[case::for_kw("for_x")]
#[case::while_kw("while_x")]
#[case::feedthrough_kw("feedthrough_x")]
#[case::exists_kw("exists_x")]
#[case::input_kw("input_x")]
#[case::in_kw("in_x")]
#[case::out_kw("out_x")]
#[case::int_kw("int_x")]
#[case::real_kw("real_x")]
#[case::bool_kw("bool_x")]
#[case::str_kw("str_x")]
#[case::fn_kw("fn_x")]
#[case::return_kw("return_x")]
#[case::break_kw("break_x")]
#[case::continue_kw("continue_x")]
#[case::not_kw("not_x")]
#[case::and_kw("and_x")]
#[case::or_kw("or_x")]
#[case::tensor_kw("tensor_x")]
#[case::macro_kw("macro_x")]
#[case::pass_kw("pass_x")]
#[case::narrow_kw("narrow_x")]
fn test_keyword_prefix_identifier(#[case] var: &str) {
    let parser = TmlParser::new();
    let input = format!("fn test():\n    {} = 0\nend\n", var);
    let ast = parser.parse(&input);
    assert!(ast.is_ok(), "Failed to parse identifier: {}", var);

    let ext_decls = ast.unwrap().ext_decls;
    assert_eq!(ext_decls.len(), 1, "Expected one function definition, got {}", ext_decls.len());

    let func_decl = match ext_decls[0] {
        ExternalDeclaration::FunctionDefinition(ref func) => func,
        _ => panic!("Expected a function definition, got {:?}", ext_decls[0]),
    };

    let stmt = match &func_decl.statement_block.statements {
        Some(statements) => {
            assert_eq!(statements.len(), 1, "Expected one statement in function body, got {}", statements.len());
            &statements[0]
        }
        None => panic!("Expected statements in function body, got none"),
    };
    let var_assign = match stmt {
        Statement::AssignmentStatement(
            AssignmentStatement::VarAssignmentStatement(assign)
        ) => assign,
        _ => panic!("Expected an assignment statement, got {:?}", stmt),
    };

    let var_name = &var_assign.var.names[0];
    assert_eq!(var_name.value, var, "Expected variable name to be '{}', got '{}'", var, var_name.value);
}