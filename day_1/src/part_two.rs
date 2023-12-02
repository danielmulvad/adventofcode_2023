/**
 * Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
 * Equipped with this new information, you now need to find the real first and last digit on each line. For example:
 * two1nine
 * eightwothree
 * abcone2threexyz
 * xtwone3four
 * 4nineeightseven2
 * zoneight234
 * 7pqrstsixteen
 * In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
 */

const NUMBERS: [(&str, u32); 9] =
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

pub struct PartTwo<'a> {
    calibration_values: &'a Vec<String>,
}

impl<'a> PartTwo<'a> {
    pub fn new(calibration_values: &'a Vec<String>) -> Self {
        Self { calibration_values }
    }

    pub fn run(&self) -> u32 {
        let result = self.calibration_values.iter().fold(0, |acc, x| {
            let num = Self::get_first_last_int_from_str(x);
            acc + num
        });
        result
    }

    fn find_lowest_index_number_from_str(string: &str) -> Option<(u32, usize)> {
        let mut lowest_index: Option<usize> = None;
        let mut lowest_index_number: Option<u32> = None;
        for (_, (number, value)) in NUMBERS.into_iter().enumerate() {
            let idx = string.find(number);
            match idx {
                Some(idx) => {
                    if lowest_index.is_none() || idx < lowest_index? {
                        lowest_index = Some(idx);
                        lowest_index_number = Some(value);
                    }
                }
                None => {}
            };
        }
        match (lowest_index, lowest_index_number) {
            (Some(lowest_index), Some(lowest_index_number)) => {
                Some((lowest_index_number, lowest_index))
            }
            _ => None,
        }
    }

    fn find_highest_index_number_from_str(string: &str) -> Option<(u32, usize)> {
        let mut highest_index: Option<usize> = None;
        let mut highest_index_number: Option<u32> = None;
        for (_, (number, value)) in NUMBERS.into_iter().enumerate() {
            let idx = string.rfind(number);
            match idx {
                Some(idx) => {
                    if highest_index.is_none() || idx > highest_index? {
                        highest_index = Some(idx);
                        highest_index_number = Some(value);
                    }
                }
                None => {}
            };
        }
        match (highest_index, highest_index_number) {
            (Some(highest_index), Some(highest_index_number)) => {
                Some((highest_index_number, highest_index))
            }
            _ => None,
        }
    }

    fn find_lowest_index_int_from_str(string: &str) -> Option<(u32, usize)> {
        for (i, c) in string.chars().enumerate() {
            if c.is_digit(10) {
                let c = c.to_digit(10).unwrap() as u32;
                return Some((c, i));
            }
        }
        None
    }

    fn find_highest_index_int_from_str(string: &str) -> Option<(u32, usize)> {
        for (i, c) in string.chars().rev().enumerate() {
            if c.is_digit(10) {
                let c = c.to_digit(10).unwrap() as u32;
                return Some((c, string.len() - i));
            }
        }
        None
    }

    fn get_first_last_int_from_str(string: &str) -> u32 {
        let first_number = Self::find_lowest_index_number_from_str(string);
        let last_number = Self::find_highest_index_number_from_str(string);

        let first_int = Self::find_lowest_index_int_from_str(string);
        let last_int = Self::find_highest_index_int_from_str(string);

        let first_digit = match (first_int, first_number) {
            (Some((first_int, first_int_index)), Some((first_number, first_number_index))) => {
                if first_int_index < first_number_index {
                    first_int
                } else {
                    first_number
                }
            }
            (Some((first_int, _)), None) => first_int,
            (None, Some((first_number, _))) => first_number,
            (None, None) => 0,
        };
        let last_digit = match (last_int, last_number) {
            (Some((last_int, last_int_index)), Some((last_number, last_number_index))) => {
                if last_int_index > last_number_index {
                    last_int
                } else {
                    last_number
                }
            }
            (Some((last_int, _)), None) => last_int,
            (None, Some((last_number, _))) => last_number,
            (None, None) => 0,
        };

        let two_digit_number = format!("{}{}", first_digit, last_digit);
        let two_digit_number = two_digit_number.parse::<u32>().unwrap();
        two_digit_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_index_number_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartTwo::find_lowest_index_number_from_str(string_one);
        assert_eq!(result_one, Some((6, 7)));

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartTwo::find_lowest_index_number_from_str(string_two);
        assert_eq!(result_two, Some((9, 6)));

        let string_three = "fx3";
        let result_three = PartTwo::find_lowest_index_number_from_str(string_three);
        assert_eq!(result_three, None);

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartTwo::find_lowest_index_number_from_str(string_four);
        assert_eq!(result_four, Some((9, 9)));

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartTwo::find_lowest_index_number_from_str(string_five);
        assert_eq!(result_five, Some((8, 20)));
    }

