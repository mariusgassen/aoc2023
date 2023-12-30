use std::str::FromStr;

use crate::aoc::Day;
use crate::aoc::tools::{read_columns, read_lines};

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new(input: String) -> Day6 {
        Day6 { input }
    }

    fn get_races(&self) -> Vec<Race> {
        read_columns(&self.input).iter().skip(1)
            .map(|c| Race::new(
                u64::from_str(&c[0]).unwrap(),
                u64::from_str(&c[1]).unwrap(),
            ))
            .collect()
    }

    fn get_race(&self) -> Race {
        let rows = read_lines(&self.input);

        let res: Vec<u64> = rows.iter()
            .map(|r| r.split_whitespace().skip(1)
                .fold("".to_string(), |acc, s| acc + s))
            .map(|s| u64::from_str(&s).unwrap())
            .collect();

        Race::new(res[0], res[1])
    }
}

struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    fn new(duration: u64, record: u64) -> Self {
        Race {
            duration,
            record,
        }
    }

    fn possible_wins(&self) -> u64 {
        let mut wins = 0;
        for speed in 0..=self.duration {
            let distance = speed * (self.duration - speed);
            if distance > self.record {
                wins += 1;
            }
        }
        wins
    }
}

impl Day for Day6 {
    fn part1(&self) -> String {
        self.get_races().iter()
            .map(|r| r.possible_wins())
            .product::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.get_race().possible_wins().to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    const INPUT: &str = r#"
        Time:      7  15   30
        Distance:  9  40  200
    "#;

    fn day() -> super::Day6 {
        super::Day6::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().run().0, "288");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().run().1, "71503");
    }
}
