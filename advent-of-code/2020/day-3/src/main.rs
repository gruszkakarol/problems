use std::io::Result;

const ROW_SIZE: usize = 31;

#[derive(Debug, PartialEq)]
enum EntityType {
    Tree,
    Empty,
}

impl From<char> for EntityType {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            _ => Self::Empty,
        }
    }
}

struct Map {
    areas: Vec<EntityType>,
}

impl Map {
    pub fn get_at(&self, cords: (usize, usize)) -> &EntityType {
        let x = if cords.0 > ROW_SIZE {
            cords.0 % ROW_SIZE
        } else {
            cords.0
        };
        let y = cords.1 * ROW_SIZE;
        let index = x + y;
        self.areas.get(index).unwrap()
    }

    pub fn height(&self) -> usize {
        self.areas.len() / ROW_SIZE
    }

    pub fn check_slope(&self, slope: (usize, usize)) -> usize {
        (1..self.height())
            .step_by(slope.1)
            .map(|y| self.get_at((slope.0 * (y / slope.1), y)))
            .filter(|e| **e == EntityType::Tree)
            .count()
    }
}

fn main() -> Result<()> {
    let areas: Vec<EntityType> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let entities: Vec<EntityType> = l.chars().map(EntityType::from).collect();
            entities
        })
        .flatten()
        .collect();

    let map = Map { areas };

    let first_answer = map.check_slope((3, 1));
    println!("The answer to the first part is {}", first_answer);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let second_answer: usize = slopes
        .iter()
        .map(|s| map.check_slope(*s))
        .product();
    println!("The answer to the second part is {}", second_answer);
    Ok(())
}
