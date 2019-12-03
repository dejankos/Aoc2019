use crate::day_1_part_1::calc_fuel_req;
use crate::day_1_part_2::cal_fuel_req_for_all;
use crate::day_2_part_1::intcode_computer;
use crate::day_2_part_2::{find_noun_verb};
use crate::day_3_part_1::calc_manhattan_distance;
use crate::day_3_part_2::find_min_steps_of_intersection;

mod day_1_data;
mod day_1_part_1;
mod day_1_part_2;
mod day_2_data;
mod day_3_data;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;

fn main() {
    println!("Ho ho ho!");

    println!("Answer to day 1 part 1 is = {}", calc_fuel_req(day_1_data::parse_input()));
    println!("Answer to day 1 part 2 is = {}", cal_fuel_req_for_all(day_1_data::parse_input()));

    println!("Answer to day 2 part 1 is = {}", intcode_computer(&mut day_2_data::parse_input())[0]);
    println!("Answer to day 2 part 2 is = {:?}", find_noun_verb());

    println!("Answer to day 3 part 1 is = {}", calc_manhattan_distance(day_3_data::parse_input().0, day_3_data::parse_input().1));
    println!("Answer to day 3 part 2 is = {}", find_min_steps_of_intersection(day_3_data::parse_input().0, day_3_data::parse_input().1));

}