use std::fs;

mod day_1;

fn main() {
    let d1_p1_sol = day_1::part_1::trebuchet(fs::read_to_string("src/day1_p1.txt").unwrap()); 
    println!("{d1_p1_sol}");
    
}
