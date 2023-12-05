use crate::days::Day;

mod days;

#[inline]
pub fn days() -> &'static [(&'static str, &'static str, &'static str, fn(&str) -> String)] {
    &[
        ("day01a", "input/day01.txt", include_str!("input/day01.txt"), days::day01a::Day01a::run), ("day01b", "input/day01.txt", include_str!("input/day01.txt"), days::day01b::Day01b::run),
        ("day02a", "input/day02.txt", include_str!("input/day02.txt"), days::day02a::Day02a::run), ("day02b", "input/day02.txt", include_str!("input/day02.txt"), days::day02b::Day02b::run),
        ("day03a", "input/day03.txt", include_str!("input/day03.txt"), days::day03a::Day03a::run), // ("day03b", "input/day03.txt", include_str!("input/day03.txt"), days::day03b::Day03b::run),
        // ("day04a", "input/day04.txt", include_str!("input/day04.txt"), days::day04a::Day04a::run), ("day04b", "input/day04.txt", include_str!("input/day04.txt"), days::day04b::Day04b::run),
        // ("day05a", "input/day05.txt", include_str!("input/day05.txt"), days::day05a::Day05a::run), ("day05b", "input/day05.txt", include_str!("input/day05.txt"), days::day05b::Day05b::run),
        // ("day06a", "input/day06.txt", include_str!("input/day06.txt"), days::day06a::Day06a::run), ("day06b", "input/day06.txt", include_str!("input/day06.txt"), days::day06b::Day06b::run),
        // ("day07a", "input/day07.txt", include_str!("input/day07.txt"), days::day07a::Day07a::run), ("day07b", "input/day07.txt", include_str!("input/day07.txt"), days::day07b::Day07b::run),
        // ("day08a", "input/day08.txt", include_str!("input/day08.txt"), days::day08a::Day08a::run), ("day08b", "input/day08.txt", include_str!("input/day08.txt"), days::day08b::Day08b::run),
        // ("day09a", "input/day09.txt", include_str!("input/day09.txt"), days::day09a::Day09a::run), ("day09b", "input/day09.txt", include_str!("input/day09.txt"), days::day09b::Day09b::run),
        // ("day10a", "input/day10.txt", include_str!("input/day10.txt"), days::day10a::Day10a::run), ("day10b", "input/day10.txt", include_str!("input/day10.txt"), days::day10b::Day10b::run),
        // ("day11a", "input/day11.txt", include_str!("input/day11.txt"), days::day11a::Day11a::run), ("day11b", "input/day11.txt", include_str!("input/day11.txt"), days::day11b::Day11b::run),
        // ("day12a", "input/day12.txt", include_str!("input/day12.txt"), days::day12a::Day12a::run), ("day12b", "input/day12.txt", include_str!("input/day12.txt"), days::day12b::Day12b::run),
        // ("day13a", "input/day13.txt", include_str!("input/day13.txt"), days::day13a::Day13a::run), ("day13b", "input/day13.txt", include_str!("input/day13.txt"), days::day13b::Day13b::run),
        // ("day14a", "input/day14.txt", include_str!("input/day14.txt"), days::day14a::Day14a::run), ("day14b", "input/day14.txt", include_str!("input/day14.txt"), days::day14b::Day14b::run),
        // ("day15a", "input/day15.txt", include_str!("input/day15.txt"), days::day15a::Day15a::run), ("day15b", "input/day15.txt", include_str!("input/day15.txt"), days::day15b::Day15b::run),
        // ("day16a", "input/day16.txt", include_str!("input/day16.txt"), days::day16a::Day16a::run), ("day16b", "input/day16.txt", include_str!("input/day16.txt"), days::day16b::Day16b::run),
        // ("day17a", "input/day17.txt", include_str!("input/day17.txt"), days::day17a::Day17a::run), ("day17b", "input/day17.txt", include_str!("input/day17.txt"), days::day17b::Day17b::run),
        // ("day18a", "input/day18.txt", include_str!("input/day18.txt"), days::day18a::Day18a::run), ("day18b", "input/day18.txt", include_str!("input/day18.txt"), days::day18b::Day18b::run),
        // ("day19a", "input/day19.txt", include_str!("input/day19.txt"), days::day19a::Day19a::run), ("day19b", "input/day19.txt", include_str!("input/day19.txt"), days::day19b::Day19b::run),
        // ("day20a", "input/day20.txt", include_str!("input/day20.txt"), days::day20a::Day20a::run), ("day20b", "input/day20.txt", include_str!("input/day20.txt"), days::day20b::Day20b::run),
        // ("day21a", "input/day21.txt", include_str!("input/day21.txt"), days::day21a::Day21a::run), ("day21b", "input/day21.txt", include_str!("input/day21.txt"), days::day21b::Day21b::run),
        // ("day22a", "input/day22.txt", include_str!("input/day22.txt"), days::day22a::Day22a::run), ("day22b", "input/day22.txt", include_str!("input/day22.txt"), days::day22b::Day22b::run),
        // ("day23a", "input/day23.txt", include_str!("input/day23.txt"), days::day23a::Day23a::run), ("day23b", "input/day23.txt", include_str!("input/day23.txt"), days::day23b::Day23b::run),
        // ("day24a", "input/day24.txt", include_str!("input/day24.txt"), days::day24a::Day24a::run), ("day24b", "input/day24.txt", include_str!("input/day24.txt"), days::day24b::Day24b::run),
        // ("day25a", "input/day25.txt", include_str!("input/day25.txt"), days::day25a::Day25a::run), ("day25b", "input/day25.txt", include_str!("input/day25.txt"), days::day25b::Day25b::run),
    ]
}

