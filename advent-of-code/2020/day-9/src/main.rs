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
    let set = (1..nums.len())
        .filter_map(|size| {
            nums.windows(size)
                .find(|set| set.iter().cloned().sum::<u64>() == expected)
        })
        .filter(|numbers| !numbers.contains(&expected))
        .collect::<Vec<_>>()[0];

    dbg!(set, set.iter().min().unwrap(), set.iter().max().unwrap());
    set.iter().min().unwrap() + set.iter().max().unwrap()
}

fn main() -> std::io::Result<()> {
    let input: Vec<u64> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    let first_answer = part_one(&input);
    println!("The answer to the first part is {}", first_answer);
    let second_answer = part_two(&input, first_answer);
    println!("The answer to the second part is {}", second_answer);
    Ok(())
}
