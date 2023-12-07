use crate::days::Day;

pub struct Day04a;

impl Day for Day04a {
    fn run(input: &str) -> String {
        let colon_separator_index = input.find(|char| char == ':').unwrap();
        let pipe_separator_index = input.find(|char| char == '|').unwrap();

        input.as_bytes()
            .split(|&b| b == b'\n')
            .map(|line| {
                let winning = &line[colon_separator_index+1..pipe_separator_index];
                let owned = &line[pipe_separator_index+1..];

                let won_amount = owned
                    .chunks_exact(3)
                    .map(|owned_card| &owned_card[1..])
                    .filter(|owned_card| winning
                        .chunks_exact(3)
                        .map(|winning_card| &winning_card[1..])
                        .any(|winning_card| &winning_card == owned_card))
                    .count() as u32;

                if won_amount == 0 { 0 } else { 2u32.pow(won_amount - 1) }
            })
            .sum::<u32>()
            .to_string()
    }
}