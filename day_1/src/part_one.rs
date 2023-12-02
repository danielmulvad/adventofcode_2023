/**
 * --- Day 1: Trebuchet?! ---
 * Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
 * You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
 * Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
 * You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
 * As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
 * The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
 * For example:
 * 1abc2
 * pqr3stu8vwx
 * a1b2c3d4e5f
 * treb7uchet
 * In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
 * Consider your entire calibration document. What is the sum of all of the calibration values?
*/

pub struct PartOne<'a> {
    calibration_values: &'a Vec<String>,
}

impl<'a> PartOne<'a> {
    pub fn new(calibration_values: &'a Vec<String>) -> Self {
        Self { calibration_values }
    }

    pub fn run(&self) -> i32 {
        let result = self.calibration_values.iter().fold(0, |acc, x| {
            let num = Self::get_first_last_int_from_str(x);
            acc + num
        });
        result
    }

    fn get_first_last_int_from_str(string: &str) -> i32 {
        let mut first_digit: i32 = 0;
        let mut last_digit: i32 = 0;
        let mut first_digit_found: bool = false;
        for (_, c) in string.chars().enumerate() {
            if c.is_digit(10) && !first_digit_found {
                first_digit = c.to_digit(10).unwrap() as i32;
                first_digit_found = true;
            }
            if c.is_digit(10) && first_digit_found {
                last_digit = c.to_digit(10).unwrap() as i32;
            }
        }
        let two_digit_number = format!("{}{}", first_digit, last_digit);
        let two_digit_number = two_digit_number.parse::<i32>().unwrap();
        two_digit_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_last_int_from_str() {
        let string_one = "7jlncfksix7rjgrpglmn9";
        let result_one = PartOne::get_first_last_int_from_str(string_one);
        assert_eq!(result_one, 79);

        let string_two = "vcgkgxninerqjltdbhqzzpd4nine23";
        let result_two = PartOne::get_first_last_int_from_str(string_two);
        assert_eq!(result_two, 43);

        let string_three = "fx3";
        let result_three = PartOne::get_first_last_int_from_str(string_three);
        assert_eq!(result_three, 33);

        let string_four = "8nrbjbpjpnineseven";
        let result_four = PartOne::get_first_last_int_from_str(string_four);
        assert_eq!(result_four, 88);

        let string_five = "7qlfhcsnxn7fpfhjcgr6eightsevenjlpchjtzpztwo";
        let result_five = PartOne::get_first_last_int_from_str(string_five);
        assert_eq!(result_five, 76);
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
        let part_one = PartOne::new(&calibration_values);
        let result = part_one.run();
        assert_eq!(result, 79 + 43 + 33 + 88 + 76);
    }
}
