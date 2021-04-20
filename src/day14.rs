use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Reindeer {
    name: String,
    speed: u32,
    move_duration: u32,
    rest_duration: u32,
}

impl Reindeer {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, name) = alpha1(s)?;
        let (s, _) = tag(" can fly ")(s)?;
        let (s, speed) = map(digit1, |v: &str| v.parse::<u32>().unwrap())(s)?;
        let (s, _) = tag(" km/s for ")(s)?;
        let (s, move_duration) = map(digit1, |v: &str| v.parse::<u32>().unwrap())(s)?;
        let (s, _) = tag(" seconds, but then must rest for ")(s)?;
        let (s, rest_duration) = map(digit1, |v: &str| v.parse::<u32>().unwrap())(s)?;

        Ok((
            s,
            Self {
                name: name.into(),
                speed,
                move_duration,
                rest_duration,
            },
        ))
    }
}

#[derive(Debug)]
pub struct ReindeerState {
    distance: u32,
    moving: bool,
    duration: u32,
    points: u32,
}

impl Default for ReindeerState {
    fn default() -> Self {
        Self {
            distance: 0,
            moving: true,
            duration: 0,
            points: 0,
        }
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Reindeer> {
    input
        .trim()
        .lines()
        .map(|line| Reindeer::parse(line.trim()).unwrap().1)
        .collect::<Vec<Reindeer>>()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Reindeer]) -> u32 {
    solve(input, true)
}

#[aoc(day14, part2)]
pub fn part2(input: &[Reindeer]) -> u32 {
    solve(input, false)
}

fn solve(input: &[Reindeer], part1: bool) -> u32 {
    let mut state_map: HashMap<&Reindeer, ReindeerState> = input
        .iter()
        .map(|r| (r, ReindeerState::default()))
        .collect();

    for _ in 0..2504 {
        for (&reindeer, mut state) in state_map.iter_mut() {
            match (
                state.moving,
                reindeer.move_duration == state.duration,
                reindeer.rest_duration == state.duration,
            ) {
                (true, true, _) => {
                    // moved for full duration, stop
                    state.moving = false;
                    state.duration = 0;
                }
                (true, false, _) => {
                    // keep moving
                    state.distance += reindeer.speed;
                }
                (false, _, true) => {
                    // rested for full duration, start moving
                    state.moving = true;
                    state.duration = 0;
                    state.distance += reindeer.speed;
                }
                (false, _, false) => { /* keep resting */ }
            }

            state.duration += 1;
        }

        if !part1 {
            let max_distance = state_map.values().map(|r| r.distance).max().unwrap();

            for state in state_map.values_mut() {
                if state.distance == max_distance {
                    state.points += 1;
                }
            }
        }
    }

    if part1 {
        state_map.values().map(|r| r.distance).max().unwrap()
    } else {
        state_map.values().map(|r| r.points).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day14.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 2660);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 1256);
    }
}
