use aoc_runner_derive::aoc;

// I can't think of a way to estimate a sensible lower bound, so this is a hack after I
// already found the answer by just starting at 1, just to make the tests run in reasonable
// time. It takes ~3.3s each part starting from 1 and just brute forcing in release mode.
//
// It would be faster to just calculate the sum as we go instead of assigning the factors to
// a Vec as we go then multiplying it out afterwards, but that's one for another day
const LOWER: usize = 700_000;

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let target = input.lines().next().unwrap().parse::<usize>().unwrap();

    for house in LOWER.. {
        let factors = factor(house);

        let total = factors.iter().map(|&f| f * 10).sum::<usize>();

        if total >= target {
            return house;
        }
    }

    unreachable!()
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let target = input.lines().next().unwrap().parse::<usize>().unwrap();

    for house in LOWER.. {
        let factors = factor(house);

        let total = factors
            .iter()
            .filter(|&&f| f * 50 >= house)
            .map(|&f| f * 11)
            .sum::<usize>();

        if total >= target {
            return house;
        }
    }

    unreachable!()
}

fn factor(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let bound = (n as f64).sqrt() as usize;

    for i in 1..bound + 1 {
        if n % i == 0 {
            factors.push(i);

            // add the inverse end of this factor if it's not a perfect square (to avoid adding it twice)
            if i * i != n {
                factors.push(n / i);
            }
        }
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day20.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 786240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 831600);
    }
}
