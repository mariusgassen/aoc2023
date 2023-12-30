use clap::{Arg, Command};

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

/// A cli interface to run the solutions for the AoC 2023.
/// The input files are downloaded automatically if they do not exist.
/// The session cookie is passed as an environment variable.
fn main() {
    let app = Command::new("AoC 2023")
        .version("0.1")
        .author("Marius Gassen")
        .about("Solutions for the Advent of Code 2023");

    let matches = app
        .subcommand(Command::new("run")
            .about("Runs the solution for a given day")
            .arg(Arg::new("day")
                .short('d')
                .long("day")
                .help("The day to run the solution for")
                .required(true)
                .value_parser(clap::value_parser!(u8))))
        .get_matches();

    match matches.subcommand() {
        Some(("run", subcommand_matches)) => {
            let day = subcommand_matches
                .get_one("day")
                .unwrap();

            let solution = aoc::run_solution(*day).unwrap();
            println!("Solution for day {}\n- Part 1: {}\n- Part 2: {}", day, solution.0, solution.1)
        }
        _ => {
            println!("Missing or invalid subcommand");
        }
    };
}
