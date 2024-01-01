use std::fs::File;

use crate::{day1, day10, day11, day2, day3, day4, day5, day6, day7, day8, day9};

pub(crate) mod tools;

pub trait Day {
    fn run(&self) -> (String, String) {
        (self.part1(), self.part2())
    }
    fn part1(&self) -> String {
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

/// Given a day between 1 and 24, download the AoC input file for that day if it not already exists.
/// Requires a session cookie to be passed as an environment variable.
pub fn get_day_input(day: u8) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let output_folder = format!("input/day{}.txt", day);
    if !std::path::Path::new("input").exists() {
        std::fs::create_dir("input")?;
    }
    if std::path::Path::new(&output_folder).exists() {
        return Ok(());
    }
    let session_cookie = read_session_cookie()?;
    download_file(&url, &session_cookie, &output_folder)?;
    Ok(())
}

/// Reads the input file for a given day and returns its content as a string.
pub fn read_input(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let input_file = format!("input/day{}.txt", day);
    let input = std::fs::read_to_string(input_file)?;
    Ok(input)
}

/// Runs the solution for a given day.
/// The solution is a function that takes a string as input and returns a string as output.
pub fn run_solution(day: u8) -> Result<(String, String), Box<dyn std::error::Error>> {
    get_day_input(day)?; // download input file if it does not exist yet
    let input = read_input(day).unwrap();

    let solution = match day {
        1 => day1::Day1::new(input).run(),
        2 => day2::Day2::new(input).run(),
        3 => day3::Day3::new(input).run(),
        4 => day4::Day4::new(input).run(),
        5 => day5::Day5::new(input).run(),
        6 => day6::Day6::new(input).run(),
        7 => day7::Day7::new(input).run(),
        8 => day8::Day8::new(input).run(),
        9 => day9::Day9::new(input).run(),
        10 => day10::Day10::new(input).run(),
        11 => day11::Day11::new(input).run(),
        // 12 => day12::Day12::new(input).run(),
        // 13 => day13::Day13::new(input).run(),
        // 14 => day14::Day14::new(input).run(),
        // 15 => day15::Day15::new(input).run(),
        // 16 => day16::Day16::new(input).run(),
        // 17 => day17::Day17::new(input).run(),
        // 18 => day18::Day18::new(input).run(),
        // 19 => day19::Day19::new(input).run(),
        // 20 => day20::Day20::new(input).run(),
        // 21 => day21::Day21::new(input).run(),
        // 22 => day22::Day22::new(input).run(),
        // 23 => day23::Day23::new(input).run(),
        // 24 => day24::Day24::new(input).run(),
        _ => panic!("Day {} not implemented yet", day)
    };
    Ok(solution)
}

/// Downloads a text file via http and writes its content to a given output folder.
/// Authentication is passed via the session cookie.
fn download_file(url: &str, session: &str, output_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let session_cookie = format!("session={}", session);
    let client = reqwest::blocking::Client::new();
    let mut resp = client.get(url)
        .header(reqwest::header::COOKIE, session_cookie)
        .send()?;
    let mut out = File::create(output_folder)?;
    std::io::copy(&mut resp, &mut out)?;
    Ok(())
}

/// Reads the session cookie from a .env file and returns it as a string.
/// The .env file should be in the same folder as the Cargo.toml file.
fn read_session_cookie() -> Result<String, Box<dyn std::error::Error>> {
    let session_cookie = dotenv::var("SESSION_COOKIE")?;
    Ok(session_cookie)
}
