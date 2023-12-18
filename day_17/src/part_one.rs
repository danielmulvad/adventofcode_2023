/*
--- Day 17: Clumsy Crucible ---
The lava starts flowing rapidly once the Lava Production Facility is operational. As you leave, the reindeer offers you a parachute, allowing you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had trouble finding anyone on your way up: half of Gear Island is empty, but the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new lavafall. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large crucibles on wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll need to find the best way to get the crucible from the lava pool to the machine parts factory. To do this, you need to minimize heat loss while choosing a route that doesn't require the crucible to go in a straight line for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>
This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only 102.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?
*/

use std::str::FromStr;

use pathfinding::directed::dijkstra::dijkstra;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    last_dx: isize,    // Last movement in x
    last_dy: isize,    // Last movement in y
    move_count: usize, // Count of moves in the current direction
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new(tiles: Vec<Vec<u8>>) -> Self {
        Self { tiles }
    }

    pub fn neighbors(&self, pos: &Position) -> Vec<(Position, u32)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

        for &(dx, dy) in &directions {
            // Check if the crucible can move in this direction
            if (dx, dy) != (-pos.last_dx, -pos.last_dy) {
                // Cannot reverse direction
                let new_x = (pos.x as isize + dx) as usize;
                let new_y = (pos.y as isize + dy) as usize;

                if new_x < self.tiles.len() && new_y < self.tiles[0].len() {
                    let same_direction = (dx, dy) == (pos.last_dx, pos.last_dy);
                    let move_count = if same_direction {
                        pos.move_count + 1
                    } else {
                        1
                    };

                    if move_count <= 3 {
                        let cost = self.tiles[new_x][new_y] as u32;
                        let new_pos = Position {
                            x: new_x,
                            y: new_y,
                            last_dx: dx,
                            last_dy: dy,
                            move_count,
                        };
                        neighbors.push((new_pos, cost));
                    }
                }
            }
        }

        neighbors
    }

    pub fn shortest_path(&self) -> u32 {
        let start = Position {
            x: 0,
            y: 0,
            last_dx: 0,
            last_dy: 0,
            move_count: 0,
        };
        let end = Position {
            x: self.tiles.len() - 1,
            y: self.tiles[0].len() - 1,
            last_dx: 0,
            last_dy: 0,
            move_count: 0,
        };

        if let Some((_, cost)) = dijkstra(
            &start,
            |p| self.neighbors(p),
            |p| p.x == end.x && p.y == end.y,
        ) {
            cost
        } else {
            u32::MAX
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect::<Vec<_>>();
        Ok(Grid::new(tiles))
    }
}

pub fn run(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    grid.shortest_path()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 102);
    }
}
