use crate::{compass::Point, Part};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Point> {
    let input = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
    let mut lights = Vec::new();

    for (y, &line) in input.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                lights.push(Point::new(x as i32, y as i32));
            }
        }
    }

    lights
}

#[aoc(day18, part1)]
pub fn part1(input: &[Point]) -> usize {
    solve(input, &Part::One)
}

#[aoc(day18, part2)]
pub fn part2(input: &[Point]) -> usize {
    solve(input, &Part::Two)
}

fn solve(input: &[Point], part: &Part) -> usize {
    let mut current = input.iter().cloned().collect::<HashSet<Point>>();
    let mut next = HashSet::new();

    for _ in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                let light = Point::new(x, y);
                let lit_neighbours = light
                    .neighbours()
                    .iter()
                    .filter(|&n| n.x >= 0 && n.y >= 0 && n.x < 100 && n.y < 100)
                    .filter(|&n| current.contains(n))
                    .count();

                if (part == &Part::Two && is_corner(&light))
                    || (current.contains(&light) && lit_neighbours >= 2 && lit_neighbours <= 3)
                    || (!current.contains(&light) && lit_neighbours == 3)
                {
                    next.insert(light);
                }
            }
        }

        // swap next state into current state
        current.clear();

        for p in next.drain() {
            current.insert(p);
        }
    }

    current.len()
}

fn is_corner(point: &Point) -> bool {
    let (x, y) = (point.x, point.y);
    (y == 99 || y == 0) && (x == 99 || x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day18.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 1061);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 1006);
    }
}
