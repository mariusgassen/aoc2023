use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::aoc::Day;
use crate::aoc::tools::read_lines;

#[derive(Eq, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    joker: bool,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32, joker: bool) -> Self {
        Hand {
            cards,
            bid,
            joker,
        }
    }
    fn get_type(&self) -> Type {
        let mut cards = self.cards.clone();
        cards.sort();

        let mut counts = HashMap::new();
        for card in cards.iter() {
            let symbol = card.symbol.clone();
            let count = counts.entry(symbol).or_insert(0);
            *count += 1;
        }


        if self.joker {
            if let Some(num_jokers) = counts.remove(&Symbol::Joker) {
                match counts.iter().max_by_key(|(_, &v)| v) {
                    Some(k) => *counts.get_mut(&k.0.to_owned()).unwrap() += num_jokers,
                    None => { counts.entry(Symbol::Ace).or_insert(5); } // all jokers, pick aces
                };
            }
        }

        let highest_entry = counts
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();


        match highest_entry.1 {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match counts.values().filter(|v| **v == 2).count() {
                1 => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            },
            2 => match counts.values().filter(|v| **v == 2).count() {
                1 => Type::OnePair,
                2 => Type::TwoPair,
                _ => Type::HighCard,
            },
            _ => Type::HighCard,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() &&
            self.cards.len() == self.cards.len() &&
            self.cards == self.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let our_type = self.get_type();
        let other_type = other.get_type();

        match our_type.partial_cmp(&other_type) {
            Some(Ordering::Equal) => {
                for (card, card_other) in self.cards.iter().zip(&other.cards) {
                    if card > card_other {
                        return Some(Ordering::Greater);
                    }
                    if card < card_other {
                        return Some(Ordering::Less);
                    }
                }
                Some(Ordering::Equal)
            }
            o => o,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let our_type = self.get_type();
        let other_type = other.get_type();

        match our_type.cmp(&other_type) {
            Ordering::Equal => {
                for (card, card_other) in self.cards.iter().zip(&other.cards) {
                    if card > card_other {
                        return Ordering::Greater;
                    }
                    if card < card_other {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        match self.cmp(&other) {
            Ordering::Greater => self,
            _ => other
        }
    }

    fn min(self, other: Self) -> Self where Self: Sized {
        match self.cmp(&other) {
            Ordering::Greater => self,
            _ => other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, Self: PartialOrd {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Type {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
enum Symbol {
    Joker = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Joker => write!(f, "_"),
            Symbol::Two => write!(f, "2"),
            Symbol::Three => write!(f, "3"),
            Symbol::Four => write!(f, "4"),
            Symbol::Five => write!(f, "5"),
            Symbol::Six => write!(f, "6"),
            Symbol::Seven => write!(f, "7"),
            Symbol::Eight => write!(f, "8"),
            Symbol::Nine => write!(f, "9"),
            Symbol::Ten => write!(f, "T"),
            Symbol::Jack => write!(f, "J"),
            Symbol::Queen => write!(f, "Q"),
            Symbol::King => write!(f, "K"),
            Symbol::Ace => write!(f, "A"),
        }
    }
}

impl FromStr for Symbol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Symbol::Two),
            "3" => Ok(Symbol::Three),
            "4" => Ok(Symbol::Four),
            "5" => Ok(Symbol::Five),
            "6" => Ok(Symbol::Six),
            "7" => Ok(Symbol::Seven),
            "8" => Ok(Symbol::Eight),
            "9" => Ok(Symbol::Nine),
            "T" => Ok(Symbol::Ten),
            "J" => Ok(Symbol::Jack),
            "Q" => Ok(Symbol::Queen),
            "K" => Ok(Symbol::King),
            "A" => Ok(Symbol::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Eq, Clone)]
struct Card {
    symbol: Symbol,
}

impl Card {
    fn new(symbol: char, joker: bool) -> Self {
        let mut s = Symbol::from_str(&symbol.to_string()).unwrap();
        if s == Symbol::Jack && joker {
            s = Symbol::Joker;
        }
        Card {
            symbol: s,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol.to_string())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.symbol.partial_cmp(&other.symbol)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.symbol.cmp(&other.symbol)
    }
}

pub struct Day7 {
    input: String,
}

impl Day7 {
    pub fn new(input: String) -> Day7 {
        Day7 { input }
    }

    fn get_hands(&self, joker: bool) -> Vec<Hand> {
        read_lines(&self.input).iter()
            .map(|r| r.split_whitespace().collect())
            .map(|h: Vec<&str>|
                Hand::new(
                    h[0].chars()
                        .map(|c| Card::new(c, joker))
                        .collect(),
                    u32::from_str(&h[1]).unwrap(),
                    joker,
                )
            ).collect()
    }
}

impl Day for Day7 {
    fn part1(&self) -> String {
        let mut hands = self.get_hands(false);
        hands.sort();
        hands.iter().enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut hands = self.get_hands(true);
        hands.sort();
        let value = hands.iter().enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum::<u32>();

        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::*;

    use super::*;

    const INPUT: &str = r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#;

    fn day() -> Day7 {
        Day7::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().part1(), "6440");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().part2(), "5905");
    }

    #[test]
    fn test_day_1_full() {
        let input = read_input(7).unwrap();
        let d = Day7::new(input);
        assert_eq!(d.part1(), "250120186");
    }

    #[test]
    fn test_day_2_full() {
        let input = read_input(7).unwrap();
        let d = Day7::new(input);
        assert_eq!(d.part2(), "250665248");
    }

    #[test]
    fn test_symbol_order() {
        assert!(Symbol::Joker < Symbol::Two);
        assert!(Symbol::Two < Symbol::Three);
        assert!(Symbol::Three < Symbol::Four);
        assert!(Symbol::Four < Symbol::Five);
        assert!(Symbol::Five < Symbol::Six);
        assert!(Symbol::Six < Symbol::Seven);
        assert!(Symbol::Seven < Symbol::Eight);
        assert!(Symbol::Eight < Symbol::Nine);
        assert!(Symbol::Nine < Symbol::Ten);
        assert!(Symbol::Ten < Symbol::Jack);
        assert!(Symbol::Jack < Symbol::Queen);
        assert!(Symbol::Queen < Symbol::King);
        assert!(Symbol::King < Symbol::Ace);
    }

    #[test]
    fn test_hand_order() {
        assert!(Type::HighCard < Type::OnePair);
        assert!(Type::OnePair < Type::TwoPair);
        assert!(Type::TwoPair < Type::ThreeOfAKind);
        assert!(Type::ThreeOfAKind < Type::FullHouse);
        assert!(Type::FullHouse < Type::FourOfAKind);
        assert!(Type::FourOfAKind < Type::FiveOfAKind);
    }
}