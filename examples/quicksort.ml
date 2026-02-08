# Quicksort

fn quicksort(arr) {
  if len(arr) <= 1 {
    return arr
  }

  let pivot = arr[0]
  let left = []
  let right = []

  let i = 1
  while i < len(arr) {
    if arr[i] < pivot {
      left = left + [arr[i]]
    } else {
      right = right + [arr[i]]
    }
    i = i + 1
  }

  return quicksort(left) + [pivot] + quicksort(right)
}

let nums = [38, 27, 43, 3, 9, 82, 10]
print("Before:")
print(nums)
print("After:")
print(quicksort(nums))
