use crate::day_1_part_1::calc_fuel_req;
use crate::day_1_part_2::cal_fuel_req_for_all;
use crate::day_2_part_1::intcode_computer;
use crate::day_2_part_2::find_noun_verb;
use crate::day_3_part_1::calc_manhattan_distance;
use crate::day_3_part_2::find_min_steps_of_intersection;
use crate::day_4_part_1::diff_passwords;
use crate::day_4_part_2::diff_passwords_improved;
use crate::day_6_data::RAW_DATA;
use crate::day_6_part_1::count_orbits;
use crate::day_6_part_2::calc_transfers;
use crate::day_7_data::RAW_INPUT;
use crate::day_7_part_1::AmplifierController;
use crate::day_8_part_1_and_part_2::{calc_pass, decode_image, print_image};

mod day_1_data;
mod day_1_part_1;
mod day_1_part_2;
mod day_2_data;
mod day_3_data;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;
mod day_4_part_1;
mod day_4_part_2;
mod day_5_part_1_and_part_2;
mod day_5_data;
mod day_6_part_1;
mod day_6_part_2;
mod day_6_data;
mod day_7_data;
mod day_8_data;
mod day_7_part_1;
mod day_8_part_1_and_part_2;
mod day_9_part_1_and_part_2;
mod int_code_computer;

fn main() {
    println!("Ho ho ho!");

    println!("Answer to day 1 part 1 is = {}", calc_fuel_req(day_1_data::parse_input()));
    println!("Answer to day 1 part 2 is = {}", cal_fuel_req_for_all(day_1_data::parse_input()));

    println!("Answer to day 2 part 1 is = {}", intcode_computer(&mut day_2_data::parse_input())[0]);
    println!("Answer to day 2 part 2 is = {:?}", find_noun_verb());

    println!("Answer to day 3 part 1 is = {}", calc_manhattan_distance(day_3_data::parse_input().0, day_3_data::parse_input().1));
    println!("Answer to day 3 part 2 is = {}", find_min_steps_of_intersection(day_3_data::parse_input().0, day_3_data::parse_input().1));

    println!("Answer to day 4 part 1 is = {}", diff_passwords(171309, 643603));
    println!("Answer to day 4 part 2 is = {}", diff_passwords_improved(171309, 643603));

    println!("Answer to day 5 part 1 is = {}", day_5_part_1_and_part_2::run(vec![1], day_5_data::parse_input()));
    println!("Answer to day 5 part 2 is = {}", day_5_part_1_and_part_2::run(vec![5], day_5_data::parse_input()));

    println!("Answer to day 6 part 1 is = {}", count_orbits(day_6_data::parse_input(day_6_data::RAW_DATA)));
    println!("Answer to day 6 part 2 is = {}", calc_transfers(day_6_data::parse_input(day_6_data::RAW_DATA)));

    println!("Answer to day 7 part 1 is = {}", AmplifierController::run(day_7_data::parse_input(RAW_INPUT)));
    //TODO part 2 - refactor int code computer to support state - halt / wait for input / output

    println!("Answer to day 8 part 1 is = {}", calc_pass(day_8_data::parse_input(25, 6, day_8_data::RAW_DATA)));
    println!("Answer to day 8 part 2 is");print_image(decode_image(day_8_data::parse_input(25, 6, day_8_data::RAW_DATA)));

    println!("Answer to day 9 part 1 is = {}", day_9_part_1_and_part_2::run(1));
    println!("Answer to day 9 part 2 is");
    day_9_part_1_and_part_2::run(2);
}