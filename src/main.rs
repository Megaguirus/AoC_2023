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
mod day_2;
mod day_3;

fn main() {
    let d1_p1 = day_1::part_1::trebuchet(fs::read_to_string("src/txts/inputxt/day1.txt").unwrap());
    println!("day 1 part 1: {d1_p1}"); //54561

    let d1_p2 =
        day_1::part_2::trebuchet_2(fs::read_to_string("src/txts/inputxt/day1.txt").unwrap());
    println!("day 1 part 2: {d1_p2}"); //54076

    let d2_p1 =
        day_2::part_1::cube_conundrum(fs::read_to_string("src/txts/inputxt/day2.txt").unwrap());
    println!("day 2 part 1: {d2_p1}"); //2265

    let d2_p2 =
        day_2::part_2::cube_conundrum_2(fs::read_to_string("src/txts/inputxt/day2.txt").unwrap());
    println!("dat 2 part 2: {d2_p2}"); //64097

    let mut multi =
        day_3::part_1::Multi::new(fs::read_to_string("src/txts/inputxt/day3.txt").unwrap());
    multi.scan_edges();
    let d3_p1 = multi.gear_ratios();
    println!("day 3 part 1: {d3_p1}"); //533784

    let d3_p1 = multi.gear_ratios_2(); //78826761
    println!("day 3 part 2: {d3_p1}");
}