    #[test]
    fn test_find_highest_index_number_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartTwo::find_highest_index_number_from_str(string_one);
        assert_eq!(result_one, Some((6, 7)));

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartTwo::find_highest_index_number_from_str(string_two);
        assert_eq!(result_two, Some((9, 24)));

        let string_three = "fx3";
        let result_three = PartTwo::find_highest_index_number_from_str(string_three);
        assert_eq!(result_three, None);

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartTwo::find_highest_index_number_from_str(string_four);
        assert_eq!(result_four, Some((7, 13)));

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartTwo::find_highest_index_number_from_str(string_five);
        assert_eq!(result_five, Some((2, 40)));
    }

    #[test]
    fn test_find_lowest_index_int_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartTwo::find_lowest_index_int_from_str(string_one);
        assert_eq!(result_one, Some((7, 0)));

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartTwo::find_lowest_index_int_from_str(string_two);
        assert_eq!(result_two, Some((4, 23)));

        let string_three = "fx3";
        let result_three = PartTwo::find_lowest_index_int_from_str(string_three);
        assert_eq!(result_three, Some((3, 2)));

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartTwo::find_lowest_index_int_from_str(string_four);
        assert_eq!(result_four, Some((8, 0)));

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartTwo::find_lowest_index_int_from_str(string_five);
        assert_eq!(result_five, Some((7, 0)));
    }

    #[test]
    fn test_find_highest_index_int_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartTwo::find_highest_index_int_from_str(string_one);
        assert_eq!(result_one, Some((9, 21)));

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartTwo::find_highest_index_int_from_str(string_two);
        assert_eq!(result_two, Some((3, 30)));

        let string_three = "fx3";
        let result_three = PartTwo::find_highest_index_int_from_str(string_three);
        assert_eq!(result_three, Some((3, 3)));

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartTwo::find_highest_index_int_from_str(string_four);
        assert_eq!(result_four, Some((8, 1)));

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartTwo::find_highest_index_int_from_str(string_five);
        assert_eq!(result_five, Some((6, 20)));
    }

    #[test]
    fn test_get_first_last_int_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartTwo::get_first_last_int_from_str(string_one);
        assert_eq!(result_one, 79);

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartTwo::get_first_last_int_from_str(string_two);
        assert_eq!(result_two, 93);

        let string_three = "fx3";
        let result_three = PartTwo::get_first_last_int_from_str(string_three);
        assert_eq!(result_three, 33);

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartTwo::get_first_last_int_from_str(string_four);
        assert_eq!(result_four, 87);

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartTwo::get_first_last_int_from_str(string_five);
        assert_eq!(result_five, 72);
    }

    #[test]
    fn test_run() {
        let calibration_values = vec![
            "7jlncfksix7rjgrpglmn9".to_string(),
            "vcgkgxninerqjltdbhqzzpd4nine23".to_string(),
            "fx3".to_string(),
            "8nrbjbpjpnineseven".to_string(),
            "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo".to_string(),
        ];
        let part_two = PartTwo::new(&calibration_values);
        let result = part_two.run();
        assert_eq!(result, 364);
    }
}
