use rustemo::Parser;
use tml_parser::tml::TmlParser;
use tml_tools::formatter::Format;

fn parse_and_format(src: &str) -> String {
    let parser = TmlParser::new();
    let ast = parser.parse(src).expect("Parse failed");
    ast.format(0)
}

// ───────────────────────── Helper ─────────────────────────

/// Normalizuje whitespace za poredjenje — uklanja
/// visak razmaka i prazne linije da test ne bude krhak.
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