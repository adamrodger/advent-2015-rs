use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let mut password = increment(input.trim());

    while !is_valid(&password) {
        password = increment(&password);
    }

    password
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    part1(&part1(input))
}

fn increment(password: &str) -> String {
    let mut output: Vec<u8> = password.into();

    for (i, &c) in password.as_bytes().iter().enumerate().rev() {
        if c == b'z' {
            output[i] = b'a';
            continue;
        }

        output[i] = c + 1;
        break;
    }

    String::from_utf8(output).expect("Generated invalid password")
}

fn is_valid(input: &str) -> bool {
    !contains_invalid_chars(input) && contains_double_pair(input) && contains_run(input)
}

fn contains_invalid_chars(input: &str) -> bool {
    input.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn contains_double_pair(input: &str) -> bool {
    if input.len() < 4 {
        return false;
    }

    let pairs = input
        .chars()
        .zip(input.chars().skip(1))
        .filter(|(a, b)| a == b)
        .collect::<HashSet<_>>();

    pairs.len() >= 2
}

fn contains_run(input: &str) -> bool {
    if input.len() < 3 {
        return false;
    }

    input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .any(|((a, b), c)| {
            (b as u8).saturating_sub(a as u8) == 1 && (c as u8).saturating_sub(b as u8) == 1
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static INPUT: &str = include_str!("../input/2015/day11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "cqjxxyzz");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "cqkaabcc");
    }

    #[test_case("xx" => "xy"; "two chars")]
    #[test_case("xz" => "ya"; "roll over")]
    #[test_case("zz" => "aa"; "overflow")]
    fn test_increment(input: &str) -> String {
        increment(input)
    }

    #[test_case("hijklmmn" => false; "invalid chars")]
    #[test_case("abbceffg" => false; "no run")]
    #[test_case("abbcegjk" => false; "no double pair")]
    #[test_case("abcdffaa" => true; "sample valid")]
    fn test_is_valid(input: &str) -> bool {
        is_valid(input)
    }

    #[test_case("i" => true)]
    #[test_case("o" => true)]
    #[test_case("l" => true)]
    #[test_case("abcdefghjkmnpqrstuvwxyz" => false)]
    fn test_contains_invalid_chars(input: &str) -> bool {
        contains_invalid_chars(input)
    }

    #[test_case("aabccd" => true; "two pairs")]
    #[test_case("aabcde" => false; "only one pair")]
    #[test_case("abcdef" => false; "no pairs")]
    fn test_contains_double_pair(input: &str) -> bool {
        contains_double_pair(input)
    }

    #[test_case("abcde" => true)]
    #[test_case("abdeg" => false)]
    fn test_contains_run(input: &str) -> bool {
        contains_run(input)
    }
}
