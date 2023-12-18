/*
--- Part Two ---
As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..
Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?
*/

use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq)]
enum SplitterType {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
enum PositionType {
    Empty,
    UpwardMirror,
    DownwardMirror,
    Splitter(SplitterType),
}

#[derive(Debug)]
struct Tile {
    position_type: PositionType,
    energized: bool,
}

type Position = (i32, i32);

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn find_max_energized_tiles(&mut self) -> (Position, i32) {
        let mut max_energized_tiles = 0;
        let mut max_energized_tiles_position = (0, 0);
        let height = self.tiles.len();
        let width = self.tiles[0].len();

        // Iterate over all possible starting edges
        for y in 0..height {
            for x in 0..width {
                // Skip non-edge tiles
                if y != 0 && y != height - 1 && x != 0 && x != width - 1 {
                    continue;
                }

                let starting_position = (x as i32, y as i32);
                let starting_direction = self.get_starting_direction(x, y, width, height);
                let mut visited_positions = HashSet::new();
                self.parse_beam(
                    starting_position,
                    starting_direction,
                    &mut visited_positions,
                );
                let energized_tiles = self.get_energized_tiles();
                if energized_tiles.len() > max_energized_tiles {
                    max_energized_tiles = energized_tiles.len();
                    max_energized_tiles_position = starting_position;
                }
                self.reset_tiles();
            }
        }
        (max_energized_tiles_position, max_energized_tiles as i32)
    }

    fn get_starting_direction(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> (i32, i32) {
        match (x, y) {
            (0, _) => (1, 0),                     // Left edge, move right
            (_, 0) => (0, 1),                     // Top edge, move down
            (x, _) if x == width - 1 => (-1, 0),  // Right edge, move left
            (_, y) if y == height - 1 => (0, -1), // Bottom edge, move up
            _ => unreachable!(),
        }
    }

    fn reset_tiles(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                tile.energized = false;
            }
        }
    }

    fn get_energized_tiles(&self) -> Vec<Position> {
        let mut energized_tiles = Vec::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.energized {
                    energized_tiles.push((x as i32, y as i32));
                }
            }
        }
        energized_tiles
    }

    fn parse_beam(
        &mut self,
        starting_position: Position,
        starting_direction: (i32, i32),
        visited_positions: &mut HashSet<(Position, (i32, i32))>,
    ) {
        let mut current_position = starting_position;
        let mut current_direction = starting_direction;

        loop {
            if current_position.0 >= self.tiles[0].len() as i32
                || current_position.1 >= self.tiles.len() as i32
                || current_position.0 < 0
                || current_position.1 < 0
            {
                break; // Out of bounds
            }

            let position_direction = (current_position, current_direction);
            if visited_positions.contains(&position_direction) {
                break; // Loop detected
            }
            visited_positions.insert(position_direction);

            let current_tile =
                &mut self.tiles[current_position.1 as usize][current_position.0 as usize];
            current_tile.energized = true;

            match &current_tile.position_type {
                PositionType::Empty => {
                    // Continue in the same direction
                }
                PositionType::UpwardMirror => {
                    current_direction = match current_direction {
                        (1, 0) => (0, -1),
                        (0, -1) => (1, 0),
                        (-1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        _ => unreachable!(),
                    };
                }
                PositionType::DownwardMirror => {
                    current_direction = match current_direction {
                        (1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (-1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => unreachable!(),
                    };
                }
                PositionType::Splitter(splitter_type) => {
                    let (dx, dy) = current_direction;
                    if (dx != 0 && (*splitter_type) == SplitterType::Horizontal)
                        || (dy != 0 && (*splitter_type) == SplitterType::Vertical)
                    {
                        // The beam is aligned with the splitter, pass through it
                        // No additional action needed here, as the beam will continue in the same direction
                    } else {
                        // The beam is perpendicular to the splitter, split it
                        if dx != 0 {
                            // Horizontal beam, split it vertically
                            self.parse_beam(current_position, (0, 1), visited_positions);
                            self.parse_beam(current_position, (0, -1), visited_positions);
                        } else if dy != 0 {
                            // Vertical beam, split it horizontally
                            self.parse_beam(current_position, (1, 0), visited_positions);
                            self.parse_beam(current_position, (-1, 0), visited_positions);
                        }
                        return; // End current beam's path after splitting
                    }
                }
            }

            current_position.0 += current_direction.0;
            current_position.1 += current_direction.1;
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let position_type = match c {
                    '.' => PositionType::Empty,
                    '/' => PositionType::UpwardMirror,
                    '\\' => PositionType::DownwardMirror,
                    '|' => PositionType::Splitter(SplitterType::Vertical),
                    '-' => PositionType::Splitter(SplitterType::Horizontal),
                    _ => panic!("Invalid character"),
                };
                row.push(Tile {
                    position_type,
                    energized: false,
                });
            }
            tiles.push(row);
        }
        Ok(Grid { tiles })
    }
}

#[tracing::instrument(skip(input))]
pub fn run(input: &str) -> i32 {
    let mut grid = Grid::from_str(input).unwrap();
    let energized_tiles = grid.find_max_energized_tiles();
    energized_tiles.1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 51);
    }
}
