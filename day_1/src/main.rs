use std::fs;

use day_1::{part_one::PartOne, part_two::PartTwo};

fn read_file_to_vector(path: &str) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();
    let file = fs::read(path).expect("Unable to read file");
    let file_string = String::from_utf8_lossy(&file);
    let file_string = file_string.to_string();
    let file_string = file_string.trim();
    let file_string = file_string.split("\n");
    for line in file_string {
        return_vec.push(line.to_string());
    }
    return_vec
}

fn main() {
    let calibration_data = read_file_to_vector("./input.txt");
    let part_one = PartOne::new(&calibration_data);
    let part_one_result = part_one.run();
    println!("Part One Result: {}", part_one_result);
    let part_two = PartTwo::new(&calibration_data);
    let part_two_result = part_two.run();
    println!("Part Two Result: {}", part_two_result);
}
