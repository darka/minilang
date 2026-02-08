# Bubble sort

fn bubble_sort(arr) {
  let n = len(arr)
  let i = 0
  while i < n {
    let j = 0
    while j < n - i - 1 {
      if arr[j] > arr[j + 1] {
        let temp = arr[j]
        arr[j] = arr[j + 1]
        arr[j + 1] = temp
      }
      j = j + 1
    }
    i = i + 1
  }
  return arr
}

let nums = [64, 34, 25, 12, 22, 11, 90]
print("Before:")
print(nums)
print("After:")
print(bubble_sort(nums))
