use crate::days::Day;

pub struct Day07a;

impl Day for Day07a {
    fn run(input: &str) -> String {
        let mut hands = input.lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(cards, bid)| (cards.chars(), bid.parse::<usize>().unwrap()))
            .map(|(cards, bid)| {
                // maps kinds A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, and 2 (13 in total)
                let mut kinds = [0usize; 13];
                // maps kinds respective to their place in the hand. A hand can be viewed as 4
                // numbers ranging from 0 to 12. This can be mapped in a u32, where every 4 bytes
                // contain a number representing the card of the hand, from highest to lowest.
                // This number can then be used to sort all the hands by strongest cards in order of
                // how they are positioned in the hand without sorting it on 4 different numbers.
                let mut highest_card_ordering = 0u32;

                cards.take(5)
                    .enumerate()
                    .for_each(|(i, card)| {
                        let kind: usize = match card {
                            'A' => 12,
                            'K' => 11,
                            'Q' => 10,
                            'J' => 9,
                            'T' => 8,
                            card => card.to_digit(10).unwrap() as usize - 2,
                        };

                        kinds[kind] += 1;
                        highest_card_ordering |= (kind as u32) << 4 * (4 - i);
                    });

                kinds.sort_unstable_by(|a, b| b.cmp(a));

                let strength: usize = match kinds[0] {
                    5 => 6,                     // 5 the same kind (Five of a kind)
                    4 => 5,                     // 4 the same kind (Four of a kind)
                    3 if kinds[1] == 2 => 4,    // 3+2 the same kind (Full house)
                    3 => 3,                     // 3 the same kind (Three of a kind)
                    2 if kinds[1] == 2 => 2,    // 2+2 the same kind (Two pair)
                    2 => 1,                     // 2 the same kind (One pair)
                    _ => 0,                     // 0 the same kind (High card)
                };
                (strength, highest_card_ordering, bid)
            })
            .collect::<Vec<_>>();

        hands.sort_unstable();

        hands.iter()
            .enumerate()
            .map(|(i, (_, _, bid))| bid * (i+1))
            .sum::<usize>()
            .to_string()
    }
}