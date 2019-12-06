use std::collections::HashMap;
use std::ops::Deref;
use std::sync::mpsc::channel;

use crate::day_6_data::Orbit;

const COM: &str = "COM";

pub fn count_orbits(orbits: Vec<Orbit>) -> usize {
    let mut chain_map: HashMap<&str, Vec<String>> = HashMap::new();
    let mut map: HashMap<&str, &str> = HashMap::new();

    for o in orbits.iter() {
        map.insert(o.1, o.0);
    }

    for v in map.keys() {
        if !chain_map.contains_key(v) {
            chain_map.insert(v, chain(v, &map));
        }
        //println!("{} - {:?}", v, chain_map[v]);
    }

    chain_map.values()
        .map(|vec| vec.len())
        .sum()
}

fn chain(val: &str, map: &HashMap<&str, &str>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    if val == COM {
        return v;
    }

    let mut current = map[val];
    while current != COM {
        v.push(current.to_string());
        current = map[current];
    }
    v.push(current.to_string());

    v
}

#[cfg(test)]
mod tests {
    use crate::{day_5_data, day_6_data};

    use super::*;

    #[test]
    fn should_count_orbits() {
        let res = count_orbits(day_6_data::parse_input(day_6_data::RAW_DATA_TEST));
        assert_eq!(res, 42);
    }
}