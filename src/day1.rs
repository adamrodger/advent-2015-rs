use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    input.chars().fold(0, |total, c| match c {
        '(' => total + 1,
        ')' => total - 1,
        _ => panic!("Unexpected char {}", c),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unexpected char {}", c),
        };

        if floor == -1 {
            return i + 1;
        }
    }

    panic!("Never reached floor -1");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test_case::test_case;

    static INPUT: &str = include_str!("../input/2015/day1.txt");

    #[test_case("(" => 1; "up one level")]
    #[test_case(")" => -1; "down one level")]
    #[test_case("(((((" => 5; "up multiple levels")]
    #[test_case(")))))" => -5; "down multiple levels")]
    #[test_case("()()" => 0; "up down up down")]
    #[test_case("(())" => 0; "up up down down")]
    fn test_part1_examples(x: &str) -> isize {
        part1(x)
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(INPUT), 138);
    }

    #[test_case(")" => 1)]
    #[test_case("(()))" => 5)]
    #[test_case("()()())" => 7)]
    fn test_part2_examples(x: &str) -> usize {
        part2(x)
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(INPUT), 1771);
    }
}
