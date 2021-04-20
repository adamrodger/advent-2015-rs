use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, value},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    first_person: String,
    second_person: String,
    happiness: i32,
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| parse_instruction(l.trim()).unwrap().1)
        .collect::<_>()
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (s, first_person) = alpha1(s)?;
    let (s, _) = tag(" would ")(s)?;
    let (s, multiplier) = alt((value(-1, tag("lose ")), value(1, tag("gain "))))(s)?;
    let (s, value) = map(digit1, |v: &str| v.parse::<i32>().unwrap())(s)?;
    let (s, _) = tag(" happiness units by sitting next to ")(s)?;
    let (s, second_person) = alpha1(s)?;

    let happiness = value * multiplier;

    Ok((
        s,
        Instruction {
            first_person: first_person.to_string(),
            second_person: second_person.to_string(),
            happiness,
        },
    ))
}

#[aoc(day13, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    solve(input, true)
}

#[aoc(day13, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    solve(input, false)
}

fn solve(input: &[Instruction], part1: bool) -> i32 {
    let people = input
        .iter()
        .map(|i| i.first_person.as_str())
        .unique()
        .collect::<Vec<&str>>();

    let lookup = input
        .iter()
        .map(|i| {
            (
                (i.first_person.as_str(), i.second_person.as_str()),
                i.happiness,
            )
        })
        .collect::<HashMap<_, _>>();

    let mut max = i32::MIN;

    for arrangement in people.iter().permutations(people.len()) {
        let chain: i32 = arrangement
            .iter()
            .zip(arrangement.iter().skip(1))
            .map(|(&&a, &&b)| lookup[&(a, b)] + lookup[&(b, a)])
            .sum();

        let total = if part1 {
            // need to create a circle so join last element back to first again
            let first = *arrangement[0];
            let last = *arrangement[arrangement.len() - 1];
            chain + lookup[&(first, last)] + lookup[&(last, first)]
        } else {
            // effectively we are sitting at the "end" of the chain and have 0 cost, so no need to close the circle
            chain
        };

        if total > max {
            max = total;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day13.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 664);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 640);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("Alice would lose 2 happiness units by sitting next to Bob."),
            Ok((
                ".",
                Instruction {
                    first_person: "Alice".to_string(),
                    second_person: "Bob".to_string(),
                    happiness: -2
                }
            ))
        );

        assert_eq!(
            parse_instruction("Carol would gain 42 happiness units by sitting next to David."),
            Ok((
                ".",
                Instruction {
                    first_person: "Carol".to_string(),
                    second_person: "David".to_string(),
                    happiness: 42
                }
            ))
        );
    }
}
