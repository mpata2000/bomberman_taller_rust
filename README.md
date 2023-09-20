# Bomberman

Individual assigment for Taller de Programacion I at FIUBA.

Link to the assignment: https://taller-1-fiuba-rust.github.io/proyecto/23C2/ejercicio_individual.html

## How to run it

```
cargo run -- maze.txt /path/to/output_dir/ x y
```

- `maze.txt` is the path to the maze file.
  - It can be a relative path. Eg: `./path/maze.txt`
  - File must be inside this project directory
- `/path/to/output_dir/` is the path to the directory where the output files will be saved.
  - It will be trim to remove the last `/` and first `/` if present and correctly add the `/` at the end.
  - Out path will be inside this project directory
- Coordinates `x` and `y` are the coordinate of the first bomb to explode.
  - `x` is the column and `y` is the row.
  - `x` and `y` must be positive integers.
  - If `x` or `y` do not hit a bomb, the program will exit with an error.