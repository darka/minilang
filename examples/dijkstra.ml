# Dijkstra's Shortest Path Algorithm
#
# Finds shortest paths from a source node to all other nodes in a
# weighted directed graph. Uses a simple linear scan to extract the
# minimum-distance unvisited node (no priority queue needed).
#
# Graph representation:
#   edges[i]   = array of neighbour nodes for node i
#   weights[i] = array of edge weights (parallel to edges[i])

let INF = 999999

# Build a sample graph with 6 nodes (0..5):
#
#        7     9
#   0 -------> 1 -------> 2
#   |         / \         ^
#   |14    10/   \15      |
#   v      v     v        |
#   5      4 ---> 3       |
#   \      ^  11  |       |
#    \  2 /       | 6     | 9
#     \  /        v       |
#      >--------> 2 <-----+
#
#   0 -> 1 (7), 0 -> 5 (14)
#   1 -> 2 (9), 1 -> 3 (15), 1 -> 4 (10)
#   3 -> 2 (6)
#   4 -> 3 (11)
#   5 -> 4 (2)

let num_nodes = 6

# Adjacency lists: edges[node] and weights[node]
let edges = [
  [1, 5],
  [2, 3, 4],
  [],
  [2],
  [3],
  [4]
]

let edge_weights = [
  [7, 14],
  [9, 15, 10],
  [],
  [6],
  [11],
  [2]
]

fn dijkstra(source) {
  # Distance from source to each node
  let dist = []
  # Previous node on shortest path (-1 = none)
  let prev = []
  # Visited flag
  let visited = []

  let i = 0
  while i < num_nodes {
    dist = dist + [INF]
    prev = prev + [-1]
    visited = visited + [0]
    i = i + 1
  }

  dist[source] = 0

  # Process all nodes
  let processed = 0
  while processed < num_nodes {
    # Find unvisited node with smallest distance
    let u = -1
    let min_dist = INF + 1
    let i = 0
    while i < num_nodes {
      if visited[i] == 0 and dist[i] < min_dist {
        min_dist = dist[i]
        u = i
      }
      i = i + 1
    }

    if u == -1 {
      # Remaining nodes are unreachable
      return dist
    }

    visited[u] = 1

    # Relax edges from u
    let j = 0
    while j < len(edges[u]) {
      let v = edges[u][j]
      let w = edge_weights[u][j]
      let alt = dist[u] + w
      if alt < dist[v] {
        dist[v] = alt
        prev[v] = u
      }
      j = j + 1
    }

    processed = processed + 1
  }

  return dist
}

# --- Demo ---

print("=== Dijkstra's Shortest Path ===")
print("")
print("Graph (6 nodes, directed edges):")
print("  0 --(7)--> 1 --(9)--> 2")
print("  0 --(14)-> 5")
print("  1 --(15)-> 3")
print("  1 --(10)-> 4")
print("  3 --(6)--> 2")
print("  4 --(11)-> 3")
print("  5 --(2)--> 4")
print("")

let source = 0
print("Source node:")
print(source)
print("")

let distances = dijkstra(source)

print("Shortest distances from node 0:")
let node = 0
while node < num_nodes {
  print("  to node:")
  print(node)
  if distances[node] == INF {
    print("  distance: unreachable")
  } else {
    print("  distance:")
    print(distances[node])
  }
  print("")
  node = node + 1
}

# Expected results:
#   0 -> 0: 0
#   0 -> 1: 7
#   0 -> 2: 16 (0->1->2 = 7+9)
#   0 -> 3: 22 (0->1->3 = 7+15)
#   0 -> 4: 16 (0->5->4 = 14+2)
#   0 -> 5: 14 (0->5)

print("Expected: [0, 7, 16, 22, 16, 14]")
print("Got:")
print(distances)
