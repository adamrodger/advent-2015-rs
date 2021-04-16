use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

pub struct Vertex {
    source: String,
    dest: String,
    weight: usize,
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vertex> {
    input
        .lines()
        .map(|line| parse_vertex(line.trim()).unwrap().1)
        .collect::<Vec<_>>()
}

fn parse_vertex(s: &str) -> IResult<&str, Vertex> {
    let (s, (source, dest)) = separated_pair(alpha1, tag(" to "), alpha1)(s)?;
    let (s, weight) = preceded(tag(" = "), digit1)(s)?;

    Ok((
        s,
        Vertex {
            source: source.to_string(),
            dest: dest.to_string(),
            weight: weight.parse().unwrap(), // safe because nom ensured it was only digits
        },
    ))
}

/// What if we just brute force it instead of implenting a proper TSP algorithm?
#[aoc(day9, part1)]
pub fn part1(input: &[Vertex]) -> usize {
    search(input).0
}

/// I can't believe that worked... The input is very kind because not every city goes to every other
/// Saves implementing Held-Karp though!
#[aoc(day9, part2)]
pub fn part2(input: &[Vertex]) -> usize {
    search(input).1
}

/// Find the min and max path between the given vertices
fn search(input: &[Vertex]) -> (usize, usize) {
    let sources = input.iter().map(|v| v.source.as_ref());
    let dests = input.iter().map(|v| v.dest.as_ref());
    let cities = sources.chain(dests).collect::<HashSet<&str>>();

    let forward = input
        .iter()
        .map(|v| ((v.source.as_ref(), v.dest.as_ref()), v.weight));
    let backward = input
        .iter()
        .map(|v| ((v.dest.as_ref(), v.source.as_ref()), v.weight));
    let edges = forward
        .chain(backward)
        .collect::<HashMap<(&str, &str), usize>>();

    let mut min_weight = usize::MAX;
    let mut max_weight = usize::MIN;

    for p in cities.iter().permutations(cities.len()) {
        let mut weight = 0usize;

        for (&&src, &&dest) in p.iter().zip(p.iter().skip(1)) {
            let key = (src, dest);
            weight += edges.get(&key).unwrap_or(&99999usize);
        }

        if weight < min_weight {
            min_weight = weight;
        }

        if weight > max_weight {
            max_weight = weight;
        }
    }

    (min_weight, max_weight)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static INPUT: &str = include_str!("../input/2015/day9.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 117);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 909);
    }
}
