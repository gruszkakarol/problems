const ROW_SIZE: i32 = 10;

// SEATS
const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

struct Grid {
    places: Vec<char>,
    // Vector of occupied places
    occupied: Vec<usize>,
}

impl Grid {
    pub fn new(places: Vec<char>) -> Self {
        Self {
            places,
            occupied: vec![],
        }
    }

    pub fn tick(&mut self) -> usize {
        self.print();

        let mut changed = 0;

        self.places = self
            .places
            .iter()
            .enumerate()
            .map(|(index, place)| {
                let neighbours = self.neighbours(index);
                let (empty, occupied): (Vec<char>, Vec<char>) = neighbours
                    .iter()
                    .filter(|&&p| p != FLOOR)
                    .partition(|&&n| n == EMPTY);
                print!(
                    "  {} {} {:?} {:?} {:?}  ",
                    index, place, &neighbours, &empty, &occupied
                );
                match (empty.len(), occupied.len(), place) {
                    (_, 0, &EMPTY) => {
                        changed += 1;
                        OCCUPIED
                    }
                    (_, 4..=8, &OCCUPIED) => {
                        changed += 1;
                        EMPTY
                    }
                    _ => *place,
                }
            })
            .collect();

        changed
    }

    // 0 1 2 3 4 5 6 7 8 9
    // 0 1 2 3 4 5 6 7 8 9
    // 0 1 2 3 4 5 6 7 8 9

    pub fn print(&self) {
        println!("\n\n");
        for (i, place) in self.places.iter().enumerate() {
            if i % ROW_SIZE as usize == 0 {
                println!();
            }

            print!("{}", place);
        }
    }

    pub fn get(&self, position: usize) -> char {
        self.places.get(position).map(|p| *p).unwrap_or(FLOOR)
    }

    pub fn neighbours(&self, position: usize) -> [char; 8] {
        if position == 9 {
            dbg!(position);
        }

        macro_rules! pos {
            ( $offset:expr ) => {{
                self.get((position as i32 + $offset) as usize)
            }};
        };

        [
            pos!(-ROW_SIZE - 1),
            pos!(-ROW_SIZE),
            pos!(-ROW_SIZE + 1),
            pos!(-1),
            pos!(1),
            pos!(ROW_SIZE - 1),
            pos!(ROW_SIZE),
            pos!(ROW_SIZE + 1),
        ]
    }
}

fn main() -> std::io::Result<()> {
    let places: Vec<char> = std::fs::read_to_string("input.txt")?
        .chars()
        .filter(|&c| c != '\n')
        .collect();

    let mut grid = Grid::new(places);

    &grid.tick();
    &grid.tick();
    &grid.tick();

    Ok(())
}
