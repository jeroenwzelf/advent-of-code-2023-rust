use crate::days::Day;

pub struct Day04a;

impl Day for Day04a {
    fn run(input: &str) -> String {
        input.lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(left, right)| (left.split_once(":").unwrap().1, right))
            .map(|(winning, owned)| (winning.split_whitespace(), owned.split_whitespace()))
            .map(|(winning, owned)| {
                let owned = owned.filter(|owned_card| winning.clone()
                    .any(|winning_card| *owned_card == winning_card))
                    .collect::<Vec<&str>>();
                if owned.is_empty() { 0 } else { 2usize.pow(owned.len() as u32 - 1) }
            })
            .sum::<usize>()
            .to_string()
    }
}