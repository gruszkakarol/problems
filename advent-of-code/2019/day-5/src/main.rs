use common::intcode_computer::{get_input, Computer, Instructions};

fn part_one() {
    let instructions = Instructions::new(1);
    let mut computer = Computer::new(get_input(), Some(instructions));
    println!("PART ONE: {:#?}", computer.run_program());
}

fn part_two() {
    let instructions = Instructions::new(5);
    let mut computer = Computer::new(get_input(), Some(instructions));
    println!("PART TWO: {:#?}", computer.run_program());
}

fn main() {
    part_one();
    part_two();
}
