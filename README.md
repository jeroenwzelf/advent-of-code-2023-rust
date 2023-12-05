# Advent of Code 2023 in Rust

My [Advent of Code 2023](https://adventofcode.com/2023) solutions in [Rust](https://www.rust-lang.org/).

## Benchmarks

Below is how long each solution runs. All solutions are measured with [`benchmark_all.rs`](./benches/benchmark_all.rs), by 'warming up' for three seconds, and then collecting 100 samples for each benchmark. Not that the graphs matter or show anything interesting. I just like graphs. 

|                                                | part A                                                                                                                                            |                                                                      part B                                                                       |
|:-----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|:-------------------------------------------------------------------------------------------------------------------------------------------------:|
| [day 1](https://adventofcode.com/2023/day/1)   | ![day01a](./target/criterion/advent-of-code-2023/day01a/benchmark/report/pdf.svg)<br/>[`[ 101.72 µs 102.52 µs 103.40 µs ]`](./src/days/day01a.rs) | ![day01b](./target/criterion/advent-of-code-2023/day01b/benchmark/report/pdf.svg)<br/>[`[ 43.643 µs 44.621 µs 45.634 µs ]`](./src/days/day01b.rs) |
| [day 2](https://adventofcode.com/2023/day/2)   | ![day02a](./target/criterion/advent-of-code-2023/day02a/benchmark/report/pdf.svg)<br/>[`[ 22.370 µs 22.522 µs 22.699 µs ]`](./src/days/day02a.rs) |               ![day02b](./target/criterion/advent-of-code-2023/day02b/benchmark/report/pdf.svg)<br/>[`[ 28.094 µs 28.132 µs 28.177 µs ]`](./src/days/day02b.rs)                |
| [day 3](https://adventofcode.com/2023/day/3)   | ![day03a](./target/criterion/advent-of-code-2023/day03a/benchmark/report/pdf.svg)<br/>[`[ 33.484 µs 33.606 µs 33.730 µs ]`](./src/days/day03a.rs)                              |                ![day03b](./target/criterion/advent-of-code-2023/day03b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day03b.rs)                |
| [day 4](https://adventofcode.com/2023/day/4)   | ![day04a](./target/criterion/advent-of-code-2023/day04a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day04a.rs)                               |                ![day04b](./target/criterion/advent-of-code-2023/day04b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day04b.rs)                |
| [day 5](https://adventofcode.com/2023/day/5)   | ![day05a](./target/criterion/advent-of-code-2023/day05a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day05a.rs)                               |                ![day05b](./target/criterion/advent-of-code-2023/day05b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day05b.rs)                |
| [day 6](https://adventofcode.com/2023/day/6)   | ![day06a](./target/criterion/advent-of-code-2023/day06a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day06a.rs)                               |                ![day06b](./target/criterion/advent-of-code-2023/day06b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day06b.rs)                |
| [day 7](https://adventofcode.com/2023/day/7)   | ![day07a](./target/criterion/advent-of-code-2023/day07a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day07a.rs)                               |                ![day07b](./target/criterion/advent-of-code-2023/day07b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day07b.rs)                |
| [day 8](https://adventofcode.com/2023/day/8)   | ![day08a](./target/criterion/advent-of-code-2023/day08a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day08a.rs)                               |                ![day08b](./target/criterion/advent-of-code-2023/day08b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day08b.rs)                |
| [day 9](https://adventofcode.com/2023/day/9)   | ![day09a](./target/criterion/advent-of-code-2023/day09a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day09a.rs)                               |                ![day09b](./target/criterion/advent-of-code-2023/day09b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day09b.rs)                |
| [day 10](https://adventofcode.com/2023/day/10) | ![day10a](./target/criterion/advent-of-code-2023/day10a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day10a.rs)                               |                ![day10b](./target/criterion/advent-of-code-2023/day10b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day10b.rs)                |
| [day 11](https://adventofcode.com/2023/day/11) | ![day11a](./target/criterion/advent-of-code-2023/day11a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day11a.rs)                               |                ![day11b](./target/criterion/advent-of-code-2023/day11b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day11b.rs)                |
| [day 12](https://adventofcode.com/2023/day/12) | ![day12a](./target/criterion/advent-of-code-2023/day12a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day12a.rs)                               |                ![day12b](./target/criterion/advent-of-code-2023/day12b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day12b.rs)                |
| [day 13](https://adventofcode.com/2023/day/13) | ![day13a](./target/criterion/advent-of-code-2023/day13a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day13a.rs)                               |                ![day13b](./target/criterion/advent-of-code-2023/day13b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day13b.rs)                |
| [day 14](https://adventofcode.com/2023/day/14) | ![day14a](./target/criterion/advent-of-code-2023/day14a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day14a.rs)                               |                ![day14b](./target/criterion/advent-of-code-2023/day14b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day14b.rs)                |
| [day 15](https://adventofcode.com/2023/day/15) | ![day15a](./target/criterion/advent-of-code-2023/day15a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day15a.rs)                               |                ![day15b](./target/criterion/advent-of-code-2023/day15b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day15b.rs)                |
| [day 16](https://adventofcode.com/2023/day/16) | ![day16a](./target/criterion/advent-of-code-2023/day16a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day16a.rs)                               |                ![day16b](./target/criterion/advent-of-code-2023/day16b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day16b.rs)                |
| [day 17](https://adventofcode.com/2023/day/17) | ![day17a](./target/criterion/advent-of-code-2023/day17a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day17a.rs)                               |                ![day17b](./target/criterion/advent-of-code-2023/day17b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day17b.rs)                |
| [day 18](https://adventofcode.com/2023/day/18) | ![day18a](./target/criterion/advent-of-code-2023/day18a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day18a.rs)                               |                ![day18b](./target/criterion/advent-of-code-2023/day18b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day18b.rs)                |
| [day 19](https://adventofcode.com/2023/day/19) | ![day19a](./target/criterion/advent-of-code-2023/day19a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day19a.rs)                               |                ![day19b](./target/criterion/advent-of-code-2023/day19b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day19b.rs)                |
| [day 20](https://adventofcode.com/2023/day/20) | ![day20a](./target/criterion/advent-of-code-2023/day20a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day20a.rs)                               |                ![day20b](./target/criterion/advent-of-code-2023/day20b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day20b.rs)                |
| [day 21](https://adventofcode.com/2023/day/21) | ![day21a](./target/criterion/advent-of-code-2023/day21a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day21a.rs)                               |                ![day21b](./target/criterion/advent-of-code-2023/day21b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day21b.rs)                |
| [day 22](https://adventofcode.com/2023/day/22) | ![day22a](./target/criterion/advent-of-code-2023/day22a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day22a.rs)                               |                ![day22b](./target/criterion/advent-of-code-2023/day22b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day22b.rs)                |
| [day 23](https://adventofcode.com/2023/day/23) | ![day23a](./target/criterion/advent-of-code-2023/day23a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day23a.rs)                               |                ![day23b](./target/criterion/advent-of-code-2023/day23b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day23b.rs)                |
| [day 24](https://adventofcode.com/2023/day/24) | ![day24a](./target/criterion/advent-of-code-2023/day24a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day24a.rs)                               |                ![day24b](./target/criterion/advent-of-code-2023/day24b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day24b.rs)                |
| [day 25](https://adventofcode.com/2023/day/25) | ![day25a](./target/criterion/advent-of-code-2023/day25a/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day25a.rs)                               |                ![day25b](./target/criterion/advent-of-code-2023/day25b/benchmark/report/pdf.svg)<br/>[`[ ]`](./src/days/day25b.rs)                |


## Run solutions
All implementations of puzzle solutions can be found inside [`./src/days/`](./src/days). The corresponding puzzle input can be found in [`./src/input/`](./src/input).

```bash
# Run all puzzles and print their solutions
cargo run

# Unit test all puzzles to see whether everything generates the correct answer 
cargo test

# or benchmark all puzzles
cargo bench
```

## License

This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.