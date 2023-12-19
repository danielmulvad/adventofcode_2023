/*
--- Day 18: Lavaduct Lagoon ---
Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######
Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?
*/

use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct HexColor {
    #[allow(dead_code)]
    red: u8,
    #[allow(dead_code)]
    green: u8,
    #[allow(dead_code)]
    blue: u8,
}

impl FromStr for HexColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let red = u8::from_str_radix(&s[2..4], 16).unwrap();
        let green = u8::from_str_radix(&s[4..6], 16).unwrap();
        let blue = u8::from_str_radix(&s[6..8], 16).unwrap();
        Ok(HexColor { red, green, blue })
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
    #[allow(dead_code)]
    color: HexColor,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = parts.next().unwrap().parse::<Direction>()?;
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        let color = parts.next().unwrap().parse::<HexColor>()?;
        Ok(Instruction {
            direction,
            distance,
            color,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position(i32, i32);
#[derive(Debug)]
struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect::<Vec<Instruction>>();
        Ok(Instructions(instructions))
    }
}

#[tracing::instrument(skip(input))]
pub fn run(input: &str) -> i32 {
    let grid_size = 1000;
    let start_pos = (grid_size / 2, grid_size / 2);
    let mut position = start_pos;
    let mut trench = HashSet::new();

    let instructions = Instructions::from_str(input).unwrap().0;
    for instruction in instructions {
        let direction = instruction.direction;
        let length = instruction.distance;
        for _ in 0..length {
            match direction {
                Direction::Up => position.1 -= 1,
                Direction::Down => position.1 += 1,
                Direction::Left => position.0 -= 1,
                Direction::Right => position.0 += 1,
            }
            trench.insert(position);
        }
    }

    // Flood fill to find the interior
    let mut outside = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((x, y)) = queue.pop_front() {
        if !trench.contains(&(x, y))
            && (x >= 0 && x < grid_size && y >= 0 && y < grid_size)
            && outside.insert((x, y))
        {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_x = x + dx;
                let next_y = y + dy;
                if next_x >= 0 && next_x < grid_size && next_y >= 0 && next_y < grid_size {
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }

    let total_volume = (0..grid_size)
        .flat_map(|x| (0..grid_size).map(move |y| (x, y)))
        .filter(|&pos| trench.contains(&pos) || !outside.contains(&pos))
        .count();

    total_volume as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 62);
    }
}
