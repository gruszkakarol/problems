use shared::input;

type Calories = u32;
struct Elf {
    pub carried_calories: Vec<Calories>,
}

impl Elf {
    pub fn total_calories(&self) -> Calories {
        self.carried_calories.iter().sum()
    }
}

fn main() {
    let elfs: Vec<Elf> = input()
        .split("\n\n")
        .map(|elf_input| {
            let carried_calories = elf_input
                .lines()
                .map(|calories| calories.parse::<Calories>().unwrap())
                .collect::<Vec<Calories>>();
            Elf { carried_calories }
        })
        .collect();

    let mut total_calories: Vec<Calories> = elfs.iter().map(|elf| elf.total_calories()).collect();
    total_calories.sort();

    let first_part = total_calories.last().unwrap();
    println!("First part: {}", first_part);
    let second_part: Calories = total_calories.iter().rev().take(3).sum();
    println!("Second part: {}", second_part);
}
