# CLAUDE.md

## Project Overview

Minilang is a small dynamically-typed scripting language with a tree-walking interpreter written in Rust. No external dependencies. Rust edition 2024.

## Build & Run

```
cargo build              # compile
cargo run -- <file.ml>   # run a minilang script
cargo run -- examples/hello.ml   # run an example
```

## Project Structure

```
src/
  main.rs          — entry point: reads file, runs lex → parse → interpret pipeline
  lexer.rs         — Token enum and Lexer (source → Vec<Token>)
  parser.rs        — AST types (Expr, Stmt) and recursive-descent Parser (tokens → AST)
  interpreter.rs   — Value enum, scope-stack Environment, tree-walking Interpreter (AST → execution)
examples/          — example .ml scripts
SKETCH.md          — EBNF grammar specification
EXAMPLE.md         — reference example with expected output
```

## Architecture

- **Lexer** scans source into tokens. Handles `#` comments, two-char operators (`==`, `!=`, `<=`, `>=`, `..`), number/string literals, and keyword lookup.
- **Parser** is recursive-descent following the grammar in SKETCH.md. Expression precedence: logic → equality → compare → term → factor → unary → call → primary.
- **Interpreter** uses a `Vec<HashMap<String, Value>>` scope stack. Functions create a new scope with params bound. Early return uses a `Signal::Return(Value)` enum. Built-ins (`print`, `len`) are handled as special cases during call evaluation.

## Language Features

- Types: Number (f64), String, Bool, Array, Function, Null
- Operators: arithmetic, string/array concatenation with `+`, comparisons, logical `and`/`or`/`not`
- Statements: `let`, assignment, index assignment, `if`/`else`, `while`, `for..in` (range), `fn`, `return`
- Built-ins: `print(value)`, `len(array|string)`

## Testing

Run the reference example to verify correctness:

```
cargo run -- examples/hello.ml     # expected: Hello, world!
```

Save EXAMPLE.md's code as a .ml file and run it — expected output is `103` then `10`.
