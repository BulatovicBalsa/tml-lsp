use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::formatter::Format;

fn parse_and_format(src: &str) -> String {
    let parser = TmlParser::new();
    let ast = parser.parse(src).expect("Parse failed");
    println!("AST: {:#?}", ast);
    ast.format(0)
}

// ───────────────────────── Helper ─────────────────────────

/// Normalize string by trimming lines and removing empty lines,
/// to make it easier to compare formatted output
fn normalize(s: &str) -> String {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_formats_to(src: &str, expected: &str) {
    let result = parse_and_format(src);
    assert_eq!(normalize(&result), normalize(expected),
               "\n--- GOT ---\n{}\n--- EXPECTED ---\n{}", result, expected);
}

// ───────────────────────── Declaration ─────────────────────────

#[test]
fn test_bool_declaration() {
    assert_formats_to(
        "bool a =                  false",
        "bool a = false",
    );
}

#[test]
fn test_int_declaration() {
    assert_formats_to(
        "int   x   =   42",
        "int x = 42",
    );
}

#[test]
fn test_real_declaration() {
    assert_formats_to(
        "real   pi =    3.14",
        "real pi = 3.14",
    );
}

#[test]
fn test_str_declaration() {
    assert_formats_to(
        r#"str   name =   "hello""#,
        r#"str name = "hello""#,
    );
}

#[test]
fn test_hex_declaration() {
    assert_formats_to(
        "int x = 0xFF",
        "int x = 0xFF",
    );
}

#[test]
fn test_binary_declaration() {
    assert_formats_to(
        "int x = 0b1010",
        "int x = 0b1010",
    );
}

// ───────────────────────── Assignment ─────────────────────────

#[test]
fn test_simple_assignment() {
    assert_formats_to(
        "x   =   5",
        "x = 5",
    );
}

#[test]
fn test_compound_assignment() {
    assert_formats_to(
        "x   +=   5",
        "x += 5",
    );
}

#[test]
fn test_tensor_assignment() {
    assert_formats_to(
        "buf[0]   =   42",
        "buf[0] = 42",
    );
}

// ───────────────────────── Math expressions ─────────────────────────

#[test]
fn test_binary_math() {
    assert_formats_to(
        "x = a   +   b",
        "x = a + b",
    );
}

#[test]
fn test_unary_minus() {
    assert_formats_to(
        "x = -5",
        "x = -5",
    );
}

#[test]
fn test_nested_math() {
    assert_formats_to(
        "x = a  +  b  *  c",
        "x = a + b * c",
    );
}

#[test]
fn test_parenthesis() {
    assert_formats_to(
        "x = (  a  +  b  )  *  c",
        "x = (a + b) * c",
    );
}

// ───────────────────────── If statement ─────────────────────────

#[test]
fn test_if_simple() {
    assert_formats_to(
        r#"
            fn     test():
                if   x > 0:
                    y = 1
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    y = 1
                end
            end
        "#,
    );
}

#[test]
fn test_if_else() {
    assert_formats_to(
        r#"
            fn test():
                if   x > 0:
                    y = 1
                else   :
                    y = 0
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    y = 1
                else:
                    y = 0
                end
            end
        "#,
    );
}

#[test]
fn test_if_elseif_else() {
    assert_formats_to(
        r#"
            fn test():
                if x > 0:
                    y = 1
                elseif x == 0:
                    y = 0
                else:
                    y = -1
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    y = 1
                elseif x == 0:
                    y = 0
                else:
                    y = -1
                end
            end
        "#,
    );
}

#[test]
fn test_if_empty_body() {
    assert_formats_to(
        r#"
            fn test():
                if x > 0:
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    pass
                end
            end
        "#,
    );
}

#[test]
fn test_if_elseif_empty_body() {
    assert_formats_to(
        r#"
            fn test():
                if x > 0:
                    y = 1
                elseif x == 0:
                else:
                    y = -1
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    y = 1
                elseif x == 0:
                    pass
                else:
                    y = -1
                end
            end
        "#,
    );
}

#[test]
fn test_if_else_empty_body() {
    assert_formats_to(
        r#"
            fn test():
                if x > 0:
                    y = 1
                else:
                end
            end
        "#,
        r#"
            fn test():
                if x > 0:
                    y = 1
                else:
                    pass
                end
            end
        "#,
    );
}

// ───────────────────────── For loop ─────────────────────────

#[test]
fn test_for_range() {
    assert_formats_to(
        r#"
            fn test():
                for   i  =  0:10:
                    x = i
                end
            end
        "#,
        r#"
            fn test():
                for i = 0:10:
                    x = i
                end
            end
        "#,
    );
}

#[test]
fn test_for_range_step() {
    assert_formats_to(
        r#"
            fn test():
                for i = 0:10:2:
                    x = i
                end
            end
        "#,
        r#"
            fn test():
                for i = 0:10:2:
                    x = i
                end
            end
        "#,
    );
}

#[test]
fn test_for_empty_body() {
    assert_formats_to(
        r#"
            fn test():
                for i = 0:10:
                end
            end
        "#,
        r#"
            fn test():
                for i = 0:10:
                    pass
                end
            end
        "#,
    );
}

// ───────────────────────── While loop ─────────────────────────

#[test]
fn test_while() {
    assert_formats_to(
        r#"
            fn test():
                while   x > 0:
                    x = x - 1
                end
            end
        "#,
        r#"
            fn test():
                while x > 0:
                    x = x - 1
                end
            end
        "#,
    );
}

#[test]
fn test_while_empty_body() {
    assert_formats_to(
        r#"
            fn test():
                while x > 0:
                end
            end
        "#,
        r#"
            fn test():
                while x > 0:
                    pass
                end
            end
        "#,
    );
}

// ───────────────────────── Function definition ─────────────────────────

#[test]
fn test_function_no_params() {
    assert_formats_to(
        r#"
            fn   test(  ):
                x = 1
            end
        "#,
        r#"
            fn test():
                x = 1
            end
        "#,
    );
}

#[test]
fn test_function_with_params() {
    assert_formats_to(
        r#"
            fn   add(  int   a ,  int   b  ):
                return a + b
            end
        "#,
        r#"
            fn add(int a, int b):
                return a + b
            end
        "#,
    );
}

#[test]
fn test_function_empty_body() {
    assert_formats_to(
        r#"
            fn   test(  ):
            end
        "#,
        r#"
            fn test():
                pass
            end
        "#,
    );
}

// ───────────────────────── Tensor indexing ─────────────────────────

#[test]
fn test_tensor_index() {
    assert_formats_to(
        "x = buf[  i  ]",
        "x = buf[i]",
    );
}

#[test]
fn test_tensor_range_index() {
    assert_formats_to(
        "x = buf[  1:5  ]",
        "x = buf[1:5]",
    );
}

// ───────────────────────── Dot access ─────────────────────────

#[test]
fn test_dot_access() {
    assert_formats_to(
        "x = a.b.c",
        "x = a.b.c",
    );
}

// ───────────────────────── Noop ─────────────────────────

#[test]
fn test_pass() {
    assert_formats_to(
        r#"
            fn test():
                pass
            end
        "#,
        r#"
            fn test():
                pass
            end
        "#,
    );
}

// ───────────────────────── Return ─────────────────────────

#[test]
fn test_empty_return() {
    assert_formats_to(
        r#"
            fn test():
                return
            end
        "#,
        r#"
            fn test():
                return
            end
        "#,
    );
}

#[test]
fn test_return_value() {
    assert_formats_to(
        r#"
            fn test():
                return   x  +  1
            end
        "#,
        r#"
            fn test():
                return x + 1
            end
        "#,
    );
}

// ───────────────────────── 1D ─────────────────────────

#[test]
fn test_1d_simple() {
    assert_formats_to(
        "a = [1, 2, 3]",
        "a = [1, 2, 3]",
    );
}

#[test]
fn test_1d_extra_whitespace() {
    assert_formats_to(
        "a = [  1  ,  2  ,  3  ]",
        "a = [1, 2, 3]",
    );
}

#[test]
fn test_1d_single_element() {
    assert_formats_to(
        "a = [42]",
        "a = [42]",
    );
}

#[test]
fn test_1d_expressions() {
    assert_formats_to(
        "a = [x + 1, y * 2, z - 3]",
        "a = [x + 1, y * 2, z - 3]",
    );
}

#[test]
fn test_1d_with_tensor_type() {
    assert_formats_to(
        "tensor<int, 3> a = [1, 2, 3]",
        "tensor<int, 3> a = [1, 2, 3]",
    );
}

// ───────────────────────── 2D — inline if it fits ─────────────────────────

#[test]
fn test_2d_simple_inline() {
    assert_formats_to(
        "b = [1, 2, 3; 1, 2, 3]",
        "b = [1, 2, 3; 1, 2, 3]",
    );
}

#[test]
fn test_2d_extra_whitespace_inline() {
    assert_formats_to(
        "b = [  1 , 2 , 3 ;   1 , 2 , 3  ]",
        "b = [1, 2, 3; 1, 2, 3]",
    );
}

#[test]
fn test_2d_multiline_input_becomes_inline() {
    // Multiline input, but result is inline because of 80 chars limit
    assert_formats_to(
        r#"b = [1, 2, 3;
              1, 2, 3]"#,
        "b = [1, 2, 3; 1, 2, 3]",
    );
}

