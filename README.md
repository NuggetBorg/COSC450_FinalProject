# COSC450 Final Project

## Rust Vs C (A comparison of Djikstra's algorithm)

Shortest Path Graph Analyzer

## Group Member Names

Jack Maust
Jacob Beeman

## Programming Language Used

Rust

## Compiler / Interpreter and Version

- Rust compiler (`rustc`)
- Cargo package manager
- Tested with a modern stable Rust toolchain (Rust 2024 edition)

To check your installed version:

```bash
rustc --version
cargo --version
```

## Required Libraries or Dependencies

This project uses the Rust standard library only. No external crates are currently listed in `Rust/src/Cargo.toml`.

## Instructions for Compiling and Running the Program

From the repository root:

```bash
cd Rust/src
cargo build
cargo run -- ../Samples/city_roads.csv
```

You can also run the program with any CSV sample file in the `Samples` directory:

```bash
cargo run -- ../Samples/small_graph.csv
cargo run -- ../Samples/disconnected.csv
```

The program expects a graph CSV file path as a command-line argument. If no valid file is supplied, the program falls back to a hardcoded local path in the source code.

## Description of the Implemented Features

This project reads a weighted graph from a CSV file and finds the shortest route between two user-selected locations.

Implemented features include:

- Reading graph data from CSV input files
- Building an adjacency map of connected locations and weights
- Allowing the user to enter a start and end location interactively
- Computing the shortest path using a shortest-path algorithm
- Reporting the total travel cost and the sequence of stops along the route
- Handling unreachable destinations by reporting that no valid path exists
- Ignoring empty or malformed rows in the CSV input while continuing processing

## Sample Input and Output

Example input file: `Samples/small_graph.csv`

```csv
from,to,weight
A,B,2
A,C,5
B,C,1
B,D,3
C,D,2
D,E,4
```

Example run:

```text
Where are you starting from?
A
Where are you ending at?
E
The total cost for starting at A and ending at E is: 6.0
Start: A
Go to: B
Go to: C
Go to: D
End:   E
```

## Known Limitations or Incomplete Features

- The program currently depends on a hardcoded default file path when no argument is passed.
- It is designed for interactive command-line use and does not provide a full CLI help menu.
- Input validation is limited to skipping malformed rows rather than fully reporting every issue in a structured way.
- There are no external dependencies or advanced features beyond the core shortest-path functionality.
- The project is still a course assignment and may not be fully generalized for all possible graph formats.

---

This repository implements a weighted graph shortest-path program in Rust for the COSC450 final project.


SAMPLE DESCRIPTIONS:

small_graph.csv A 6-node graph with competing routes, small enough to check by hand.

city_roads.csv A 15-edge graph with decimal weights and readable names like Airport and Stadium.

disconnected.csv Two separate components (A-B-C and X-Y-Z). Going from A to Z should report “unreachable.”

edge_cases_valid.csv Valid but tricky rows: duplicate edges with different weights, a self-loop, zero weight, a tiny weight, a huge weight, and scientific notation.

invalid_input.csv Bad rows: missing fields, an empty name, a non-numeric weight, a negative weight, an empty weight, and an extra column. Your loader should report each one and keep going without crashing.

empty.csv and header_only.csv A graph with zero edges.

large_graph.csv 1,000 nodes and about 5,000 edges, for the timing feature.
