use regex::Regex;

use crate::aoc::Day;
use crate::aoc::tools::read_lines;

pub struct Day3 {
    input: String,
}

impl Day3 {
    pub fn new(input: String) -> Day3 {
        Day3 { input }
    }

    fn get_grid(&self) -> Grid {
        Grid::new(read_lines(&self.input).iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>())
    }
}

struct Grid {
    symbol_regex: Regex,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid { symbol_regex: Regex::new("[^\\d.]").unwrap(), grid }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y).and_then(|l| l.get(x)).copied()
    }

    fn is_digit(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            Some(c) => c.is_digit(10),
            None => false,
        }
    }

    fn get_adjacent_numbers(&self, x: usize, y: usize) -> Vec<i64> {
        let mut adjacent_numbers = Vec::new();
        for j in y.saturating_sub(1)..y + 2 {
            let mut is_num = false;
            for i in x.saturating_sub(1)..x + 2 {
                if i == x && j == y {
                    is_num = false;
                    continue;
                }
                let symbol = self.get(i, j);
                if symbol.is_some() && symbol.unwrap().is_digit(10) {
                    // Only add number if it is not part a previously added number
                    if !is_num {
                        let num = self.get_full_number(i, j);
                        adjacent_numbers.push(num);
                        is_num = true;
                    }
                    continue;
                }
                is_num = false;
            }
        }
        adjacent_numbers
    }
    fn gear_ratio(&self, x: usize, y: usize) -> Option<i64> {
        let gear = match self.get(x, y) {
            Some(c) => c == '*',
            None => false,
        };

        if !gear {
            return None;
        }

        let adj = self.get_adjacent_numbers(x, y);
        if adj.len() != 2 {
            return None;
        }
        return Some(adj.iter().fold(1, |acc, x| acc * x));
    }

    fn get_full_number(&self, x: usize, y: usize) -> i64 {
        if !self.is_digit(x, y) {
            return 0;
        }

        let mut start = x;
        let mut end = x;

        while start > 0 {
            let previous = start.saturating_sub(1);
            if !self.is_digit(previous, y) {
                break;
            }
            start = previous
        }

        while end < self.width() {
            let next = end + 1;
            if !self.is_digit(next, y) {
                break;
            }
            end = next
        }

        self.grid[y][start..end + 1]
            .iter().collect::<String>()
            .parse::<i64>().unwrap()
    }

    fn has_adjacent_symbol(&self, x: usize, y: usize) -> bool {
        for i in x.saturating_sub(1)..x + 2 {
            for j in y.saturating_sub(1)..y + 2 {
                if i == x && j == y {
                    continue;
                }
                let symbol = self.get(i, j);
                if symbol.is_some() && self.symbol_regex.is_match(&symbol.unwrap().to_string()) {
                    return true;
                }
            }
        }
        false
    }


    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl Day for Day3 {
    fn part1(&self) -> String {
        let grid = self.get_grid();

        let mut sum = 0;

        let mut current: Option<String> = None;
        let mut adjacent = false;

        for y in 0..grid.width() {
            for x in 0..grid.height() {
                if grid.is_digit(x, y) {
                    match current {
                        Some(ref mut s) => s.push(grid.get(x, y).unwrap()),
                        None => current = Some(grid.get(x, y).unwrap().to_string()),
                    }
                    if grid.has_adjacent_symbol(x, y) {
                        adjacent = true;
                    }
                } else {
                    if adjacent && current.is_some() {
                        sum += current.unwrap().parse::<u32>().unwrap()
                    }

                    current = None;
                    adjacent = false;
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let grid = self.get_grid();

        let mut sum = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                sum += grid.gear_ratio(x, y).unwrap_or_else(|| 0)
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    const INPUT: &str = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;

    fn day() -> super::Day3 {
        super::Day3::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().part1(), "4361");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().part2(), "467835");
    }
}