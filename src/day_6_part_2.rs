use std::collections::HashMap;

use crate::day_6_data::Orbit;
use crate::day_6_part_1::{chain_orbits, COM};

pub fn calc_transfers(orbits: Vec<Orbit>) -> usize {
    let chained: HashMap<&'static str, Vec<String>> = chain_orbits(orbits);

    let mut you = &chained["YOU"];
    let mut san = &chained["SAN"];

    you.to_owned().reverse();
    san.to_owned().reverse();

    let mut matched_orbit = "-1";
    'outer: for orbit in san {
        for o in you {
            println!("santa = {} you = {}", orbit, o);
            if o.as_str() != COM && orbit.as_str() == o.as_str() {
                matched_orbit = orbit;
                break 'outer;
            }
        }
    }
    let match_chain = &chained[matched_orbit];

    (you.len() - match_chain.len()) + (san.len() - match_chain.len()) - 2
}


#[cfg(test)]
mod tests {
    use crate::{day_5_data, day_6_data};

    use super::*;

    #[test]
    fn should_count_orbits() {
        let res = calc_transfers(day_6_data::parse_input(day_6_data::RAW_DATA_TEST_PART_2));
        assert_eq!(res, 4);
    }
}
