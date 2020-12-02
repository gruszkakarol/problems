use std::io::Result;

struct Rule {
    min: u8,
    max: u8,
    char: char,
}

fn parse_number(next: &str) -> u8 {
    next.parse::<u8>().expect("Received invalid input")
}

impl Rule {
    pub fn new(input: &str) -> Self {
        // Minimum and maximum occurrence of given character are separated by whitespace.
        // e.g 9-18 k
        let mut str_parts = input.split_whitespace();
        // Policy range is separated by "-".
        let mut ranges_part = str_parts
            .next()
            .expect("Rule must contain policy range.")
            .split("-");

        let min = parse_number(ranges_part.next().expect("Invalid minimal value."));
        let max = parse_number(ranges_part.next().expect("Invalid maximum value."));
        let char = str_parts
            .next()
            .expect("Rule must contain policy char.")
            .chars()
            .next()
            .expect("And it can't be empty.");

        Self { min, max, char }
    }
}

struct Password<'a> {
    rule: Rule,
    value: &'a str,
}

impl<'a> Password<'a> {
    pub fn new(input: &'a str) -> Self {
        // Rule and password are separated by ":".
        // e.g Rule(1-7 q):Value(qqqqxvqrkbqqztlqlzq)
        let mut str_parts = input.split(':');
        let rule = Rule::new(str_parts.next().expect("Rule can't be empty."));
        let value = str_parts.next().expect("Password can't be empty.").trim();
        Self { rule, value }
    }

    pub fn is_valid_v1(&self) -> bool {
        let char_count = self.value.chars().filter(|c| *c == self.rule.char).count();
        char_count >= self.rule.min as usize && char_count <= self.rule.max as usize
    }

    pub fn nth_char(&self, nth: usize) -> char {
        self.value
            .chars()
            .nth(nth)
            .expect("Tried to access non-existent character of the password.")
    }

    pub fn is_valid_v2(&self) -> bool {
        // This subtraction could possibly be dangerous if there were invalid positions, but there are none
        let char_at_min = self.nth_char((self.rule.min - 1) as usize);
        let char_at_max = self.nth_char((self.rule.max - 1) as usize);

        // Only one rule can be truthful at once, hence the XOR
        (char_at_max == self.rule.char ) ^ (char_at_min == self.rule.char)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let passwords: Vec<Password> = input.lines().map(Password::new).collect();

    let valid_passwords_count_v1 = passwords.iter().filter(|p| p.is_valid_v1()).count();
    let valid_passwords_count_v2 = passwords.iter().filter(|p| p.is_valid_v2()).count();
    println!(
        "The answer to the first part is: {}",
        valid_passwords_count_v1
    );

    println!(
        "The answer to the second part is: {}",
        valid_passwords_count_v2
    );

    Ok(())
}
