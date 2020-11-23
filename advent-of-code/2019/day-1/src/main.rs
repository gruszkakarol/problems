use std::fs::read_to_string;
use std::io::prelude::*;
use std::io::{Error, Lines};

fn calculate_fuel(fuel: f32) -> f32 {
    ((fuel / 3.0).floor() - 2.0).max(0.0)
}

fn part_one(input: &Vec<f32>) -> f32 {
    input.iter().map(|fuel| calculate_fuel(*fuel)).sum::<f32>()
}

fn calculate_total_fuel(fuel: f32, total: f32) -> f32 {
    if fuel <= 0.0 {
        return total;
    }
    let calculated_fuel = calculate_fuel(fuel);
    calculate_total_fuel(calculated_fuel, total + calculated_fuel)
}

fn part_two(input: &Vec<f32>) -> f32 {
    input
        .iter()
        .map(|initial_fuel| calculate_total_fuel(*initial_fuel, 0.0))
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("input.txt")?
        .lines()
        .map(|num| {
            num.parse::<f32>()
                .expect("Unexpected string that is not a number")
        })
        .collect::<Vec<f32>>();

    let first_answer = part_one(&input);
    let second_answer = part_two(&input);
    (1..1000).
    println!("THE ANSWER TO FIRST PART IS: {}", first_answer);
    println!("THE ANSWER TO SECOND PART IS: {}", second_answer);
    Ok(())
}
