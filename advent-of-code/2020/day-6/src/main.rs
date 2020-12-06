use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Group {
    answers: HashMap<char, usize>,
    size: usize,
}

fn main() -> std::io::Result<()> {
    let groups: Vec<Group> = std::fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|l| {
            let answers = l
                .chars()
                .filter(|c| *c != '\n')
                .map(|char| (char, l.chars().filter(|&c| c == char).count()))
                .collect::<HashMap<char, usize>>();
            Group {
                answers,
                size: l.chars().filter(|&c| c == '\n').count() + 1,
            }
        })
        .collect();
    let first_answer: usize = groups.iter().map(|group| group.answers.len()).sum();
    println!("The answer to the first part is {}", first_answer);

    let second_answer: usize = groups
        .iter()
        .map(|g| {
            g.answers
                .iter()
                .filter(|(_, occurrences)| **occurrences == g.size)
                .count()
        })
        .sum();

    println!("The answer to the second part is {}", second_answer);
    Ok(())
}
