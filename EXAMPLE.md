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
