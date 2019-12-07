use std::panic::resume_unwind;

use itertools::Itertools;

use crate::day_5_part_1_and_part_2::IntCodeComputer;

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
    use crate::day_5_part_1_and_part_2::IntCodeComputer;
    use crate::day_7_data;
    use crate::day_7_data::{RAW_INPUT, RAW_INPUT_TEST_1, RAW_INPUT_TEST_2, RAW_INPUT_TEST_3};
    use crate::day_7_part_1::AmplifierController;

    #[test]
    fn testing_if_phase_setting_is_what_I_think_it_is() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_1);

        let mut d1 = data.clone();
        let res1 = IntCodeComputer::run(vec![0, 4], d1);

        let mut d2 = data.clone();
        let res2 = IntCodeComputer::run(vec![res1 as u32, 3], d2);

        let mut d3 = data.clone();
        let res3 = IntCodeComputer::run(vec![res2 as u32, 2], d3);

        let mut d4 = data.clone();
        let res4 = IntCodeComputer::run(vec![res3 as u32, 1], d4);

        let mut d5 = data.clone();
        let res5 = IntCodeComputer::run(vec![res4 as u32, 0], d5);

        assert_eq!(res5, 43210);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_1() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_1);
        let mut amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run(), 43210);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_2() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_2);
        let mut amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run(), 54321);
    }

    #[test]
    fn should_find_highest_signal_on_test_data_3() {
        let data = day_7_data::parse_input(RAW_INPUT_TEST_3);
        let mut amp_ctrl = AmplifierController::new(vec![0], data);
        assert_eq!(amp_ctrl.run(), 65210);
    }

    #[test]
    fn should_find_highest_signal_on_input_data() {
        let data = day_7_data::parse_input(RAW_INPUT);
        let mut amp_ctrl = AmplifierController::new(vec![0], data);
        println!(amp_ctrl.run(), 338603);
    }
}
