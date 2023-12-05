use std::str::FromStr;

use itertools::Itertools;
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
#[derive(Copy, Clone, Debug)]
struct Seed {
    start: i64,
    range: i64,
}

#[derive(Debug)]
pub struct PartTwo {
    input: String,
}

impl PartTwo {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn run(&self) -> i64 {
        Self::part2(&self.input)
    }

    fn prune_seeds(seeds: &[Seed]) -> Vec<Seed> {
        let mut output: Vec<Seed> = Vec::new();
        let mut end = -1;
        for seed in seeds {
            if seed.start + seed.range - 1 <= end {
                continue;
            }
            if seed.start > end {
                output.push(Seed {
                    start: seed.start,
                    range: seed.range,
                });
                end = seed.start + seed.range - 1;
                continue;
            }
            output.push(Seed {
                start: end + 1,
                range: seed.start + seed.range - end,
            });
            end = seed.start + seed.range - 1;
        }
        output
    }

    fn part2(input: &str) -> i64 {
        let mut inputs = input.trim().split("\n\n");

        // Processing seeds
        let seeds: Vec<Seed> = {
            let seed_input = inputs.next().unwrap().trim().split_once(": ").unwrap().1;
            seed_input
                .split_whitespace()
                .tuples()
                .map(|(start, range)| Seed {
                    start: start.parse().unwrap(),
                    range: range.parse().unwrap(),
                })
                .sorted_by_key(|seed| seed.start)
                .collect()
        };
        let pruned_seeds = Self::prune_seeds(&seeds);

        // Processing transformers
        let transformers: Result<Vec<Vec<SeedMap>>, &'static str> = inputs
            .map(|x| x.trim().lines().skip(1).map(SeedMap::from_str).collect())
            .collect();
        let transformers = transformers.unwrap();

        pruned_seeds
            .par_iter()
            .map(|x| {
                debug!("Processing seed: {:?}", x);
                let result = (x.start..x.start + x.range)
                    .map(|i| {
                        transformers.iter().fold(i, |output, maps| {
                            maps.iter()
                                .find_map(|map| {
                                    let result = map.transform(output);
                                    if result == output {
                                        None
                                    } else {
                                        Some(result)
                                    }
                                })
                                .unwrap_or(output)
                        })
                    })
                    .min()
                    .unwrap_or(i64::MAX);
                debug!("Finished parsing seed {:?} with result {}", x, result);
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
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = PartTwo::new(input).run();
        assert_eq!(output, 46);
    }
}
