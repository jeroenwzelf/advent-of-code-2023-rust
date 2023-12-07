use crate::days::Day;

pub struct Day03b;

impl Day for Day03b {
    fn run(input: &str) -> String {
        let schematic = input.as_bytes();
        let line_length = input.chars().position(|char| char.eq(&'\n')).unwrap() as isize;
        (0..schematic.len())
            // find indexes of all gears
            .filter(|i| schematic[*i] == b'*')
            // get start and end indexes of all adjacent gear numbers
            .map(|i| {
                let mut adjacent_number_indexes = [
                    -line_length-2, -line_length-1, -line_length,   // above
                    -1, 1,                                          // left/right
                    line_length, line_length+1, line_length+2       // below
                ].iter()
                    .map(|offset| i as isize + offset)
                    .filter(|position| position >= &0 && position < &(schematic.len() as isize))
                    .map(|position| (position as usize, schematic.get(position as usize)))
                    .filter(|(_, char)| char.is_some() && char.unwrap().is_ascii_digit())
                    .map(|(position, _)|
                        (position - schematic.iter()
                            .rev()
                            .skip(schematic.len() - position)
                            .position(|char| !char.is_ascii_digit()).unwrap(),
                         position + schematic.iter()
                             .skip(position)
                             .position(|char| !char.is_ascii_digit()).unwrap() - 1
                    ))
                    .collect::<Vec<(usize, usize)>>();

                adjacent_number_indexes.dedup();
                adjacent_number_indexes
            })
            .filter(|adjacent_number_indexes| adjacent_number_indexes.len() == 2)
            // get adjacent numbers from adjacent number indexes
            .map(|adjacent_number_indexes|
                adjacent_number_indexes.iter().map(|(start, end)| {
                    schematic[*start..=*end].iter()
                        .fold(0usize, |number, char| (number * 10) + (*char - b'0') as usize)
                }).collect::<Vec<usize>>()
            )
            // calculate gear ratio
            .map(|adjacent_numbers| adjacent_numbers[0] * adjacent_numbers[1])
            .sum::<usize>()
            .to_string()
    }
}