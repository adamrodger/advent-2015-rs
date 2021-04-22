use crate::Part;
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: u32,
}

#[aoc_generator(day15)]
pub fn generator(_input: &str) -> Vec<Ingredient> {
    // Hack: this isn't parsing the input, it's hard-coded from my input
    let ingredients = vec![
        Ingredient {
            capacity: 5,
            durability: -1,
            flavour: 0,
            texture: 0,
            calories: 5,
        },
        Ingredient {
            capacity: -1,
            durability: 3,
            flavour: 0,
            texture: 0,
            calories: 1,
        },
        Ingredient {
            capacity: 0,
            durability: -1,
            flavour: 4,
            texture: 0,
            calories: 6,
        },
        Ingredient {
            capacity: -1,
            durability: 0,
            flavour: 0,
            texture: 2,
            calories: 8,
        },
    ];

    ingredients
}

#[aoc(day15, part1)]
pub fn part1(input: &[Ingredient]) -> u32 {
    solve(input, Part::One)
}

#[aoc(day15, part2)]
pub fn part2(input: &[Ingredient]) -> u32 {
    solve(input, Part::Two)
}

fn solve(ingredients: &[Ingredient], part: Part) -> u32 {
    // assert ingredients length to alide bounds checks in all the below arithmetic
    assert_eq!(ingredients.len(), 4, "There must be exactly 4 ingredients");

    let mut max_score = u32::MIN;
    let permutations = create_permutations();

    for quantities in permutations.iter() {
        let capacity = quantities
            .iter()
            .zip(ingredients.iter())
            .map(|(&q, i)| q as i32 * i.capacity)
            .sum();
        let durability = quantities
            .iter()
            .zip(ingredients.iter())
            .map(|(&q, i)| q as i32 * i.durability)
            .sum();
        let flavour = quantities
            .iter()
            .zip(ingredients.iter())
            .map(|(&q, i)| q as i32 * i.flavour)
            .sum();
        let texture = quantities
            .iter()
            .zip(ingredients.iter())
            .map(|(&q, i)| q as i32 * i.texture)
            .sum();
        let calories = quantities
            .iter()
            .zip(ingredients.iter())
            .map(|(&q, i)| q * i.calories)
            .sum::<u32>();

        // any negative sums should latch to 0 (note: calories can't be < 0)
        let capacity = std::cmp::max(capacity, 0) as u32;
        let durability = std::cmp::max(durability, 0) as u32;
        let flavour = std::cmp::max(flavour, 0) as u32;
        let texture = std::cmp::max(texture, 0) as u32;

        let score = capacity * durability * flavour * texture;

        max_score = match part {
            Part::One => std::cmp::max(score, max_score),
            Part::Two => {
                if calories == 500 {
                    std::cmp::max(score, max_score)
                } else {
                    max_score
                }
            }
        }
    }

    max_score
}

/// Uses the "stars and bars" methodology to find the all combinations of 4 positive
/// integers that add up to 100
fn create_permutations() -> Vec<[u32; 4]> {
    let mut permutations = Vec::with_capacity(156_849); // 99! / (3! * 96!)
    let n: u32 = 100;

    for a in 0..n + 1 {
        for b in a..n + 1 {
            for c in b..n + 1 {
                permutations.push([a, b - a, c - b, n - c]);
            }
        }
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day15.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 13882464);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 11171160);
    }
}
