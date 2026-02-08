use minilang::interpreter::Interpreter;
use minilang::lexer::Lexer;
use minilang::parser::Parser;

fn run(source: &str) -> Result<Vec<String>, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;
    Ok(interpreter.output)
}

fn run_ok(source: &str) -> Vec<String> {
    run(source).expect("expected program to succeed")
}

fn run_err(source: &str) -> String {
    run(source).expect_err("expected program to fail")
}

// ===== Arithmetic & Operators =====

#[test]
fn arithmetic_add() {
    assert_eq!(run_ok("print(2 + 3)"), vec!["5"]);
}

#[test]
fn arithmetic_sub() {
    assert_eq!(run_ok("print(10 - 4)"), vec!["6"]);
}

#[test]
fn arithmetic_mul() {
    assert_eq!(run_ok("print(3 * 7)"), vec!["21"]);
}

#[test]
fn arithmetic_div() {
    assert_eq!(run_ok("print(15 / 4)"), vec!["3.75"]);
}

#[test]
fn arithmetic_mod() {
    assert_eq!(run_ok("print(10 % 3)"), vec!["1"]);
}

#[test]
fn arithmetic_precedence() {
    assert_eq!(run_ok("print(2 + 3 * 4)"), vec!["14"]);
}

#[test]
fn arithmetic_unary_neg() {
    assert_eq!(run_ok("print(-5)"), vec!["-5"]);
}

#[test]
fn arithmetic_integer_display() {
    assert_eq!(run_ok("print(42 + 0)"), vec!["42"]);
}

// ===== Booleans & Logic =====

#[test]
fn bool_literals() {
    assert_eq!(run_ok("print(true)\nprint(false)"), vec!["true", "false"]);
}

#[test]
fn bool_comparisons() {
    assert_eq!(run_ok("print(3 < 5)\nprint(5 < 3)"), vec!["true", "false"]);
}

#[test]
fn bool_equality() {
    assert_eq!(run_ok("print(1 == 1)\nprint(1 != 2)"), vec!["true", "true"]);
}

#[test]
fn bool_and_or() {
    assert_eq!(
        run_ok("print(true and false)\nprint(true or false)"),
        vec!["false", "true"]
    );
}

#[test]
fn bool_not() {
    assert_eq!(run_ok("print(not true)\nprint(not false)"), vec!["false", "true"]);
}

// ===== Strings =====

#[test]
fn string_literal() {
    assert_eq!(run_ok("print(\"hello\")"), vec!["hello"]);
}

#[test]
fn string_concatenation() {
    assert_eq!(run_ok("print(\"foo\" + \"bar\")"), vec!["foobar"]);
}

#[test]
fn string_len() {
    assert_eq!(run_ok("print(len(\"hello\"))"), vec!["5"]);
}

// ===== Arrays =====

#[test]
fn array_literal() {
    assert_eq!(run_ok("print([1, 2, 3])"), vec!["[1, 2, 3]"]);
}

#[test]
fn array_indexing() {
    assert_eq!(run_ok("let a = [10, 20, 30]\nprint(a[1])"), vec!["20"]);
}

#[test]
fn array_index_assign() {
    assert_eq!(
        run_ok("let a = [1, 2, 3]\na[1] = 99\nprint(a)"),
        vec!["[1, 99, 3]"]
    );
}

#[test]
fn array_concat() {
    assert_eq!(run_ok("print([1, 2] + [3, 4])"), vec!["[1, 2, 3, 4]"]);
}

#[test]
fn array_len() {
    assert_eq!(run_ok("print(len([10, 20, 30]))"), vec!["3"]);
}

// ===== Variables & Scoping =====

#[test]
fn var_let() {
    assert_eq!(run_ok("let x = 42\nprint(x)"), vec!["42"]);
}

#[test]
fn var_assign() {
    assert_eq!(run_ok("let x = 1\nx = 2\nprint(x)"), vec!["2"]);
}

#[test]
fn var_block_shadow() {
    assert_eq!(
        run_ok("let x = 1\nif true {\n  let x = 99\n  print(x)\n}\nprint(x)"),
        vec!["99", "1"]
    );
}

#[test]
fn var_outer_mutation() {
    assert_eq!(
        run_ok("let x = 1\nif true {\n  x = 99\n}\nprint(x)"),
        vec!["99"]
    );
}

#[test]
fn var_undefined_error() {
    let err = run_err("print(noSuchVar)");
    assert!(err.contains("Undefined variable"));
}

#[test]
fn var_null() {
    assert_eq!(
        run_ok("fn f() { return }\nprint(f())"),
        vec!["null"]
    );
}

// ===== Control Flow =====

#[test]
fn if_true_branch() {
    assert_eq!(
        run_ok("if true { print(1) } else { print(2) }"),
        vec!["1"]
    );
}

