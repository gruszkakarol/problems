use std::fs::read_to_string;

pub fn input() -> String {
    read_to_string("input.txt").expect("input.txt not found")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
