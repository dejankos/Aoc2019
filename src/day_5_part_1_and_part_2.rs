use std::borrow::Borrow;
use std::collections::HashMap;

pub struct IntCodeComputer {
    input: u32,
    data: Vec<isize>,
    output: Vec<isize>,
    position: usize,
    modes: HashMap<u8, bool>,
}

impl IntCodeComputer {
    pub fn run(input: u32, data: Vec<isize>) -> isize {
        let mut cmp = IntCodeComputer::new(input, data);
        let res = cmp.compute();

        res[res.len() - 1]
    }

    fn new(input: u32, data: Vec<isize>) -> Self {
        IntCodeComputer {
            input,
            data,
            output: vec![],
            position: 0,
            modes: HashMap::new(),
        }
    }

    fn compute(&mut self) -> Vec<isize> {
        loop {
            let v = self.data[self.position];
            /*
            *ABCDE
            * 1002   - > This is why we LPAD with zeros
            *
            *DE - two-digit opcode,      02 == opcode 2
            *C - mode of 1st parameter,  0 == position mode
            * B - mode of 2nd parameter,  1 == immediate mode
            * A - mode of 3rd parameter,  0 == position mode,
                                              omitted due to being a leading zero
            */
            let str_v = format!("{:0>5}", v.to_string());

            //last two digits - codes from 1 - 99
            //DE
            let op_code = str_v[3..].parse::<u32>().unwrap();
            //map from right to left if it's positional or immediate mode
            //C
            let c = str_v[2..3].parse::<u32>().unwrap();
            self.modes.insert(1, c == 0);
            //B
            let b = str_v[1..2].parse::<u32>().unwrap();
            self.modes.insert(2, b == 0);
            //A
            let a = str_v[0..1].parse::<u32>().unwrap();
            self.modes.insert(3, a == 0);

            if op_code == 99 {
                break;
            }

            self.execute_op_code(op_code);
        }

        self.output.clone()
    }


    fn execute_op_code(&mut self, op_code: u32) {
        match op_code {
            1 => {
                let result_index = self.data[self.position + 3] as usize;
                self.data[result_index] = self.get_param(1) + self.get_param(2);
                self.position += 4;
            }
            2 => {
                let result_index = self.data[self.position + 3] as usize;
                self.data[result_index] = self.get_param(1) * self.get_param(2);
                self.position += 4;
            }
            3 => {
                let result_index = self.data[self.position + 1] as usize;
                self.data[result_index] = self.input as isize;
                self.position += 2;
            }
            4 => {
                self.output.push(self.get_param(1));
                self.position += 2;
            }
            5 => {
                if self.get_param(1) != 0 {
                    self.position = self.get_param(2) as usize;
                } else {
                    self.position += 3;
                }
            }
            6 => {
                if self.get_param(1) == 0 {
                    self.position = self.get_param(2) as usize;
                } else {
                    self.position += 3;
                }
            }
            7 => {
                let third_idx =  self.data[(self.position + 3) as usize] as usize;
                if self.get_param(1) < self.get_param(2) {
                    self.data[third_idx] = 1;
                } else {
                    self.data[third_idx] = 0;
                }
                self.position += 4;
            }
            8 => {
                let third_idx =  self.data[(self.position + 3) as usize] as usize;
                if self.get_param(1) == self.get_param(2) {
                    self.data[third_idx] = 1;
                } else {
                    self.data[third_idx] = 0;
                }
                self.position += 4;
            }
            _ => panic!("Unknown op code !")
        }
    }

    fn get_param(&self, i: usize) -> isize {
        let current = self.data[self.position + i];
        if self.modes[(i as u8).borrow()] {
            //position mode
            self.data[current as usize]
        } else {
            //immediate mode
            current
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5_data;

    use super::*;

    #[test]
    fn should_compute_smth() {
        let data = vec!(1002, 4, 3, 4, 33);
        let input = 1;

        let result = IntCodeComputer::run(input, data);
        println!("{:?}", result);
    }

    #[test]
    fn should_compute_part_1() {
        assert_eq!(IntCodeComputer::run(1, day_5_data::parse_input()), 13978427);
    }

    #[test]
    fn should_compute_part_2() {
        assert_eq!(IntCodeComputer::run(5, day_5_data::parse_input()), 11189491);
    }
}
