/*
*/

#[tracing::instrument(skip(_input))]
pub fn run(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test_data.txt");
        let output = run(input);
        assert_eq!(output, 0);
    }
}
