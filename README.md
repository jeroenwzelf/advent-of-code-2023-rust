# Advent of Code 2023 in Rust
My [Advent of Code 2023](https://adventofcode.com/2023) solutions in [Rust](https://www.rust-lang.org/).

## Run solutions
All implementations of puzzle solutions can be found inside [`./src/days/`](./src/days). The corresponding puzzle input can be found in [`./src/input/`](./src/input).

```bash
# Run all puzzles and print their solutions
cargo run

# Run specific puzzles day01a and day01b
cargo run day01a day01b

# Unit test all puzzles to see whether everything generates the correct answer 
cargo test

# Unit test specific puzzles for day01
cargo test -- day01

# Benchmark all puzzles
cargo bench

# Benchmark specific puzzles for day01
cargo bench -- advent-of-code-2023/day01
```

## Benchmarks
All solutions are measured with [`benchmark_all.rs`](./benches/benchmark_all.rs), by 'warming up' for three seconds, and then collecting 100 samples for each benchmark. A violin plot comparison of all benchmarks is generated below. Not that these are measured very scientifically. I just like graphs.

![violin plot of all benchmarks](./target/criterion/advent-of-code-2023/report/violin.svg)

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.