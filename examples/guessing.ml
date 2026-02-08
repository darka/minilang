# Simple number guessing logic
# Demonstrates nested ifs, booleans, and functions

fn check_guess(secret, guess) {
  if guess == secret {
    print("Correct!")
    return true
  }
  if guess < secret {
    print("Too low")
  } else {
    print("Too high")
  }
  return false
}

let secret = 42

check_guess(secret, 20)
check_guess(secret, 60)
check_guess(secret, 35)
check_guess(secret, 42)
