use std::fs;

mod day_1;

fn main() {
    let f =fs::read_to_string("src/day1_P1.txt").unwrap();
    println!("{f}");
}
