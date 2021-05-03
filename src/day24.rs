use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[usize]) -> usize {
    best(input, 3)
}

#[aoc(day24, part2)]
pub fn part2(input: &[usize]) -> usize {
    best(input, 4)
}

fn best(input: &[usize], size: usize) -> usize {
    let total = input.iter().sum::<usize>();
    let target = total / size;
    let mut best = usize::MAX;

    for k in 3..input.len() {
        for combination in input.iter().combinations(k) {
            let mut product = 1;
            let mut acc = 0;

            for &x in combination {
                acc += x;
                product *= x;

                if acc >= target {
                    break;
                }
            }

            if acc != target {
                continue;
            }

            if product < best {
                best = product;
            }
        }

        if best < usize::MAX {
            return best;
        }
    }

    panic!("Tried every value and found no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day24.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 10439961859);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 72050269);
    }
}
