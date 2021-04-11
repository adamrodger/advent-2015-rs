use aoc_runner_derive::aoc;
use rayon::prelude::*;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    check_hash(input, 5, 300_000) // 254575
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    check_hash(input, 6, 1_100_100) // 1038736
}

fn check_hash(input: &str, leading_zeroes: usize, search_size: usize) -> usize {
    let input = input.trim();
    let range = (1..search_size).collect::<Vec<_>>();
    let expected = std::iter::repeat('0')
        .take(leading_zeroes)
        .collect::<String>();

    *range
        .par_iter()
        .find_any(|&i| {
            let check = format!("{}{}", input, i);
            let hash = md5::compute(check);
            let formatted_hash = format!("{:x}", hash);
            formatted_hash[0..leading_zeroes] == expected
        })
        .unwrap_or_else(|| {
            panic!(
                "Couldn't find a hash starting with {} or more zeroes",
                leading_zeroes
            )
        })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static INPUT: &str = include_str!("../input/2015/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 254575);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1038736);
    }
}
