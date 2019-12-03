use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

pub fn calc_manhattan_distance(f: Vec<&str>, s: Vec<&str>) -> isize {
    let h_set1 = calc_path(f);
    let h_set2 = calc_path(s);

    find_intersections(h_set1, h_set2)
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap()
}


fn find_intersections(mut h_set1: HashSet<Point>, h_set2: HashSet<Point>) -> Vec<Point> {
    h_set1.drain()
        .filter(|p| h_set2.contains(p))
        .collect()
}

fn calc_path(data: Vec<&str>) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut x: isize = 0;
    let mut y: isize = 0;

    for d in data {
        let direction = &d[0..1];
        let moves = d[1..].parse::<isize>().unwrap();
        for _ in 0..moves {
            let point = create_point(direction, x, y);
            x = point.x;
            y = point.y;
            points.insert(point);
        }
    }

    points
}

fn create_point(direction: &str, mut x: isize, mut y: isize) -> Point {
    match direction {
        "L" => {
            x = x - 1;
            Point { x, y }
        }
        "R" => {
            x = x + 1;
            Point { x, y }
        }
        "U" => {
            y = y + 1;
            Point { x, y }
        }
        "D" => {
            y = y - 1;
            Point { x, y }
        }
        _ => panic!("Unknown direction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_path_1() {
        let data1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let data2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(159, calc_manhattan_distance(data1, data2));
    }

    #[test]
    fn should_calc_path_2() {
        let data1 = vec!["R8", "U5", "L5", "D3"];
        let data2 = vec!["U7", "R6", "D4", "L4"];
        assert_eq!(6, calc_manhattan_distance(data1, data2));
    }

    #[test]
    fn should_calc_path_3() {
        let data1 = vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"];
        let data2 = vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"];
        assert_eq!(135, calc_manhattan_distance(data1, data2));
    }
}