use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Computer {
    a: usize,
    b: usize,
    pointer: usize,
    instructions: Vec<Instruction>,
}

#[derive(Copy, Clone, Debug)]
pub enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "a," => Ok(Register::A),
            "b" | "b," => Ok(Register::B),
            error => Err(format!("Invalid register: {}", error)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Half(Register),
    Triple(Register),
    Inc(Register),
    Jump(isize),
    JumpEven(Register, isize),
    JumpOne(Register, isize),
}

impl Computer {
    fn execute(&mut self) {
        while self.pointer < self.instructions.len() {
            match self.instructions[self.pointer] {
                Instruction::Half(r) => match r {
                    Register::A => self.a /= 2,
                    Register::B => self.b /= 2,
                },
                Instruction::Triple(r) => match r {
                    Register::A => self.a *= 3,
                    Register::B => self.b *= 3,
                },
                Instruction::Inc(r) => match r {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                },
                Instruction::Jump(offset) => {
                    self.change_pointer(offset);
                    continue;
                }
                Instruction::JumpEven(r, offset) => {
                    let value = match r {
                        Register::A => self.a,
                        Register::B => self.b,
                    };

                    if value % 2 == 0 {
                        self.change_pointer(offset);
                        continue;
                    }
                }
                Instruction::JumpOne(r, offset) => {
                    let value = match r {
                        Register::A => self.a,
                        Register::B => self.b,
                    };

                    if value == 1 {
                        self.change_pointer(offset);
                        continue;
                    }
                }
            }

            self.pointer += 1;
        }
    }

    fn change_pointer(&mut self, offset: isize) {
        let p = (self.pointer as isize) + offset;

        self.pointer = if p < 0 || (p as usize) >= self.instructions.len() {
            usize::MAX // bit hacky
        } else {
            p as usize
        };
    }
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Computer {
    let instructions = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();

            match &split[..] {
                ["hlf", r] => Instruction::Half(Register::from_str(r).unwrap()),
                ["tpl", r] => Instruction::Triple(Register::from_str(r).unwrap()),
                ["inc", r] => Instruction::Inc(Register::from_str(r).unwrap()),
                ["jmp", offset] => Instruction::Jump(offset.parse::<isize>().unwrap()),
                ["jie", r, offset] => Instruction::JumpEven(
                    Register::from_str(r).unwrap(),
                    offset.parse::<isize>().unwrap(),
                ),
                ["jio", r, offset] => Instruction::JumpOne(
                    Register::from_str(r).unwrap(),
                    offset.parse::<isize>().unwrap(),
                ),
                _ => panic!("Invalid instruction: {}", line),
            }
        })
        .collect();

    Computer {
        a: 0,
        b: 0,
        pointer: 0,
        instructions,
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Computer) -> usize {
    let mut computer = input.clone();
    computer.execute();
    computer.b
}

#[aoc(day23, part2)]
pub fn part2(input: &Computer) -> usize {
    let mut computer = input.clone();
    computer.a = 1;
    computer.execute();
    computer.b
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day23.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 170);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 247);
    }
}
