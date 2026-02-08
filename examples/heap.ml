# Binary Heap Priority Queue (Min-Heap)
#
# A min-heap stored as an array where the smallest element
# is always at the root. Supports push and pop operations.
#
# For node at index i:
#   parent:      floor((i-1) / 2)
#   left child:  2*i + 1
#   right child: 2*i + 2

# Heap state: array + logical size
let heap = []
let heap_size = 0

# Floor-division by 2 (minilang uses float division)
fn div2(n) {
  return (n - n % 2) / 2
}

fn heap_swap(i, j) {
  let tmp = heap[i]
  heap[i] = heap[j]
  heap[j] = tmp
}

# Restore heap property upward from index i
fn sift_up(i) {
  while i > 0 {
    let p = div2(i - 1)
    if heap[i] < heap[p] {
      heap_swap(i, p)
      i = p
    } else {
      return
    }
  }
}

# Restore heap property downward from index i
fn sift_down(i) {
  while true {
    let smallest = i
    let left = 2 * i + 1
    let right = 2 * i + 2

    if left < heap_size and heap[left] < heap[smallest] {
      smallest = left
    }
    if right < heap_size and heap[right] < heap[smallest] {
      smallest = right
    }

    if smallest != i {
      heap_swap(i, smallest)
      i = smallest
    } else {
      return
    }
  }
}

# Push a value onto the heap
fn heap_push(value) {
  if heap_size < len(heap) {
    heap[heap_size] = value
  } else {
    heap = heap + [value]
  }
  heap_size = heap_size + 1
  sift_up(heap_size - 1)
}

# Pop the minimum value from the heap
fn heap_pop() {
  let min_val = heap[0]
  heap_size = heap_size - 1
  heap[0] = heap[heap_size]
  sift_down(0)
  return min_val
}

# Peek at the minimum without removing it
fn heap_peek() {
  return heap[0]
}

# Print the logical contents of the heap
fn heap_print() {
  let result = []
  let i = 0
  while i < heap_size {
    result = result + [heap[i]]
    i = i + 1
  }
  print(result)
}

# --- Demo ---

print("=== Binary Heap Priority Queue ===")
print("")

# Push elements in arbitrary order
print("Pushing: 42, 15, 28, 7, 33, 10, 3, 50")
heap_push(42)
heap_push(15)
heap_push(28)
heap_push(7)
heap_push(33)
heap_push(10)
heap_push(3)
heap_push(50)

print("Heap after pushes:")
heap_print()
print("")

# Peek at the minimum
print("Peek (min):")
print(heap_peek())
print("")

# Pop all elements — should come out in sorted order
print("Popping all elements (sorted order):")
let sorted = []
while heap_size > 0 {
  let val = heap_pop()
  sorted = sorted + [val]
}
print(sorted)
print("")

# Demonstrate using the heap as a task scheduler
print("=== Task Scheduler (by priority) ===")
print("")

# Lower number = higher priority
# Push tasks with priorities: 5, 1, 3, 4, 2
heap_push(5)
heap_push(1)
heap_push(3)
heap_push(4)
heap_push(2)

print("Processing tasks in priority order:")
let order = []
while heap_size > 0 {
  order = order + [heap_pop()]
}
print(order)
