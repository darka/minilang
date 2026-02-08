# Brainfuck Interpreter
#
# Interprets Brainfuck programs represented as arrays of instruction
# codes. Since minilang has no string indexing or char codes, we use
# integer constants for the 8 Brainfuck commands:
#
#   1 = >   (move pointer right)
#   2 = <   (move pointer left)
#   3 = +   (increment cell)
#   4 = -   (decrement cell)
#   5 = .   (output cell value)
#   6 = ,   (input — not supported, treated as no-op)
#   7 = [   (jump past matching ] if cell is zero)
#   8 = ]   (jump back to matching [ if cell is nonzero)
#
# Output is collected as an array of numbers. To make results
# readable, we also provide an ASCII lookup table for common values.

let GT  = 1
let LT  = 2
let ADD = 3
let SUB = 4
let DOT = 5
let COM = 6
let OBR = 7
let CBR = 8

# ASCII lookup for printable characters (32..126)
# We store characters as strings at index = ascii code
fn make_ascii_table() {
  let t = []
  # Fill 0..31 with "?"
  let i = 0
  while i < 32 {
    t = t + ["?"]
    i = i + 1
  }
  # 32 = space
  t = t + [" "]  # 32
  t = t + ["!"]  # 33
  t = t + ["?"]  # 34 (double-quote, not representable)
  t = t + ["#"]  # 35
  t = t + ["$"]  # 36
  t = t + ["%"]  # 37
  t = t + ["&"]  # 38
  t = t + ["'"]  # 39
  t = t + ["("]  # 40
  t = t + [")"]  # 41
  t = t + ["*"]  # 42
  t = t + ["+"]  # 43
  t = t + [","]  # 44
  t = t + ["-"]  # 45
  t = t + ["."]  # 46
  t = t + ["/"]  # 47
  # 48..57 = digits 0-9
  t = t + ["0"]  # 48
  t = t + ["1"]  # 49
  t = t + ["2"]  # 50
  t = t + ["3"]  # 51
  t = t + ["4"]  # 52
  t = t + ["5"]  # 53
  t = t + ["6"]  # 54
  t = t + ["7"]  # 55
  t = t + ["8"]  # 56
  t = t + ["9"]  # 57
  t = t + [":"]  # 58
  t = t + [";"]  # 59
  t = t + ["<"]  # 60
  t = t + ["="]  # 61
  t = t + [">"]  # 62
  t = t + ["?"]  # 63
  t = t + ["@"]  # 64
  # 65..90 = A-Z
  t = t + ["A"]  # 65
  t = t + ["B"]  # 66
  t = t + ["C"]  # 67
  t = t + ["D"]  # 68
  t = t + ["E"]  # 69
  t = t + ["F"]  # 70
  t = t + ["G"]  # 71
  t = t + ["H"]  # 72
  t = t + ["I"]  # 73
  t = t + ["J"]  # 74
  t = t + ["K"]  # 75
  t = t + ["L"]  # 76
  t = t + ["M"]  # 77
  t = t + ["N"]  # 78
  t = t + ["O"]  # 79
  t = t + ["P"]  # 80
  t = t + ["Q"]  # 81
  t = t + ["R"]  # 82
  t = t + ["S"]  # 83
  t = t + ["T"]  # 84
  t = t + ["U"]  # 85
  t = t + ["V"]  # 86
  t = t + ["W"]  # 87
  t = t + ["X"]  # 88
  t = t + ["Y"]  # 89
  t = t + ["Z"]  # 90
  t = t + ["["]  # 91
  t = t + ["?"]  # 92 (backslash, not representable)
  t = t + ["]"]  # 93
  t = t + ["^"]  # 94
  t = t + ["_"]  # 95
  t = t + ["`"]  # 96
  # 97..122 = a-z
  t = t + ["a"]  # 97
  t = t + ["b"]  # 98
  t = t + ["c"]  # 99
  t = t + ["d"]  # 100
  t = t + ["e"]  # 101
  t = t + ["f"]  # 102
  t = t + ["g"]  # 103
  t = t + ["h"]  # 104
  t = t + ["i"]  # 105
  t = t + ["j"]  # 106
  t = t + ["k"]  # 107
  t = t + ["l"]  # 108
  t = t + ["m"]  # 109
  t = t + ["n"]  # 110
  t = t + ["o"]  # 111
  t = t + ["p"]  # 112
  t = t + ["q"]  # 113
  t = t + ["r"]  # 114
  t = t + ["s"]  # 115
  t = t + ["t"]  # 116
  t = t + ["u"]  # 117
  t = t + ["v"]  # 118
  t = t + ["w"]  # 119
  t = t + ["x"]  # 120
  t = t + ["y"]  # 121
  t = t + ["z"]  # 122
  return t
}

let ascii = make_ascii_table()

# Convert output array (numeric ASCII codes) to a string
fn to_string(output) {
  let s = ""
  let i = 0
  while i < len(output) {
    let code = output[i]
    if code >= 32 and code < len(ascii) {
      s = s + ascii[code]
    } else {
      s = s + "?"
    }
    i = i + 1
  }
  return s
}

