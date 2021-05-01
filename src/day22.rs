use std::{cmp::max, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Boss {
    hp: usize,
    damage: usize,
}

pub enum Outcome {
    Lose,
    Win(usize),
}

#[derive(Copy, Clone, Debug)]
struct GameState {
    hero_hp: usize,
    mana: usize,
    boss_hp: usize,
    boss_attack: usize,
    shield_effect: usize,
    poison_effect: usize,
    recharge_effect: usize,
    total_cost: usize,
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Result<Boss, ParseIntError> {
    let mut lines = input.trim().lines();

    fn parse_number(line: &str) -> Result<usize, ParseIntError> {
        line.trim().split(": ").nth(1).unwrap().parse()
    }

    let boss = Boss {
        hp: parse_number(lines.next().unwrap())?,
        damage: parse_number(lines.next().unwrap())?,
    };

    Ok(boss)
}

#[aoc(day22, part1)]
pub fn part1(input: &Boss) -> usize {
    let state = GameState {
        mana: 500,
        hero_hp: 50,
        boss_hp: input.hp,
        boss_attack: input.damage,
        poison_effect: 0,
        shield_effect: 0,
        recharge_effect: 0,
        total_cost: 0
    };

    match play(&state) {
        Outcome::Lose => panic!("No winning move"),
        Outcome::Win(cost) => cost
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &Boss) -> usize {
    todo!()
}

fn play(state: &GameState) -> Outcome {
    if state.hero_hp == 0 || state.mana == 0 {
        return Outcome::Lose;
    }

    if state.boss_hp == 0 {
        return Outcome::Win(state.total_cost);
    }

    let mut possible = Vec::new();
    let mut next = state.clone();
    let mut armour = 0;

    // apply effects
    if next.recharge_effect > 0 {
        next.mana += 101;
        next.recharge_effect -= 1;
    }

    if next.poison_effect > 0 {
        next.boss_hp = next.boss_hp.saturating_sub(3);
        next.poison_effect -= 1;

        if next.boss_hp == 0 {
            return Outcome::Win(state.total_cost);
        }
    }

    if next.shield_effect > 0 {
        armour = 7;
        next.shield_effect -= 1;
    }

    // branch along each possible spell and return the min path cost
    if state.mana >= 229 && state.recharge_effect == 0 {
        next.mana -= 229;
        next.total_cost += 229;
        next.recharge_effect = 5;

        if let Outcome::Win(cost) = play(&next) {
            possible.push(cost);
        }
    }

    if state.mana >= 173 && state.poison_effect == 0 {
        next.mana -= 173;
        next.total_cost += 173;
        next.poison_effect = 6;

        if let Outcome::Win(cost) = play(&next) {
            possible.push(cost);
        }
    }

    if state.mana >= 113 && state.shield_effect == 0 {
        next.mana -= 113;
        next.total_cost += 113;
        next.shield_effect = 6;

        if let Outcome::Win(cost) = play(&next) {
            possible.push(cost);
        }
    }

    if state.mana >= 73 {
        next.mana -= 73;
        next.total_cost += 73;
        next.boss_hp = next.boss_hp.saturating_sub(2);
        next.hero_hp += 2;

        if let Outcome::Win(cost) = play(&next) {
            possible.push(cost);
        }
    }

    if state.mana >= 53 {
        next.mana -= 53;
        next.total_cost += 53;
        next.boss_hp = next.boss_hp.saturating_sub(4);

        if let Outcome::Win(cost) = play(&next) {
            possible.push(cost);
        }
    }

    // boss turn
    let boss_attack = max(state.boss_attack.saturating_sub(armour), 1);
    next.hero_hp -= boss_attack;

    if next.hero_hp == 0 {
        return Outcome::Lose;
    }

    Outcome::Win(*possible.iter().min().expect("No possible moves"))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day22.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part2(&input), 0);
    }
}
