use day_3::{file_reader, part_one::PartOne, part_two::PartTwo};
use std::io::BufRead;

fn main() {
    let buf_reader = file_reader("./input.txt");
    let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap_or_default()).collect();

    let part_one = PartOne::new(&lines).run();
    println!("Part One: {}", part_one);

    let part_two = PartTwo::new(&lines).run();
    println!("Part Two: {}", part_two);
}
