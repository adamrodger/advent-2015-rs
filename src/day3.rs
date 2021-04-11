use std::collections::HashSet;

use aoc_runner_derive::aoc;

use crate::compass::{Direction, Point};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Direction> {
    input.chars().map(|c| Direction::from_char(&c)).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Direction]) -> usize {
    let locations = input
        .iter()
        .scan(Point::default(), |current, direction| {
            let next = current.move_direction(direction);
            *current = next;
            Some(next)
        })
        .collect::<HashSet<_>>();

    locations.len() + 1 // plus initial position
}

#[aoc(day3, part2)]
pub fn part2(input: &[Direction]) -> usize {
    let santa_locations = input
        .iter()
        .step_by(2)
        .scan(Point::default(), |current, direction| {
            let next = current.move_direction(direction);
            *current = next;
            Some(next)
        });

    let robot_locations =
        input
            .iter()
            .skip(1)
            .step_by(2)
            .scan(Point::default(), |current, direction| {
                let next = current.move_direction(direction);
                *current = next;
                Some(next)
            });

    let unique_locations = santa_locations
        .chain(robot_locations)
        .collect::<HashSet<_>>();

    unique_locations.len()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, Direction};

    static INPUT: &str = include_str!("../input/2015/day3.txt");

    #[test]
    fn test_generator() {
        let actual = generator("^v><");
        let expected = &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_real() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 2081);
    }

    #[test]
    fn test_part2_real() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 2341);
    }
}
