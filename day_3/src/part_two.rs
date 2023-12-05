use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

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

pub struct PartTwo {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}
impl PartTwo {
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
        let mut gear_product_sum = 0;

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

            if adjacent_numbers.len() == 2 {
                gear_product_sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
            }
        }

        gear_product_sum
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
    fn test_part_two() {
        let buf_reader = file_reader("./test_data.txt");
        let part_two = PartTwo::new(buf_reader).run();
        assert_eq!(part_two, 467835);
    }
}
