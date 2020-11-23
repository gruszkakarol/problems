mod graph;
mod utils;
use graph::{Graph, Objects};
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("input.txt").expect("Input not found");

    let vertices: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            let vertices = line.split(")").collect::<Vec<&str>>();
            (
                *vertices.get(0).expect("Invalid input"),
                *vertices.get(1).expect("Invalid input"),
            )
        })
        .collect();

    let objects = Objects::from(vertices.clone());
    println!("PART ONE: {}", objects.checksum());
    println!(
        "PART TWO: {}",
        objects
            .distance("YOU", "SAN")
            .expect("Something went wrong and we couldn't find the smallest distance to santa :(")
            - 1
    );
    Ok(())
}
