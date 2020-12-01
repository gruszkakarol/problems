use std::io::Result;

fn part_one(numbers: &Vec<u32>) -> u32 {
    for a in numbers {
        for b in numbers {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

fn part_two(numbers: &Vec<u32>) -> u32 {
    for a in numbers {
        for b in numbers {
            for c in numbers {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let numbers: Vec<u32> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|num| num.parse::<u32>().expect("Input consists only of numbers"))
        .collect();

    println!("The answer to the first part is {}", part_one(&numbers));
    println!("The answer to the second part is {}", part_two(&numbers));

    Ok(())
}
