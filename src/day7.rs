use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, str::FromStr};

/// Source for a wire value
#[derive(Debug)]
pub enum Source {
    /// Source is the value of another wire
    Wire(String),

    /// Source is a raw numeric value
    Raw(u16),
}

impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s
            .chars()
            .next()
            .ok_or("Empty input supplied to Source::for_str")?;

        Ok(if c.is_ascii_digit() {
            let value = s.parse::<u16>().map_err(|e| e.to_string())?;
            Source::Raw(value)
        } else {
            Source::Wire(s.to_owned())
        })
    }
}

/// Operation to perform on a wire as part of an [Instruction]
#[derive(Debug)]
pub enum Operation {
    /// Set the destination wire to the source value
    Set(Source),

    /// Set the destination wire to the negation of the source value
    Not(Source),

    /// Set the destination wire to the bitwise AND of two source wires
    And(Source, Source),

    /// Set the destination wire to the bitwise OR of two source wires
    Or(Source, Source),

    /// Set the destination wire to the value of the first param bitshifted to
    /// the left by the value of the second param
    LShift(Source, Source),

    /// Set the destination wire to the value of the first param bitshifted to
    /// the right by the value of the second param
    RShift(Source, Source),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        Ok(match parts.len() {
            1 => {
                let a = Source::from_str(parts[0])?;
                Operation::Set(a)
            }
            2 => {
                let a = Source::from_str(parts[1])?;
                Operation::Not(a)
            }
            3 => {
                let a = Source::from_str(parts[0])?;
                let b = Source::from_str(parts[2])?;

                match parts[1] {
                    "AND" => Operation::And(a, b),
                    "OR" => Operation::Or(a, b),
                    "LSHIFT" => Operation::LShift(a, b),
                    "RSHIFT" => Operation::RShift(a, b),
                    _ => return Err(format!("Unsupported operation: {}", s)),
                }
            }
            _ => return Err(format!("Unsupported operation: {}", s)),
        })
    }
}

/// Instruction to perform which generates the value of the destination wire
#[derive(Debug)]
pub struct Instruction {
    /// Operation to perform
    operation: Operation,

    /// Destination wire ID
    dest: String,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<_>>();

        let operation = Operation::from_str(parts[0])?;
        let dest = parts[1].to_owned();

        Ok(Instruction { operation, dest })
    }
}

/// Parse the input file to a set of [instructions](Instruction) for generating wire values
#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

/// Get the value of wire `a` which is generated by the given [instructions](Instruction)
#[aoc(day7, part1)]
pub fn part1(input: &[Instruction]) -> u16 {
    execute(input, false)
}

/// Get the value of wire `a` which is generated by the given [instructions](Instruction)
/// after replacing the input for wire `b` with the value returned from part 1
#[aoc(day7, part2)]
pub fn part2(input: &[Instruction]) -> u16 {
    execute(input, true)
}

/// Execute the given instructions
///
/// - *input* - Input instructions
/// - *part_2* - indicates whether part 2 is running or not
fn execute(input: &[Instruction], part_2: bool) -> u16 {
    let mut wires = HashMap::with_capacity(input.len());
    let mut instructions = HashMap::with_capacity(input.len());

    for i in input.iter() {
        instructions.insert(i.dest.as_ref(), i);
    }

    let b_override = Instruction {
        operation: Operation::Set(Source::Raw(16076u16)),
        dest: instructions["b"].dest.clone(),
    };

    if part_2 {
        // override b with the value from part 1
        instructions.insert("b", &b_override);
    }

    get_value_by_wire("a", &mut wires, &instructions)
}

/// Get the value of the given wire. This will run recursively backwards from the desired wire until
/// all input wire values are known.
///
/// - *id* - ID of the wire to get
/// - *wires* - Lookup of currently known wire values by ID
/// - *instructions* - Lookup of instructions for generating a wire value by destination ID
fn get_value_by_wire(
    id: &str,
    wires: &mut HashMap<String, u16>,
    instructions: &HashMap<&str, &Instruction>,
) -> u16 {
    if wires.contains_key(id) {
        return wires[id];
    }

    let instruction = instructions[id];

    let value = match &instruction.operation {
        Operation::Set(a) => get_value_by_source(a, wires, instructions),
        Operation::And(a, b) => {
            let a = get_value_by_source(a, wires, instructions);
            let b = get_value_by_source(b, wires, instructions);
            a & b
        }
        Operation::Or(a, b) => {
            let a = get_value_by_source(a, wires, instructions);
            let b = get_value_by_source(b, wires, instructions);
            a | b
        }
        Operation::Not(a) => {
            let a = get_value_by_source(a, wires, instructions);
            !a
        }
        Operation::LShift(a, b) => {
            let a = get_value_by_source(a, wires, instructions);
            let b = get_value_by_source(b, wires, instructions);
            a << b
        }
        Operation::RShift(a, b) => {
            let a = get_value_by_source(a, wires, instructions);
            let b = get_value_by_source(b, wires, instructions);
            a >> b
        }
    };

    wires.insert(id.to_string(), value);
    value
}

/// Dereference a source to get a concrete value. Raw sources will yield the value immediately
/// whereas Wire source will lookup the value of the referenced input wire.
///
/// - *source* - Source to dereference
/// - *wires* - Lookup of currently known wire values by ID
/// - *instructions* - Lookup of instructions for generating a wire value by destination ID
fn get_value_by_source(
    source: &Source,
    wires: &mut HashMap<String, u16>,
    instructions: &HashMap<&str, &Instruction>,
) -> u16 {
    match source {
        Source::Wire(id) => get_value_by_wire(id, wires, instructions),
        Source::Raw(value) => *value,
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static INPUT: &str = include_str!("../input/2015/day7.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 16076);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 2797);
    }
}