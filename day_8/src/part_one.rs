/*
--- Day 8: Haunted Wasteland ---
You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
*/

use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Map {
    left: String,
    right: String,
}

impl FromStr for Map {
    type Err = nom::Err<nom::error::Error<&'static str>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .trim_matches(|p| p == '(' || p == ')')
            .split_once(", ")
            .unwrap_or_else(|| panic!("Failed to parse: {}", s));
        let left = left.to_string();
        let right = right.to_string();
        Ok(Self { left, right })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .nth(0)
        .expect("Failed to get first line")
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        })
        .collect()
}

fn parse_map(input: &str) -> HashMap<&str, Map> {
    let mut map = HashMap::new();
    for line in input.lines().skip(2) {
        let (left, right) = line.split_once(" = ").unwrap();
        let right = right.parse::<Map>().unwrap();
        map.insert(left, right);
    }
    map
}

fn navigate_to_zzz(node_map: &HashMap<&str, Map>, directions: &[Direction]) -> u32 {
    let mut current_node = "AAA";
    let mut directions_index: usize = 0;
    let mut steps = 0;

    while current_node != "ZZZ" {
        let direction = &directions[directions_index];
        let map = node_map.get(current_node).unwrap();
        match direction {
            Direction::Left => current_node = &map.left,
            Direction::Right => current_node = &map.right,
        }
        directions_index += 1;
        if directions_index == directions.len() {
            directions_index = 0;
        }
        steps += 1;
    }

    steps
}

#[tracing::instrument]
pub fn run(input: &str) -> u32 {
    let directions = parse_directions(input);
    let map = parse_map(input);
    navigate_to_zzz(&map, &directions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 6);
    }
}
