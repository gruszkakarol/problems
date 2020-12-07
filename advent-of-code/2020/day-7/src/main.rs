use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    amount: usize,
    color: String,
}

type Rules = HashMap<String, Vec<Bag>>;

fn remove_suffix(str: &str) -> String {
    str.replace("bags", "").replace("bag", "").trim().to_owned()
}

fn parse_rule(str: &str) -> (String, Vec<Bag>) {
    let mut parts = str.split("contain");
    let holder = remove_suffix(parts.next().unwrap());
    let contains: Vec<Bag> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|bag| {
            let bag = bag.trim().replace(".", "");
            let amount = if bag == "no other bags" {
                0
            } else {
                bag[0..1].parse::<usize>().unwrap()
            };
            let color = remove_suffix(&bag[1..]);

            Bag { amount, color }
        })
        .collect();

    (holder, contains)
}

const GOLDEN_COLOR: &str = "shiny gold";

// Count how many golden bags given bags can contain
fn can_hold_golden_bag(bags: &Vec<Bag>, rules: &Rules) -> bool {
    bags.iter()
        .map(|bag| match (&bag.amount, bag.color.as_ref()) {
            (0, _) => false,
            (_, GOLDEN_COLOR) => true,
            _ => rules
                .get(&bag.color)
                .map(|bags| can_hold_golden_bag(bags, rules))
                .unwrap_or(false),
        })
        .find(|&can_hold| can_hold == true)
        .unwrap_or(false)
}

fn count_bags_inside(color: &str, rules: &Rules) -> usize {
    rules
        .get(color)
        .map(|bags| {
            bags.iter()
                .map(|b| {
                    if b.amount == 0 {
                        0
                    } else {
                        b.amount + b.amount * count_bags_inside(&b.color, rules)
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}

fn main() -> std::io::Result<()> {
    let rules: Rules = std::fs::read_to_string("input.txt")?
        .lines()
        .map(parse_rule)
        .collect();

    let first_part = rules
        .iter()
        .map(|(_, bags)| can_hold_golden_bag(bags, &rules))
        .filter(|&c| c == true)
        .count();

    let second_part = count_bags_inside(GOLDEN_COLOR, &rules);

    println!("The answer to the first part is {}", first_part);
    println!("The answer to the second part is {}", second_part);

    Ok(())
}
