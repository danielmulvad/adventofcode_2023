/*
--- Day 16: The Floor Will Be Lava ---
With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

If the beam encounters empty space (.), it continues in the same direction.
If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..
Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?
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
    let starting_position = (0, 0);
    let starting_direction = (1, 0);
    let mut visited_positions = HashSet::new();
    grid.parse_beam(
        starting_position,
        starting_direction,
        &mut visited_positions,
    );
    let energized_tiles = grid.get_energized_tiles();
    energized_tiles.len() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 46);
    }
}
