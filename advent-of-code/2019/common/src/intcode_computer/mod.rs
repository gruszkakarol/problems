mod opcode;
mod utils;
use opcode::{Opcode, ParameterMode, Variant as OpcodeVariant};
pub use utils::{get_input, Either};

pub type Program = Vec<i32>;

#[derive(Debug, Clone)]
pub struct Instructions {
    pub input: i32,
    pub output: Vec<i32>,
}

impl Instructions {
    pub fn new(input: i32) -> Self {
        Self {
            input,
            output: Vec::new(),
        }
    }

    pub fn add_output(&mut self, output: i32) {
        self.output.push(output);
    }
}
#[derive(Debug)]
pub struct Computer {
    program_backup: Program,
    program: Program,
    curr: usize,
    instructions: Option<Instructions>,
    should_break: bool,
}

impl Computer {
    pub fn new(program: Program, instructions: Option<Instructions>) -> Self {
        Self {
            program_backup: program.clone(),
            curr: 0,
            program,
            instructions,
            should_break: false,
        }
    }

    pub fn reload_program(&mut self) {
        self.program = self.program_backup.clone();
        self.curr = 0;
        self.should_break = false;
    }

    fn next(&mut self) -> Option<i32> {
        let item = self.program.get(self.curr).cloned();
        self.curr += 1;
        item
    }

    fn read_memory(&self, index: usize) -> i32 {
        self.program
            .get(index)
            .cloned()
            .expect("Tried to access invalid place in the memory")
    }

    fn parameter(&self, offset: usize, opcode: &Opcode) -> i32 {
        let param = self.read_memory(self.curr + offset);
        match opcode.nth_parameter_mode(offset) {
            ParameterMode::Immediate => param,
            ParameterMode::Position => self.read_memory(param as usize),
        }
    }

    fn position(&self, offset: usize, opcode: &Opcode) -> usize {
        let param = self.read_memory(self.curr + offset);
        match opcode.nth_parameter_mode(offset) {
            ParameterMode::Immediate => panic!(
                "Computer is in invalid state. Position parameter can't be in immediate mode!"
            ),
            ParameterMode::Position => param as usize,
        }
    }

    fn params_3(&mut self, opcode: &Opcode) -> (i32, i32, usize) {
        (
            self.parameter(0, &opcode),
            self.parameter(1, &opcode),
            self.position(2, &opcode),
        )
    }

    fn params_2(&mut self, opcode: &Opcode) -> (i32, usize) {
        (
            self.parameter(0, &opcode),
            self.parameter(1, &opcode) as usize,
        )
    }

    fn evaluate_opcode(&mut self, opcode: Opcode) {
        match opcode.variant {
            OpcodeVariant::One => {
                let (a, b, c) = self.params_3(&opcode);
                self.program[c] = a + b;
            }
            OpcodeVariant::Two => {
                let (a, b, c) = self.params_3(&opcode);
                self.program[c] = a * b;
            }
            OpcodeVariant::Three => {
                let a = self.position(0, &opcode);
                let input = &self
                    .instructions
                    .clone()
                    .expect("This program need instructions in order to run")
                    .input;
                self.program[a] = *input;
            }
            OpcodeVariant::Four => {
                let a = self.parameter(0, &opcode);
                self.instructions
                    .as_mut()
                    .expect("This program need instructions in order to run")
                    .add_output(a);
            }
            OpcodeVariant::Five => {
                let (a, b) = self.params_2(&opcode);
                match a != 0 {
                    true => self.curr = b,
                    false => self.curr += 2,
                }
            }
            OpcodeVariant::Six => {
                let (a, b) = self.params_2(&opcode);
                match a == 0 {
                    true => self.curr = b,
                    false => self.curr += 2,
                }
            }
            OpcodeVariant::Seven => {
                let (a, b, c) = self.params_3(&opcode);
                self.program[c] = if a < b { 1 } else { 0 };
            }
            OpcodeVariant::Eight => {
                let (a, b, c) = self.params_3(&opcode);
                self.program[c] = if a == b { 1 } else { 0 };
            }
            OpcodeVariant::Break => {
                self.should_break = true;
            }
        }
        self.curr += opcode.pointer_size();
    }

    pub fn run_program(&mut self) -> Either<i32, Vec<i32>> {
        while let (Some(opcode), false) = (self.next(), self.should_break) {
            self.evaluate_opcode(opcode.into());
        }
        self.instructions
            .clone()
            .map_or(Either::Left(self.program[0]), |i| Either::Right(i.output))
    }

