# Z Algorithm for Pattern Matching
#
# The Z algorithm builds a Z-array for a string S where Z[i] is the
# length of the longest substring starting at i that matches a prefix
# of S. By concatenating pattern + sentinel + text, occurrences of the
# pattern in the text can be found in O(n + m) time.
#
# Since minilang has no string indexing, we represent strings as arrays
# of character codes (integers).

# Build the Z-array for a given array
fn z_array(s) {
  let n = len(s)
  let z = []

  # Initialize z with zeroes
  let i = 0
  while i < n {
    z = z + [0]
    i = i + 1
  }

  # Z[0] is defined as 0 (or n) by convention
  z[0] = 0

  # [l, r] tracks the rightmost Z-box found so far
  let l = 0
  let r = 0

  i = 1
  while i < n {
    if i < r {
      # We're inside a known Z-box; copy the value, capped at box boundary
      let copy = z[i - l]
      let remaining = r - i
      if copy < remaining {
        z[i] = copy
      } else {
        z[i] = remaining
      }
    }

    # Try to extend match from z[i] onward
    while i + z[i] < n and s[z[i]] == s[i + z[i]] {
      z[i] = z[i] + 1
    }

    # Update [l, r] if this Z-box extends further right
    if i + z[i] > r {
      l = i
      r = i + z[i]
    }

    i = i + 1
  }

  return z
}

# Search for all occurrences of pattern in text.
# Returns an array of starting indices where pattern appears.
fn z_search(pattern, text) {
  let m = len(pattern)
  let n = len(text)

  # Build concatenation: pattern + [-1] + text
  # -1 acts as sentinel (won't match any character)
  let concat = pattern + [-1] + text
  let z = z_array(concat)

  let matches = []
  let i = m + 1
  while i < len(concat) {
    if z[i] == m {
      matches = matches + [i - m - 1]
    }
    i = i + 1
  }

  return matches
}

# Helper: convert a string to an array of character codes
# (Using simple mapping: a=1, b=2, ..., z=26, space=0)
fn to_codes(chars) {
  return chars  # We'll pass arrays directly
}

# Helper: print match positions visually
fn show_matches(text, pattern, positions) {
  print("  Text length:")
  print(len(text))
  print("  Pattern length:")
  print(len(pattern))
  print("  Matches at indices:")
  print(positions)
}

# --- Demo ---

print("=== Z Algorithm - Pattern Matching ===")
print("")

# Example 1: Simple pattern search
# text:    a a b a a b a a b
# pattern: a a b
# Using: a=1, b=2
let text1    = [1, 1, 2, 1, 1, 2, 1, 1, 2]
let pattern1 = [1, 1, 2]

print("Example 1: Find [1,1,2] in [1,1,2,1,1,2,1,1,2]")
let result1 = z_search(pattern1, text1)
show_matches(text1, pattern1, result1)
print("  Expected: [0, 3, 6]")
print("")

# Example 2: Overlapping matches
# text:    a a a a a
# pattern: a a
let text2    = [1, 1, 1, 1, 1]
let pattern2 = [1, 1]

print("Example 2: Find [1,1] in [1,1,1,1,1]")
let result2 = z_search(pattern2, text2)
show_matches(text2, pattern2, result2)
print("  Expected: [0, 1, 2, 3]")
print("")

# Example 3: No match
let text3    = [1, 2, 3, 4, 5]
let pattern3 = [6, 7]

print("Example 3: Find [6,7] in [1,2,3,4,5]")
let result3 = z_search(pattern3, text3)
show_matches(text3, pattern3, result3)
print("  Expected: []")
print("")

# Example 4: Show the Z-array itself
let s = [1, 1, 2, 1, 1, 2, 1, 1]
print("Example 4: Z-array of [1,1,2,1,1,2,1,1]")
let z = z_array(s)
print("  Z-array: ")
print(z)
print("  Expected: [0, 1, 0, 5, 1, 0, 2, 1]")
