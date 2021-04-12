use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Operation {
    On,
    Off,
    Toggle,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Operation::On),
            "off" => Ok(Operation::Off),
            "toggle" => Ok(Operation::Toggle),
            _ => Err(format!("Unrecognised operation: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    start: (usize, usize),
    stop: (usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        let operation = Operation::from_str(parts[parts.len() - 4]).unwrap();

        let start = parts[parts.len() - 3].split(',').collect::<Vec<_>>();
        let start = (start[0].parse().unwrap(), start[1].parse().unwrap());

        let stop = parts[parts.len() - 1].split(',').collect::<Vec<_>>();
        let stop = (stop[0].parse().unwrap(), stop[1].parse().unwrap());

        Ok(Instruction {
            operation,
            start,
            stop,
        })
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut grid = [[false; 1000]; 1000];

    for instruction in input {
        #[allow(clippy::needless_range_loop)]
        for x in instruction.start.0..instruction.stop.0 + 1 {
            for y in instruction.start.1..instruction.stop.1 + 1 {
                match instruction.operation {
                    Operation::On => grid[x][y] = true,
                    Operation::Off => grid[x][y] = false,
                    Operation::Toggle => grid[x][y] = !grid[x][y],
                };
            }
        }
    }

    grid.iter().flatten().filter(|&&v| v).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Instruction]) -> usize {
    let mut grid = [[0u8; 1000]; 1000];

    for instruction in input {
        #[allow(clippy::needless_range_loop)]
        for x in instruction.start.0..instruction.stop.0 + 1 {
            for y in instruction.start.1..instruction.stop.1 + 1 {
                match instruction.operation {
                    Operation::On => grid[x][y] += 1,
                    Operation::Off => grid[x][y] = grid[x][y].saturating_sub(1),
                    Operation::Toggle => grid[x][y] += 2,
                };
            }
        }
    }

    grid.iter().flatten().map(|&v| v as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static INPUT: &str = include_str!("../input/2015/day6.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 377891);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 14110788);
    }
}
