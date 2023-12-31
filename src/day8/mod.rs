use std::str::FromStr;
use std::thread;

use crate::aoc::Day;

pub struct Day8 {
    input: String,
}

struct Journey {
    current: String,
    instructions: Vec<Instruction>,
    current_instruction: u32,
    network: Network,
    steps: u32,
}

impl Journey {
    fn new(instructions: Vec<Instruction>, network: Network) -> Self {
        let start = String::from("AAA");
        Self {
            instructions,
            current_instruction: 0,
            network,
            current: start,
            steps: 0,
        }
    }

    fn set_start(&mut self, start: String) {
        self.current = start.clone();
    }

    pub fn travel(&mut self) {
        let mut current_node = self.network.get_node(&self.current).unwrap();

        // Reset
        if self.current_instruction == self.instructions.len() as u32 {
            self.current_instruction = 0;
        }

        let instruction = self.instructions[self.current_instruction as usize].clone();

        self.steps += 1;
        match instruction {
            Instruction::L => {
                current_node = self.network.get_node(&current_node.left).unwrap();
            }
            Instruction::R => {
                current_node = self.network.get_node(&current_node.right).unwrap();
            }
        }
        self.current = current_node.id.clone();
        self.current_instruction += 1;
    }

    fn is_end(&self) -> bool {
        self.current.ends_with("Z")
    }
}

#[derive(Clone)]
enum Instruction {
    L,
    R,
}

impl Instruction {
    fn new(input: &str) -> Self {
        match input {
            "L" => Self::L,
            "R" => Self::R,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction::new(s))
    }
}

#[derive(Clone)]
struct Network {
    nodes: Vec<Node>,
}

impl Network {
    fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter()
            .find(|n| n.id == id)
    }
}

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn new(id: String, left: String, right: String) -> Self {
        Self { id, left, right }
    }

    fn is_start(&self) -> bool {
        self.id.ends_with("A")
    }
}

impl Day8 {
    pub(crate) fn new(input: String) -> Self {
        Self { input }
    }

    fn split_input(s: &String) -> Vec<String> {
        s
            .trim()
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn parse_instructions(s: &String) -> Vec<Instruction> {
        s.chars()
            .map(|s| Instruction::new(s.to_string().trim()))
            .collect()
    }

    fn parse_nodes(s: &String) -> Vec<Node> {
        s.split("\n")
            .map(|s| {
                let mut parts = s.split(" = ");
                let id = parts.next().unwrap().trim().to_string();

                let mut parts = parts.next().unwrap()
                    .strip_prefix("(").unwrap()
                    .strip_suffix(")").unwrap()
                    .split(",");

                let left = parts.next().unwrap().trim().to_string();
                let right = parts.next().unwrap().trim().to_string();
                Node::new(id, left, right)
            })
            .collect()
    }

    fn get_network(&self) -> Network {
        let data = Day8::split_input(&self.input);
        let nodes = Day8::parse_nodes(&data[1]);
        Network::new(nodes)
    }

    fn get_instructions(&self) -> Vec<Instruction> {
        let data = Day8::split_input(&self.input);
        Day8::parse_instructions(&data[0])
    }
    fn get_journey(&self) -> Journey {
        let data = Day8::split_input(&self.input);
        let instructions = Day8::parse_instructions(&data[0]);
        let nodes = Day8::parse_nodes(&data[1]);

        let network = Network::new(nodes);
        Journey::new(instructions, network)
    }
}

impl Day for Day8 {
    fn part1(&self) -> String {
        let mut journey = self.get_journey();
        while !journey.is_end() {
            journey.travel();
        }
        journey.steps.to_string()
    }

    fn part2(&self) -> String {
        let network = self.get_network();
        let instructions = self.get_instructions();
        let mut journeys: Vec<Journey> = network.nodes.clone().iter()
            .filter(|n| n.is_start())
            .map(|n| {
                let mut journey = Journey::new(instructions.clone(), network.clone());
                journey.set_start(n.id.clone());
                journey
            })
            .collect();

        // TODO: To many iterations, need a shortcut (gcd of each journey path?)
        loop {
            // println!("Iteration: {}", i);
            let done = thread::scope(|s| {
                let handles = journeys.iter_mut()
                    .map(|j| s.spawn(|| {
                        j.travel();
                        j
                    }))
                    .collect::<Vec<_>>();

                handles.into_iter()
                    .map(|handle| handle.join().unwrap())
                    .all(|j| j.is_end())
            });

            if done {
                break;
            }
        }

        journeys.iter()
            .map(|j| j.steps)
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    "#;

    const INPUT_2: &str = r#"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "#;

    const INPUT_3: &str = r#"
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    "#;

    #[test]
    fn test_part1() {
        let day = Day8::new(INPUT.to_string());
        assert_eq!(day.part1(), "2");
    }

    #[test]
    fn test_part_1_loop() {
        let day = Day8::new(INPUT_2.to_string());
        assert_eq!(day.part1(), "6");
    }

    #[test]
    fn test_part2() {
        let day = Day8::new(INPUT_3.to_string());
        assert_eq!(day.part2(), "6");
    }
}