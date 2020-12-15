fn count_arrangements(input: &[usize]) -> usize {
    let adapter = input.get(0).unwrap();
    let one_jolt_diff = input.iter().position(|n| *n - adapter == 1);
    let three_jolts_diff = input.iter().position(|n| *n - adapter == 3);
    println!("{:?} {:?}", one_jolt_diff, three_jolts_diff);
    match (one_jolt_diff, three_jolts_diff) {
        (Some(o_start), Some(t_start)) => {
            count_arrangements(&input[o_start..]) + count_arrangements(&input[t_start..])
        }
        (Some(o_start), None) => 1 + count_arrangements(&input[o_start..]),
        (None, Some(t_start)) => 1 + count_arrangements(&input[t_start..]),
        (None, None) => 0,
    }
}

fn main() -> std::io::Result<()> {
    let mut input: Vec<usize> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);

    let (ones, threes): (Vec<usize>, Vec<usize>) = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(curr, next)| next - curr)
        .partition(|diff| *diff == 1);

    let first_answer = ones.len() * threes.len();
    println!("The answer to the first part is {}", first_answer);
    dbg!(&input);
    let second_answer = count_arrangements(&input);
    dbg!(second_answer);

    Ok(())
}
