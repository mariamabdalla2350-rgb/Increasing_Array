# Increasing Array

A Rust solution to the classic **Increasing Array** problem, paired with an interactive HTML visualizer that walks through the greedy algorithm step by step.

## Problem

Given an array of `n` integers, find the minimum number of moves required to make the array **non-decreasing**, where a single move increases one element by 1. Elements can only be increased, never decreased.

**Input**
```
n
a_1 a_2 ... a_n
```

**Output**
```
The minimum number of moves
```

**Example**
```
Input:
5
3 2 5 1 7

Output:
5
```

## Approach

A single left-to-right greedy scan is sufficient:

1. Track the previous element's value as the current "floor."
2. If the current element is lower than the floor, raise it up to the floor's value and add the difference to a running move counter.
3. If the current element is already greater than or equal to the floor, leave it untouched and move on.

Because each element only ever needs to match the tallest value seen so far to its left, this greedy choice is always optimal — no backtracking or recomputation is required.

- **Time complexity:** O(n)
- **Space complexity:** O(1) extra space (in-place)

## Project Structure

```
increasing-array/
├── Cargo.toml
├── src/
│   └── main.rs              # Solver: reads input, runs the greedy scan, prints result
└── visualizer.html           # Standalone interactive visualizer (no build step required)
```

## Usage

### Rust solver

```bash
cargo run
```

Input is read from stdin in the format described above, for example:

```bash
echo "8
3 1 4 1 5 9 2 6" | cargo run
```

### Visualizer

Open `visualizer.html` directly in any browser — no server or build step needed.

- Enter your own comma-separated array, or click **Randomize**
- Use **Step** to move through the scan one comparison at a time, or **Play** to run it automatically
- Each bar represents one array element; when a value is raised to match its left neighbor, the added amount is highlighted and labeled
- The total moves counter mirrors the exact value the Rust program computes

## Running Tests

```bash
cargo test
```

## Tech Stack

- **Language:** Rust
- **Build tool:** Cargo
- **Visualizer:** HTML, CSS, vanilla JavaScript

## Author

Mariam Abdalla — B.Sc. Health Informatics, Technische Hochschule Deggendorf (European Campus Rottal-Inn)
# Increasing_Array
Minimum-moves solver for the Increasing Array problem (Rust) — greedy O(n) scan, with an interactive HTML visualizer of the terracing process.
