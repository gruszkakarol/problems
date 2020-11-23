use common::intcode_computer::{get_input, Computer, Instructions};

fn main() {
    let program = get_input();

    let amplifier_a = Computer::from((program, Instructions::new(0)));

}
