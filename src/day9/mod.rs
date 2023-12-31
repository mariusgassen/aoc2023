use std::ops::{Add, Sub};

use crate::aoc::Day;
use crate::aoc::tools::read_lines;

pub trait VecExt<T>: AsMut<Vec<T>> {
    fn prepend(&mut self, v: T) {
        self.as_mut().splice(0..0, vec![v].drain(..));
    }
}

impl<T> VecExt<T> for Vec<T> {}

struct Dataset {
    histories: Vec<History>,
}

impl Dataset {
    fn new(histories: Vec<History>) -> Self {
        Self { histories }
    }
}

struct History {
    numbers: Vec<i64>,
}

impl History {
    fn new(numbers: Vec<i64>) -> Self {
        Self { numbers }
    }

    fn extrapolate(&mut self, forward: bool) -> i64 {
        // Create initial state
        let mut stages = Vec::new();

        let mut previous: Vec<i64> = self.numbers.clone();
        stages.push(previous.clone());

        loop {
            let mut stage = Vec::new();
            for i in 0..previous.len() - 1 {
                stage.push(previous[i + 1] - previous[i]);
            }
            previous = stage.clone();
            stages.push(stage.clone());
            if stage.iter().all(|v| *v == 0) {
                break;
            }
        }

        // Start from the bottom
        if forward {
            stages.last_mut().unwrap().push(0);
        } else {
            stages.last_mut().unwrap().prepend(0);
        }

        let max_stage = stages.len() - 1;
        for i in (0..max_stage).rev() {
            let extrapolated = match forward {
                true => {
                    let lower_last = stages.get(i + 1).unwrap().last().unwrap();
                    let last = stages.get(i).unwrap().last().unwrap();
                    last.add(lower_last)
                }
                false => {
                    let lower_first = stages.get(i + 1).unwrap().first().unwrap();
                    let first = stages.get(i).unwrap().first().unwrap();
                    first.sub(lower_first)
                }
            };
            if forward {
                stages.get_mut(i).unwrap().push(extrapolated);
            } else {
                stages.get_mut(i).unwrap().prepend(extrapolated);
            }
        }

        if forward {
            stages
                .first_mut().unwrap()
                .last_mut().unwrap()
                .clone()
        } else {
            stages
                .first_mut().unwrap()
                .first_mut().unwrap()
                .clone()
        }
    }
}

pub struct Day9 {
    input: String,
}

impl Day9 {
    fn read_dataset(&self) -> Dataset {
        Dataset::new(
            read_lines(&self.input).iter()
                .map(|l| History::new(l.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect()))
                .collect::<Vec<History>>())
    }

    pub fn new(input: String) -> Day9 {
        Day9 { input }
    }
}

impl Day for Day9 {
    fn part1(&self) -> String {
        let mut dataset = self.read_dataset();
        let result: i64 = dataset.histories.iter_mut()
            .map(|h| {
                h.extrapolate(true)
            })
            .sum();
        result.to_string()
    }

    fn part2(&self) -> String {
        let mut dataset = self.read_dataset();
        let result: i64 = dataset.histories.iter_mut()
            .map(|h| {
                h.extrapolate(false)
            })
            .sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    use super::*;

    const INPUT: &str = r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "#;

    #[test]
    fn test_part1() {
        let day = Day9::new(INPUT.to_string());
        assert_eq!(day.part1(), "114");
    }

    #[test]
    fn test_part2() {
        let day = Day9::new(INPUT.to_string());
        assert_eq!(day.part2(), "2".to_string());
    }

    const INPUT_2: &str = "10 13 16 21 30 45";

    #[test]
    fn test_part2_single() {
        let day = Day9::new(INPUT_2.to_string());
        assert_eq!(day.part2(), "5".to_string());
    }
}