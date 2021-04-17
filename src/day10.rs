use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 50)
}

fn solve(input: &str, i: i32) -> usize {
    let mut current = input.trim().to_string();
    let mut temp = String::with_capacity(3_600_000);

    for _ in 0..i {
        let mut chars = current.chars().peekable();
        let mut count = 1u32;

        while let Some(c) = chars.next() {
            if chars.peek().is_none() || c != *chars.peek().unwrap() {
                temp.push(std::char::from_digit(count, 10).expect("Count > 9 is not supported"));
                temp.push(c);
                count = 1;
            } else {
                count += 1;
            }
        }

        current = temp.clone();
        temp.clear();
    }

    current.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 252594);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3579328);
    }
}