#[test]
fn test_2d_long_becomes_multiline() {
    // Long id — cannot fit in one line, must be multiline
    assert_formats_to(
    "b = [very_long_variable_name_a, very_long_variable_name_b; very_long_variable_name_c, very_long_variable_name_d]",
r#"b = [
            very_long_variable_name_a, very_long_variable_name_b;
            very_long_variable_name_c, very_long_variable_name_d
        ]"#,
    );
}

#[test]
fn test_2d_with_tensor_type() {
    assert_formats_to(
        "tensor<int, 2, 2> a = [1, 2; 3, 4]",
        "tensor<int, 2, 2> a = [1, 2; 3, 4]",
    );
}

// ───────────────────────── 3D — always multiline ─────────────────────────

#[test]
fn test_3d_simple() {
    assert_formats_to(
    "c = [1, 2; 3, 4 | 5, 6; 7, 8]",
r#"c = [
            1, 2;
            3, 4 |
            5, 6;
            7, 8
        ]"#,
    );
}

#[test]
fn test_3d_multiline_input() {
    assert_formats_to(
        r#"c = [1, 2, 3;
              1, 2, 3 |
              1, 2, 3;
              1, 2, 3
          ]"#,
        r#"c = [
            1, 2, 3;
            1, 2, 3 |
            1, 2, 3;
            1, 2, 3
        ]"#,
    );
}

