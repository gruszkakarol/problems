use common::intcode_computer::{get_input, Computer, Either};

const ERROR_MSG: &str = "This program shouldn't return vector of outputs!";

fn first_part() {
    let mut computer = Computer::new(get_input(), None);
    &computer.replace_position(1, 12);
    &computer.replace_position(2, 2);
    match computer.run_program() {
        Either::Left(val) => println!("ANSWER TO THE FIRST PART: {:#?}", val),
        Either::Right(_) => println!("{}", ERROR_MSG),
    }
    println!("ANSWER TO THE FIRST PART: {:#?}", computer.run_program());
}

fn second_part() {
    let mut computer = Computer::new(get_input(), None);
    for x in 0..100 {
        for y in 0..100 {
            &computer.reload_program();
            &computer.replace_position(1, x);
            &computer.replace_position(2, y);
            let result = &computer.run_program();
            match result {
                Either::Left(val) => {
                    if *val == 19690720 {
                        println!("ANSWER TO THE SECOND PART: {}", 100 * x + y);
                        return;
                    }
                }
                Either::Right(_) => panic!(ERROR_MSG),
            }
        }
    }
    println!("ANSWER TO THE SECOND PART: {:#?}", second_part());
}

fn main() -> std::io::Result<()> {
    first_part();
    second_part();
    Ok(())
}
