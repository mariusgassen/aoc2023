use std::str::FromStr;
use std::string::ParseError;

use crate::aoc::Day;
use crate::aoc::tools::read_lines;

struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card::new(self.id, self.winning_numbers.clone(), self.numbers.clone())
    }
}

impl Card {
    fn new(id: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Card {
        Card { id, winning_numbers, numbers }
    }

    fn get_winners(&self) -> Vec<usize> {
        self.numbers.iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|n| *n)
            .collect::<Vec<usize>>()
    }

    fn read_numbers(s: &str) -> Vec<usize> {
        s.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }
}

struct Table {
    cards: Vec<Card>,
}

impl Table {
    fn new(cards: Vec<Card>) -> Table {
        Table { cards }
    }

    fn get_value(&self) -> usize {
        self.cards.iter()
            // Get number of winning numbers for each card
            .map(|c| c.get_winners().len())
            // Remove cards with no winning numbers
            .filter(|s| *s > 0)
            // Value is 2 ^ (w-1) for each card
            .map(|w| 1 << (w - 1))
            .sum()
    }

    fn play(&mut self) -> usize {
        let mut total_cards = 0;

        let mut hand = Vec::new();
        hand.append(&mut self.cards.clone());

        while hand.len() > 0 {
            let card = hand.pop().unwrap();
            total_cards += 1;

            let winners = card.get_winners().len();
            if winners == 0 {
                continue;
            }

            for i in card.id + 1..card.id + winners + 1 {
                match self.get_copy(i) {
                    Some(copy) => hand.push(copy),
                    None => (),
                }
            }
        }

        total_cards
    }
    fn get_copy(&self, id: usize) -> Option<Card> {
        if id > self.cards.len() {
            return None;
        }
        Some(self.cards[id - 1].clone())
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();
        let id = parts[0]
            .strip_prefix("Card").unwrap().trim()
            .parse::<usize>().unwrap();

        let number_parts = parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = Card::read_numbers(number_parts[0]);
        let numbers = Card::read_numbers(number_parts[1]);
        Ok(Card::new(id, winning_numbers, numbers))
    }
}

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new(input: String) -> Day4 {
        Day4 { input }
    }

    fn get_table(&self) -> Table {
        let cards = read_lines(&self.input)
            .iter()
            .map(|l| l.parse::<Card>().unwrap())
            .collect::<Vec<Card>>();

        Table::new(cards)
    }
}

impl Day for Day4 {
    fn part1(&self) -> String {
        self.get_table().get_value().to_string()
    }

    fn part2(&self) -> String {
        // Get the table
        self.get_table().play().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    const INPUT: &str = r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    fn day() -> super::Day4 {
        super::Day4::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().part1(), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().part2(), "30");
    }
}