use crate::days::Day;

pub struct Day01a;

impl Day for Day01a {
    fn run(input: &str) -> String {
        input.lines()
            .map(|line| (
                line.chars().find(|char| char.is_ascii_digit()).unwrap(),
                line.chars().rev().find(|char| char.is_ascii_digit()).unwrap()
            ))
            .map(|(first, last)|
                first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap())
            .sum::<u32>()
            .to_string()
    }
}