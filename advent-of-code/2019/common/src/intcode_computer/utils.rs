use super::Program;
use std::fmt::Debug;
use std::fs::read_to_string;

pub fn get_input() -> Program {
    read_to_string("input.txt")
        .expect("You provided invalid program file!")
        .split(",")
        .map(|num| {
            num.parse::<i32>()
                .expect("Unexpected string that is not a number")
        })
        .collect::<Program>()
}
#[derive(Debug, PartialEq)]
pub struct DigitIter(pub i32);

impl Iterator for DigitIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % 10;
            self.0 /= 10;
            Some(ret)
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Either<A: Debug, B: Debug> {
    Left(A),
    Right(B),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DigitIter correctly parses number into digits in reversed order
    #[test]
    fn correctly_parses_digits() {
        let mut digits = DigitIter(1234);
        assert_eq!(&digits.next(), &Some(4));
        assert_eq!(&digits.next(), &Some(3));
        assert_eq!(&digits.next(), &Some(2));
        assert_eq!(&digits.next(), &Some(1));
    }
}
