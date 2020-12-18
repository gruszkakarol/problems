#[derive(Debug, Clone, Copy)]
enum Course {
    North,
    South,
    East,
    West,
}

impl From<i32> for Course {
    fn from(value: i32) -> Self {
        use Course::*;
        dbg!("initial", value);
        let value = value % 360;
        dbg!(value);
        match value {
            0 => North,
            90 => East,
            180 => South,
            270 => West,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for Course {
    fn into(self) -> i32 {
        use Course::*;

        match self {
            North => 0,
            East => 90,
            South => 180,
            West => 270,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Action {
    Move { course: Course, value: i32 },
    Turn { direction: Direction, value: i32 },
    Forward { value: i32 },
}

impl Action {
    pub fn new(str: &str) -> Self {
        let value: i32 = str[1..].parse::<i32>().unwrap();
        match &str[0..1] {
            "N" => Action::Move {
                course: Course::North,
                value,
            },
            "S" => Action::Move {
                course: Course::South,
                value,
            },
            "E" => Action::Move {
                course: Course::East,
                value,
            },
            "W" => Action::Move {
                course: Course::West,
                value,
            },
            "L" => Action::Turn {
                direction: Direction::Left,
                value,
            },
            "R" => Action::Turn {
                direction: Direction::Right,
                value,
            },
            "F" => Action::Forward { value },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Ship {
    // coordinates
    x: i32,
    y: i32,
    current_course: Course,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            current_course: Course::East,
        }
    }

    // move
    pub fn mv(&mut self, course: Course, value: i32) {
        match course {
            Course::North => self.y += value,
            Course::South => self.y -= value,
            Course::East => self.x += value,
            Course::West => self.x -= value,
        };
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::Move { course, value } => {
                self.mv(course, value);
            }
            Action::Forward { value } => {
                self.mv(self.current_course, value);
            }
            Action::Turn { direction, value } => {
                let course_degrees: i32 = self.current_course.into();
                let new_course_degrees = match direction {
                    Direction::Right => course_degrees + value,
                    Direction::Left => (course_degrees - value as i32) + 360,
                };

                self.current_course = Course::from(new_course_degrees)
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() -> std::io::Result<()> {
    let actions: Vec<Action> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(Action::new)
        .collect();

    let mut ship = Ship::new();

    for action in actions {
        ship.perform_action(action);
    }

    let first_answer = ship.manhattan_distance();
    println!("The answer to the first part is: {}", first_answer);
    Ok(())
}
