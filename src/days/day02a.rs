use crate::days::Day;

pub struct Day02a;

const MAX_CUBES_RED: usize = 12;
const MAX_CUBES_GREEN: usize = 13;
const MAX_CUBES_BLUE: usize = 14;

impl Day for Day02a {
    fn run(input: &str) -> String {
        input.lines()
            .map(|line| line.split_once(":").unwrap())
            .filter(|(_, subsets)| subsets.split(|char| char == ';' || char == ',')
                .map(|cubes_of_particular_color| cubes_of_particular_color[1..].split_once(" ").unwrap())
                .all(|(amount, color)| amount.parse::<usize>().unwrap() <= match color {
                    "red" => MAX_CUBES_RED,
                    "green" => MAX_CUBES_GREEN,
                    "blue" => MAX_CUBES_BLUE,
                    _ => { panic!("Invalid cube color '{}'!", color) }
                })
            )
            .map(|(game, _)| game.split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap())
            .sum::<usize>()
            .to_string()
    }
}