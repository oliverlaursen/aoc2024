use std::{fmt, fs};

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
pub struct Instruction {
    dir: Direction,
    amount: i64
}

impl Instruction {
    pub fn from_string(val:&str) -> Instruction {
        let (dir_letter, amount) = val.split_at(1);
        let dir = if dir_letter == "L" {
            Direction::Left
        } else {
            Direction::Right
        };
        return Instruction { dir: dir, amount: amount.parse().unwrap()}
    }

}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = match self.dir{
            Direction::Left => "L",
            Direction::Right => "R",
        };
        f.write_str(format!("{}{}", letter, self.amount).as_str())?;
        Ok(())
    }
}

struct Counter {
    zeroes_hit: u32,
    state: i64
}


impl Counter {
    fn new() -> Self {
        Self { zeroes_hit: 0, state: 50 }
    }
    
    pub fn handle_instruction1(&mut self, instruction: &Instruction) {
        let result = match instruction.dir {
            Direction::Left => self.state - instruction.amount,
            Direction::Right => self.state + instruction.amount,
        };
        let result = ((result % 100) + 100) % 100;

        self.state = result;
        if result == 0 {
            self.zeroes_hit += 1;
        }
    }

    pub fn handle_instruction2(&mut self, instruction: &Instruction) {
        let multiplier = match instruction.dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        for _ in 1..instruction.amount + 1{
            let result = (self.state + multiplier) % 100;
            self.state = result;
            if result == 0 {
                self.zeroes_hit += 1;
            }
        }
    }
}

pub fn solve() {
    let input = fs::read_to_string("input/2025/input1.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines()
        .map(|line| Instruction::from_string(line))
        .collect();

    let mut counter = Counter::new();
    instructions
        .iter()
        .for_each(|inst| counter.handle_instruction2(inst));
    println!("{}", counter.zeroes_hit);

}

