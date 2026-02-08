# Array manipulation

fn map_double(arr) {
  let result = []
  let i = 0
  while i < len(arr) {
    result = result + [arr[i] * 2]
    i = i + 1
  }
  return result
}

fn filter_even(arr) {
  let result = []
  let i = 0
  while i < len(arr) {
    if arr[i] % 2 == 0 {
      result = result + [arr[i]]
    }
    i = i + 1
  }
  return result
}

let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

print("Original:")
print(nums)

print("Doubled:")
print(map_double(nums))

print("Evens only:")
print(filter_even(nums))
