use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Asteroid {
    x: isize,
    y: isize,
}

pub fn find_max_visible(asteroid_map: &str) -> usize {
    find_visible_per_asteroid(
        map_to_asteroids(asteroid_map)
    )
        .iter()
        .max_by_key(|ast_count| ast_count.1)
        .map(|ast| ast.1)
        .unwrap()
}

fn find_visible_per_asteroid(all: Vec<Asteroid>) -> Vec<(Asteroid, usize)> {
    all.iter()
        .map(|current| {
            let mut set: HashSet<(isize, isize)> = HashSet::new();
            let mut max_len = 0;
            all.iter().for_each(|other| {
                if current != other {
                    set.insert(angle(current, other));
                }
            });

            if set.len() > max_len { max_len = set.len() }

            (current.clone(), max_len)
        })
        .collect()
}

fn map_to_asteroids(asteroid_map: &str) -> Vec<Asteroid> {
    asteroid_map.lines().enumerate()
        .map(|(y, line)| {
            line.chars().into_iter().enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Asteroid { x: x as isize, y: y as isize })
        })
        .flatten()
        .collect()
}

fn angle(current: &Asteroid, other: &Asteroid) -> (isize, isize) {
    let (x, y) = (current.x - other.x, current.y - other.y);
    let gcd = gcd(x.abs(), y.abs());

    if gcd != 0 {
        (x / gcd, y / gcd)
    } else {
        (x, y)
    }
}

fn gcd(x: isize, y: isize) -> isize {
    if x == 0 {
        y
    } else if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_10_data;

    use super::*;

    #[test]
    fn should_find_max_visible_for_input_data_1() {
        assert_eq!(8, find_max_visible(day_10_data::TEST_DATA_1));
    }

    #[test]
    fn should_find_max_visible_for_input_data_2() {
        assert_eq!(33, find_max_visible(day_10_data::TEST_DATA_2));
    }

    #[test]
    fn should_find_max_visible_for_input_data_3() {
        assert_eq!(210, find_max_visible(day_10_data::TEST_DATA_3));
    }

    #[test]
    fn should_find_max_visible_for_input_data() {
        assert_eq!(303, find_max_visible(day_10_data::INPUT_DATA));
    }
}