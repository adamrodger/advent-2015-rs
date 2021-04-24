use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .sorted_unstable()
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[u32]) -> usize {
    let mut acc = 0;
    let mut max_sum = 0;

    for &x in input.iter() {
        acc += x;
        max_sum += 1;

        if acc >= 150 {
            break;
        }
    }

    acc = 0;
    let mut min_sum = 0;

    for &x in input.iter().rev() {
        acc += x;
        min_sum += 1;

        if acc >= 150 {
            break;
        }
    }

    (min_sum..max_sum + 1)
        .map(|k| {
            input
                .iter()
                .combinations(k)
                .map(|c| c.into_iter().sum::<u32>())
                .filter(|&sum| sum == 150)
                .count()
        })
        .sum()
}

#[aoc(day17, part2)]
pub fn part2(input: &[u32]) -> usize {
    let mut acc = 0;
    let mut min_sum = 0;

    for &x in input.iter().rev() {
        acc += x;
        min_sum += 1;

        if acc >= 150 {
            break;
        }
    }

    input
        .iter()
        .combinations(min_sum)
        .map(|c| c.into_iter().sum::<u32>())
        .filter(|&sum| sum == 150)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day17.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 654);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 57);
    }
}
