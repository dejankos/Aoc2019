use std::collections::HashMap;

use crate::int_code_computer::State::{Halt, Output, Running};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    Initialized,
    Running,
    AwaitInput,
    Halt,
    Output,
}

#[derive(Debug)]
pub struct Computer {
    input: Vec<isize>,
    program: Vec<isize>,
    state: State,
    pub(crate) output: Vec<isize>,
    position: usize,
    relative_base: isize,
}

impl Computer {
    pub fn new(input: Vec<isize>, program: Vec<isize>) -> Self {
        Computer {
            input,
            program,
            state: State::Initialized,
            output: vec![],
            position: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) -> State {
        self.state = Running;

        while self.keep_going() {
            let (op_code, a, b, c) = self.read_step_data();
            self.position += self.position_steps(op_code);

            match op_code {
                1 => {
                    self.program[c as usize] = a + b;
                }
                2 => {
                    self.program[c as usize] = a * b;
                }
                3 => {
                    let input = self.input.pop().unwrap(); // for day 7 part 2 check if input available first
                    self.program[c as usize] = input;
                }
                4 => {
                    self.output.push(a);
                    self.state = Output;
                }
                5 => {
                    if a != 0 {
                        self.position = b as usize;
                    }
                }
                6 => {
                    if a == 0 {
                        self.position = b as usize;
                    }
                }
                7 => {
                    self.program[c as usize] = (a < b) as isize;
                }
                8 => {
                    self.program[c as usize] = (a == b) as isize;
                }
                9 => {
                    self.relative_base += a;
                }
                99 => {
                    self.state = Halt;
                }
                _ => panic!("Op code not supported!")
            }
        }


        self.state
    }

    fn position_steps(&self, op_code: isize) -> usize {
        match op_code {
            1 | 2 | 7 | 8 => 4,
            3 |4 | 9 => 2,
            5 | 6 => 3,
            99 => 0,
            _ => panic!("Op code not supported!")
        }
    }

    fn read_step_data(&self) -> (isize, isize, isize, isize) {
        let (op_code, a, b, c) = self.read_instruction();
        if op_code == 99 { (op_code, 0, 0, 0) } else {
            (
                op_code,
                self.read_mode_based(1, a),
                self.read_mode_based(2, b),
                self.read_mode_based(3, c)
            )
        }
    }

    fn read_mode_based(&self, offset: isize, mode: isize) -> isize {
        match offset {
            3 => self.read_write_position(offset, mode),
            _ => self.read_position(offset, mode)
        }
    }

    fn read_instruction(&self) -> (isize, isize, isize, isize) {
        let value = self.program[self.position];
        let op_code = value % 100;
        let a = (value / 100) % 10;
        let b = (value / 1000) % 10;
        let c = (value / 10000) % 10;

        (op_code, a, b, c)
    }

    fn read_position(&self, pos_offset: isize, pos_mode: isize) -> isize {
        let pos_val = self.read_from_position(pos_offset);

        match pos_mode {
            0 => self.program[pos_val as usize],
            1 => pos_val,
            2 => self.program[(pos_val + self.relative_base) as usize],
            _ => panic!("Position mode not supported!")
        }
    }

    fn read_write_position(&self, pos_offset: isize, pos_mode: isize) -> isize {
        let pos_val = self.read_from_position(pos_offset);
        match pos_mode {
            0 | 1 => pos_val,
            2 => pos_val + self.relative_base,
            _ => panic!("Position mode not supported!")
        }
    }

    fn read_from_position(&self, pos_offset: isize) -> isize {
        self.program[self.position + pos_offset as usize]
    }

    fn keep_going(&self) -> bool {
        match &self.state {
            State::AwaitInput | State::Halt => false,
            _ => true
        }
    }
}