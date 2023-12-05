use day_3::{file_reader, part_one::PartOne, part_two::PartTwo};

fn main() {
    let buf_reader = file_reader("./input.txt");
    let part_one = PartOne::new(buf_reader).run();
    println!("Part One: {}", part_one);

    let buf_reader = file_reader("./input.txt");
    let part_two = PartTwo::new(buf_reader).run();
    println!("Part Two: {}", part_two);
}