#[cfg(test)]
mod tests {
    use crate::days;
    use crate::days::Day;

    // Macro for generating basic unit test for any day in above array generated by days()
    macro_rules! generate_tests {
        ($($name:ident, $input:expr, $expected:expr, $function:expr;)+) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($function($input), $expected);
                }
            )+
        };
    }

    generate_tests! {
        day01a, include_str!("../src/input/day01.txt"), "52974", days::day01a::Day01a::run; day01b, include_str!("../src/input/day01.txt"), "53340", days::day01b::Day01b::run;
        day02a, include_str!("../src/input/day02.txt"), "2632", days::day02a::Day02a::run; day02b, include_str!("../src/input/day02.txt"), "69629", days::day02b::Day02b::run;
        day03a, include_str!("../src/input/day03.txt"), "535078", days::day03a::Day03a::run; // day03b, include_str!("../src/input/day03.txt"), "?", days::day03b::Day03b::run;
        // day04a, include_str!("../src/input/day04.txt"), "?", days::day04a::Day04a::run; day04b, include_str!("../src/input/day04.txt"), "?", days::day04b::Day04b::run;
        // day05a, include_str!("../src/input/day05.txt"), "?", days::day05a::Day05a::run; day05b, include_str!("../src/input/day05.txt"), "?", days::day05b::Day05b::run;
        // day06a, include_str!("../src/input/day06.txt"), "?", days::day06a::Day06a::run; day06b, include_str!("../src/input/day06.txt"), "?", days::day06b::Day06b::run;
        // day07a, include_str!("../src/input/day07.txt"), "?", days::day07a::Day07a::run; day07b, include_str!("../src/input/day07.txt"), "?", days::day07b::Day07b::run;
        // day08a, include_str!("../src/input/day08.txt"), "?", days::day08a::Day08a::run; day08b, include_str!("../src/input/day08.txt"), "?", days::day08b::Day08b::run;
        // day09a, include_str!("../src/input/day09.txt"), "?", days::day09a::Day09a::run; day09b, include_str!("../src/input/day09.txt"), "?", days::day09b::Day09b::run;
        // day10a, include_str!("../src/input/day10.txt"), "?", days::day10a::Day10a::run; day10b, include_str!("../src/input/day10.txt"), "?", days::day10b::Day10b::run;
        // day11a, include_str!("../src/input/day11.txt"), "?", days::day11a::Day11a::run; day11b, include_str!("../src/input/day11.txt"), "?", days::day11b::Day11b::run;
        // day12a, include_str!("../src/input/day12.txt"), "?", days::day12a::Day12a::run; day12b, include_str!("../src/input/day12.txt"), "?", days::day12b::Day12b::run;
        // day13a, include_str!("../src/input/day13.txt"), "?", days::day13a::Day13a::run; day13b, include_str!("../src/input/day13.txt"), "?", days::day13b::Day13b::run;
        // day14a, include_str!("../src/input/day14.txt"), "?", days::day14a::Day14a::run; day14b, include_str!("../src/input/day14.txt"), "?", days::day14b::Day14b::run;
        // day15a, include_str!("../src/input/day15.txt"), "?", days::day15a::Day15a::run; day15b, include_str!("../src/input/day15.txt"), "?", days::day15b::Day15b::run;
        // day16a, include_str!("../src/input/day16.txt"), "?", days::day16a::Day16a::run; day16b, include_str!("../src/input/day16.txt"), "?", days::day16b::Day16b::run;
        // day17a, include_str!("../src/input/day17.txt"), "?", days::day17a::Day17a::run; day17b, include_str!("../src/input/day17.txt"), "?", days::day17b::Day17b::run;
        // day18a, include_str!("../src/input/day18.txt"), "?", days::day18a::Day18a::run; day18b, include_str!("../src/input/day18.txt"), "?", days::day18b::Day18b::run;
        // day19a, include_str!("../src/input/day19.txt"), "?", days::day19a::Day19a::run; day19b, include_str!("../src/input/day19.txt"), "?", days::day19b::Day19b::run;
        // day20a, include_str!("../src/input/day20.txt"), "?", days::day20a::Day20a::run; day20b, include_str!("../src/input/day20.txt"), "?", days::day20b::Day20b::run;
        // day21a, include_str!("../src/input/day21.txt"), "?", days::day21a::Day21a::run; day21b, include_str!("../src/input/day21.txt"), "?", days::day21b::Day21b::run;
        // day22a, include_str!("../src/input/day22.txt"), "?", days::day22a::Day22a::run; day22b, include_str!("../src/input/day22.txt"), "?", days::day22b::Day22b::run;
        // day23a, include_str!("../src/input/day23.txt"), "?", days::day23a::Day23a::run; day23b, include_str!("../src/input/day23.txt"), "?", days::day23b::Day23b::run;
        // day24a, include_str!("../src/input/day24.txt"), "?", days::day24a::Day24a::run; day24b, include_str!("../src/input/day24.txt"), "?", days::day24b::Day24b::run;
        // day25a, include_str!("../src/input/day25.txt"), "?", days::day25a::Day25a::run; day25b, include_str!("../src/input/day25.txt"), "?", days::day25b::Day25b::run;
    }
}