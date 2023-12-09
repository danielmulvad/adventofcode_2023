/*
--- Part Two ---
Of course, it would be nice to have even more history included in your report. Surely it's safe to just extrapolate backwards as well, right?

For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the beginning of your sequence of zeroes, then fill in new first values for each previous sequence.

In particular, here is what the third example history looks like when extrapolating back in time:

5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0
Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: 5.

Doing this for the remaining example data above results in previous values of -3 for the first history and 0 for the second history. Adding all three new values together produces 2.

Analyze your OASIS report again, this time extrapolating the previous value for each history. What is the sum of these extrapolated values?

*/
fn get_previous_value(history: &[i32]) -> i32 {
    let mut sequences = vec![history.to_vec()];

    while sequences.last().unwrap().iter().any(|&x| x != 0) {
        let last_sequence = sequences.last().unwrap();
        let diffs: Vec<i32> = last_sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        sequences.push(diffs);
    }

    for i in (1..sequences.len()).rev() {
        let diff = sequences[i][0];
        let new_value = sequences[i - 1][0] - diff;
        sequences[i - 1].insert(0, new_value);
    }

    sequences[0][0]
}

fn sum_of_previous_values(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .map(|history| get_previous_value(history))
        .sum()
}

#[tracing::instrument(skip(input))]
pub fn run(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let result = sum_of_previous_values(&reports);
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 2);
    }
}
