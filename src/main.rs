use std::env;
use std::process::exit;
use advent_of_code_2023_rust::days;

mod days;

fn main() {
    let args: Vec<_> = env::args().collect();

    // no arguments given, run all days
    if args.len() <= 1 {
        for day in days() {
            println!("Result of {}: {}", day.0, day.3(day.2));
        }
    }
    // with arguments describing which day(s) to run
    else {
        args.iter()
            .skip(1)
            .for_each(|arg|
                match days().iter().find(|day| arg.eq(day.0)) {
                    None => { eprintln!("Could not find day {}!", arg); exit(1); }
                    Some(day) => { println!("Result of {}: {}", day.0, day.3(day.2)); }
                });
    }
}