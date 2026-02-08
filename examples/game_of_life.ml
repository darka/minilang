# Conway's Game of Life
#
# A cellular automaton on a grid where each cell is alive or dead.
# Rules (applied simultaneously to every cell each generation):
#   1. Live cell with 2 or 3 neighbours survives
#   2. Dead cell with exactly 3 neighbours becomes alive
#   3. All other cells die or stay dead
#
# The grid is stored as a flat array of size rows*cols.
# Index (r, c) = r * cols + c
#
# Note: minilang copies arrays into function scopes, so we use
# direct index assignment instead of setter helpers.

let rows = 10
let cols = 20

# Create a blank grid (all dead = 0)
fn make_grid() {
  let g = []
  let i = 0
  while i < rows * cols {
    g = g + [0]
    i = i + 1
  }
  return g
}

fn get(grid, r, c) {
  if r < 0 or r >= rows or c < 0 or c >= cols {
    return 0
  }
  return grid[r * cols + c]
}

# Count live neighbours of cell (r, c)
fn neighbours(grid, r, c) {
  let count = 0
  let dr = -1
  while dr <= 1 {
    let dc = -1
    while dc <= 1 {
      if not (dr == 0 and dc == 0) {
        count = count + get(grid, r + dr, c + dc)
      }
      dc = dc + 1
    }
    dr = dr + 1
  }
  return count
}

# Advance the grid by one generation
fn step(grid) {
  let next = make_grid()
  let r = 0
  while r < rows {
    let c = 0
    while c < cols {
      let n = neighbours(grid, r, c)
      let alive = get(grid, r, c)
      if alive == 1 and (n == 2 or n == 3) {
        next[r * cols + c] = 1
      }
      if alive == 0 and n == 3 {
        next[r * cols + c] = 1
      }
      c = c + 1
    }
    r = r + 1
  }
  return next
}

# Print the grid using # for alive, . for dead
fn print_grid(grid) {
  let r = 0
  while r < rows {
    let line = ""
    let c = 0
    while c < cols {
      if grid[r * cols + c] == 1 {
        line = line + "#"
      } else {
        line = line + "."
      }
      c = c + 1
    }
    print(line)
    r = r + 1
  }
}

# --- Demo: Glider ---

print("=== Conway's Game of Life ===")
print("")
print("Pattern: Glider (moves down-right)")
print("")

let grid = make_grid()

# Classic glider pattern starting near top-left:
#   .#.
#   ..#
#   ###
grid[1 * cols + 1] = 1
grid[2 * cols + 2] = 1
grid[3 * cols + 0] = 1
grid[3 * cols + 1] = 1
grid[3 * cols + 2] = 1

# Run for 20 generations, printing every 4th
let gen = 0
while gen <= 20 {
  if gen % 4 == 0 {
    print("--- Generation:")
    print(gen)
    print_grid(grid)
    print("")
  }
  grid = step(grid)
  gen = gen + 1
}

# --- Demo: Blinker (period-2 oscillator) ---

print("=== Blinker Oscillator ===")
print("")

rows = 5
cols = 5
grid = make_grid()

# Horizontal blinker in the center
grid[2 * cols + 1] = 1
grid[2 * cols + 2] = 1
grid[2 * cols + 3] = 1

gen = 0
while gen < 4 {
  print("--- Generation:")
  print(gen)
  print_grid(grid)
  print("")
  grid = step(grid)
  gen = gen + 1
}
