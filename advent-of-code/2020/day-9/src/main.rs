use itertools::Itertools;

const PREAMBLE_SIZE: usize = 25;

fn part_one(nums: &[u64]) -> u64 {
    let preamble = nums.windows(PREAMBLE_SIZE);
    let numbers = nums.iter().skip(PREAMBLE_SIZE);
    *numbers
        .zip(preamble)
        .find(|(num, preamble)| {
            !preamble
                .iter()
                .combinations(2)
                .find(|c| c.iter().cloned().sum::<u64>() == **num)
                .is_some()
        })
        .map(|(searched_num, _)| searched_num)
        .unwrap()
}

fn part_two(nums: &[u64], expected: u64) -> u64 {
    let sets = std::iter::repeat(nums)
        .enumerate()
        .take(nums.len())
        .map(|(i, vec)| &vec[0..i])
        .find(|vec| vec.iter().sum::<u64>() == expected);
    dbg!(sets);
    0
}

fn main() -> std::io::Result<()> {
    let input: Vec<u64> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    let first_answer = part_one(&input);
    println!("The answer to the first part is {}", first_answer);
    let second_answer = part_two(&input, first_answer);
    Ok(())
}
