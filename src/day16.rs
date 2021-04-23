use crate::Part;
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Default)]
pub struct Candidate {
    id: u32,
    akitas: Option<u32>,
    goldfish: Option<u32>,
    cars: Option<u32>,
    cats: Option<u32>,
    children: Option<u32>,
    samoyeds: Option<u32>,
    trees: Option<u32>,
    vizslas: Option<u32>,
    pomeranians: Option<u32>,
    perfumes: Option<u32>,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<Candidate> {
    lazy_static! {
        static ref SUE: Regex = Regex::new(r"Sue (\d+)").unwrap();
        static ref ITEM: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
    }

    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            let id = SUE
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            let mut sue = Candidate {
                id,
                ..Default::default()
            };

            for item in ITEM.captures_iter(l) {
                let label = item.get(1).unwrap().as_str();
                let quantity = item.get(2).unwrap().as_str().parse::<u32>().unwrap();

                match label {
                    "akitas" => sue.akitas = Some(quantity),
                    "goldfish" => sue.goldfish = Some(quantity),
                    "cars" => sue.cars = Some(quantity),
                    "cats" => sue.cats = Some(quantity),
                    "children" => sue.children = Some(quantity),
                    "samoyeds" => sue.samoyeds = Some(quantity),
                    "trees" => sue.trees = Some(quantity),
                    "vizslas" => sue.vizslas = Some(quantity),
                    "pomeranians" => sue.pomeranians = Some(quantity),
                    "perfumes" => sue.perfumes = Some(quantity),
                    _ => panic!("Unrecognised label: {}", label),
                }
            }

            sue
        })
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &[Candidate]) -> u32 {
    solve(input, Part::One)
}

#[aoc(day16, part2)]
pub fn part2(input: &[Candidate]) -> u32 {
    solve(input, Part::Two)
}

fn solve(input: &[Candidate], part: Part) -> u32 {
    for sue in input {
        let mut matches = 0;

        if sue.children.unwrap_or_default() == 3 {
            matches += 1;
        }

        if (part == Part::One && sue.cats.unwrap_or_default() == 7)
            || (part == Part::Two && sue.cats.unwrap_or_default() > 7)
        {
            matches += 1;
        }

        if sue.samoyeds.unwrap_or_default() == 2 {
            matches += 1;
        }

        if (part == Part::One && sue.pomeranians.unwrap_or_default() == 3)
            || (part == Part::Two && sue.pomeranians.unwrap_or(4) < 3)
        {
            matches += 1;
        }

        if sue.akitas.unwrap_or(1) == 0 {
            matches += 1;
        }

        if sue.vizslas.unwrap_or(1) == 0 {
            matches += 1;
        }

        if (part == Part::One && sue.goldfish.unwrap_or(6) == 5)
            || (part == Part::Two && sue.goldfish.unwrap_or(6) < 5)
        {
            matches += 1;
        }

        if (part == Part::One && sue.trees.unwrap_or_default() == 3)
            || (part == Part::Two && sue.trees.unwrap_or_default() > 3)
        {
            matches += 1;
        }

        if sue.cars.unwrap_or_default() == 2 {
            matches += 1;
        }

        if sue.perfumes.unwrap_or_default() == 1 {
            matches += 1;
        }

        if matches >= 3 {
            return sue.id;
        }
    }

    panic!("Didn't find a matching Sue!")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day16.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 373);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 260);
    }
}
