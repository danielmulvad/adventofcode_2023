/*
--- Part Two ---
The crucibles of lava simply aren't large enough to provide an adequate supply of lava to the machine parts factory. Instead, the Elves are going to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not only do they have trouble going in a straight line, but they also have trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move a maximum of ten consecutive blocks without turning.

In the above example, an ultra crucible could follow this path to minimize heat loss:

2>>>>>>>>1323
32154535v5623
32552456v4254
34465858v5452
45466578v>>>>
143859879845v
445787698776v
363787797965v
465496798688v
456467998645v
122468686556v
254654888773v
432267465553v
In the above example, an ultra crucible would incur the minimum possible heat loss of 94.

Here's another example:

111111111111
999999999991
999999999991
999999999991
999999999991
Sadly, an ultra crucible would need to take an unfortunate path like this one:

1>>>>>>>1111
9999999v9991
9999999v9991
9999999v9991
9999999v>>>>
This route causes the ultra crucible to incur the minimum possible heat loss of 71.

Directing the ultra crucible from the lava pool to the machine parts factory, what is the least heat loss it can incur?
*/

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashMap, VecDeque};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

struct Grid {
    boundaries: Position,
    grid: HashMap<Position, u32>,
}

impl Grid {
    fn parse_grid(input: &str) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    grid.insert(Position::new(x as i32, y as i32), digit);
                }
            }
        }
        let boundaries = Position::new(
            input.lines().next().unwrap().len() as i32,
            input.lines().count() as i32,
        );
        Grid { boundaries, grid }
    }

    fn successors(
        &self,
        (position, deque): &(Position, VecDeque<Position>),
    ) -> Vec<((Position, VecDeque<Position>), u32)> {
        let diffs: Vec<Position> = deque
            .iter()
            .tuple_windows()
            .map(|(a, b)| Position::new(a.x - b.x, a.y - b.y))
            .collect();
        let last_diff = diffs.get(0);

        let maybe_first_diff_count = diffs.iter().dedup_with_count().next();
        let options = if let Some(diff_count) = maybe_first_diff_count {
            let num_consecutive_straight_diffs = diff_count.0;
            let must_turn = num_consecutive_straight_diffs >= 10;
            let must_go_straight = num_consecutive_straight_diffs < 4;

            if must_turn {
                [
                    Position::new(1, 0),
                    Position::new(-1, 0),
                    Position::new(0, -1),
                    Position::new(0, 1),
                ]
                .into_iter()
                .filter(|option| Some(option) != last_diff)
                .collect::<Vec<Position>>()
            } else if must_go_straight {
                vec![*last_diff.unwrap()]
            } else {
                vec![
                    Position::new(1, 0),
                    Position::new(-1, 0),
                    Position::new(0, -1),
                    Position::new(0, 1),
                ]
            }
        } else {
            vec![
                Position::new(1, 0),
                Position::new(-1, 0),
                Position::new(0, -1),
                Position::new(0, 1),
            ]
        };

        options
            .into_par_iter()
            .filter_map(|pos_diff| {
                let next_position = Position::new(position.x + pos_diff.x, position.y + pos_diff.y);
                if (0..self.boundaries.x).contains(&next_position.x)
                    && (0..self.boundaries.y).contains(&next_position.y)
                {
                    if deque.len() > 2 && deque[1] == next_position {
                        return None;
                    }

                    let mut new_deque = deque.clone();
                    new_deque.push_front(next_position);

                    if new_deque.len() > 14 {
                        new_deque.pop_back();
                    }
                    Some((next_position, new_deque))
                } else {
                    None
                }
            })
            .map(|pos| {
                let next_cost = *self.grid.get(&pos.0).unwrap();
                (pos, next_cost)
            })
            .collect::<Vec<((Position, VecDeque<Position>), u32)>>()
    }

    fn success(goal: &Position, (win, deque): &(Position, VecDeque<Position>)) -> bool {
        let diffs: Vec<Position> = deque
            .iter()
            .tuple_windows()
            .map(|(a, b)| Position::new(a.x - b.x, a.y - b.y))
            .collect();
        let maybe_first_diff_count = diffs.iter().dedup_with_count().next();

        maybe_first_diff_count.is_some_and(|(count, _)| count >= 4) && win == goal
    }

    pub fn shortest_path(
        &self,
        start: Position,
        goal: Position,
    ) -> (Vec<(Position, VecDeque<Position>)>, u32) {
        let mut deque = VecDeque::new();
        deque.push_back(start);

        let result: (Vec<(Position, VecDeque<Position>)>, u32) = dijkstra(
            &(start, deque),
            |args| self.successors(&args),
            |args| Self::success(&goal, args),
        )
        .expect("should have a valid path");

        self.print();
        result
    }

    pub fn print(&self) {
        for y in 0..self.boundaries.y {
            for x in 0..self.boundaries.x {
                match self.grid.get(&Position::new(x, y)) {
                    Some(_) => {
                        print!("#");
                    }
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

pub fn run(input: &str) -> u32 {
    let row_count = input.lines().count() as i32;
    let column_count = input.lines().next().unwrap().len() as i32;
    let goal = Position::new(column_count - 1, row_count - 1);
    let start = Position::new(0, 0);

    let grid = Grid::parse_grid(input);
    let result = grid.shortest_path(start, goal);

    result.1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 94);
    }
}
