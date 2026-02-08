# minilang

A small dynamically-typed scripting language implemented in Rust. No dependencies.

## Building

```
cargo build
```

## Usage

```
cargo run -- <file.ml>
```

## Language Overview

```python
# Variables
let x = 10
let name = "hello"
let flag = true

# Arrays
let nums = [1, 2, 3, 4]
print(nums[0])       # 1
print(len(nums))     # 4

# Functions
fn square(n) {
  return n * n
}
print(square(5))     # 25

# Control flow
if x > 5 {
  print("big")
} else {
  print("small")
}

# While loop
let i = 0
while i < 3 {
  print(i)
  i = i + 1
}

# For loop (range)
for i in 0..5 {
  print(i)
}
```

### Types

- Numbers: `42`, `3.14`
- Strings: `"hello"`
- Booleans: `true`, `false`
- Arrays: `[1, 2, 3]`
- Functions
- Null

### Operators

| Operator | Description |
|---|---|
| `+` `-` `*` `/` `%` | Arithmetic (also `+` for string/array concatenation) |
| `==` `!=` | Equality |
| `<` `<=` `>` `>=` | Comparison |
| `and` `or` `not` | Logical (short-circuit) |
| `-` (unary) | Negation |

### Built-in Functions

- `print(value)` — print a value to stdout
- `len(array)` — return the length of an array or string

### Comments

Lines starting with `#` are comments.

## Examples

See the [`examples/`](examples/) directory:

- `hello.ml` — Hello world
- `fibonacci.ml` — First 20 Fibonacci numbers
- `fizzbuzz.ml` — FizzBuzz
- `factorial.ml` — Recursive factorial
- `arrays.ml` — Map and filter with array concatenation
- `guessing.ml` — Number guessing logic
- `sorting.ml` — Bubble sort

## Grammar

The full EBNF grammar is in [SKETCH.md](SKETCH.md).
