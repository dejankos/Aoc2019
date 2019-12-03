use crate::day_3_part_1::{calc_path, find_intersections};

pub fn find_min_steps_of_intersection(f: Vec<&str>, s: Vec<&str>) -> usize {
    let h_set1 = calc_path(f);
    let h_set2 = calc_path(s);

    find_intersections(&h_set1, &h_set2).iter()
        .map(|p| h_set1.get(p).unwrap().steps + h_set2.get(p).unwrap().steps)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_path_1() {
        let data1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let data2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(610, find_min_steps_of_intersection(data1, data2));
    }

    #[test]
    fn should_calc_path_2() {
        let data1 = vec!["R8", "U5", "L5", "D3"];
        let data2 = vec!["U7", "R6", "D4", "L4"];
        assert_eq!(30, find_min_steps_of_intersection(data1, data2));
    }

    #[test]
    fn should_calc_path_3() {
        let data1 = vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"];
        let data2 = vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"];
        assert_eq!(410, find_min_steps_of_intersection(data1, data2));
    }
}