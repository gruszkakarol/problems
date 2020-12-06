use std::collections::HashSet;

#[derive(Debug)]
struct Seat {
    row: u8,
    column: u8,
    id: u16,
}

impl Seat {
    pub fn new(row: u8, column: u8) -> Self {
        Seat {
            row,
            column,
            id: (row as u16 * 8) + column as u16,
        }
    }
}

fn to_bin(c: char) -> char {
    match c {
        'F' | 'L' => '0',
        'B' | 'R' => '1',
        _ => unreachable!(),
    }
}

// TODO: Avoid the allocation
fn to_num(str: &str) -> u8 {
    u8::from_str_radix(&str.chars().map(to_bin).collect::<String>(), 2).unwrap()
}

fn main() -> std::io::Result<()> {
    let seats: Vec<_> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let row = to_num(&l[0..=6]);
            let column = to_num(&l[7..]);
            Seat::new(row, column)
        })
        .collect();
    let seats_ids: Vec<u16> = seats.iter().map(|s| s.id).collect();
    let smallest_id = *seats_ids.iter().min().unwrap();
    let highest_id = *seats_ids.iter().max().unwrap();

    println!("The answer to the first part is {}", highest_id);

    let all_seats_ids: HashSet<u16> = (smallest_id..highest_id).collect();
    let ids: HashSet<u16> = seats.iter().map(|s| s.id).collect();
    let your_seat = all_seats_ids.difference(&ids).next().unwrap();
    println!("The answer to the second part is {}", your_seat);
    Ok(())
}
