use day_8::{part_one, part_two};
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

    let part_one = part_one::run(string);
    info!("Part One: {}", part_one);

    let part_two = part_two::run(string);
    info!("Part Two: {}", part_two);
}
