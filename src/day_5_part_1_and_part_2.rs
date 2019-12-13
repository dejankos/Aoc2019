use crate::int_code_computer::Computer;

pub fn run(input: Vec<isize>, program: Vec<isize>) -> isize {
    let mut cmp = Computer::new(input, program);
    let res = cmp.run();

    res.1[res.1.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::day_5_data;

    use super::*;

    #[test]
    fn should_compute_part_1() {
        assert_eq!(run(vec![1], day_5_data::parse_input()), 13978427);
    }

    #[test]
    fn should_compute_part_2() {
        assert_eq!(run(vec![5], day_5_data::parse_input()), 11189491);
    }
}
