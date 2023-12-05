use std::{fs::File, io::BufReader};

pub mod part_one;
pub mod part_two;

pub fn file_reader(file_loc: &str) -> BufReader<File> {
    let file = File::open(file_loc).unwrap();
    let reader = BufReader::new(file);
    reader
}
