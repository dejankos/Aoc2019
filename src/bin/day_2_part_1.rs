use crate::day_2_data;

const EXIT_SIG: usize = 99;

pub fn intcode_computer(program: &mut Vec<usize>) -> Vec<usize> {
    let mut current = 0;

    while program[current] != EXIT_SIG {
        let result = process_opcode(program[current], program[current + 1], program[current + 2], &program);
        let result_index = program[current + 3];

        program[result_index] = result;

        current = current + 4;
    }

    program.to_owned()
}


fn process_opcode(opcode: usize, f: usize, s: usize, program: &Vec<usize>) -> usize {
    match opcode {
        1 => program[f] + program[s],
        2 => program[f] * program[s],
        _ => panic!("Unreachable code!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_run_intcode_computer() {
        assert_eq!(vec![2, 0, 0, 0, 99], intcode_computer(&mut vec![1, 0, 0, 0, 99]));
        assert_eq!(vec![2, 3, 0, 6, 99], intcode_computer(&mut vec![2, 3, 0, 3, 99]));
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], intcode_computer(&mut vec![2, 4, 4, 5, 99, 0]));
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], intcode_computer(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }
}

fn main() {
    println!("Answer to part one is = {}", intcode_computer(&mut day_2_data::parse_input())[0]);
}