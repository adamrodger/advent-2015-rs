use aoc_runner_derive::aoc;

// deliberately not done with regex to make it fast
#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|&line| {
            let line = line.trim();
            contains_three_vowels(line) && contains_double(line) && !contains_naughty_string(line)
        })
        .count()
}

// ok fine, we'll use regex then
#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|&line| {
            let line = line.trim();
            contains_repeating_pair(line) && contains_separated_char(line)
        })
        .count()
}

fn contains_three_vowels(line: &str) -> bool {
    line.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
        >= 3
}

fn contains_naughty_string(line: &str) -> bool {
    line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
}

fn contains_double(line: &str) -> bool {
    line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b)
}

fn contains_repeating_pair(line: &str) -> bool {
    if line.len() < 4 {
        return false;
    }

    for i in 0..(line.len() - 3) {
        let head = &line[i..i + 2];
        let tail = &line[i + 2..];

        if tail.contains(head) {
            return true;
        }
    }

    false
}

fn contains_separated_char(line: &str) -> bool {
    line.chars().zip(line.chars().skip(2)).any(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static INPUT: &str = include_str!("../input/2015/day5.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 255);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 55);
    }
}
