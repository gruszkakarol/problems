use itertools::repeat_n;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Opcode {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

impl Opcode {
    pub fn parse(str: &str) -> Self {
        let mut parts = str.split_whitespace();
        let opcode_type = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        match opcode_type {
            "jmp" => Opcode::Jmp(value),
            "acc" => Opcode::Acc(value),
            "nop" => Opcode::Nop(value),
            _ => unreachable!(),
        }
    }
}

struct VM {
    ip: usize,
    opcodes: Vec<Opcode>,
    acc: i32,
    opcodes_history: HashSet<usize>,
}

#[derive(Debug)]
enum Returned {
    Break(i32),
    Natural(i32),
}

impl VM {
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self {
            opcodes,
            opcodes_history: HashSet::new(),
            ip: 0,
            acc: 0,
        }
    }

    pub fn jmp(&mut self, val: i32) {
        // Danger, danger, danger
        self.ip = (self.ip as i32 + val) as usize;
    }

    pub fn run(&mut self) -> Returned {
        while let Some(opcode) = self.opcodes.get(self.ip) {
            if self.opcodes_history.contains(&self.ip) {
                return Returned::Break(self.acc);
            } else {
                self.opcodes_history.insert(self.ip);
            }

            match opcode {
                Opcode::Jmp(jmp) => {
                    self.jmp(*jmp);
                    continue;
                }
                Opcode::Acc(acc) => self.acc += *acc,
                Opcode::Nop(nop) => {}
            }
            self.ip += 1;
        }
        Returned::Natural(self.acc)
    }
}

fn main() -> std::io::Result<()> {
    let opcodes: Vec<Opcode> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(Opcode::parse)
        .collect();
    let mut VM = VM::new(opcodes.clone());
    let first_part = VM.run();
    println!("The answer to the first part is {:?}", first_part);

    let opcodes_count = opcodes.len();
    for fixed_opcodes in repeat_n(opcodes, opcodes_count)
        .enumerate()
        .filter_map(|(i, mut vec)| {
            let item = vec.get(i).unwrap();
            match item {
                Opcode::Jmp(jmp) => {
                    vec[i] = Opcode::Nop(*jmp);
                    Some(vec)
                }
                Opcode::Nop(nop) => {
                    vec[i] = Opcode::Jmp(*nop);
                    Some(vec)
                }
                Opcode::Acc(_) => None,
            }
        })
    {
        let mut VM = VM::new(fixed_opcodes);
        let result = VM.run();
        match result {
            Returned::Natural(natural) => {
                println!("The answer to the second part is {}", natural);
            }
            Returned::Break(_) => {}
        }
    }
    Ok(())
}
