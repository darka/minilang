# Print the first 20 Fibonacci numbers

let a = 0
let b = 1

for i in 0..20 {
  print(a)
  let temp = a + b
  a = b
  b = temp
}
