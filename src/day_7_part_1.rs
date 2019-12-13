use std::borrow::Borrow;
use std::collections::HashMap;

use itertools::Itertools;

pub struct IntCodeComputer {
    input: Vec<u32>,
    data: Vec<isize>,
    output: Vec<isize>,
    position: usize,
    modes: HashMap<u8, bool>,
}

impl IntCodeComputer {
    pub fn run(input: Vec<u32>, data: Vec<isize>) -> isize {
        let mut cmp = IntCodeComputer::new(input, data);
        let res = cmp.compute();

        res[res.len() - 1]
    }

    fn new(input: Vec<u32>, data: Vec<isize>) -> Self {
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
                let pop = self.input.pop().unwrap() as isize;
                self.data[result_index] = pop;
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
                let third_idx = self.data[(self.position + 3) as usize] as usize;
                if self.get_param(1) < self.get_param(2) {
                    self.data[third_idx] = 1;
                } else {
                    self.data[third_idx] = 0;
                }
                self.position += 4;
            }
            8 => {
                let third_idx = self.data[(self.position + 3) as usize] as usize;
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

#[derive(Debug)]
pub struct AmplifierController {
    input: Vec<u32>,
    data: Vec<isize>,
    phase_settings_per: Vec<Vec<u32>>,
}

impl AmplifierController {
    pub fn run(data: Vec<isize>) -> u32 {
        AmplifierController::new(vec![0], data).run_()
    }

    pub fn new(input: Vec<u32>, data: Vec<isize>) -> Self {
        AmplifierController {
            input,
            data,
            phase_settings_per: AmplifierController::gen_permutations(vec![0, 1, 2, 3, 4]),
        }
    }

    pub fn run_(&self) -> u32 {
        let mut res = 0u32;
        for i in &self.phase_settings_per {
            let mut output = 0u32;
            for j in i {
                let input = vec![output, *j];
                output = IntCodeComputer::run(input, self.data.clone()) as u32;
            }

            if output > res {
                res = output;
            }
        }

        res
    }

    fn gen_permutations(sequence: Vec<u32>) -> Vec<Vec<u32>> {
        sequence.into_iter()
            .permutations(5)
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::day_7_data;
    use crate::day_7_data::{RAW_INPUT, RAW_INPUT_TEST_1, RAW_INPUT_TEST_2, RAW_INPUT_TEST_3};
    use crate::day_7_part_1::{AmplifierController, IntCodeComputer};
    use crate::int_code_computer::Computer;

    #[test]
    fn testing_if_phase_setting_is_what_i_think_it_is() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_1);

        let d1 = data.clone();
        let res1 = IntCodeComputer::run(vec![0, 4], d1);


        let mut cmp = Computer::new(vec![0, 4], data.clone());
        let res = cmp.run();

        println!("{:?}", res1);
        println!("{:?}", res);

        let d2 = data.clone();
        let res2 = IntCodeComputer::run(vec![res1 as u32, 3], d2);

        let d3 = data.clone();
        let res3 = IntCodeComputer::run(vec![res2 as u32, 2], d3);

        let d4 = data.clone();
        let res4 = IntCodeComputer::run(vec![res3 as u32, 1], d4);

        let d5 = data.clone();
        let res5 = IntCodeComputer::run(vec![res4 as u32, 0], d5);

        assert_eq!(res5, 43210);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_1() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_1);
        let amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run_(), 43210);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_2() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_2);
        let amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run_(), 54321);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_3() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_3);
        let amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run_(), 65210);
    }

    #[test]
    fn should_find_highest_signal_on_input_data() {
        let data = day_7_data::parse_input(RAW_INPUT);
        let amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run_(), 338603);
    }
}
