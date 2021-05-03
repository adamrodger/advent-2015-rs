use crate::compass::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day25)]
pub fn generator(input: &str) -> Point {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"row (\d+), column (\d+)").unwrap();
    }

    let line = input.lines().next().unwrap();
    let captures = RE.captures(line).unwrap();
    let row = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let column = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    Point::new(column, row)
}

#[aoc(day25, part1)]
pub fn part1(input: &Point) -> u64 {
    let mut row = 1;
    let mut col = 1;
    let mut code: u64 = 20151125;
    let mul: u64 = 252533;
    let div: u64 = 33554393;

    // move through the diagonals until you get to the right column/row
    loop {
        col += 1;
        row -= 1;

        if row == 0 {
            row = col;
            col = 1;
        }

        code = (code * mul) % div;

        if row == input.y && col == input.x {
            return code;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day25.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 19980801);
    }
}
