use crate::days::Day;

pub struct Day04b;

impl Day for Day04b {
    fn run(input: &str) -> String {
        let colon_separator_index = input.find(|char| char == ':').unwrap();
        let pipe_separator_index = input.find(|char| char == '|').unwrap();
        let mut copies = [1usize; 215];

        input.as_bytes()
            .split(|&b| b == b'\n')
            .enumerate()
            .map(|(card, line)| {
                let winning = &line[colon_separator_index+1..pipe_separator_index];
                let owned = &line[pipe_separator_index+1..];

                let original_card_copy_amount = copies[card];

                let won_amount = owned
                    .chunks_exact(3)
                    .map(|owned_card| &owned_card[1..])
                    .filter(|owned_card| winning
                        .chunks_exact(3)
                        .map(|winning_card| &winning_card[1..])
                        .any(|winning_card| &winning_card == owned_card))
                    .count();

                (card..card+won_amount).for_each(|card| copies[card + 1] += original_card_copy_amount);
                original_card_copy_amount * won_amount + 1
            })
            .sum::<usize>()
            .to_string()
    }
}