#[test]
fn if_false_branch() {
    assert_eq!(
        run_ok("if false { print(1) } else { print(2) }"),
        vec!["2"]
    );
}

#[test]
fn if_no_else() {
    assert_eq!(run_ok("if false { print(1) }"), Vec::<String>::new());
}

#[test]
fn while_loop() {
    assert_eq!(
        run_ok("let i = 0\nwhile i < 3 {\n  print(i)\n  i = i + 1\n}"),
        vec!["0", "1", "2"]
    );
}

#[test]
fn for_range() {
    assert_eq!(
        run_ok("for i in 0..4 { print(i) }"),
        vec!["0", "1", "2", "3"]
    );
}

#[test]
fn for_empty_range() {
    assert_eq!(run_ok("for i in 5..5 { print(i) }"), Vec::<String>::new());
}

// ===== Functions =====

#[test]
fn fn_basic_call() {
    assert_eq!(
        run_ok("fn greet() { print(42) }\ngreet()"),
        vec!["42"]
    );
}

#[test]
fn fn_params() {
    assert_eq!(
        run_ok("fn add(a, b) { print(a + b) }\nadd(3, 4)"),
        vec!["7"]
    );
}

#[test]
fn fn_return_value() {
    assert_eq!(
        run_ok("fn double(x) { return x * 2 }\nprint(double(5))"),
        vec!["10"]
    );
}

#[test]
fn fn_recursion() {
    assert_eq!(
        run_ok(
            "fn fib(n) {\n  if n <= 1 { return n }\n  return fib(n - 1) + fib(n - 2)\n}\nprint(fib(7))"
        ),
        vec!["13"]
    );
}

#[test]
fn fn_implicit_null_return() {
    assert_eq!(
        run_ok("fn noop() { let x = 1 }\nprint(noop())"),
        vec!["null"]
    );
}

#[test]
fn fn_early_return() {
    assert_eq!(
        run_ok("fn f(x) {\n  if x > 0 { return 1 }\n  return 0\n}\nprint(f(5))\nprint(f(-1))"),
        vec!["1", "0"]
    );
}

#[test]
fn fn_wrong_arg_count() {
    let err = run_err("fn f(a, b) { return a + b }\nf(1)");
    assert!(err.contains("Expected 2 arguments, got 1"));
}

// ===== Built-ins =====

#[test]
fn builtin_print_types() {
    assert_eq!(
        run_ok("print(42)\nprint(\"hi\")\nprint(true)"),
        vec!["42", "hi", "true"]
    );
}

#[test]
fn builtin_len_type_error() {
    let err = run_err("len(42)");
    assert!(err.contains("len() requires array or string"));
}

#[test]
fn builtin_len_arg_count() {
    let err = run_err("len([1], [2])");
    assert!(err.contains("len() takes exactly 1 argument"));
}

// ===== Error Handling =====

#[test]
fn error_division_by_zero() {
    // Rust f64 division by zero produces infinity, not an error
    let out = run_ok("print(1 / 0)");
    assert_eq!(out, vec!["inf"]);
}

#[test]
fn error_type_error_arithmetic() {
    let err = run_err("let x = 1 + true");
    assert!(err.contains("requires two numbers"));
}

#[test]
fn error_type_error_comparison() {
    let err = run_err("let x = \"a\" < 1");
    assert!(err.contains("requires two numbers"));
}

#[test]
fn error_index_out_of_bounds() {
    let err = run_err("let a = [1, 2]\nprint(a[5])");
    assert!(err.contains("out of bounds"));
}

#[test]
fn error_call_non_function() {
    let err = run_err("let x = 5\nx()");
    assert!(err.contains("non-function"));
}

// ===== Parser Robustness =====

#[test]
fn parser_empty_program() {
    assert_eq!(run_ok(""), Vec::<String>::new());
}

#[test]
fn parser_comments_only() {
    assert_eq!(run_ok("# just a comment\n# another one"), Vec::<String>::new());
}

#[test]
fn parser_unterminated_string() {
    let err = run_err("print(\"hello)");
    assert!(err.contains("Unterminated string"));
}

#[test]
fn parser_unexpected_token() {
    let err = run_err(")");
    assert!(err.contains("Unexpected token"));
}

// ===== Regression: EXAMPLE.md =====

#[test]
fn reference_program() {
    let source = r#"
# comments start with #

let x = 10
let y = 3

fn pow2(n) {
  return n * n
}

if x > y {
  print(pow2(x) + y)
} else {
  print(0)
}

let nums = [1, 2, 3, 4]
let i = 0
let sum = 0

while i < len(nums) {
  sum = sum + nums[i]
  i = i + 1
}

print(sum)
"#;
    assert_eq!(run_ok(source), vec!["103", "10"]);
}
