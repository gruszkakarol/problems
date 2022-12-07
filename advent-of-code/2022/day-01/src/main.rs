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

    let part_one = &elfs.iter().map(|elf| elf.total_calories()).max().unwrap();
    println!("The answer to the part one is {}", part_one);
}
