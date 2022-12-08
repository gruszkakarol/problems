#[derive(PartialEq, PartialOrd)]
enum GameChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for GameChoice {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => GameChoice::Rock,
            "B" | "Y" => GameChoice::Paper,
            "C" | "Z" => GameChoice::Scissors,
            _ => panic!("Invalid input!"),
        }
    }
}

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

type GameOutcome = usize;

impl GameChoice {
    pub fn play(self, other: GameChoice) -> GameOutcome {
        use GameChoice::*;

        let round_points = match (&self, &other) {
            (Rock, Scissors) => WIN,
            (Scissors, Rock) => LOSE,
            _ => {
                if self > other {
                    WIN
                } else if self == other {
                    DRAW
                } else {
                    LOSE
                }
            }
        };

        let shape_points = self as usize;

        round_points + shape_points
    }
}

fn main() {
    let choices: Vec<GameOutcome> = shared::input()
        .lines()
        .map(|line| {
            let opponent = GameChoice::from(&line[0..1]);
            let you = GameChoice::from(&line[2..3]);
            you.play(opponent)
        })
        .collect();

    println!("First part: {}", choices.iter().sum::<usize>());
}
