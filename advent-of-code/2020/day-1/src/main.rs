use itertools::Itertools;
use std::io::Result;

const SEARCHED_SUM: u32 = 2020;

fn sum(items: &[&u32]) -> u32 {
    items.iter().map(|&&x| x).sum()
}

fn solve(numbers: &Vec<u32>, entries_size: usize) -> u32 {
    numbers
        .iter()
        .combinations(entries_size)
        .find(|c| sum(&c) == SEARCHED_SUM)
        .unwrap()
        .into_iter()
        .product()
}

fn get_input() -> Result<Vec<u32>> {
    Ok(std::fs::read_to_string("input.txt")?
        .lines()
        .map(|num| num.parse::<u32>().expect("Input consists only of numbers"))
        .collect::<Vec<u32>>())
}

fn main() -> Result<()> {
    let numbers = get_input()?;
    println!("The answer to the first part is {}", solve(&numbers, 2));
    println!("The answer to the second part is {}", solve(&numbers, 3));
    Ok(())
}

// Making sure that I won't break anything playing with the code
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answers() -> Result<()> {
        let numbers = get_input()?;
        assert_eq!(solve(&numbers, 2), 870331);
        assert_eq!(solve(&numbers, 3), 283025088);
        Ok(())
    }
}
