use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_opt, multi::separated_list1,
    IResult,
};
use std::ops::Index;

#[derive(Debug)]
pub struct Computer {
    memory: Vec<i32>,
    program_counter: usize,
}

impl Computer {
    fn new(memory: Vec<i32>) -> Computer {
        Computer {
            memory,
            program_counter: 0,
        }
    }

    fn fetch_op_code(&self) -> i32 {
        self.memory[self.program_counter]
    }

    fn step(&mut self) {
        let op_code = self.fetch_op_code();

        match op_code {
            1 => {
                let pos1 = self.memory[self.program_counter + 1] as usize;
                let arg1 = self.memory[pos1];
                let pos2 = self.memory[self.program_counter + 2] as usize;
                let arg2 = self.memory[pos2];
                let dst = self.memory[self.program_counter + 3] as usize;

                self.memory[dst] = arg1 + arg2;
            }
            2 => {
                let pos1 = self.memory[self.program_counter + 1] as usize;
                let arg1 = self.memory[pos1];
                let pos2 = self.memory[self.program_counter + 2] as usize;
                let arg2 = self.memory[pos2];
                let dst = self.memory[self.program_counter + 3] as usize;

                self.memory[dst] = arg1 * arg2;
            }
            _ => panic!(),
        }

        self.program_counter += 4;
    }

    pub fn step_until_halt(&mut self) {
        loop {
            let op_code = self.fetch_op_code();

            if op_code == 99 {
                return;
            }

            self.step()
        }
    }

    pub fn set(&mut self, index: usize, value: i32) {
        self.memory[index] = value;
    }
}

impl From<&str> for Computer {
    fn from(raw: &str) -> Computer {
        Computer::new(parse_program(raw))
    }
}

impl Index<usize> for Computer {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        self.memory.index(index)
    }
}

fn parse_program(i: &str) -> Vec<i32> {
    separated_list1(tag(","), number)(i).unwrap().1
}

fn number(i: &str) -> IResult<&str, i32> {
    map_opt(digit1, |s: &str| s.parse::<i32>().ok())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer() {
        let mut computer = Computer::from("1,9,10,3,2,3,11,0,99,30,40,50");

        computer.step();
        assert_eq!(
            computer.memory,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        computer.step();
        assert_eq!(
            computer.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
