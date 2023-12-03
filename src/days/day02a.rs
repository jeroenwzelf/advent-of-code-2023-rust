use crate::days::Day;

pub struct Day01a;

impl Day for Day01a {
    fn run(input: &str) -> String {
        input.lines()
            .map(|line| line.chars().filter(|char| char.is_ascii_digit()))
            .map(|mut digits| (digits.next(), digits.last()))
            .map(|(a, b)| { (a.unwrap(), if b.is_some() { b.unwrap() } else { a.unwrap() }) })
            .map(|(a, b)| format!("{}{}", a, b).parse::<usize>().unwrap())
            .sum::<usize>()
            .to_string()
    }
}