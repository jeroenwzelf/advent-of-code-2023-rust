use std::iter;
use crate::days::Day;

pub struct Day03a;

impl Day for Day03a {
    fn run(input: &str) -> String {
        let schematic = input.as_bytes();
        let line_length = input.chars().position(|char| char.eq(&'\n')).unwrap() as isize;

        (0..schematic.len())
            // find indexes of starting digits of all numbers
            .filter(|i| schematic[*i].is_ascii_digit() &&
                (*i == 0 || (*i > 1 && !schematic[*i - 1].is_ascii_digit())))
            // find indexes of ending digits of all numbers
            .map(|i| (i as isize, i as isize + schematic.iter()
                .skip(i)
                .position(|char| !char.is_ascii_digit()).unwrap() as isize - 1))
            // filter for numbers adjacent to symbols
            .filter(|(start, end)| {
                (start-2-line_length..end+1-line_length) // above
                    .chain(start+line_length..end+3+line_length) // below
                    .chain(iter::once(start-1)) // left
                    .chain(iter::once(end+1)) // right
                    .any(|i| i >= 0 && schematic.get(i as usize)
                        .map_or(false, |char| char.is_ascii_punctuation() && char != &b'.'))
            })
            // get number from indexes
            .map(|(start, end)| schematic[start as usize..=end as usize].iter()
                .fold(0usize, |number, char| (number * 10) + (*char - b'0') as usize))
            .sum::<usize>()
            .to_string()
    }
}