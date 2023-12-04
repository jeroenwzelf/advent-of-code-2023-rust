use crate::days::Day;

pub struct Day02b;

impl Day for Day02b {
    fn run(input: &str) -> String {
        input.lines()
            .map(|line| line.split_once(":").unwrap().1)
            .map(|subsets| {
                let mut rgb = [0, 0, 0];
                subsets.split(|char| char == ';' || char == ',')
                    .map(|cubes_of_particular_color| cubes_of_particular_color[1..].split_once(" ").unwrap())
                    .for_each(|(amount, color)| {
                        let amount = amount.parse::<usize>().unwrap();
                        match color {
                            "red" => rgb[0] = rgb[0].max(amount),
                            "green" => rgb[1] = rgb[1].max(amount),
                            "blue" => rgb[2] = rgb[2].max(amount),
                            _ => { panic!("Invalid cube color '{}'!", color) }
                        };
                    });
                rgb[0] * rgb[1] * rgb[2]
            })
            .sum::<usize>()
            .to_string()
    }
}