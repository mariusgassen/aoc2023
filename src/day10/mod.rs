use std::ops::{Add, Div, Mul, Sub};

use crate::aoc::Day;
use crate::aoc::tools::read_lines;
use crate::day10::Dir::{E, N, S, W};

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn as_dir(&self) -> (i32, i32) {
        match self {
            N => (0, -1),
            S => (0, 1),
            E => (1, 0),
            W => (-1, 0),
        }
    }

    fn x(&self) -> i32 {
        self.as_dir().0
    }

    fn y(&self) -> i32 {
        self.as_dir().1
    }
}

struct Maze {
    map: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Maze {
    fn new(input: Vec<String>) -> Maze {
        let mut map = Vec::new();
        let mut start = (0, 0);

        for (y, line) in input.into_iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Ground,
                    '|' => Tile::NS,
                    '-' => Tile::EW,
                    'L' => Tile::NE,
                    'J' => Tile::NW,
                    '7' => Tile::SW,
                    'F' => Tile::SE,
                    'S' => Tile::Start,
                    _ => panic!("Invalid character in input: {}", c),
                };
                if tile == Tile::Start {
                    start = (x, y);
                }
                row.push(tile);
            }
            map.push(row);
        }
        Maze {
            map,
            start,
        }
    }

    fn find_start_dir(&self) -> Dir {
        let north = &self.map[self.start.1.saturating_sub(1)][self.start.0];
        let south = &self.map[self.start.1.add(1)][self.start.0];
        let east = &self.map[self.start.1][self.start.0.add(1)];
        let west = &self.map[self.start.1][self.start.0.saturating_sub(1)];

        match north {
            Tile::NS | Tile::SE | Tile::SW => Dir::N,
            _ => match south {
                Tile::NS | Tile::NE | Tile::NW => Dir::S,
                _ => match east {
                    Tile::EW | Tile::NW | Tile::SW => Dir::E,
                    _ => match west {
                        Tile::EW | Tile::NE | Tile::SE => Dir::W,
                        _ => panic!("No valid start direction found for {:?}", self.start),
                    }
                }
            }
        }
    }

    fn find_loop(&self) -> Vec<(usize, usize)> {
        let mut dir = self.find_start_dir(); // Two possibilities
        let mut x = (self.start.0 as i32 + dir.x()) as usize;
        let mut y = (self.start.1 as i32 + dir.y()) as usize;

        let mut _loop = vec![self.start];

        loop {
            let tile = self.map[y][x].clone();
            match tile {
                Tile::NS => {
                    dir = match dir {
                        N => N,
                        S => S,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::EW => {
                    dir = match dir {
                        E => E,
                        W => W,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::NE => {
                    dir = match dir {
                        S => E,
                        W => N,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::NW => {
                    dir = match dir {
                        S => W,
                        E => N,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::SW => {
                    dir = match dir {
                        N => W,
                        E => S,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::SE => {
                    dir = match dir {
                        N => E,
                        W => S,
                        _ => panic!("Invalid direction: {:?}", dir),
                    }
                }
                Tile::Ground => {
                    panic!("Invalid tile: {:?}", tile);
                }
                Tile::Start => {
                    return _loop;
                }
            }
            _loop.push((x, y));
            x = (x as i32 + dir.x()) as usize;
            y = (y as i32 + dir.y()) as usize;
        }
    }

    fn shoelace(&self) -> i64 {
        let points: Vec<(usize, usize)> = self.find_loop();
        let len = points.len();

        points.iter()
            .enumerate()
            .fold(0 as i64, |s, (i, (x, y))| {
                let l = (i + 1) % len;
                let _x: i64 = *x as i64;
                let _y: i64 = *y as i64;
                s + (_y * points[l].0 as i64) - (_x * points[l].1 as i64)
            })
            .abs()
            .div(2)
    }
}

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new(input: String) -> Day10 {
        Day10 {
            input
        }
    }
    fn get_maze(&self) -> Maze {
        let lines = read_lines(&self.input);
        Maze::new(lines)
    }
}

impl Day for Day10 {
    fn part1(&self) -> String {
        let maze = self.get_maze();
        let length = maze.find_loop().len();
        let max_distance = (length as f32 / 2.0).ceil() as u32;
        max_distance.to_string()
    }
    fn part2(&self) -> String {
        let maze = self.get_maze();
        let points = maze.find_loop();
        let area = maze.shoelace()
            // Not sure why this, probably removing loop points itself
            .mul(2)
            .sub(points.len() as i64)
            .div(2)
            .add(1);
        area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1_1: &str = r#"
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
    "#;

    const INPUT_1_2: &str = r#"
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    "#;

    #[test]
    fn test_part1() {
        let day10 = Day10::new(INPUT_1_1.to_string());
        assert_eq!(day10.part1(), "4");
    }

    #[test]
    fn test_part1_2() {
        let day10 = Day10::new(INPUT_1_2.to_string());
        assert_eq!(day10.part1(), "8");
    }

    const INPUT_2_1: &str = r#"
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........
    "#;

    const INPUT_2_2: &str = r#"
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
    "#;

    const INPUT_2_3: &str = r#"
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    "#;

    #[test]
    fn test_part2_1() {
        let day10 = Day10::new(INPUT_2_1.to_string());
        assert_eq!(day10.part2(), "4".to_string());
    }

    #[test]
    fn test_part2_2() {
        let day10 = Day10::new(INPUT_2_2.to_string());
        assert_eq!(day10.part2(), "8".to_string());
    }

    #[test]
    fn test_part2_3() {
        let day10 = Day10::new(INPUT_2_3.to_string());
        assert_eq!(day10.part2(), "10".to_string());
    }
}