use std::cmp::max;

use crate::aoc::Day;
use crate::aoc::tools::read_lines;

pub struct Day2 {
    input: String,
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(r: u32, g: u32, b: u32) -> Cubes {
        Cubes { red: r, green: g, blue: b }
    }

    fn total(&self) -> u32 {
        self.red + self.green + self.blue
    }

    fn possible(&self, bag: &Cubes) -> bool {
        (self.red == 0 || self.red <= bag.red)
            && (self.green == 0 || self.green <= bag.green)
            && (self.blue == 0 || self.blue <= bag.blue)
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    moves: Vec<Cubes>,
}

impl Game {
    fn min_bag(&self) -> Cubes {
        self.moves.iter()
            .fold(Cubes::new(0, 0, 0), |acc, c| {
                Cubes::new(
                    max(acc.red, c.red),
                    max(acc.green, c.green),
                    max(acc.blue, c.blue),
                )
            })
    }
}

impl Game {
    fn new(id: u32, moves: Vec<Cubes>) -> Game {
        Game { id, moves }
    }
}

impl Day2 {
    pub fn new(input: String) -> Day2 {
        Day2 { input }
    }

    fn parse_game(l: &String) -> Game {
        let mut parts = l.split(":");

        let id = parts.next()
            .unwrap()
            .trim()
            .strip_prefix("Game")
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        let moves = parts.next()
            .unwrap().split(";")
            .map(|p| Day2::parse_move(p))
            .collect();

        Game::new(id, moves)
    }

    fn parse_move(s: &str) -> Cubes {
        let parts = s.trim().split(",");
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        parts.for_each(|p| {
            let mut color = p.trim().split(" ");
            let count = color.next().unwrap().parse::<u32>().unwrap();
            let c = color.next().unwrap();
            match c {
                "red" => r = count,
                "green" => g = count,
                "blue" => b = count,
                _ => panic!("Invalid color {}", c),
            }
        });

        Cubes::new(r, g, b)
    }

    fn possible(bag: &Cubes, game: &Game) -> bool {
        game.moves.iter()
            .fold(true,
                  |acc, m|
                      acc && m.possible(bag)
                          && m.total() <= bag.total(),
            )
    }
}

impl Day for Day2 {
    fn part1(&self) -> String {
        let cubes = Cubes::new(12, 13, 14);

        let valid_games = read_lines(&self.input).iter()
            .map(|l| Day2::parse_game(l))
            .filter(|g| Day2::possible(&cubes, &g))
            .map(|g| g.id)
            .collect::<Vec<u32>>();

        valid_games.iter()
            .fold(0, |acc, id| acc + id)
            .to_string()
    }

    fn part2(&self) -> String {
        let powers = read_lines(&self.input).iter()
            .map(|l| Day2::parse_game(l))
            .map(|g| g.min_bag())
            .map(|g| g.power())
            .fold(0, |acc, p| acc + p);

        powers.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    fn day() -> super::Day2 {
        super::Day2::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().part1(), "8");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().part2(), "2286");
    }
}