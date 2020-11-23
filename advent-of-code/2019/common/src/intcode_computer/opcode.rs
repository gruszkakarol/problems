use super::utils::DigitIter;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(num: i32) -> Self {
        match num {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("Invalid parameter mode!"),
        }
    }
}

#[derive(Debug)]
pub enum Variant {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Break,
}

impl From<(i32, i32)> for Variant {
    fn from(digits: (i32, i32)) -> Self {
        match digits.0 + digits.1 * 10 {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            99 => Self::Break,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    parameter_modes: Vec<ParameterMode>,
    pub variant: Variant,
}

impl Opcode {
    pub fn nth_parameter_mode(&self, n: usize) -> ParameterMode {
        self.parameter_modes
            .get(n)
            .cloned()
            .unwrap_or(ParameterMode::Position)
    }

    pub fn pointer_size(&self) -> usize {
        match self.variant {
            Variant::One => 3,
            Variant::Two => 3,
            Variant::Three => 1,
            Variant::Four => 1,
            Variant::Five => 0,
            Variant::Six => 0,
            Variant::Seven => 3,
            Variant::Eight => 3,
            Variant::Break => 0,
        }
    }
}

impl From<i32> for Opcode {
    fn from(num: i32) -> Self {
        let mut digits = DigitIter(num);
        let variant = Variant::from((
            digits.next().expect("Something went wrong"),
            digits.next().unwrap_or(0),
        ));
        Self {
            variant,
            parameter_modes: digits.map(ParameterMode::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Parameter modes are correctly created from numbers
    fn parameter_mode_from_numbers() {
        let position = ParameterMode::from(0);
        assert_eq!(position, ParameterMode::Position);
        let immediate = ParameterMode::from(1);
        assert_eq!(immediate, ParameterMode::Immediate);
    }

    /// It panics on unknown number
    #[test]
    #[should_panic(expected = "Invalid parameter mode!")]
    fn unknown_parameter_mode() {
        ParameterMode::from(100);
    }
}
