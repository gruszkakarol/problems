// SEATS
const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

struct Plane {
    places: Vec<Vec<char>>,
}

impl Plane {
    pub fn new(places: Vec<Vec<char>>) -> Self {
        Self { places }
    }

    pub fn tick(&mut self) -> usize {
        let mut changed = 0;

        self.places = self
            .places
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .cloned()
                    .enumerate()
                    .map(|(cell_index, seat)| {
                        // Get seat's neighbours
                        let neighbours = self.neighbours((cell_index as i32, row_index as i32));
                        // Count occupied and empty neighboring seats
                        let (empty, occupied): (Vec<char>, Vec<char>) = neighbours
                            .iter()
                            .filter_map(|&n| n.filter(|&c| c != FLOOR))
                            .partition(|&n| n == EMPTY);
                        // Apply rules and see whether seat should change its state or not
                        match (empty.len(), occupied.len(), seat) {
                            (_, 0, EMPTY) => {
                                changed += 1;
                                OCCUPIED
                            }
                            (_, 4..=8, OCCUPIED) => {
                                changed += 1;
                                EMPTY
                            }
                            _ => seat,
                        }
                    })
                    .collect()
            })
            .collect();

        changed
    }

    pub fn count_occupied(&self) -> usize {
        self.places
            .iter()
            .map(|row| row.iter().filter(|&&c| c == OCCUPIED).count())
            .sum()
    }

    pub fn print(&self) {
        println!("\n\n");

        for row in &self.places {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn get(&self, position: (i32, i32), offset: (i32, i32)) -> Option<char> {
        if position.0 + offset.0 < 0 || position.1 + offset.1 < 0 {
            None
        } else {
            self.places
                .get((position.1 + offset.1) as usize)
                .map(|row| row.get((position.0 + offset.0) as usize).map(|&c| c))
                .flatten()
        }
    }

    pub fn neighbours(&self, position: (i32, i32)) -> [Option<char>; 8] {
        [
            self.get(position, (-1, -1)),
            self.get(position, (0, -1)),
            self.get(position, (1, -1)),
            self.get(position, (-1, 0)),
            self.get(position, (1, 0)),
            self.get(position, (-1, 1)),
            self.get(position, (0, 1)),
            self.get(position, (1, 1)),
        ]
    }

    pub fn first_part(&mut self) -> usize {
        while self.tick() != 0 {
            self.print();
        }

        self.count_occupied()
    }
}

fn main() -> std::io::Result<()> {
    let places: Vec<Vec<char>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().filter(|&c| c != '\n').collect())
        .collect();

    let mut plane = Plane::new(places);

    let first_answer = plane.first_part();
    println!("The answer to the first part is {}", first_answer);
    Ok(())
}
