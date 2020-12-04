use std::collections::HashMap;

struct Passport<'a> {
    dict: Dictionary<'a>,
}

struct Dictionary<'a>(HashMap<&'a str, &'a str>);

impl<'a> Dictionary<'a> {
    pub fn has_keys(&self, keys: &[&str]) -> bool {
        keys.iter().all(|k| self.0.contains_key(k))
    }

    pub fn get_key(&self, key: &str) -> &str {
        self.0.get(key).unwrap()
    }
}

fn min_max(val: usize, min: usize, max: usize) -> bool {
    val >= min && val <= max
}

fn height(val: &str) -> bool {
    if val.contains("in") || val.contains("cm") {
        let number = val[0..val.len() - 2].parse::<usize>().unwrap();

        match val.contains("in") {
            true => number >= 59 && number <= 76,
            false => number >= 150 && number <= 193,
        }
    } else {
        false
    }
}

fn hcl(val: &str) -> bool {
    let mut chars = val.chars();
    if let Some(c @ '#') = chars.next() {
        let z = chars.all(|c| match c {
            '0'..='9' => true,
            'a'..='f' => true,
            _ => false,
        });
        z
    } else {
        false
    }
}

fn ecl(val: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
}

fn pid(val: &str) -> bool {
    val.chars().all(char::is_alphanumeric) && val.len() == 9
}

impl<'a> Passport<'a> {
    pub fn new(content: &'a str) -> Self {
        let dict = Dictionary(
            content
                .split(&[' ', '\n'][..])
                .map(|str| {
                    let mut keys = str.split(":");
                    (keys.next().unwrap(), keys.next().unwrap())
                })
                .collect(),
        );
        Passport { dict }
    }

    pub fn get_str(&self, key: &str) -> &str {
        self.dict.get_key(key)
    }

    pub fn get_num(&self, key: &str) -> usize {
        self.get_str(key).parse::<usize>().unwrap()
    }

    pub fn is_valid_v1(&self) -> bool {
        self.dict
            .has_keys(&["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
    }

    pub fn is_valid_v2(&self) -> bool {
        if self.is_valid_v1() {
            [
                min_max(self.get_num("byr"), 1920, 2002),
                min_max(self.get_num("iyr"), 2010, 2020),
                min_max(self.get_num("eyr"), 2020, 2030),
                height(self.get_str("hgt")),
                hcl(self.get_str("hcl")),
                ecl(self.get_str("ecl")),
                pid(self.get_str("pid")),
            ]
            .iter()
            .all(|b| *b == true)
        } else {
            false
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let valid_passports_count_v1 = input
        .split("\n\n")
        .map(Passport::new)
        .filter(|p| p.is_valid_v1())
        .count();

    println!(
        "The answer to the first part is: {}",
        valid_passports_count_v1
    );

    let valid_passports_count_v2 = input
        .split("\n\n")
        .map(Passport::new)
        .filter(|p| p.is_valid_v2())
        .count();

    println!(
        "The answer to the second part is: {}",
        valid_passports_count_v2
    );

    Ok(())
}
