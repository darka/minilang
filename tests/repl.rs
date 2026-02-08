use std::process::{Command, Stdio};
use std::io::Write;

fn repl(input: &str) -> (String, String, bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_minilang"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start minilang");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

#[test]
fn repl_banner() {
    let (stdout, _, ok) = repl("");
    assert!(ok);
    assert!(stdout.contains("minilang REPL"));
}

#[test]
fn repl_prompt() {
    let (stdout, _, _) = repl("");
    assert!(stdout.contains(">> "));
}

#[test]
fn repl_print() {
    let (stdout, _, ok) = repl("print(42)\n");
    assert!(ok);
    assert!(stdout.contains("42"));
}

#[test]
fn repl_variable_persistence() {
    let (stdout, _, ok) = repl("let x = 10\nprint(x)\n");
    assert!(ok);
    assert!(stdout.contains("10"));
}

#[test]
fn repl_function_persistence() {
    let (stdout, _, ok) = repl("fn double(n) { return n * 2 }\nprint(double(7))\n");
    assert!(ok);
    assert!(stdout.contains("14"));
}

#[test]
fn repl_multiple_outputs() {
    let (stdout, _, ok) = repl("print(1)\nprint(2)\nprint(3)\n");
    assert!(ok);
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(stdout.contains("3"));
}

#[test]
fn repl_error_recovers() {
    let (stdout, stderr, ok) = repl("print(noSuchVar)\nprint(99)\n");
    assert!(ok);
    assert!(stderr.contains("Undefined variable"));
    assert!(stdout.contains("99"));
}

#[test]
fn repl_parse_error_recovers() {
    let (stdout, stderr, ok) = repl(")\nprint(1)\n");
    assert!(ok);
    assert!(stderr.contains("Parse error") || stderr.contains("Unexpected token"));
    assert!(stdout.contains("1"));
}

#[test]
fn repl_empty_lines_skipped() {
    let (stdout, _, ok) = repl("\n\n\nprint(5)\n");
    assert!(ok);
    assert!(stdout.contains("5"));
}

#[test]
fn repl_mutation_across_lines() {
    let (stdout, _, ok) = repl("let x = 1\nx = 2\nprint(x)\n");
    assert!(ok);
    assert!(stdout.contains("2"));
}

#[test]
fn repl_exit_on_eof() {
    // No trailing newline - just EOF
    let (_, _, ok) = repl("");
    assert!(ok);
}
