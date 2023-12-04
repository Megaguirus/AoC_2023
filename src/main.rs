use std::fs;
/*
todo:
- an args system to execute a day-part individually 
- reorganize the module tree somehow idk

notes:
- yes i used Strings as args and i keep convernting between Strings, &str and slices, 
cry about it, cuz i def did.
*/


mod day_1;

fn main() {
    let d1_p1_sol =
        day_1::part_1::trebuchet(fs::read_to_string("src/txts/inputxt/day1_p1.txt").unwrap());
    println!("day 1 part 1: {d1_p1_sol}");

    let d1_p2_sol =
        day_1::part_2::trebuchet_2(fs::read_to_string("src/txts/inputxt/day1_p2.txt").unwrap());
    println!("day 1 part 2: {d1_p2_sol}");
}