    pub fn replace_position(&mut self, index: usize, new: i32) {
        self.program.get_mut(index).map(|e| *e = new);
    }
}

impl From<&Vec<i32>> for Computer {
    fn from(program: &Vec<i32>) -> Self {
        Self {
            program_backup: program.clone(),
            curr: 0,
            program: program.clone(),
            instructions: None,
            should_break: false,
        }
    }
}

impl From<(Vec<i32>, Instructions)> for Computer {
    fn from((program, instructions): (Vec<i32>, Instructions)) -> Self {
        Self {
            program_backup: program.clone(),
            curr: 0,
            program,
            instructions: Some(instructions),
            should_break: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reloads program
    #[test]
    fn reload_program() {
        let program = vec![1, 2, 3, 2, 99];
        let mut computer = Computer::from(&program);
        computer.run_program();
        assert_eq!(computer.program, vec![1, 2, 5, 2, 99]);
        computer.reload_program();
        assert_eq!(computer.program, program);
    }

    /// Replaces position of the item in the memory
    #[test]
    fn replace_position() {
        let mut computer = Computer::from(&vec![1, 2, 3]);
        assert_eq!(computer.read_memory(0), 1);
        computer.replace_position(0, 3);
        assert_eq!(computer.read_memory(0), 3);

        assert_eq!(computer.read_memory(1), 2);
        computer.replace_position(1, 5);
        assert_eq!(computer.read_memory(1), 5);
    }

    /// Reads the memory
    #[test]
    fn read_memory() {
        let computer = Computer::from(&vec![1, 2, 3]);
        assert_eq!(computer.read_memory(0), 1);
        assert_eq!(computer.read_memory(1), 2);
        assert_eq!(computer.read_memory(2), 3);
    }

    /// Reads the position parameters
    #[test]
    fn read_position_parameters() {
        let mut computer = Computer::from(&vec![1, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        // first parameter(2) points to index 2, so that is 3
        assert_eq!(3, computer.parameter(0, &opcode));
        // second parameter(3) points to index 3, so that is 5
        assert_eq!(5, computer.parameter(1, &opcode));
    }

    /// Reads the immediate parameters
    #[test]
    fn read_immediate_parameters() {
        let mut computer = Computer::from(&vec![1101, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        // first parameter(2) is simply 2
        assert_eq!(2, computer.parameter(0, &opcode));
        // second parameter(3) is simply 3
        assert_eq!(3, computer.parameter(1, &opcode));
    }

    /// Reads the position
    #[test]
    fn read_position() {
        let mut computer = Computer::from(&vec![1101, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        assert_eq!(computer.position(2, &opcode), 5);
    }

    /// Panics when position is read in immediate mode
    #[test]
    #[should_panic(
        expected = "Computer is in invalid state. Position parameter can't be in immediate mode!"
    )]
    fn read_position_immediate() {
        let mut computer = Computer::from(&vec![11101, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        computer.position(2, &opcode);
    }

    /// Reads (parameter, parameter, position) from the memory
    #[test]
    fn read_parameters_3() {
        let mut computer = Computer::from(&vec![1101, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        let params = computer.params_3(&opcode);
        assert_eq!(params, (2, 3, 5));
    }

    /// Reads (parameter, parameter) from the memory
    #[test]
    fn read_parameters_2() {
        let mut computer = Computer::from(&vec![1105, 2, 3, 5, 99]);
        let opcode: Opcode = computer.next().unwrap().into();
        let params = computer.params_2(&opcode);
        assert_eq!(params, (2, 3));
    }

    /// Evaluates Opcode One
    #[test]
    fn evaluates_one() {
        let mut computer = Computer::from(&vec![1101, 2, 3, 3, 99]);
        computer.run_program();
        assert_eq!(computer.program, vec![1101, 2, 3, 5, 99]);
    }

    /// Evaluates Opcode Two
    #[test]
    fn evaluates_two() {
        let mut computer = Computer::from(&vec![1102, 2, 3, 3, 99]);
        computer.run_program();
        assert_eq!(computer.program, vec![1102, 2, 3, 6, 99]);
    }

    /// Evaluates Opcode Three
    #[test]
    fn evaluates_three() {
        let mut computer = Computer::new(vec![3, 1, 99], Some(Instructions::new(5)));
        computer.run_program();
        assert_eq!(computer.program, vec![3, 5, 99]);
    }

    /// Evaluates Opcode Four
    #[test]
    fn evaluates_four() {
        let mut computer = Computer::new(vec![4, 1, 99], Some(Instructions::new(5)));
        computer.run_program();
        assert_eq!(computer.instructions.unwrap().output, vec![1]);
    }

    /// Evaluates Opcode Five without jump
    #[test]
    fn evaluates_five_no_jump() {
        let mut computer = Computer::from(&vec![1105, 0, 2, 99]);
        let opcode = computer.next().unwrap().into();
        // stack pointer equals only the size of an opcode movement
        assert_eq!(computer.curr, 1);
        computer.evaluate_opcode(opcode);
        // stack pointer increases its size by 2, because we didn't jump
        assert_eq!(computer.curr, 3);
        assert_eq!(computer.program, vec![1105, 0, 2, 99]);
    }

    /// Evaluates Opcode Five with jump
    #[test]
    fn evaluates_five_jump() {
        let mut computer = Computer::from(&vec![1105, 1, 5, 1102, 10, 20, 99]);
        let opcode = computer.next().unwrap().into();
        // stack pointer equals only the size of an opcode movement
        assert_eq!(computer.curr, 1);
        computer.evaluate_opcode(opcode);
        // stack pointer increases to the second parameter value which is 5 in immediate mode
        assert_eq!(computer.curr, 5);
        assert_eq!(computer.program, vec![1105, 1, 5, 1102, 10, 20, 99]);
    }

    /// Evaluates Opcode Six without jump
    #[test]
    fn evaluates_six_no_jump() {
        let mut computer = Computer::from(&vec![1106, 4, 2, 99]);
        let opcode = computer.next().unwrap().into();
        // stack pointer equals only the size of an opcode movement
        assert_eq!(computer.curr, 1);
        computer.evaluate_opcode(opcode);
        // stack pointer increases its size by 2, because we didn't jump
        assert_eq!(computer.curr, 3);
        assert_eq!(computer.program, vec![1106, 4, 2, 99]);
    }

    /// Evaluates Opcode Six with jump
    #[test]
    fn evaluates_six_jump() {
        let mut computer = Computer::from(&vec![1106, 0, 5, 1102, 10, 20, 99]);
        let opcode = computer.next().unwrap().into();
        // stack pointer equals only the size of an opcode movement
        assert_eq!(computer.curr, 1);
        computer.evaluate_opcode(opcode);
        // stack pointer increases to the second parameter value which is 5 in immediate mode
        assert_eq!(computer.curr, 5);
        assert_eq!(computer.program, vec![1106, 0, 5, 1102, 10, 20, 99]);
    }

    /// Evaluates Opcode Seven if condition is true
    #[test]
    fn evaluates_seven_true() {
        let mut computer = Computer::from(&vec![1107, 0, 5, 3, 99]);
        let opcode = computer.next().unwrap().into();
        computer.evaluate_opcode(opcode);
        assert_eq!(computer.program, vec![1107, 0, 5, 1, 99])
    }

    /// Evaluates Opcode Seven if condition is true
    #[test]
    fn evaluates_seven_false() {
        let mut computer = Computer::from(&vec![1107, 0, 5, 3, 99]);
        let opcode = computer.next().unwrap().into();
        computer.evaluate_opcode(opcode);
        assert_eq!(computer.program, vec![1107, 0, 5, 1, 99])
    }

    /// Evaluates Opcode Eight
    #[test]
    fn evaluates_eight_true() {
        let mut computer = Computer::from(&vec![1108, 0, 0, 3, 99]);
        let opcode = computer.next().unwrap().into();
        computer.evaluate_opcode(opcode);
        assert_eq!(computer.program, vec![1108, 0, 0, 1, 99]);
    }

    /// Stops the program
    #[test]
    fn stop_execution() {
        let mut computer = Computer::from(&vec![1108, 0, 0, 3, 99]);
        assert!(!computer.should_break);
        let opcode = Opcode::from(99);
        computer.evaluate_opcode(opcode);
        assert!(computer.should_break);
    }

    /// Returns the first item in the memory as the result of the execution
    #[test]
    fn returns_result_left() {
        let mut computer = Computer::from(&vec![1108, 0, 0, 3, 99]);
        assert_eq!(computer.run_program(), Either::Left(1108));
    }

    /// Returns the items in the output as the result of the execution
    #[test]
    fn returns_result_right() {
        let mut computer = Computer::new(vec![1104, 1, 99], Some(Instructions::new(0)));
        assert_eq!(computer.run_program(), Either::Right(vec![1]));
    }
}
