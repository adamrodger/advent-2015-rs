use aoc_runner_derive::{aoc, aoc_generator};
use std::{cmp::max, num::ParseIntError};

type Mana = usize;
type GameResult = Result<GameState, Outcome>;

pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

impl Spell {
    fn cost(&self) -> Mana {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

pub enum Outcome {
    Lose,
    Win(Mana),
}

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    hero_hp: usize,
    hero_mana: Mana,
    total_mana: Mana,
    boss_hp: usize,
    boss_attack: usize,
    poison_effect: u8,
    shield_effect: u8,
    recharge_effect: u8,
    hard_mode: bool,
}

impl GameState {
    /// Play the game to find the cheapest winning scenario
    fn play(&self) -> Mana {
        let results = SPELLS
            .iter()
            .filter(|&spell| spell.cost() <= self.hero_mana)
            .map(|spell| match self.next_round(spell) {
                Ok(next) => next.play(),
                Err(Outcome::Lose) => Mana::MAX,
                Err(Outcome::Win(cost)) => cost,
            });

        results.min().unwrap_or(Mana::MAX)
    }

    /// Play an entire round, returning early if the game ends
    fn next_round(&self, spell: &Spell) -> GameResult {
        self.hard_mode_effect()?
            .apply_effects()?
            .cast(spell)?
            .apply_effects()?
            .boss_turn()
    }

    /// Apply any active effects and reduce any active timers
    fn apply_effects(&self) -> GameResult {
        let game = *self;

        let game = if game.poison_effect > 0 {
            GameState {
                boss_hp: game.boss_hp.saturating_sub(3),
                poison_effect: game.poison_effect - 1,
                ..game
            }
        } else {
            game
        };

        let game = if game.shield_effect > 0 {
            GameState {
                shield_effect: game.shield_effect - 1,
                ..game
            }
        } else {
            game
        };

        let game = if game.recharge_effect > 0 {
            GameState {
                hero_mana: game.hero_mana + 101,
                recharge_effect: game.recharge_effect - 1,
                ..game
            }
        } else {
            game
        };

        game.outcome()
    }

    /// Check if hard mode applies
    fn hard_mode_effect(&self) -> GameResult {
        let next = if self.hard_mode {
            GameState {
                hero_hp: self.hero_hp.saturating_sub(1),
                ..*self
            }
        } else {
            *self
        };

        next.outcome()
    }

    /// Cast the given spell
    fn cast(&self, spell: &Spell) -> GameResult {
        let cost = spell.cost();

        let game = GameState {
            hero_mana: self.hero_mana.saturating_sub(cost),
            total_mana: self.total_mana + cost,
            ..*self
        };

        let game = match *spell {
            Spell::MagicMissile => GameState {
                boss_hp: self.boss_hp.saturating_sub(4),
                ..game
            },
            Spell::Drain => GameState {
                boss_hp: self.boss_hp.saturating_sub(2),
                hero_mana: self.hero_mana + 2,
                ..game
            },
            Spell::Shield => GameState {
                shield_effect: 6,
                ..game
            },
            Spell::Poison => GameState {
                poison_effect: 6,
                ..game
            },
            Spell::Recharge => GameState {
                recharge_effect: 5,
                ..game
            },
        };

        game.outcome()
    }

    /// Perform the boss's turn
    fn boss_turn(&self) -> GameResult {
        let armour = if self.shield_effect > 0 { 7 } else { 0 };
        let damage = max(self.boss_attack.saturating_sub(armour), 1);

        let game = GameState {
            hero_hp: self.hero_hp.saturating_sub(damage),
            ..*self
        };

        game.outcome()
    }

    /// Check the outcome of the current game state
    fn outcome(self) -> GameResult {
        if self.boss_hp == 0 {
            Err(Outcome::Win(self.total_mana))
        } else if self.hero_hp == 0 {
            Err(Outcome::Lose)
        } else {
            // continue playing
            Ok(self)
        }
    }
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Result<GameState, ParseIntError> {
    let mut lines = input.trim().lines();

    fn parse_number(line: &str) -> Result<usize, ParseIntError> {
        line.trim().split(": ").nth(1).unwrap().parse()
    }

    Ok(GameState {
        boss_hp: parse_number(lines.next().unwrap())?,
        boss_attack: parse_number(lines.next().unwrap())?,
        hero_hp: 50,
        hero_mana: 500,
        total_mana: 0,
        poison_effect: 0,
        shield_effect: 0,
        recharge_effect: 0,
        hard_mode: false,
    })
}

#[aoc(day22, part1)]
pub fn part1(input: &GameState) -> Mana {
    input.play()
}

#[aoc(day22, part2)]
pub fn part2(input: &GameState) -> Mana {
    let game = GameState {
        hard_mode: true,
        ..*input
    };

    game.play()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day22.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part1(&input), 1824);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part2(&input), 1937);
    }
}
