use day_6::part_one::PartOne;
use day_6::part_two::PartTwo;
use dotenv::dotenv;
use tracing::info;

fn setup() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

fn main() {
    setup();

    let string = include_str!("../input.txt");

    let part_one = PartOne::new(string).run();
    info!("Part One: {}", part_one);

    let part_two = PartTwo::new(string).run();
    info!("Part Two: {}", part_two);
}
