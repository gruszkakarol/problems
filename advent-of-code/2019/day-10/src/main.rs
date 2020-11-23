use std::fs::read_to_string;

enum Position {
    Asteroid,
    Empty,
}

impl From<char> for Position {
    fn from(input: char) -> Self {
        match input {
            '#' => Self::Asteroid,
            '.' => Self::Empty,
            _ => panic!("You provided wrong input!")
        }
    }
}


fn input() -> Vec<Vec<Position>> {
    read_to_string("input.txt")
        .expect("Input is required!")
        .lines()
        .map(|l| l.chars().map(Position::from).collect::<Vec<Position>>())
        .collect::<Vec<Vec<Position>>>()
}

fn main() {
    println!("Hello, world!");
}
