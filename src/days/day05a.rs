use crate::days::Day;

pub struct Day05a;

fn parse_map(map: &str) -> Vec<(usize, usize, usize)> {
    map.lines()
        .skip(1)
        .take_while(|line| line.chars().next().is_some_and(|char| char.is_ascii_digit()))
        .map(|line| line.split_whitespace())
        .map(|mut line| (
            line.next().unwrap().parse::<usize>().unwrap(), // destination range start
            line.next().unwrap().parse::<usize>().unwrap(), // source range start
            line.next().unwrap().parse::<usize>().unwrap()  // range length
        )).collect()
}

impl Day for Day05a {
    fn run(input: &str) -> String {
        let mut input = input.split(":").skip(1);
        let seeds = input.next().unwrap()
            .split_whitespace()
            .take_while(|word| word.chars().next().unwrap().is_ascii_digit())
            .map(|seed| seed.parse::<usize>().unwrap());
        let maps = input.map(|map| parse_map(map)).collect::<Vec<Vec<_>>>();

        seeds.map(|mut seed| {
            for map in maps.iter() {
                seed = map.iter()
                    .find(|(_, source, length)| seed >= *source && seed < source + length)
                    .map_or(seed, |(dest, source, _)| dest + seed - source);
            }
            seed
        }).min().unwrap().to_string()
    }
}