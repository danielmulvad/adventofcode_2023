use std::str::FromStr;

use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct SeedMap {
    start: i64,
    end: i64,
    offset: i64,
}

impl FromStr for SeedMap {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

        if parts.len() != 3 {
            return Err("Input string must have exactly three parts");
        }

        let destination: i64 = parts[0].parse().map_err(|_| "Invalid destination")?;
        let source: i64 = parts[1].parse().map_err(|_| "Invalid source")?;
        let range: i64 = parts[2].parse().map_err(|_| "Invalid range")?;

        Ok(SeedMap {
            start: source,
            end: source + (range - 1),
            offset: destination - source,
        })
    }
}

impl SeedMap {
    fn transform(&self, input: i64) -> i64 {
        if input >= self.start && input <= self.end {
            return input + self.offset;
        }
        input
    }
}

#[derive(Debug)]
pub struct PartOne {
    input: String,
}

impl PartOne {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn run(&self) -> i64 {
        Self::part1(&self.input)
    }

    fn part1(input: &str) -> i64 {
        let mut inputs = input.trim().split("\n\n");

        // Processing seeds
        let seeds: Vec<i64> = inputs
            .next()
            .unwrap()
            .trim()
            .split_once(": ")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Processing transformers
        let transformers: Result<Vec<Vec<SeedMap>>, &'static str> = inputs
            .map(|x| x.trim().lines().skip(1).map(SeedMap::from_str).collect())
            .collect();
        let transformers = transformers.unwrap();

        // Parallel processing of seeds
        seeds
            .par_iter()
            .map(|&x| {
                debug!("Processing seed: {}", x);
                let result = transformers.iter().fold(x, |output, maps| {
                    maps.iter()
                        .find_map(|map| {
                            let result = map.transform(output);
                            (result != output).then(|| result)
                        })
                        .unwrap_or(output)
                });
                debug!("Finished processing seed: {} with result: {}", x, result);
                result
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test_data.txt");
        let output = PartOne::new(input).run();
        assert_eq!(output, 35);
    }
}
