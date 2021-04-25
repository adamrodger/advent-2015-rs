use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub struct Input {
    molecule: String,
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    input: String,
    output: String,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" => ").collect::<Vec<_>>();

        match split.len() {
            2 => Ok(Rule {
                input: split[0].into(),
                output: split[1].into(),
            }),
            _ => Err(format!("Unrecognised rule string: {}", s)),
        }
    }
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Input {
    let rules = input
        .lines()
        .map(|l| l.trim())
        .take_while(|&l| !l.is_empty())
        .map(|l| Rule::from_str(l).unwrap())
        .collect();

    Input {
        molecule: input.lines().last().unwrap().trim().into(),
        rules,
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    let mut generated = HashSet::new();

    let mut lookup: HashMap<&str, Vec<&Rule>> = HashMap::new();
    for (key, rules) in &input.rules.iter().group_by(|&r| &r.input) {
        lookup.insert(key, rules.collect::<Vec<_>>());
    }

    // check 1-length rules
    for (i, c) in input.molecule.chars().enumerate() {
        let key: &str = &format!("{}", c);

        if lookup.contains_key(key) {
            let start = &input.molecule[..i];
            let end = &input.molecule[i + 1..];

            for &rule in &lookup[key] {
                let replaced: String = format!("{}{}{}", start, rule.output, end);
                generated.insert(replaced);
            }
        }
    }

    // check 2-length rules
    for (i, (c1, c2)) in input
        .molecule
        .chars()
        .zip(input.molecule.chars().skip(1))
        .enumerate()
    {
        let key: &str = &format!("{}{}", c1, c2);

        if lookup.contains_key(key) {
            let start = &input.molecule[..i];
            let end = &input.molecule[i + 2..];

            for &rule in &lookup[key] {
                let replaced: String = format!("{}{}{}", start, rule.output, end);
                generated.insert(replaced);
            }
        }
    }

    generated.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> usize {
    let mut count = 0;
    let mut molecule = input.molecule.clone();

    // sort from longest to shortest output
    let mut rules = input.rules.iter().collect::<Vec<_>>();
    rules.sort_unstable_by(|a, b| b.output.len().cmp(&a.output.len()));

    while !molecule.trim_matches('e').is_empty() {
        for rule in input.rules.iter() {
            if molecule.contains(&rule.output) {
                molecule = molecule.replacen(&rule.output, &rule.input, 1);
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day19.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 535);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 212);
    }
}
