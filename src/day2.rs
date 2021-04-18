use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::min;

pub struct Present {
    length: usize,
    width: usize,
    height: usize,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|line| {
            let dimensions = line
                .trim()
                .split('x')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>();
            debug_assert_eq!(dimensions.len(), 3);

            Present {
                length: dimensions[0],
                width: dimensions[1],
                height: dimensions[2],
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Present]) -> usize {
    input
        .iter()
        .map(|present| {
            let front = present.width * present.height;
            let top = present.width * present.length;
            let side = present.length * present.height;
            let smallest = min(min(front, top), side);

            2 * front + 2 * top + 2 * side + smallest
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Present]) -> usize {
    input
        .iter()
        .map(|present| {
            let front = present.width + present.height;
            let top = present.width + present.length;
            let side = present.length + present.height;
            let smallest = min(min(front, top), side);

            let bow = present.length * present.width * present.height;

            smallest * 2 + bow
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static INPUT: &str = include_str!("../input/2015/day2.txt");

    #[test]
    fn test_part1_real() {
        let presents = generator(INPUT);
        assert_eq!(part1(&presents), 1586300);
    }

    #[test]
    fn test_part2_real() {
        let presents = generator(INPUT);
        assert_eq!(part2(&presents), 3737498);
    }
}
