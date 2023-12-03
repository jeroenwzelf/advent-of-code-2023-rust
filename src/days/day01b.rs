use crate::days::Day;

pub struct Day01b;

const DIGITS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn to_digit(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit().then_some((line[i] - b'0') as usize)
        .or(DIGITS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(digit, _)| digit + 1)
        )
}

impl Day for Day01b {
    fn run(input: &str) -> String {
        input.as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| (0..line.len()).find_map(|i| to_digit(line, i)).unwrap() * 10
                + (0..line.len()).rev().find_map(|i| to_digit(line, i)).unwrap()
            )
            .sum::<usize>()
            .to_string()
    }
}