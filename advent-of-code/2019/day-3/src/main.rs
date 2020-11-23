use std::collections::hash_map::VacantEntry;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Point = (i32, i32, i32);

fn manhattan(point: &(i32, i32)) -> usize {
    (point.0.abs() + point.1.abs()) as usize
}

fn get_points(line: &String) -> Vec<Point> {
    let mut position: Point = (0, 0, 0);

    line.split(',')
        .map(|wire_points| {
            let direction = &wire_points[0..1];

            let length = wire_points[1..]
                .parse::<usize>()
                .expect("Unexpected non number value");

            let dir = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!("Unexpected direction"),
            };

            position.2 = 0;

            (1..length + 1)
                .map(|a| {
                    let new_point: Point = (position.0 + dir.0, position.1 + dir.1, position.2 + 1);
                    position = new_point;
                    new_point
                })
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect()
}

fn to_set(connections: &Vec<Point>) -> HashSet<&Point> {
    connections.iter().collect()
}

fn main() -> std::io::Result<()> {
    // read gravity assist program from input file
    let input = read_to_string("input.txt")?
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let first_wire = get_points(&input[0]);
    let second_wire = get_points(&input[1]);

    let first_points: HashSet<(i32, i32)> = first_wire.iter().map(|(x, y, _)| (*x, *y)).collect();
    let second_points: HashSet<(i32, i32)> = second_wire.iter().map(|(x, y, _)| (*x, *y)).collect();

    let intersections = first_points
        .intersection(&second_points)
        .collect::<Vec<&(i32, i32)>>();

    let closest = intersections
        .iter()
        .map(|point| manhattan(point))
        .min()
        .expect("Unexpected end of input");

    println!("{:?} first wire", first_wire);
    println!("{:?} second wire", second_wire);

    let shortest_delay = intersections
        .iter()
        .map(|(i_x, i_y)| {
            let (_, _, a) = first_wire
                .iter()
                .find(|(p_x, p_y, _)| i_x == p_x && i_y == p_y)
                .unwrap();

            let (_, _, b) = second_wire
                .iter()
                .find(|(p_x, p_y, _)| i_x == p_x && i_y == p_y)
                .unwrap();
            println!("{} {} {:?}", a, b, (i_x, i_y));
            a + b
        })
        .min();
    println!("PART ONE: {:#?}", closest);
    println!("PART TWO: {:#?}", shortest_delay);
    Ok(())
}
