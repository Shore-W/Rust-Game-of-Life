# Rust Conway's Game of Life:
A Rust implementation of Conway's Game of Life, a cellular automaton simulation. The main function initializes a 16x16 grid and sets the initial coordinates for a simple pattern. The program then iteratively computes and prints the next generations of the grid.

## Function: `next_generation`
```rust
fn next_generation(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    // Computes the next generation of the grid based on Conway's Game of Life rules.
    // Returns a new matrix reflecting the updated cell states.
    // ...
```

## Function: `main`
```rust
fn main() {
    // Initializes a 16x16 grid and sets initial coordinates for a pattern.
    // Prints the initial state and iteratively computes and prints the next generations.
    
    // ...
```
## How to Run:
Ensure you have Rust installed.
Copy the provided code into a file named main.rs.
Open a terminal in the directory containing main.rs.
Run the following commands:
```rust
cargo run
```
## Example Output:
Initial grid:<br>
--X------------ <br>
-X-------------<br>
XXX------------<br>

Iteration 1:<br>
---------------<br>
-XXX-----------<br>
--X------------<br>
--X------------<br>

Iteration 2:<br>
---------------<br>
---X-----------<br>
-X-X-----------<br>
--XX-----------<br>

...

Iteration 10:<br>
---------------<br>
---------------<br>
---X-----------<br>
----XX---------<br>
---XX----------<br>

