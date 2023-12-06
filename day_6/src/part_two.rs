use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
/*
--- Part Two ---
As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200
...now instead means this:

Time:      71530
Distance:  940200
Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?
*/
#[derive(Debug)]
pub struct Race {
    total_time: i64,
    record_distance: i64,
}

impl Race {
    fn new(total_time: i64, record_distance: i64) -> Self {
        Self {
            total_time,
            record_distance,
        }
    }

    fn calculate_ways_to_win(&self) -> i64 {
        let mut ways_to_win = 0;
        for x in 0..self.total_time {
            let distance = x * (self.total_time - x);
            if distance > self.record_distance {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }
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
        let races = self.parse_input();
        let ways_to_win_per_race: Vec<i64> = races
            .par_iter()
            .map(|race| race.calculate_ways_to_win())
            .collect();
        ways_to_win_per_race.par_iter().map(|&x| x as i64).product()
    }

    fn parse_input(&self) -> Vec<Race> {
        let mut lines = self.input.lines();

        let time_line = lines
            .next()
            .unwrap_or("")
            .split_whitespace()
            .skip(1)
            .collect::<String>();
        let distance_line = lines
            .next()
            .unwrap_or("")
            .split_whitespace()
            .skip(1)
            .collect::<String>();

        let total_time = time_line.parse::<i64>().unwrap_or(0);
        let record_distance = distance_line.parse::<i64>().unwrap_or(0);

        vec![Race::new(total_time, record_distance)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = PartTwo::new(input).run();
        assert_eq!(output, 288);
    }
}
