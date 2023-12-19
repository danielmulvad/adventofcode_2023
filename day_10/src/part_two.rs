/*
--- Part Two ---
You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....
In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?
*/

use std::collections::{HashMap, VecDeque};

#[tracing::instrument(skip(input))]
pub fn run(input: &str) -> i32 {
    let mut m = Vec::new();
    for line in input.lines() {
        m.push(line);
    }

    let n = HashMap::from([
        ('|', vec![(0, -1), (0, 1)]),
        ('-', vec![(-1, 0), (1, 0)]),
        ('L', vec![(0, -1), (1, 0)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('7', vec![(-1, 0), (0, 1)]),
        ('F', vec![(1, 0), (0, 1)]),
    ]);

    let mut start = (0, 0);
    let mut found = false;

    for (yi, line) in m.iter().enumerate() {
        if let Some(xi) = line.chars().position(|c| c == 'S') {
            start = (xi, yi);
            found = true;
            break;
        }
    }

    assert!(found);

    let mut q = VecDeque::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dx, dy) in &directions {
        let new_x = start.0 as isize + dx;
        let new_y = start.1 as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            if let Some(c) = m
                .get(new_y as usize)
                .and_then(|line| line.chars().nth(new_x as usize))
            {
                if let Some(dirs) = n.get(&c) {
                    for &(dx2, dy2) in dirs {
                        let next_x = new_x + dx2;
                        let next_y = new_y + dy2;
                        if next_x >= 0 && next_y >= 0 {
                            q.push_back((1, (next_x as usize, next_y as usize)));
                        }
                    }
                }
            }
        }
    }

    let mut dists = HashMap::from([(start, 0)]);

    while let Some((d, (x, y))) = q.pop_front() {
        if dists.contains_key(&(x, y)) {
            continue;
        }

        dists.insert((x, y), d);

        if let Some(dirs) = n.get(&m[y].chars().nth(x).unwrap()) {
            for &(dx, dy) in dirs {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if new_x >= 0 && new_y >= 0 {
                    q.push_back((d + 1, (new_x as usize, new_y as usize)));
                }
            }
        }
    }

    let w = m[0].len();
    let h = m.len();
    let mut inside_count = 0;
    for (y, line) in m.iter().enumerate() {
        for (x, _) in line.chars().enumerate() {
            if dists.contains_key(&(x, y)) {
                continue;
            }

            let mut crosses = 0;
            let (mut x2, mut y2) = (x, y);

            while x2 < w && y2 < h {
                let c2 = m[y2].chars().nth(x2).unwrap();
                if dists.contains_key(&(x2, y2)) && c2 != 'L' && c2 != '7' {
                    crosses += 1;
                }
                x2 += 1;
                y2 += 1;
            }

            if crosses % 2 == 1 {
                inside_count += 1;
            }
        }
    }

    inside_count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 4);
    }
}
