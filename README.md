# COSC450 Final Project

## Project Title

Shortest Path Graph Analyzer

## Group Member Names

- [Add group member name 1]
- [Add group member name 2]
- [Add group member name 3]

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