#[test]
fn test_3d_with_tensor_type() {
    assert_formats_to(
        "tensor<int, 2, 2, 2> a = [1, 2; 3, 4 | 1, 2; 3, 4]",
        r#"tensor<int, 2, 2, 2> a = [
            1, 2;
            3, 4 |
            1, 2;
            3, 4
        ]"#,
    );
}

// ───────────────────────── Indent inside function ─────────────────────────

#[test]
fn test_3d_inside_function_indented() {
    // Inside: indent=1, closing bracket must be indented as well
    assert_formats_to(
        r#"
            fn test():
                c = [1, 2; 3, 4 | 5, 6; 7, 8]
            end
        "#,
        r#"
            fn test():
                c = [
                    1, 2;
                    3, 4 |
                    5, 6;
                    7, 8
                ]
            end
        "#,
    );
    let x = r#"
            fn test():
                c = [1, 2; 3, 4 | 5, 6; 7, 8]
            end
        "#;
    let parser = TmlParser::new();
    println!("{:#?}", parser.parse(x).expect("Parse failed").format(0));
}

#[test]
fn test_2d_inside_function_stays_inline() {
    assert_formats_to(
        r#"
            fn test():
                b = [1, 2; 3, 4]
            end
        "#,
        r#"
            fn test():
                b = [1, 2; 3, 4]
            end
        "#,
    );
}

// ───────────────────────── Array of arrays ─────────────────────────

#[test]
fn test_arr_of_arrays() {
    assert_formats_to(
        r#"arr_of_arrays = [[1, 2], [1, 2], [1, 2]]"#,
        r#"arr_of_arrays = [[1, 2], [1, 2], [1, 2]]"#,
    );
}

#[test]
fn test_arr_of_arrays_multiline_input() {
    assert_formats_to(
        r#"arr_of_arrays = [
            [1, 2],
            [1, 2],
            [1, 2]
        ]"#,
        r#"arr_of_arrays = [[1, 2], [1, 2], [1, 2]]"#,
    );
}

#[test]
fn test_cube_of_arrays() {
    assert_formats_to(
        r#"cube_of_arrays = [[1], [2]; [1], [2] | [1], [2]; [1], [2]]"#,
        r#"cube_of_arrays = [
            [1], [2];
            [1], [2] |
            [1], [2];
            [1], [2]
        ]"#,
    );
}

// ───────────────────────── Array of expressions ─────────────────────────

#[test]
fn test_arr_of_expressions() {
    assert_formats_to(
        r#"arr_of_arrays = [arr + 1, arr + 2, arr + 3]"#,
        r#"arr_of_arrays = [arr + 1, arr + 2, arr + 3]"#,
    );
}

// ───────────────────────── Broadcasting scalar ─────────────────────────

#[test]
fn test_broadcast_scalar() {
    // tensor<int, 3> a = 1 — not a tensor literal, just a scalar rvalue
    assert_formats_to(
        "tensor<int, 3> a = 1",
        "tensor<int, 3> a = 1",
    );
}

// ───────────────────────── Nested tensor type ─────────────────────────

#[test]
fn test_array_of_arrays_with_tensor_type() {
    assert_formats_to(
        "tensor<tensor<int, 2>, 2> a = [[1, 2], [3, 4]]",
        "tensor<tensor<int, 2>, 2> a = [[1, 2], [3, 4]]",
    );
}

#[test]
fn test_reset_state() {
    // Reset state between tests
    assert_formats_to(
        r#"a = x == 'string which should be valid'"#,
        "t.reset?.type reset_state? = 0u"
    );
}