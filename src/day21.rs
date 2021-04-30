use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    num::ParseIntError,
};

const STARTING_HP: usize = 100;

pub struct Battle {
    boss: Player,
    weapons: [Equipment; 5],
    armour: [Equipment; 6],
    rings: [Equipment; 8],
}

pub struct Player {
    hp: usize,
    damage: usize,
    armour: usize,
}

pub struct Equipment {
    cost: usize,
    damage: usize,
    armour: usize,
}

impl Equipment {
    pub fn new(cost: usize, damage: usize, armour: usize) -> Self {
        Equipment {
            cost,
            damage,
            armour,
        }
    }
}

pub enum Turn {
    Hero,
    Boss,
}

pub enum Outcome {
    Lose(usize),
    Win(usize),
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Result<Battle, ParseIntError> {
    let mut lines = input.trim().lines();

    fn parse_number(line: &str) -> Result<usize, ParseIntError> {
        line.trim().split(": ").nth(1).unwrap().parse()
    }

    let boss = Player {
        hp: parse_number(lines.next().unwrap())?,
        damage: parse_number(lines.next().unwrap())?,
        armour: parse_number(lines.next().unwrap())?,
    };

    let weapons = [
        Equipment::new(8, 4, 0),
        Equipment::new(10, 5, 0),
        Equipment::new(25, 6, 0),
        Equipment::new(40, 7, 0),
        Equipment::new(74, 8, 0),
    ];

    let armour = [
        Equipment::new(0, 0, 0), // no armour
        Equipment::new(13, 0, 1),
        Equipment::new(31, 0, 2),
        Equipment::new(53, 0, 3),
        Equipment::new(75, 0, 4),
        Equipment::new(102, 0, 5),
    ];

    let rings = [
        Equipment::new(0, 0, 0), // no ring left hand
        Equipment::new(0, 0, 0), // no ring right hand
        Equipment::new(25, 1, 0),
        Equipment::new(50, 2, 0),
        Equipment::new(100, 3, 0),
        Equipment::new(20, 0, 1),
        Equipment::new(40, 0, 2),
        Equipment::new(80, 0, 3),
    ];

    Ok(Battle {
        boss,
        weapons,
        armour,
        rings,
    })
}

#[aoc(day21, part1)]
pub fn part1(input: &Battle) -> usize {
    let mut min_cost = usize::MAX;

    let loadouts = generate_loadouts(input);

    for loadout in loadouts.iter() {
        let outcome = fight(&input.boss, loadout);

        if let Outcome::Win(cost) = outcome {
            min_cost = min(min_cost, cost);
        }
    }

    min_cost
}

#[aoc(day21, part2)]
pub fn part2(input: &Battle) -> usize {
    let mut max_cost = usize::MIN;

    let loadouts = generate_loadouts(input);

    for loadout in loadouts.iter() {
        let outcome = fight(&input.boss, loadout);

        if let Outcome::Lose(cost) = outcome {
            max_cost = max(max_cost, cost);
        }
    }

    max_cost
}

/// Generate all the possible different loadouts of equipment that can be
/// equipped by the hero
fn generate_loadouts(input: &Battle) -> Vec<[&Equipment; 4]> {
    let mut loadouts = Vec::new();

    for weapon in input.weapons.iter() {
        for armour in input.armour.iter() {
            for rings in input.rings.iter().combinations(2) {
                let equipment = [weapon, armour, rings[0], rings[1]];
                loadouts.push(equipment);
            }
        }
    }

    loadouts
}

/// Simulate the boss fight using the given equipment loadout
///
/// Returns - Fight outcome with associated equipment cost
fn fight(boss: &Player, equipped: &[&Equipment; 4]) -> Outcome {
    let mut hero_hp = STARTING_HP;
    let mut boss_hp = boss.hp;
    let mut turn = Turn::Hero;

    let mut hero_attack = 0;
    let mut hero_defence = 0;
    let mut equipment_cost = 0;

    for &e in equipped.iter() {
        hero_attack += e.damage;
        hero_defence += e.armour;
        equipment_cost += e.cost;
    }

    while boss_hp > 0 && hero_hp > 0 {
        turn = match turn {
            Turn::Hero => {
                let damage = max(hero_attack.saturating_sub(boss.armour), 1);
                boss_hp = boss_hp.saturating_sub(damage);

                Turn::Boss
            }
            Turn::Boss => {
                let damage = max(boss.damage.saturating_sub(hero_defence), 1);
                hero_hp = hero_hp.saturating_sub(damage);

                Turn::Hero
            }
        }
    }

    if boss_hp == 0 {
        Outcome::Win(equipment_cost)
    } else {
        Outcome::Lose(equipment_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day21.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part1(&input), 121);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part2(&input), 201);
    }
}
