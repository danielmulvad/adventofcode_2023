use core::ops::Range;
/**
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug)]
struct Symbol {
    line_num: usize,
    range: Range<usize>,
}

#[derive(Debug)]
struct Number {
    line_num: usize,
    range: Range<usize>,
    value: usize,
}

pub struct PartOne {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}
impl PartOne {
    fn parse_lines(buf_reader: BufReader<File>) -> (Vec<Number>, Vec<Symbol>) {
        let num_regex = Regex::new(r"(\d+)").unwrap();
        let symbol_regex = Regex::new(r"([^a-zA-z\d.\n])").unwrap();

        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        for (index, line) in buf_reader.lines().map(Result::unwrap).enumerate() {
            numbers.extend(num_regex.find_iter(&line).map(|m| Number {
                line_num: index,
                range: m.range(),
                value: m.as_str().parse().unwrap(),
            }));

            symbols.extend(symbol_regex.find_iter(&line).map(|sym| Symbol {
                line_num: index,
                range: sym.range(),
            }));
        }

        (numbers, symbols)
    }

    pub fn new(buf_reader: BufReader<File>) -> Self {
        let (numbers, symbols) = Self::parse_lines(buf_reader);
        Self { numbers, symbols }
    }

    pub fn run(&self) -> usize {
        let mut part_sum = 0;

        for symbol in &self.symbols {
            let symbol_range = symbol.range.start - 1..symbol.range.end + 1;

            let adjacent_numbers: Vec<_> = self
                .numbers
                .iter()
                .filter(|num| {
                    let num_range = num.range.start..num.range.end;
                    num.line_num >= symbol.line_num.saturating_sub(1)
                        && num.line_num <= symbol.line_num + 1
                        && num_range.overlaps(&symbol_range)
                })
                .collect();

            part_sum += adjacent_numbers.iter().map(|num| num.value).sum::<usize>();
        }

        part_sum
    }
}

trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for Range<usize> {
    fn overlaps(&self, other: &Self) -> bool {
        max(self.start, other.start) < min(self.end, other.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_reader;

    use super::*;

    #[test]
    fn test_part_one() {
        let buf_reader = file_reader("./test_data.txt");
        let part_one = PartOne::new(buf_reader).run();
        assert_eq!(part_one, 4361);
    }
}
