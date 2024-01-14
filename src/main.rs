use crate::days::day1::cal_val;
use crate::days::day2::day2_val_part1;
use crate::days::day2::day2_val_part2;
use crate::days::day3::day3_val_part1;
use crate::days::day3::day3_val_part2;
use crate::days::day4::day4_val_part1;
use crate::days::day4::day4_val_part2;
use crate::days::day5::day5_val_part1;
// use crate::days::day5::day5_val_part2;

pub mod days;

fn main() {
    println!("day 1: {}", cal_val());
    println!("day 2 part 1: {}", day2_val_part1());
    println!("day 2 part 2: {}", day2_val_part2());
    println!("day 3 part 1: {}", day3_val_part1());
    println!("day 3 part 2: {}", day3_val_part2());
    println!("day 4 part 1: {}", day4_val_part1());
    println!("day 4 part 2: {}", day4_val_part2());
    println!("day 5 part 1: {}", day5_val_part1());
}
