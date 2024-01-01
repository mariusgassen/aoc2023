use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::aoc::Day;

pub struct Day11 {
    input: String,
}

impl Day11 {
    pub fn new(input: String) -> Day11 {
        Day11 { input }
    }
}

impl Day for Day11 {
    fn part1(&self) -> String {
        let mut universe = Universe::new(&self.input);
        universe.expand_by(2);

        let total_distance: u64 = universe.galaxy_pairs().iter()
            .map(|(a, b)| universe.distance(*a, *b))
            .sum();

        total_distance.to_string()
    }

    fn part2(&self) -> String {
        let mut universe = Universe::new(&self.input);
        universe.expand_by(1000000);

        let total_distance: u64 = universe.galaxy_pairs().iter()
            .map(|(a, b)| universe.distance(*a, *b))
            .sum();

        total_distance.to_string()
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum Space {
    Empty,
    Galaxy,
}

impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("Unknown space type"),
        }
    }
}

struct Universe {
    grid: Vec<Vec<Space>>,
    expanded_rows: Vec<usize>,
    expanded_columns: Vec<usize>,
    expansion: usize,
}

impl Universe {
    fn new(input: &String) -> Universe {
        Universe::from_str(input.as_str()).unwrap()
    }

    fn expand_by(&mut self, factor: usize) {
        self.expansion += factor;
        let mut rows: Vec<usize> = self.grid.iter()
            .enumerate()
            .filter(|(_, row)| row.iter()
                .all(|space| space == &Space::Empty))
            .map(|(i, _)| i)
            .collect();

        let mut columns: Vec<usize> = self.get_columns().iter()
            .enumerate()
            .filter(|(_, column)| column.iter()
                .all(|space| space == &Space::Empty))
            .map(|(i, _)| i)
            .collect();

        self.expanded_rows.append(&mut rows);
        self.expanded_columns.append(&mut columns);
    }

    fn get_columns(&self) -> Vec<Vec<Space>> {
        let mut columns = Vec::new();
        for i in 0..self.grid[0].len() {
            columns.push(self.get_column(i));
        }
        columns
    }
    fn get_column(&self, index: usize) -> Vec<Space> {
        self.grid.iter()
            .map(|row| row[index].clone())
            .collect()
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == Space::Galaxy {
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }

    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let galaxies = self.galaxies();
        let mut pairs = Vec::new();
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                pairs.push((galaxies[i], galaxies[j]));
            }
        }
        pairs
    }

    fn distance(&self, a: (usize, usize), b: (usize, usize)) -> u64 {
        let (x1, x2) = if a.0 > b.0 { (a.0, b.0) } else { (b.0, a.0) };
        let (y1, y2) = if a.1 > b.1 { (a.1, b.1) } else { (b.1, a.1) };
        let x = (x1 - x2) as u64;
        let y = (y1 - y2) as u64;

        let extra_rows = self.expanded_rows.iter()
            .filter(|row| row < &&y1 && row > &&y2)
            .count() as u64;

        let extra_columns = self.expanded_columns.iter()
            .filter(|row| row < &&x1 && row > &&x2)
            .count() as u64;

        (x + y) + (extra_rows + extra_columns) * (self.expansion as u64 - 1)
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.iter()
            .map(|row| row.iter()
                .map(|space| match space {
                    Space::Empty => '.',
                    Space::Galaxy => '#',
                })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

impl FromStr for Universe {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(Space::from_char).collect())
            .collect();
        Ok(Universe { grid, expanded_rows: vec![], expanded_columns: vec![], expansion: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "#;

    #[test]
    fn test_part1() {
        let day11 = Day11 { input: INPUT.to_string() };
        assert_eq!(day11.part1(), "374".to_string());
    }

    #[test]
    fn test_part2_1() {
        let mut universe = Universe::new(&INPUT.to_string());
        universe.expand_by(10);

        let total_distance: u64 = universe.galaxy_pairs().iter()
            .map(|(a, b)| universe.distance(*a, *b))
            .sum();

        assert_eq!(total_distance.to_string(), "1030".to_string());
    }

    #[test]
    fn test_part2_2() {
        let mut universe = Universe::new(&INPUT.to_string());
        universe.expand_by(100);

        let total_distance: u64 = universe.galaxy_pairs().iter()
            .map(|(a, b)| universe.distance(*a, *b))
            .sum();

        assert_eq!(total_distance.to_string(), "8410".to_string());
    }
}