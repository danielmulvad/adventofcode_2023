use day_3::part_one::PartOne;
use day_3::part_two::PartTwo;
use dotenv::dotenv;
use log::info;

fn setup() {
    dotenv().ok();
    env_logger::init();
}

fn main() {
    setup();

    let string = include_str!("../input.txt");

    let part_one = PartOne::new(string).run();
    info!("Part One: {}", part_one);

    let part_two = PartTwo::new(string).run();
    info!("Part Two: {}", part_two);
}
