use std::collections::HashMap;
use rust_decimal::Decimal;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Asteroid {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct AsteroidStation {
    asteroid: Asteroid,
    slope_map: HashMap<Decimal, Vec<Asteroid>>,
}

pub fn find_best_location(asteroid_map: &str) {
    let asteroids = map_to_asteroids(asteroid_map);

    let possible_stations =asteroids.iter()
        .map(|asteroid| calc_slopes(asteroid, &asteroids))
        .collect::<Vec<AsteroidStation>>();

    println!("{:?}", possible_stations);


}

fn map_to_asteroids(asteroid_map: &str) -> Vec<Asteroid> {
    asteroid_map.lines().enumerate()
        .map(|(y, line)| {
            line.chars().into_iter().enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, c)| Asteroid { x, y })
        })
        .flatten()
        .collect::<Vec<Asteroid>>()
}


fn calc_slopes(current: &Asteroid ,all: &Vec<Asteroid>) -> AsteroidStation {
    let mut slope_map: HashMap<Decimal, Vec<Asteroid>> = HashMap::new();

    all
        .iter()
        .for_each(|asteroid| {
            if current != asteroid {
                let (c_y, a_y) = (Decimal::new(current.y as i64, 0), Decimal::new(asteroid.y as i64, 0));
                let (c_x, a_x) = (Decimal::new(current.x as i64, 0), Decimal::new(asteroid.x as i64, 0));
                let zero: Decimal = 0.into();
                let min: Decimal = std::usize::MIN.into();

                println!("{:?} / {:?}", c_y - a_y, c_x - a_x);
                let slope: Decimal = if c_x - a_x == zero { min } else {(c_y - a_y) / (c_x - a_x)};

                //hm hm
                if slope_map.contains_key(&slope) {
                    slope_map.get_mut(&slope).unwrap().push(asteroid.to_owned());
                } else {
                    slope_map.insert(slope, vec![current.to_owned(),asteroid.to_owned()]);
                }
            }
        });


    AsteroidStation {asteroid: current.clone(), slope_map}
}


#[cfg(test)]
mod tests {
    use crate::day_10_data;

    use super::*;

    #[test]
    fn should_find_best_location() {

        println!("{}", 2 / -1);
        find_best_location(day_10_data::TEST_DATA_1)
    }
}