# Run a Brainfuck program
# Returns the output as an array of numeric values
fn bf_run(program) {
  let tape_size = 256
  let tape = []
  let i = 0
  while i < tape_size {
    tape = tape + [0]
    i = i + 1
  }

  let ptr = 0          # data pointer
  let pc = 0           # program counter
  let output = []
  let prog_len = len(program)

  while pc < prog_len {
    let cmd = program[pc]

    if cmd == GT {
      ptr = ptr + 1
    }
    if cmd == LT {
      ptr = ptr - 1
    }
    if cmd == ADD {
      tape[ptr] = tape[ptr] + 1
    }
    if cmd == SUB {
      tape[ptr] = tape[ptr] - 1
    }
    if cmd == DOT {
      output = output + [tape[ptr]]
    }
    if cmd == OBR {
      if tape[ptr] == 0 {
        # Jump forward to matching ]
        let depth = 1
        while depth > 0 {
          pc = pc + 1
          if program[pc] == OBR {
            depth = depth + 1
          }
          if program[pc] == CBR {
            depth = depth - 1
          }
        }
      }
    }
    if cmd == CBR {
      if tape[ptr] != 0 {
        # Jump back to matching [
        let depth = 1
        while depth > 0 {
          pc = pc - 1
          if program[pc] == CBR {
            depth = depth + 1
          }
          if program[pc] == OBR {
            depth = depth - 1
          }
        }
      }
    }

    pc = pc + 1
  }

  return output
}

# --- Demo ---

print("=== Brainfuck Interpreter ===")
print("")

# Program 1: "Hello World!"
# BF: ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
# Encoded as integer instruction array:

let hello = [
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD,                         # ++++++++ (set cell 0 to 8)
  OBR,                                                            # [
    GT, ADD, ADD, ADD, ADD,                                       #   >++++
    OBR,                                                          #   [
      GT, ADD, ADD,                                               #     >++
      GT, ADD, ADD, ADD,                                          #     >+++
      GT, ADD, ADD, ADD,                                          #     >+++
      GT, ADD,                                                    #     >+
      LT, LT, LT, LT, SUB,                                      #     <<<<-
    CBR,                                                          #   ]
    GT, ADD,                                                      #   >+
    GT, ADD,                                                      #   >+
    GT, SUB,                                                      #   >-
    GT, GT, ADD,                                                  #   >>+
    OBR, LT, CBR,                                                 #   [<]
    LT, SUB,                                                      #   <-
  CBR,                                                            # ]
  GT, GT, DOT,                                                    # >>.
  GT, SUB, SUB, SUB, DOT,                                         # >---.
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, DOT, DOT,                    # +++++++..
  ADD, ADD, ADD, DOT,                                             # +++.
  GT, GT, DOT,                                                    # >>.
  LT, SUB, DOT,                                                  # <-.
  LT, DOT,                                                        # <.
  ADD, ADD, ADD, DOT,                                             # +++.
  SUB, SUB, SUB, SUB, SUB, SUB, DOT,                              # ------.
  SUB, SUB, SUB, SUB, SUB, SUB, SUB, SUB, DOT,                    # --------.
  GT, GT, ADD, DOT,                                               # >>+.
  GT, ADD, ADD, DOT                                               # >++.
]

print("Program 1: Hello World")
let result = bf_run(hello)
print("  Raw output (ASCII codes):")
print(result)
print("  As text:")
print(to_string(result))
print("")

# Program 2: Add two numbers (3 + 5 = 8)
# Cell 0 = 3, Cell 1 = 5, then move cell 1 into cell 0
# BF: +++>+++++[<+>-]<   (result in cell 0, then output value)
# We add 48 to convert to ASCII digit for display

let add = [
  ADD, ADD, ADD,                          # +++ (cell 0 = 3)
  GT,                                     # >
  ADD, ADD, ADD, ADD, ADD,                # +++++ (cell 1 = 5)
  OBR,                                    # [
    LT, ADD, GT, SUB,                    #   <+>-
  CBR,                                    # ]
  LT,                                     # < (back to cell 0, now 8)
  # Add 48 to get ASCII '8' (= 56)
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 8
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 16
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 24
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 32
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 40
  ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, # 48
  DOT                                     # . (output '8')
]

print("Program 2: Add 3 + 5")
result = bf_run(add)
print("  Raw output:")
print(result)
print("  As text:")
print(to_string(result))
print("")

# Program 3: Count from 1 to 5
# Output ASCII '1' through '5' with spaces between them
# Start with 49 (ASCII '1'), print, add space, increment, repeat

let count = [
  # Set cell 0 to 49 (ASCII '1'): 6*8+1
  ADD, ADD, ADD, ADD, ADD, ADD,           # ++++++ (6)
  OBR, GT, ADD, ADD, ADD, ADD, ADD, ADD, ADD, ADD, LT, SUB, CBR,  # [>++++++++<-] (cell 1 = 48)
  GT, ADD,                                 # >+ (cell 1 = 49 = '1')
  DOT,                                     # . (print '1')
  ADD, DOT,                               # +. (print '2')
  ADD, DOT,                               # +. (print '3')
  ADD, DOT,                               # +. (print '4')
  ADD, DOT                                # +. (print '5')
]

print("Program 3: Count 1 to 5")
result = bf_run(count)
print("  Raw output:")
print(result)
print("  As text:")
print(to_string(result))
